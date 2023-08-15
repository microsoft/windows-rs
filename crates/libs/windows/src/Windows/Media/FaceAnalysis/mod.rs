#[doc(hidden)]
#[repr(transparent)]
pub struct IDetectedFace(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDetectedFace {
    type Vtable = IDetectedFace_Vtbl;
}
impl ::core::clone::Clone for IDetectedFace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDetectedFace {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8200d454_66bc_34df_9410_e89400195414);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDetectedFace_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub FaceBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapBounds) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    FaceBox: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceDetector(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFaceDetector {
    type Vtable = IFaceDetector_Vtbl;
}
impl ::core::clone::Clone for IFaceDetector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFaceDetector {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16b672dc_fe6f_3117_8d95_c3f04d51630c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceDetector_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub DetectFacesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    DetectFacesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub DetectFacesWithSearchAreaAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, searcharea: super::super::Graphics::Imaging::BitmapBounds, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    DetectFacesWithSearchAreaAsync: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub MinDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    MinDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetMinDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetMinDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub MaxDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    MaxDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetMaxDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetMaxDetectableFaceSize: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceDetectorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFaceDetectorStatics {
    type Vtable = IFaceDetectorStatics_Vtbl;
}
impl ::core::clone::Clone for IFaceDetectorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFaceDetectorStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc042d67_9047_33f6_881b_6746c1b218b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceDetectorStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub GetSupportedBitmapPixelFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    GetSupportedBitmapPixelFormats: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub IsBitmapPixelFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    IsBitmapPixelFormatSupported: usize,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceTracker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFaceTracker {
    type Vtable = IFaceTracker_Vtbl;
}
impl ::core::clone::Clone for IFaceTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFaceTracker {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6ba67d8c_a841_4420_93e6_2420a1884fcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceTracker_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ProcessNextFrameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, videoframe: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProcessNextFrameAsync: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub MinDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    MinDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetMinDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetMinDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub MaxDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    MaxDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetMaxDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetMaxDetectableFaceSize: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceTrackerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFaceTrackerStatics {
    type Vtable = IFaceTrackerStatics_Vtbl;
}
impl ::core::clone::Clone for IFaceTrackerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFaceTrackerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe9629198_1801_3fa5_932e_31d767af6c4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceTrackerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub GetSupportedBitmapPixelFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    GetSupportedBitmapPixelFormats: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub IsBitmapPixelFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    IsBitmapPixelFormatSupported: usize,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Media_FaceAnalysis\"`*"]
#[repr(transparent)]
pub struct DetectedFace(::windows_core::IUnknown);
impl DetectedFace {
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn FaceBox(&self) -> ::windows_core::Result<super::super::Graphics::Imaging::BitmapBounds> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FaceBox)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for DetectedFace {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.FaceAnalysis.DetectedFace;{8200d454-66bc-34df-9410-e89400195414})");
}
impl ::core::clone::Clone for DetectedFace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DetectedFace {
    type Vtable = IDetectedFace_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DetectedFace {
    const IID: ::windows_core::GUID = <IDetectedFace as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DetectedFace {
    const NAME: &'static str = "Windows.Media.FaceAnalysis.DetectedFace";
}
::windows_core::imp::interface_hierarchy!(DetectedFace, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DetectedFace {}
unsafe impl ::core::marker::Sync for DetectedFace {}
#[doc = "*Required features: `\"Media_FaceAnalysis\"`*"]
#[repr(transparent)]
pub struct FaceDetector(::windows_core::IUnknown);
impl FaceDetector {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub fn DetectFacesAsync<P0>(&self, image: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>>
    where
        P0: ::windows_core::IntoParam<super::super::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DetectFacesAsync)(::windows_core::Interface::as_raw(this), image.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub fn DetectFacesWithSearchAreaAsync<P0>(&self, image: P0, searcharea: super::super::Graphics::Imaging::BitmapBounds) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>>
    where
        P0: ::windows_core::IntoParam<super::super::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DetectFacesWithSearchAreaAsync)(::windows_core::Interface::as_raw(this), image.into_param().abi(), searcharea, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn MinDetectableFaceSize(&self) -> ::windows_core::Result<super::super::Graphics::Imaging::BitmapSize> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinDetectableFaceSize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetMinDetectableFaceSize(&self, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMinDetectableFaceSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn MaxDetectableFaceSize(&self) -> ::windows_core::Result<super::super::Graphics::Imaging::BitmapSize> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxDetectableFaceSize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetMaxDetectableFaceSize(&self, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxDetectableFaceSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<FaceDetector>> {
        Self::IFaceDetectorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub fn GetSupportedBitmapPixelFormats() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::BitmapPixelFormat>> {
        Self::IFaceDetectorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSupportedBitmapPixelFormats)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn IsBitmapPixelFormatSupported(bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows_core::Result<bool> {
        Self::IFaceDetectorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsBitmapPixelFormatSupported)(::windows_core::Interface::as_raw(this), bitmappixelformat, &mut result__).from_abi(result__)
        })
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::IFaceDetectorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFaceDetectorStatics<R, F: FnOnce(&IFaceDetectorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FaceDetector, IFaceDetectorStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for FaceDetector {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.FaceAnalysis.FaceDetector;{16b672dc-fe6f-3117-8d95-c3f04d51630c})");
}
impl ::core::clone::Clone for FaceDetector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for FaceDetector {
    type Vtable = IFaceDetector_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FaceDetector {
    const IID: ::windows_core::GUID = <IFaceDetector as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FaceDetector {
    const NAME: &'static str = "Windows.Media.FaceAnalysis.FaceDetector";
}
::windows_core::imp::interface_hierarchy!(FaceDetector, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for FaceDetector {}
unsafe impl ::core::marker::Sync for FaceDetector {}
#[doc = "*Required features: `\"Media_FaceAnalysis\"`*"]
#[repr(transparent)]
pub struct FaceTracker(::windows_core::IUnknown);
impl FaceTracker {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProcessNextFrameAsync<P0>(&self, videoframe: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>>
    where
        P0: ::windows_core::IntoParam<super::VideoFrame>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProcessNextFrameAsync)(::windows_core::Interface::as_raw(this), videoframe.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn MinDetectableFaceSize(&self) -> ::windows_core::Result<super::super::Graphics::Imaging::BitmapSize> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinDetectableFaceSize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetMinDetectableFaceSize(&self, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMinDetectableFaceSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn MaxDetectableFaceSize(&self) -> ::windows_core::Result<super::super::Graphics::Imaging::BitmapSize> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxDetectableFaceSize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetMaxDetectableFaceSize(&self, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxDetectableFaceSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<FaceTracker>> {
        Self::IFaceTrackerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub fn GetSupportedBitmapPixelFormats() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::BitmapPixelFormat>> {
        Self::IFaceTrackerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSupportedBitmapPixelFormats)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn IsBitmapPixelFormatSupported(bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows_core::Result<bool> {
        Self::IFaceTrackerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsBitmapPixelFormatSupported)(::windows_core::Interface::as_raw(this), bitmappixelformat, &mut result__).from_abi(result__)
        })
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::IFaceTrackerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFaceTrackerStatics<R, F: FnOnce(&IFaceTrackerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FaceTracker, IFaceTrackerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for FaceTracker {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.FaceAnalysis.FaceTracker;{6ba67d8c-a841-4420-93e6-2420a1884fcf})");
}
impl ::core::clone::Clone for FaceTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for FaceTracker {
    type Vtable = IFaceTracker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FaceTracker {
    const IID: ::windows_core::GUID = <IFaceTracker as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FaceTracker {
    const NAME: &'static str = "Windows.Media.FaceAnalysis.FaceTracker";
}
::windows_core::imp::interface_hierarchy!(FaceTracker, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for FaceTracker {}
unsafe impl ::core::marker::Sync for FaceTracker {}
