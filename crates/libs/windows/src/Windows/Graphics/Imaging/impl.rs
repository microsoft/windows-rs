#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBitmapBufferImpl: Sized + IClosableImpl + IMemoryBufferImpl {
    fn GetPlaneCount(&self) -> ::windows::core::Result<i32>;
    fn GetPlaneDescription(&self, index: i32) -> ::windows::core::Result<BitmapPlaneDescription>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBitmapBuffer {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapBuffer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBitmapBufferVtbl {
    pub const fn new<Impl: IBitmapBufferImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBitmapBufferVtbl {
        unsafe extern "system" fn GetPlaneCount<Impl: IBitmapBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPlaneCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPlaneDescription<Impl: IBitmapBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, result__: *mut BitmapPlaneDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPlaneDescription(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBitmapBuffer>, base.5, GetPlaneCount::<Impl, OFFSET>, GetPlaneDescription::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapCodecInformationImpl: Sized {
    fn CodecId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn FileExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MimeTypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapCodecInformation {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapCodecInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapCodecInformationVtbl {
    pub const fn new<Impl: IBitmapCodecInformationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBitmapCodecInformationVtbl {
        unsafe extern "system" fn CodecId<Impl: IBitmapCodecInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CodecId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileExtensions<Impl: IBitmapCodecInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FileExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FriendlyName<Impl: IBitmapCodecInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MimeTypes<Impl: IBitmapCodecInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MimeTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBitmapCodecInformation>, base.5, CodecId::<Impl, OFFSET>, FileExtensions::<Impl, OFFSET>, FriendlyName::<Impl, OFFSET>, MimeTypes::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapDecoderImpl: Sized {
    fn BitmapContainerProperties(&self) -> ::windows::core::Result<BitmapPropertiesView>;
    fn DecoderInformation(&self) -> ::windows::core::Result<BitmapCodecInformation>;
    fn FrameCount(&self) -> ::windows::core::Result<u32>;
    fn GetPreviewAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageStream>>;
    fn GetFrameAsync(&self, frameindex: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapFrame>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapDecoder {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapDecoder";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapDecoderVtbl {
    pub const fn new<Impl: IBitmapDecoderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBitmapDecoderVtbl {
        unsafe extern "system" fn BitmapContainerProperties<Impl: IBitmapDecoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BitmapContainerProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecoderInformation<Impl: IBitmapDecoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DecoderInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameCount<Impl: IBitmapDecoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviewAsync<Impl: IBitmapDecoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPreviewAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameAsync<Impl: IBitmapDecoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frameindex: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFrameAsync(frameindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBitmapDecoder>, base.5, BitmapContainerProperties::<Impl, OFFSET>, DecoderInformation::<Impl, OFFSET>, FrameCount::<Impl, OFFSET>, GetPreviewAsync::<Impl, OFFSET>, GetFrameAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapDecoderStaticsImpl: Sized {
    fn BmpDecoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn JpegDecoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn PngDecoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TiffDecoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GifDecoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn JpegXRDecoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IcoDecoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetDecoderInformationEnumerator(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<BitmapCodecInformation>>;
    fn CreateAsync(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapDecoder>>;
    fn CreateWithIdAsync(&self, decoderid: &::windows::core::GUID, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapDecoder>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapDecoderStatics {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapDecoderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapDecoderStaticsVtbl {
    pub const fn new<Impl: IBitmapDecoderStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBitmapDecoderStaticsVtbl {
        unsafe extern "system" fn BmpDecoderId<Impl: IBitmapDecoderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BmpDecoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JpegDecoderId<Impl: IBitmapDecoderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).JpegDecoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PngDecoderId<Impl: IBitmapDecoderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PngDecoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TiffDecoderId<Impl: IBitmapDecoderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TiffDecoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GifDecoderId<Impl: IBitmapDecoderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GifDecoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JpegXRDecoderId<Impl: IBitmapDecoderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).JpegXRDecoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IcoDecoderId<Impl: IBitmapDecoderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IcoDecoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDecoderInformationEnumerator<Impl: IBitmapDecoderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDecoderInformationEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAsync<Impl: IBitmapDecoderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAsync(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithIdAsync<Impl: IBitmapDecoderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, decoderid: ::windows::core::GUID, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithIdAsync(&*(&decoderid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBitmapDecoderStatics>, base.5, BmpDecoderId::<Impl, OFFSET>, JpegDecoderId::<Impl, OFFSET>, PngDecoderId::<Impl, OFFSET>, TiffDecoderId::<Impl, OFFSET>, GifDecoderId::<Impl, OFFSET>, JpegXRDecoderId::<Impl, OFFSET>, IcoDecoderId::<Impl, OFFSET>, GetDecoderInformationEnumerator::<Impl, OFFSET>, CreateAsync::<Impl, OFFSET>, CreateWithIdAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapDecoderStatics2Impl: Sized {
    fn HeifDecoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn WebpDecoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapDecoderStatics2 {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapDecoderStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapDecoderStatics2Vtbl {
    pub const fn new<Impl: IBitmapDecoderStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBitmapDecoderStatics2Vtbl {
        unsafe extern "system" fn HeifDecoderId<Impl: IBitmapDecoderStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HeifDecoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WebpDecoderId<Impl: IBitmapDecoderStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WebpDecoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBitmapDecoderStatics2>, base.5, HeifDecoderId::<Impl, OFFSET>, WebpDecoderId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapEncoderImpl: Sized {
    fn EncoderInformation(&self) -> ::windows::core::Result<BitmapCodecInformation>;
    fn BitmapProperties(&self) -> ::windows::core::Result<BitmapProperties>;
    fn BitmapContainerProperties(&self) -> ::windows::core::Result<BitmapProperties>;
    fn IsThumbnailGenerated(&self) -> ::windows::core::Result<bool>;
    fn SetIsThumbnailGenerated(&self, value: bool) -> ::windows::core::Result<()>;
    fn GeneratedThumbnailWidth(&self) -> ::windows::core::Result<u32>;
    fn SetGeneratedThumbnailWidth(&self, value: u32) -> ::windows::core::Result<()>;
    fn GeneratedThumbnailHeight(&self) -> ::windows::core::Result<u32>;
    fn SetGeneratedThumbnailHeight(&self, value: u32) -> ::windows::core::Result<()>;
    fn BitmapTransform(&self) -> ::windows::core::Result<BitmapTransform>;
    fn SetPixelData(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, width: u32, height: u32, dpix: f64, dpiy: f64, pixels: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GoToNextFrameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GoToNextFrameWithEncodingOptionsAsync(&self, encodingoptions: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapEncoder {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapEncoder";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapEncoderVtbl {
    pub const fn new<Impl: IBitmapEncoderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBitmapEncoderVtbl {
        unsafe extern "system" fn EncoderInformation<Impl: IBitmapEncoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EncoderInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapProperties<Impl: IBitmapEncoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BitmapProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapContainerProperties<Impl: IBitmapEncoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BitmapContainerProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThumbnailGenerated<Impl: IBitmapEncoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsThumbnailGenerated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsThumbnailGenerated<Impl: IBitmapEncoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsThumbnailGenerated(value).into()
        }
        unsafe extern "system" fn GeneratedThumbnailWidth<Impl: IBitmapEncoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GeneratedThumbnailWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeneratedThumbnailWidth<Impl: IBitmapEncoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetGeneratedThumbnailWidth(value).into()
        }
        unsafe extern "system" fn GeneratedThumbnailHeight<Impl: IBitmapEncoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GeneratedThumbnailHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeneratedThumbnailHeight<Impl: IBitmapEncoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetGeneratedThumbnailHeight(value).into()
        }
        unsafe extern "system" fn BitmapTransform<Impl: IBitmapEncoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BitmapTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPixelData<Impl: IBitmapEncoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, width: u32, height: u32, dpix: f64, dpiy: f64, pixels_array_size: u32, pixels: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPixelData(pixelformat, alphamode, width, height, dpix, dpiy, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&pixels), pixels_array_size as _)).into()
        }
        unsafe extern "system" fn GoToNextFrameAsync<Impl: IBitmapEncoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GoToNextFrameAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GoToNextFrameWithEncodingOptionsAsync<Impl: IBitmapEncoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GoToNextFrameWithEncodingOptionsAsync(&*(&encodingoptions as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushAsync<Impl: IBitmapEncoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FlushAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IBitmapEncoder>,
            base.5,
            EncoderInformation::<Impl, OFFSET>,
            BitmapProperties::<Impl, OFFSET>,
            BitmapContainerProperties::<Impl, OFFSET>,
            IsThumbnailGenerated::<Impl, OFFSET>,
            SetIsThumbnailGenerated::<Impl, OFFSET>,
            GeneratedThumbnailWidth::<Impl, OFFSET>,
            SetGeneratedThumbnailWidth::<Impl, OFFSET>,
            GeneratedThumbnailHeight::<Impl, OFFSET>,
            SetGeneratedThumbnailHeight::<Impl, OFFSET>,
            BitmapTransform::<Impl, OFFSET>,
            SetPixelData::<Impl, OFFSET>,
            GoToNextFrameAsync::<Impl, OFFSET>,
            GoToNextFrameWithEncodingOptionsAsync::<Impl, OFFSET>,
            FlushAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapEncoderStaticsImpl: Sized {
    fn BmpEncoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn JpegEncoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn PngEncoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TiffEncoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GifEncoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn JpegXREncoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetEncoderInformationEnumerator(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<BitmapCodecInformation>>;
    fn CreateAsync(&self, encoderid: &::windows::core::GUID, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>>;
    fn CreateWithEncodingOptionsAsync(&self, encoderid: &::windows::core::GUID, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, encodingoptions: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>>;
    fn CreateForTranscodingAsync(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, bitmapdecoder: &::core::option::Option<BitmapDecoder>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>>;
    fn CreateForInPlacePropertyEncodingAsync(&self, bitmapdecoder: &::core::option::Option<BitmapDecoder>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapEncoderStatics {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapEncoderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapEncoderStaticsVtbl {
    pub const fn new<Impl: IBitmapEncoderStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBitmapEncoderStaticsVtbl {
        unsafe extern "system" fn BmpEncoderId<Impl: IBitmapEncoderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BmpEncoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JpegEncoderId<Impl: IBitmapEncoderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).JpegEncoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PngEncoderId<Impl: IBitmapEncoderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PngEncoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TiffEncoderId<Impl: IBitmapEncoderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TiffEncoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GifEncoderId<Impl: IBitmapEncoderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GifEncoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JpegXREncoderId<Impl: IBitmapEncoderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).JpegXREncoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEncoderInformationEnumerator<Impl: IBitmapEncoderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEncoderInformationEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAsync<Impl: IBitmapEncoderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encoderid: ::windows::core::GUID, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAsync(&*(&encoderid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithEncodingOptionsAsync<Impl: IBitmapEncoderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encoderid: ::windows::core::GUID, stream: ::windows::core::RawPtr, encodingoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithEncodingOptionsAsync(
                &*(&encoderid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType),
                &*(&encodingoptions as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForTranscodingAsync<Impl: IBitmapEncoderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, bitmapdecoder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForTranscodingAsync(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType), &*(&bitmapdecoder as *const <BitmapDecoder as ::windows::core::Abi>::Abi as *const <BitmapDecoder as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForInPlacePropertyEncodingAsync<Impl: IBitmapEncoderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmapdecoder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForInPlacePropertyEncodingAsync(&*(&bitmapdecoder as *const <BitmapDecoder as ::windows::core::Abi>::Abi as *const <BitmapDecoder as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IBitmapEncoderStatics>,
            base.5,
            BmpEncoderId::<Impl, OFFSET>,
            JpegEncoderId::<Impl, OFFSET>,
            PngEncoderId::<Impl, OFFSET>,
            TiffEncoderId::<Impl, OFFSET>,
            GifEncoderId::<Impl, OFFSET>,
            JpegXREncoderId::<Impl, OFFSET>,
            GetEncoderInformationEnumerator::<Impl, OFFSET>,
            CreateAsync::<Impl, OFFSET>,
            CreateWithEncodingOptionsAsync::<Impl, OFFSET>,
            CreateForTranscodingAsync::<Impl, OFFSET>,
            CreateForInPlacePropertyEncodingAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapEncoderStatics2Impl: Sized {
    fn HeifEncoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapEncoderStatics2 {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapEncoderStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapEncoderStatics2Vtbl {
    pub const fn new<Impl: IBitmapEncoderStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBitmapEncoderStatics2Vtbl {
        unsafe extern "system" fn HeifEncoderId<Impl: IBitmapEncoderStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HeifEncoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBitmapEncoderStatics2>, base.5, HeifEncoderId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapEncoderWithSoftwareBitmapImpl: Sized {
    fn SetSoftwareBitmap(&self, bitmap: &::core::option::Option<SoftwareBitmap>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapEncoderWithSoftwareBitmap {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapEncoderWithSoftwareBitmap";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapEncoderWithSoftwareBitmapVtbl {
    pub const fn new<Impl: IBitmapEncoderWithSoftwareBitmapImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBitmapEncoderWithSoftwareBitmapVtbl {
        unsafe extern "system" fn SetSoftwareBitmap<Impl: IBitmapEncoderWithSoftwareBitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSoftwareBitmap(&*(&bitmap as *const <SoftwareBitmap as ::windows::core::Abi>::Abi as *const <SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBitmapEncoderWithSoftwareBitmap>, base.5, SetSoftwareBitmap::<Impl, OFFSET>)
    }
}
pub trait IBitmapFrameImpl: Sized {
    fn GetThumbnailAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageStream>>;
    fn BitmapProperties(&self) -> ::windows::core::Result<BitmapPropertiesView>;
    fn BitmapPixelFormat(&self) -> ::windows::core::Result<BitmapPixelFormat>;
    fn BitmapAlphaMode(&self) -> ::windows::core::Result<BitmapAlphaMode>;
    fn DpiX(&self) -> ::windows::core::Result<f64>;
    fn DpiY(&self) -> ::windows::core::Result<f64>;
    fn PixelWidth(&self) -> ::windows::core::Result<u32>;
    fn PixelHeight(&self) -> ::windows::core::Result<u32>;
    fn OrientedPixelWidth(&self) -> ::windows::core::Result<u32>;
    fn OrientedPixelHeight(&self) -> ::windows::core::Result<u32>;
    fn GetPixelDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>>;
    fn GetPixelDataTransformedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: &::core::option::Option<BitmapTransform>, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>>;
}
impl ::windows::core::RuntimeName for IBitmapFrame {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapFrame";
}
impl IBitmapFrameVtbl {
    pub const fn new<Impl: IBitmapFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBitmapFrameVtbl {
        unsafe extern "system" fn GetThumbnailAsync<Impl: IBitmapFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetThumbnailAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapProperties<Impl: IBitmapFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BitmapProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapPixelFormat<Impl: IBitmapFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BitmapPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BitmapPixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapAlphaMode<Impl: IBitmapFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BitmapAlphaMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BitmapAlphaMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DpiX<Impl: IBitmapFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DpiX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DpiY<Impl: IBitmapFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DpiY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PixelWidth<Impl: IBitmapFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PixelWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PixelHeight<Impl: IBitmapFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PixelHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OrientedPixelWidth<Impl: IBitmapFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OrientedPixelWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OrientedPixelHeight<Impl: IBitmapFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OrientedPixelHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelDataAsync<Impl: IBitmapFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPixelDataAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelDataTransformedAsync<Impl: IBitmapFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: ::windows::core::RawPtr, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPixelDataTransformedAsync(pixelformat, alphamode, &*(&transform as *const <BitmapTransform as ::windows::core::Abi>::Abi as *const <BitmapTransform as ::windows::core::DefaultType>::DefaultType), exiforientationmode, colormanagementmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IBitmapFrame>,
            base.5,
            GetThumbnailAsync::<Impl, OFFSET>,
            BitmapProperties::<Impl, OFFSET>,
            BitmapPixelFormat::<Impl, OFFSET>,
            BitmapAlphaMode::<Impl, OFFSET>,
            DpiX::<Impl, OFFSET>,
            DpiY::<Impl, OFFSET>,
            PixelWidth::<Impl, OFFSET>,
            PixelHeight::<Impl, OFFSET>,
            OrientedPixelWidth::<Impl, OFFSET>,
            OrientedPixelHeight::<Impl, OFFSET>,
            GetPixelDataAsync::<Impl, OFFSET>,
            GetPixelDataTransformedAsync::<Impl, OFFSET>,
        )
    }
}
pub trait IBitmapFrameWithSoftwareBitmapImpl: Sized + IBitmapFrameImpl {
    fn GetSoftwareBitmapAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
    fn GetSoftwareBitmapConvertedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
    fn GetSoftwareBitmapTransformedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: &::core::option::Option<BitmapTransform>, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
}
impl ::windows::core::RuntimeName for IBitmapFrameWithSoftwareBitmap {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapFrameWithSoftwareBitmap";
}
impl IBitmapFrameWithSoftwareBitmapVtbl {
    pub const fn new<Impl: IBitmapFrameWithSoftwareBitmapImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBitmapFrameWithSoftwareBitmapVtbl {
        unsafe extern "system" fn GetSoftwareBitmapAsync<Impl: IBitmapFrameWithSoftwareBitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSoftwareBitmapAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSoftwareBitmapConvertedAsync<Impl: IBitmapFrameWithSoftwareBitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSoftwareBitmapConvertedAsync(pixelformat, alphamode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSoftwareBitmapTransformedAsync<Impl: IBitmapFrameWithSoftwareBitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: ::windows::core::RawPtr, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSoftwareBitmapTransformedAsync(pixelformat, alphamode, &*(&transform as *const <BitmapTransform as ::windows::core::Abi>::Abi as *const <BitmapTransform as ::windows::core::DefaultType>::DefaultType), exiforientationmode, colormanagementmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBitmapFrameWithSoftwareBitmap>, base.5, GetSoftwareBitmapAsync::<Impl, OFFSET>, GetSoftwareBitmapConvertedAsync::<Impl, OFFSET>, GetSoftwareBitmapTransformedAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapPropertiesImpl: Sized + IBitmapPropertiesViewImpl {
    fn SetPropertiesAsync(&self, propertiestoset: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapProperties {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapProperties";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapPropertiesVtbl {
    pub const fn new<Impl: IBitmapPropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBitmapPropertiesVtbl {
        unsafe extern "system" fn SetPropertiesAsync<Impl: IBitmapPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertiestoset: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPropertiesAsync(&*(&propertiestoset as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBitmapProperties>, base.5, SetPropertiesAsync::<Impl, OFFSET>)
    }
}
pub trait IBitmapPropertiesViewImpl: Sized {
    fn GetPropertiesAsync(&self, propertiestoretrieve: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapPropertySet>>;
}
impl ::windows::core::RuntimeName for IBitmapPropertiesView {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapPropertiesView";
}
impl IBitmapPropertiesViewVtbl {
    pub const fn new<Impl: IBitmapPropertiesViewImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBitmapPropertiesViewVtbl {
        unsafe extern "system" fn GetPropertiesAsync<Impl: IBitmapPropertiesViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertiestoretrieve: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertiesAsync(&*(&propertiestoretrieve as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBitmapPropertiesView>, base.5, GetPropertiesAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapTransformImpl: Sized {
    fn ScaledWidth(&self) -> ::windows::core::Result<u32>;
    fn SetScaledWidth(&self, value: u32) -> ::windows::core::Result<()>;
    fn ScaledHeight(&self) -> ::windows::core::Result<u32>;
    fn SetScaledHeight(&self, value: u32) -> ::windows::core::Result<()>;
    fn InterpolationMode(&self) -> ::windows::core::Result<BitmapInterpolationMode>;
    fn SetInterpolationMode(&self, value: BitmapInterpolationMode) -> ::windows::core::Result<()>;
    fn Flip(&self) -> ::windows::core::Result<BitmapFlip>;
    fn SetFlip(&self, value: BitmapFlip) -> ::windows::core::Result<()>;
    fn Rotation(&self) -> ::windows::core::Result<BitmapRotation>;
    fn SetRotation(&self, value: BitmapRotation) -> ::windows::core::Result<()>;
    fn Bounds(&self) -> ::windows::core::Result<BitmapBounds>;
    fn SetBounds(&self, value: &BitmapBounds) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapTransform {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapTransform";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapTransformVtbl {
    pub const fn new<Impl: IBitmapTransformImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBitmapTransformVtbl {
        unsafe extern "system" fn ScaledWidth<Impl: IBitmapTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScaledWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaledWidth<Impl: IBitmapTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetScaledWidth(value).into()
        }
        unsafe extern "system" fn ScaledHeight<Impl: IBitmapTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScaledHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaledHeight<Impl: IBitmapTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetScaledHeight(value).into()
        }
        unsafe extern "system" fn InterpolationMode<Impl: IBitmapTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BitmapInterpolationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InterpolationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterpolationMode<Impl: IBitmapTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: BitmapInterpolationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInterpolationMode(value).into()
        }
        unsafe extern "system" fn Flip<Impl: IBitmapTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BitmapFlip) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Flip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlip<Impl: IBitmapTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: BitmapFlip) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFlip(value).into()
        }
        unsafe extern "system" fn Rotation<Impl: IBitmapTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BitmapRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Rotation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotation<Impl: IBitmapTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: BitmapRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRotation(value).into()
        }
        unsafe extern "system" fn Bounds<Impl: IBitmapTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BitmapBounds) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBounds<Impl: IBitmapTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: BitmapBounds) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBounds(&*(&value as *const <BitmapBounds as ::windows::core::Abi>::Abi as *const <BitmapBounds as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBitmapTransform>, base.5, ScaledWidth::<Impl, OFFSET>, SetScaledWidth::<Impl, OFFSET>, ScaledHeight::<Impl, OFFSET>, SetScaledHeight::<Impl, OFFSET>, InterpolationMode::<Impl, OFFSET>, SetInterpolationMode::<Impl, OFFSET>, Flip::<Impl, OFFSET>, SetFlip::<Impl, OFFSET>, Rotation::<Impl, OFFSET>, SetRotation::<Impl, OFFSET>, Bounds::<Impl, OFFSET>, SetBounds::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapTypedValueImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Type(&self) -> ::windows::core::Result<super::super::Foundation::PropertyType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapTypedValue {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapTypedValue";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapTypedValueVtbl {
    pub const fn new<Impl: IBitmapTypedValueImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBitmapTypedValueVtbl {
        unsafe extern "system" fn Value<Impl: IBitmapTypedValueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IBitmapTypedValueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::PropertyType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBitmapTypedValue>, base.5, Value::<Impl, OFFSET>, Type::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapTypedValueFactoryImpl: Sized {
    fn Create(&self, value: &::core::option::Option<::windows::core::IInspectable>, r#type: super::super::Foundation::PropertyType) -> ::windows::core::Result<BitmapTypedValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapTypedValueFactory {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapTypedValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapTypedValueFactoryVtbl {
    pub const fn new<Impl: IBitmapTypedValueFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBitmapTypedValueFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IBitmapTypedValueFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, r#type: super::super::Foundation::PropertyType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBitmapTypedValueFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPixelDataProviderImpl: Sized {
    fn DetachPixelData(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPixelDataProvider {
    const NAME: &'static str = "Windows.Graphics.Imaging.IPixelDataProvider";
}
#[cfg(feature = "implement_exclusive")]
impl IPixelDataProviderVtbl {
    pub const fn new<Impl: IPixelDataProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPixelDataProviderVtbl {
        unsafe extern "system" fn DetachPixelData<Impl: IPixelDataProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DetachPixelData() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPixelDataProvider>, base.5, DetachPixelData::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISoftwareBitmapImpl: Sized + IClosableImpl {
    fn BitmapPixelFormat(&self) -> ::windows::core::Result<BitmapPixelFormat>;
    fn BitmapAlphaMode(&self) -> ::windows::core::Result<BitmapAlphaMode>;
    fn PixelWidth(&self) -> ::windows::core::Result<i32>;
    fn PixelHeight(&self) -> ::windows::core::Result<i32>;
    fn IsReadOnly(&self) -> ::windows::core::Result<bool>;
    fn SetDpiX(&self, value: f64) -> ::windows::core::Result<()>;
    fn DpiX(&self) -> ::windows::core::Result<f64>;
    fn SetDpiY(&self, value: f64) -> ::windows::core::Result<()>;
    fn DpiY(&self) -> ::windows::core::Result<f64>;
    fn LockBuffer(&self, mode: BitmapBufferAccessMode) -> ::windows::core::Result<BitmapBuffer>;
    fn CopyTo(&self, bitmap: &::core::option::Option<SoftwareBitmap>) -> ::windows::core::Result<()>;
    fn CopyFromBuffer(&self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn CopyToBuffer(&self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn GetReadOnlyView(&self) -> ::windows::core::Result<SoftwareBitmap>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISoftwareBitmap {
    const NAME: &'static str = "Windows.Graphics.Imaging.ISoftwareBitmap";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISoftwareBitmapVtbl {
    pub const fn new<Impl: ISoftwareBitmapImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISoftwareBitmapVtbl {
        unsafe extern "system" fn BitmapPixelFormat<Impl: ISoftwareBitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BitmapPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BitmapPixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapAlphaMode<Impl: ISoftwareBitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BitmapAlphaMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BitmapAlphaMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PixelWidth<Impl: ISoftwareBitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PixelWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PixelHeight<Impl: ISoftwareBitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PixelHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadOnly<Impl: ISoftwareBitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDpiX<Impl: ISoftwareBitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDpiX(value).into()
        }
        unsafe extern "system" fn DpiX<Impl: ISoftwareBitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DpiX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDpiY<Impl: ISoftwareBitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDpiY(value).into()
        }
        unsafe extern "system" fn DpiY<Impl: ISoftwareBitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DpiY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockBuffer<Impl: ISoftwareBitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: BitmapBufferAccessMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LockBuffer(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTo<Impl: ISoftwareBitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).CopyTo(&*(&bitmap as *const <SoftwareBitmap as ::windows::core::Abi>::Abi as *const <SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CopyFromBuffer<Impl: ISoftwareBitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).CopyFromBuffer(&*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CopyToBuffer<Impl: ISoftwareBitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).CopyToBuffer(&*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetReadOnlyView<Impl: ISoftwareBitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReadOnlyView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISoftwareBitmap>,
            base.5,
            BitmapPixelFormat::<Impl, OFFSET>,
            BitmapAlphaMode::<Impl, OFFSET>,
            PixelWidth::<Impl, OFFSET>,
            PixelHeight::<Impl, OFFSET>,
            IsReadOnly::<Impl, OFFSET>,
            SetDpiX::<Impl, OFFSET>,
            DpiX::<Impl, OFFSET>,
            SetDpiY::<Impl, OFFSET>,
            DpiY::<Impl, OFFSET>,
            LockBuffer::<Impl, OFFSET>,
            CopyTo::<Impl, OFFSET>,
            CopyFromBuffer::<Impl, OFFSET>,
            CopyToBuffer::<Impl, OFFSET>,
            GetReadOnlyView::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISoftwareBitmapFactoryImpl: Sized {
    fn Create(&self, format: BitmapPixelFormat, width: i32, height: i32) -> ::windows::core::Result<SoftwareBitmap>;
    fn CreateWithAlpha(&self, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> ::windows::core::Result<SoftwareBitmap>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISoftwareBitmapFactory {
    const NAME: &'static str = "Windows.Graphics.Imaging.ISoftwareBitmapFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISoftwareBitmapFactoryVtbl {
    pub const fn new<Impl: ISoftwareBitmapFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISoftwareBitmapFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ISoftwareBitmapFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: BitmapPixelFormat, width: i32, height: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(format, width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithAlpha<Impl: ISoftwareBitmapFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithAlpha(format, width, height, alpha) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISoftwareBitmapFactory>, base.5, Create::<Impl, OFFSET>, CreateWithAlpha::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISoftwareBitmapStaticsImpl: Sized {
    fn Copy(&self, source: &::core::option::Option<SoftwareBitmap>) -> ::windows::core::Result<SoftwareBitmap>;
    fn Convert(&self, source: &::core::option::Option<SoftwareBitmap>, format: BitmapPixelFormat) -> ::windows::core::Result<SoftwareBitmap>;
    fn ConvertWithAlpha(&self, source: &::core::option::Option<SoftwareBitmap>, format: BitmapPixelFormat, alpha: BitmapAlphaMode) -> ::windows::core::Result<SoftwareBitmap>;
    fn CreateCopyFromBuffer(&self, source: &::core::option::Option<super::super::Storage::Streams::IBuffer>, format: BitmapPixelFormat, width: i32, height: i32) -> ::windows::core::Result<SoftwareBitmap>;
    fn CreateCopyWithAlphaFromBuffer(&self, source: &::core::option::Option<super::super::Storage::Streams::IBuffer>, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> ::windows::core::Result<SoftwareBitmap>;
    fn CreateCopyFromSurfaceAsync(&self, surface: &::core::option::Option<super::DirectX::Direct3D11::IDirect3DSurface>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
    fn CreateCopyWithAlphaFromSurfaceAsync(&self, surface: &::core::option::Option<super::DirectX::Direct3D11::IDirect3DSurface>, alpha: BitmapAlphaMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISoftwareBitmapStatics {
    const NAME: &'static str = "Windows.Graphics.Imaging.ISoftwareBitmapStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISoftwareBitmapStaticsVtbl {
    pub const fn new<Impl: ISoftwareBitmapStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISoftwareBitmapStaticsVtbl {
        unsafe extern "system" fn Copy<Impl: ISoftwareBitmapStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Copy(&*(&source as *const <SoftwareBitmap as ::windows::core::Abi>::Abi as *const <SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Convert<Impl: ISoftwareBitmapStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, format: BitmapPixelFormat, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Convert(&*(&source as *const <SoftwareBitmap as ::windows::core::Abi>::Abi as *const <SoftwareBitmap as ::windows::core::DefaultType>::DefaultType), format) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertWithAlpha<Impl: ISoftwareBitmapStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, format: BitmapPixelFormat, alpha: BitmapAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConvertWithAlpha(&*(&source as *const <SoftwareBitmap as ::windows::core::Abi>::Abi as *const <SoftwareBitmap as ::windows::core::DefaultType>::DefaultType), format, alpha) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCopyFromBuffer<Impl: ISoftwareBitmapStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, format: BitmapPixelFormat, width: i32, height: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCopyFromBuffer(&*(&source as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), format, width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCopyWithAlphaFromBuffer<Impl: ISoftwareBitmapStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCopyWithAlphaFromBuffer(&*(&source as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), format, width, height, alpha) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCopyFromSurfaceAsync<Impl: ISoftwareBitmapStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, surface: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCopyFromSurfaceAsync(&*(&surface as *const <super::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::Abi>::Abi as *const <super::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCopyWithAlphaFromSurfaceAsync<Impl: ISoftwareBitmapStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, surface: ::windows::core::RawPtr, alpha: BitmapAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCopyWithAlphaFromSurfaceAsync(&*(&surface as *const <super::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::Abi>::Abi as *const <super::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::DefaultType>::DefaultType), alpha) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISoftwareBitmapStatics>, base.5, Copy::<Impl, OFFSET>, Convert::<Impl, OFFSET>, ConvertWithAlpha::<Impl, OFFSET>, CreateCopyFromBuffer::<Impl, OFFSET>, CreateCopyWithAlphaFromBuffer::<Impl, OFFSET>, CreateCopyFromSurfaceAsync::<Impl, OFFSET>, CreateCopyWithAlphaFromSurfaceAsync::<Impl, OFFSET>)
    }
}
