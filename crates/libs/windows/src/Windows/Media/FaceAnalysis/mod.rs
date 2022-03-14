#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Media_FaceAnalysis\"`*"]
#[repr(transparent)]
pub struct DetectedFace(::windows::core::IUnknown);
impl DetectedFace {
    #[doc = "*Required features: `\"Media_FaceAnalysis\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn FaceBox(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapBounds> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::Imaging::BitmapBounds = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FaceBox)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapBounds>(result__)
        }
    }
}
impl ::core::clone::Clone for DetectedFace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DetectedFace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DetectedFace {}
impl ::core::fmt::Debug for DetectedFace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DetectedFace").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DetectedFace {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.FaceAnalysis.DetectedFace;{8200d454-66bc-34df-9410-e89400195414})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DetectedFace {
    type Vtable = IDetectedFace_Vtbl;
    const IID: ::windows::core::GUID = <IDetectedFace as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DetectedFace {
    const NAME: &'static str = "Windows.Media.FaceAnalysis.DetectedFace";
}
impl ::core::convert::From<DetectedFace> for ::windows::core::IUnknown {
    fn from(value: DetectedFace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DetectedFace> for ::windows::core::IUnknown {
    fn from(value: &DetectedFace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DetectedFace {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DetectedFace {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DetectedFace> for ::windows::core::IInspectable {
    fn from(value: DetectedFace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DetectedFace> for ::windows::core::IInspectable {
    fn from(value: &DetectedFace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DetectedFace {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DetectedFace {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DetectedFace {}
unsafe impl ::core::marker::Sync for DetectedFace {}
#[doc = "*Required features: `\"Media_FaceAnalysis\"`*"]
#[repr(transparent)]
pub struct FaceDetector(::windows::core::IUnknown);
impl FaceDetector {
    #[doc = "*Required features: `\"Media_FaceAnalysis\"`, `\"Foundation_Collections\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub fn DetectFacesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::SoftwareBitmap>>(&self, image: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DetectFacesAsync)(::core::mem::transmute_copy(this), image.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_FaceAnalysis\"`, `\"Foundation_Collections\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub fn DetectFacesWithSearchAreaAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::SoftwareBitmap>, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapBounds>>(&self, image: Param0, searcharea: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DetectFacesWithSearchAreaAsync)(::core::mem::transmute_copy(this), image.into_param().abi(), searcharea.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_FaceAnalysis\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn MinDetectableFaceSize(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapSize> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::Imaging::BitmapSize = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MinDetectableFaceSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapSize>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_FaceAnalysis\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetMinDetectableFaceSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapSize>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMinDetectableFaceSize)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_FaceAnalysis\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn MaxDetectableFaceSize(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapSize> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::Imaging::BitmapSize = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxDetectableFaceSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapSize>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_FaceAnalysis\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetMaxDetectableFaceSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapSize>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxDetectableFaceSize)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_FaceAnalysis\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FaceDetector>> {
        Self::IFaceDetectorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<FaceDetector>>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_FaceAnalysis\"`, `\"Foundation_Collections\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub fn GetSupportedBitmapPixelFormats() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::BitmapPixelFormat>> {
        Self::IFaceDetectorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSupportedBitmapPixelFormats)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::BitmapPixelFormat>>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_FaceAnalysis\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn IsBitmapPixelFormatSupported(bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::Result<bool> {
        Self::IFaceDetectorStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBitmapPixelFormatSupported)(::core::mem::transmute_copy(this), bitmappixelformat, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_FaceAnalysis\"`*"]
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IFaceDetectorStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFaceDetectorStatics<R, F: FnOnce(&IFaceDetectorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FaceDetector, IFaceDetectorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for FaceDetector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FaceDetector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FaceDetector {}
impl ::core::fmt::Debug for FaceDetector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FaceDetector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FaceDetector {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.FaceAnalysis.FaceDetector;{16b672dc-fe6f-3117-8d95-c3f04d51630c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FaceDetector {
    type Vtable = IFaceDetector_Vtbl;
    const IID: ::windows::core::GUID = <IFaceDetector as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FaceDetector {
    const NAME: &'static str = "Windows.Media.FaceAnalysis.FaceDetector";
}
impl ::core::convert::From<FaceDetector> for ::windows::core::IUnknown {
    fn from(value: FaceDetector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FaceDetector> for ::windows::core::IUnknown {
    fn from(value: &FaceDetector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FaceDetector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FaceDetector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FaceDetector> for ::windows::core::IInspectable {
    fn from(value: FaceDetector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FaceDetector> for ::windows::core::IInspectable {
    fn from(value: &FaceDetector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FaceDetector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FaceDetector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FaceDetector {}
unsafe impl ::core::marker::Sync for FaceDetector {}
#[doc = "*Required features: `\"Media_FaceAnalysis\"`*"]
#[repr(transparent)]
pub struct FaceTracker(::windows::core::IUnknown);
impl FaceTracker {
    #[doc = "*Required features: `\"Media_FaceAnalysis\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProcessNextFrameAsync<'a, Param0: ::windows::core::IntoParam<'a, super::VideoFrame>>(&self, videoframe: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProcessNextFrameAsync)(::core::mem::transmute_copy(this), videoframe.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_FaceAnalysis\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn MinDetectableFaceSize(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapSize> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::Imaging::BitmapSize = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MinDetectableFaceSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapSize>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_FaceAnalysis\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetMinDetectableFaceSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapSize>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMinDetectableFaceSize)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_FaceAnalysis\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn MaxDetectableFaceSize(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapSize> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::Imaging::BitmapSize = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxDetectableFaceSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapSize>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_FaceAnalysis\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetMaxDetectableFaceSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapSize>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxDetectableFaceSize)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_FaceAnalysis\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FaceTracker>> {
        Self::IFaceTrackerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<FaceTracker>>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_FaceAnalysis\"`, `\"Foundation_Collections\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub fn GetSupportedBitmapPixelFormats() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::BitmapPixelFormat>> {
        Self::IFaceTrackerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSupportedBitmapPixelFormats)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::BitmapPixelFormat>>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_FaceAnalysis\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn IsBitmapPixelFormatSupported(bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::Result<bool> {
        Self::IFaceTrackerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBitmapPixelFormatSupported)(::core::mem::transmute_copy(this), bitmappixelformat, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_FaceAnalysis\"`*"]
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IFaceTrackerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFaceTrackerStatics<R, F: FnOnce(&IFaceTrackerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FaceTracker, IFaceTrackerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for FaceTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FaceTracker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FaceTracker {}
impl ::core::fmt::Debug for FaceTracker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FaceTracker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FaceTracker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.FaceAnalysis.FaceTracker;{6ba67d8c-a841-4420-93e6-2420a1884fcf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FaceTracker {
    type Vtable = IFaceTracker_Vtbl;
    const IID: ::windows::core::GUID = <IFaceTracker as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FaceTracker {
    const NAME: &'static str = "Windows.Media.FaceAnalysis.FaceTracker";
}
impl ::core::convert::From<FaceTracker> for ::windows::core::IUnknown {
    fn from(value: FaceTracker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FaceTracker> for ::windows::core::IUnknown {
    fn from(value: &FaceTracker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FaceTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FaceTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FaceTracker> for ::windows::core::IInspectable {
    fn from(value: FaceTracker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FaceTracker> for ::windows::core::IInspectable {
    fn from(value: &FaceTracker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FaceTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FaceTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FaceTracker {}
unsafe impl ::core::marker::Sync for FaceTracker {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDetectedFace(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDetectedFace {
    type Vtable = IDetectedFace_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8200d454_66bc_34df_9410_e89400195414);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDetectedFace_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub FaceBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapBounds) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    FaceBox: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceDetector(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFaceDetector {
    type Vtable = IFaceDetector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16b672dc_fe6f_3117_8d95_c3f04d51630c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceDetector_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub DetectFacesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    DetectFacesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub DetectFacesWithSearchAreaAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, searcharea: super::super::Graphics::Imaging::BitmapBounds, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    DetectFacesWithSearchAreaAsync: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub MinDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    MinDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetMinDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetMinDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub MaxDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    MaxDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetMaxDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetMaxDetectableFaceSize: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceDetectorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFaceDetectorStatics {
    type Vtable = IFaceDetectorStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc042d67_9047_33f6_881b_6746c1b218b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceDetectorStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub GetSupportedBitmapPixelFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    GetSupportedBitmapPixelFormats: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub IsBitmapPixelFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    IsBitmapPixelFormatSupported: usize,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceTracker(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFaceTracker {
    type Vtable = IFaceTracker_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ba67d8c_a841_4420_93e6_2420a1884fcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceTracker_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ProcessNextFrameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, videoframe: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProcessNextFrameAsync: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub MinDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    MinDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetMinDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetMinDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub MaxDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    MaxDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetMaxDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetMaxDetectableFaceSize: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceTrackerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFaceTrackerStatics {
    type Vtable = IFaceTrackerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9629198_1801_3fa5_932e_31d767af6c4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceTrackerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub GetSupportedBitmapPixelFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    GetSupportedBitmapPixelFormats: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub IsBitmapPixelFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    IsBitmapPixelFormatSupported: usize,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
