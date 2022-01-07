#[cfg(feature = "implement_exclusive")]
pub trait IDetectedFaceImpl: Sized {
    fn FaceBox(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapBounds>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDetectedFace {
    const NAME: &'static str = "Windows.Media.FaceAnalysis.IDetectedFace";
}
#[cfg(feature = "implement_exclusive")]
impl IDetectedFaceVtbl {
    pub const fn new<Impl: IDetectedFaceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDetectedFaceVtbl {
        unsafe extern "system" fn FaceBox<Impl: IDetectedFaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapBounds) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FaceBox() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDetectedFace>, base.5, FaceBox::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFaceDetectorImpl: Sized {
    fn DetectFacesAsync(&self, image: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>>;
    fn DetectFacesWithSearchAreaAsync(&self, image: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>, searcharea: &super::super::Graphics::Imaging::BitmapBounds) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>>;
    fn MinDetectableFaceSize(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapSize>;
    fn SetMinDetectableFaceSize(&self, value: &super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::Result<()>;
    fn MaxDetectableFaceSize(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapSize>;
    fn SetMaxDetectableFaceSize(&self, value: &super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFaceDetector {
    const NAME: &'static str = "Windows.Media.FaceAnalysis.IFaceDetector";
}
#[cfg(feature = "implement_exclusive")]
impl IFaceDetectorVtbl {
    pub const fn new<Impl: IFaceDetectorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFaceDetectorVtbl {
        unsafe extern "system" fn DetectFacesAsync<Impl: IFaceDetectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DetectFacesAsync(&*(&image as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetectFacesWithSearchAreaAsync<Impl: IFaceDetectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, searcharea: super::super::Graphics::Imaging::BitmapBounds, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DetectFacesWithSearchAreaAsync(&*(&image as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType), &*(&searcharea as *const <super::super::Graphics::Imaging::BitmapBounds as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapBounds as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinDetectableFaceSize<Impl: IFaceDetectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinDetectableFaceSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinDetectableFaceSize<Impl: IFaceDetectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMinDetectableFaceSize(&*(&value as *const <super::super::Graphics::Imaging::BitmapSize as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapSize as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxDetectableFaceSize<Impl: IFaceDetectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxDetectableFaceSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxDetectableFaceSize<Impl: IFaceDetectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxDetectableFaceSize(&*(&value as *const <super::super::Graphics::Imaging::BitmapSize as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapSize as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFaceDetector>, base.5, DetectFacesAsync::<Impl, OFFSET>, DetectFacesWithSearchAreaAsync::<Impl, OFFSET>, MinDetectableFaceSize::<Impl, OFFSET>, SetMinDetectableFaceSize::<Impl, OFFSET>, MaxDetectableFaceSize::<Impl, OFFSET>, SetMaxDetectableFaceSize::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFaceDetectorStaticsImpl: Sized {
    fn CreateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FaceDetector>>;
    fn GetSupportedBitmapPixelFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::BitmapPixelFormat>>;
    fn IsBitmapPixelFormatSupported(&self, bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::Result<bool>;
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFaceDetectorStatics {
    const NAME: &'static str = "Windows.Media.FaceAnalysis.IFaceDetectorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFaceDetectorStaticsVtbl {
    pub const fn new<Impl: IFaceDetectorStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFaceDetectorStaticsVtbl {
        unsafe extern "system" fn CreateAsync<Impl: IFaceDetectorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedBitmapPixelFormats<Impl: IFaceDetectorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSupportedBitmapPixelFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBitmapPixelFormatSupported<Impl: IFaceDetectorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBitmapPixelFormatSupported(bitmappixelformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupported<Impl: IFaceDetectorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFaceDetectorStatics>, base.5, CreateAsync::<Impl, OFFSET>, GetSupportedBitmapPixelFormats::<Impl, OFFSET>, IsBitmapPixelFormatSupported::<Impl, OFFSET>, IsSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFaceTrackerImpl: Sized {
    fn ProcessNextFrameAsync(&self, videoframe: &::core::option::Option<super::VideoFrame>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>>;
    fn MinDetectableFaceSize(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapSize>;
    fn SetMinDetectableFaceSize(&self, value: &super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::Result<()>;
    fn MaxDetectableFaceSize(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapSize>;
    fn SetMaxDetectableFaceSize(&self, value: &super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFaceTracker {
    const NAME: &'static str = "Windows.Media.FaceAnalysis.IFaceTracker";
}
#[cfg(feature = "implement_exclusive")]
impl IFaceTrackerVtbl {
    pub const fn new<Impl: IFaceTrackerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFaceTrackerVtbl {
        unsafe extern "system" fn ProcessNextFrameAsync<Impl: IFaceTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, videoframe: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProcessNextFrameAsync(&*(&videoframe as *const <super::VideoFrame as ::windows::core::Abi>::Abi as *const <super::VideoFrame as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinDetectableFaceSize<Impl: IFaceTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinDetectableFaceSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinDetectableFaceSize<Impl: IFaceTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMinDetectableFaceSize(&*(&value as *const <super::super::Graphics::Imaging::BitmapSize as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapSize as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxDetectableFaceSize<Impl: IFaceTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxDetectableFaceSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxDetectableFaceSize<Impl: IFaceTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxDetectableFaceSize(&*(&value as *const <super::super::Graphics::Imaging::BitmapSize as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapSize as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFaceTracker>, base.5, ProcessNextFrameAsync::<Impl, OFFSET>, MinDetectableFaceSize::<Impl, OFFSET>, SetMinDetectableFaceSize::<Impl, OFFSET>, MaxDetectableFaceSize::<Impl, OFFSET>, SetMaxDetectableFaceSize::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFaceTrackerStaticsImpl: Sized {
    fn CreateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FaceTracker>>;
    fn GetSupportedBitmapPixelFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::BitmapPixelFormat>>;
    fn IsBitmapPixelFormatSupported(&self, bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::Result<bool>;
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFaceTrackerStatics {
    const NAME: &'static str = "Windows.Media.FaceAnalysis.IFaceTrackerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFaceTrackerStaticsVtbl {
    pub const fn new<Impl: IFaceTrackerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFaceTrackerStaticsVtbl {
        unsafe extern "system" fn CreateAsync<Impl: IFaceTrackerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedBitmapPixelFormats<Impl: IFaceTrackerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSupportedBitmapPixelFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBitmapPixelFormatSupported<Impl: IFaceTrackerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBitmapPixelFormatSupported(bitmappixelformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupported<Impl: IFaceTrackerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFaceTrackerStatics>, base.5, CreateAsync::<Impl, OFFSET>, GetSupportedBitmapPixelFormats::<Impl, OFFSET>, IsBitmapPixelFormatSupported::<Impl, OFFSET>, IsSupported::<Impl, OFFSET>)
    }
}
