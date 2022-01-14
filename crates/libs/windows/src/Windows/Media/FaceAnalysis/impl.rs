#[cfg(all(feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait IDetectedFace_Impl: Sized {
    fn FaceBox(&mut self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapBounds>;
}
#[cfg(all(feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDetectedFace {
    const NAME: &'static str = "Windows.Media.FaceAnalysis.IDetectedFace";
}
#[cfg(all(feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl IDetectedFace_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDetectedFace_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDetectedFace_Vtbl {
        unsafe extern "system" fn FaceBox<Impl: IDetectedFace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapBounds) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FaceBox() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDetectedFace, BASE_OFFSET>(), FaceBox: FaceBox::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDetectedFace as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait IFaceDetector_Impl: Sized {
    fn DetectFacesAsync(&mut self, image: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>>;
    fn DetectFacesWithSearchAreaAsync(&mut self, image: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>, searcharea: &super::super::Graphics::Imaging::BitmapBounds) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>>;
    fn MinDetectableFaceSize(&mut self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapSize>;
    fn SetMinDetectableFaceSize(&mut self, value: &super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::Result<()>;
    fn MaxDetectableFaceSize(&mut self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapSize>;
    fn SetMaxDetectableFaceSize(&mut self, value: &super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFaceDetector {
    const NAME: &'static str = "Windows.Media.FaceAnalysis.IFaceDetector";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl IFaceDetector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaceDetector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaceDetector_Vtbl {
        unsafe extern "system" fn DetectFacesAsync<Impl: IFaceDetector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetectFacesAsync(&*(&image as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetectFacesWithSearchAreaAsync<Impl: IFaceDetector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, searcharea: super::super::Graphics::Imaging::BitmapBounds, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetectFacesWithSearchAreaAsync(&*(&image as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType), &*(&searcharea as *const <super::super::Graphics::Imaging::BitmapBounds as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapBounds as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinDetectableFaceSize<Impl: IFaceDetector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinDetectableFaceSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinDetectableFaceSize<Impl: IFaceDetector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinDetectableFaceSize(&*(&value as *const <super::super::Graphics::Imaging::BitmapSize as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapSize as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxDetectableFaceSize<Impl: IFaceDetector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxDetectableFaceSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxDetectableFaceSize<Impl: IFaceDetector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxDetectableFaceSize(&*(&value as *const <super::super::Graphics::Imaging::BitmapSize as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapSize as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFaceDetector, BASE_OFFSET>(),
            DetectFacesAsync: DetectFacesAsync::<Impl, IMPL_OFFSET>,
            DetectFacesWithSearchAreaAsync: DetectFacesWithSearchAreaAsync::<Impl, IMPL_OFFSET>,
            MinDetectableFaceSize: MinDetectableFaceSize::<Impl, IMPL_OFFSET>,
            SetMinDetectableFaceSize: SetMinDetectableFaceSize::<Impl, IMPL_OFFSET>,
            MaxDetectableFaceSize: MaxDetectableFaceSize::<Impl, IMPL_OFFSET>,
            SetMaxDetectableFaceSize: SetMaxDetectableFaceSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaceDetector as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait IFaceDetectorStatics_Impl: Sized {
    fn CreateAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FaceDetector>>;
    fn GetSupportedBitmapPixelFormats(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::BitmapPixelFormat>>;
    fn IsBitmapPixelFormatSupported(&mut self, bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::Result<bool>;
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFaceDetectorStatics {
    const NAME: &'static str = "Windows.Media.FaceAnalysis.IFaceDetectorStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl IFaceDetectorStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaceDetectorStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaceDetectorStatics_Vtbl {
        unsafe extern "system" fn CreateAsync<Impl: IFaceDetectorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedBitmapPixelFormats<Impl: IFaceDetectorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedBitmapPixelFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBitmapPixelFormatSupported<Impl: IFaceDetectorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBitmapPixelFormatSupported(bitmappixelformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupported<Impl: IFaceDetectorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFaceDetectorStatics, BASE_OFFSET>(),
            CreateAsync: CreateAsync::<Impl, IMPL_OFFSET>,
            GetSupportedBitmapPixelFormats: GetSupportedBitmapPixelFormats::<Impl, IMPL_OFFSET>,
            IsBitmapPixelFormatSupported: IsBitmapPixelFormatSupported::<Impl, IMPL_OFFSET>,
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaceDetectorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait IFaceTracker_Impl: Sized {
    fn ProcessNextFrameAsync(&mut self, videoframe: &::core::option::Option<super::VideoFrame>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>>;
    fn MinDetectableFaceSize(&mut self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapSize>;
    fn SetMinDetectableFaceSize(&mut self, value: &super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::Result<()>;
    fn MaxDetectableFaceSize(&mut self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapSize>;
    fn SetMaxDetectableFaceSize(&mut self, value: &super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFaceTracker {
    const NAME: &'static str = "Windows.Media.FaceAnalysis.IFaceTracker";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl IFaceTracker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaceTracker_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaceTracker_Vtbl {
        unsafe extern "system" fn ProcessNextFrameAsync<Impl: IFaceTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, videoframe: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessNextFrameAsync(&*(&videoframe as *const <super::VideoFrame as ::windows::core::Abi>::Abi as *const <super::VideoFrame as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinDetectableFaceSize<Impl: IFaceTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinDetectableFaceSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinDetectableFaceSize<Impl: IFaceTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinDetectableFaceSize(&*(&value as *const <super::super::Graphics::Imaging::BitmapSize as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapSize as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxDetectableFaceSize<Impl: IFaceTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxDetectableFaceSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxDetectableFaceSize<Impl: IFaceTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxDetectableFaceSize(&*(&value as *const <super::super::Graphics::Imaging::BitmapSize as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapSize as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFaceTracker, BASE_OFFSET>(),
            ProcessNextFrameAsync: ProcessNextFrameAsync::<Impl, IMPL_OFFSET>,
            MinDetectableFaceSize: MinDetectableFaceSize::<Impl, IMPL_OFFSET>,
            SetMinDetectableFaceSize: SetMinDetectableFaceSize::<Impl, IMPL_OFFSET>,
            MaxDetectableFaceSize: MaxDetectableFaceSize::<Impl, IMPL_OFFSET>,
            SetMaxDetectableFaceSize: SetMaxDetectableFaceSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaceTracker as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait IFaceTrackerStatics_Impl: Sized {
    fn CreateAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FaceTracker>>;
    fn GetSupportedBitmapPixelFormats(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::BitmapPixelFormat>>;
    fn IsBitmapPixelFormatSupported(&mut self, bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::Result<bool>;
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFaceTrackerStatics {
    const NAME: &'static str = "Windows.Media.FaceAnalysis.IFaceTrackerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl IFaceTrackerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaceTrackerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaceTrackerStatics_Vtbl {
        unsafe extern "system" fn CreateAsync<Impl: IFaceTrackerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedBitmapPixelFormats<Impl: IFaceTrackerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedBitmapPixelFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBitmapPixelFormatSupported<Impl: IFaceTrackerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBitmapPixelFormatSupported(bitmappixelformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupported<Impl: IFaceTrackerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFaceTrackerStatics, BASE_OFFSET>(),
            CreateAsync: CreateAsync::<Impl, IMPL_OFFSET>,
            GetSupportedBitmapPixelFormats: GetSupportedBitmapPixelFormats::<Impl, IMPL_OFFSET>,
            IsBitmapPixelFormatSupported: IsBitmapPixelFormatSupported::<Impl, IMPL_OFFSET>,
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaceTrackerStatics as ::windows::core::Interface>::IID
    }
}
