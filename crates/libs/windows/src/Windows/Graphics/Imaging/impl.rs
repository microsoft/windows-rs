#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBitmapBuffer_Impl: Sized + super::super::Foundation::IClosable_Impl + super::super::Foundation::IMemoryBuffer_Impl {
    fn GetPlaneCount(&mut self) -> ::windows::core::Result<i32>;
    fn GetPlaneDescription(&mut self, index: i32) -> ::windows::core::Result<BitmapPlaneDescription>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBitmapBuffer {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapBuffer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBitmapBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapBuffer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapBuffer_Vtbl {
        unsafe extern "system" fn GetPlaneCount<Impl: IBitmapBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPlaneCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPlaneDescription<Impl: IBitmapBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, result__: *mut BitmapPlaneDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPlaneDescription(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapBuffer, BASE_OFFSET>(),
            GetPlaneCount: GetPlaneCount::<Impl, IMPL_OFFSET>,
            GetPlaneDescription: GetPlaneDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBitmapCodecInformation_Impl: Sized {
    fn CodecId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn FileExtensions(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn FriendlyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MimeTypes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBitmapCodecInformation {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapCodecInformation";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBitmapCodecInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapCodecInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapCodecInformation_Vtbl {
        unsafe extern "system" fn CodecId<Impl: IBitmapCodecInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CodecId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileExtensions<Impl: IBitmapCodecInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FriendlyName<Impl: IBitmapCodecInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MimeTypes<Impl: IBitmapCodecInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MimeTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapCodecInformation, BASE_OFFSET>(),
            CodecId: CodecId::<Impl, IMPL_OFFSET>,
            FileExtensions: FileExtensions::<Impl, IMPL_OFFSET>,
            FriendlyName: FriendlyName::<Impl, IMPL_OFFSET>,
            MimeTypes: MimeTypes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapCodecInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IBitmapDecoder_Impl: Sized {
    fn BitmapContainerProperties(&mut self) -> ::windows::core::Result<BitmapPropertiesView>;
    fn DecoderInformation(&mut self) -> ::windows::core::Result<BitmapCodecInformation>;
    fn FrameCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetPreviewAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageStream>>;
    fn GetFrameAsync(&mut self, frameindex: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapFrame>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBitmapDecoder {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapDecoder";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IBitmapDecoder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapDecoder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapDecoder_Vtbl {
        unsafe extern "system" fn BitmapContainerProperties<Impl: IBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitmapContainerProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecoderInformation<Impl: IBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecoderInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameCount<Impl: IBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviewAsync<Impl: IBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreviewAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameAsync<Impl: IBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frameindex: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameAsync(frameindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapDecoder, BASE_OFFSET>(),
            BitmapContainerProperties: BitmapContainerProperties::<Impl, IMPL_OFFSET>,
            DecoderInformation: DecoderInformation::<Impl, IMPL_OFFSET>,
            FrameCount: FrameCount::<Impl, IMPL_OFFSET>,
            GetPreviewAsync: GetPreviewAsync::<Impl, IMPL_OFFSET>,
            GetFrameAsync: GetFrameAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapDecoder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IBitmapDecoderStatics_Impl: Sized {
    fn BmpDecoderId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn JpegDecoderId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn PngDecoderId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TiffDecoderId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GifDecoderId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn JpegXRDecoderId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IcoDecoderId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetDecoderInformationEnumerator(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<BitmapCodecInformation>>;
    fn CreateAsync(&mut self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapDecoder>>;
    fn CreateWithIdAsync(&mut self, decoderid: &::windows::core::GUID, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapDecoder>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBitmapDecoderStatics {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapDecoderStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IBitmapDecoderStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapDecoderStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapDecoderStatics_Vtbl {
        unsafe extern "system" fn BmpDecoderId<Impl: IBitmapDecoderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BmpDecoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JpegDecoderId<Impl: IBitmapDecoderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JpegDecoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PngDecoderId<Impl: IBitmapDecoderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PngDecoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TiffDecoderId<Impl: IBitmapDecoderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TiffDecoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GifDecoderId<Impl: IBitmapDecoderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GifDecoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JpegXRDecoderId<Impl: IBitmapDecoderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JpegXRDecoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IcoDecoderId<Impl: IBitmapDecoderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IcoDecoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDecoderInformationEnumerator<Impl: IBitmapDecoderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDecoderInformationEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAsync<Impl: IBitmapDecoderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAsync(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithIdAsync<Impl: IBitmapDecoderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, decoderid: ::windows::core::GUID, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithIdAsync(&*(&decoderid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapDecoderStatics, BASE_OFFSET>(),
            BmpDecoderId: BmpDecoderId::<Impl, IMPL_OFFSET>,
            JpegDecoderId: JpegDecoderId::<Impl, IMPL_OFFSET>,
            PngDecoderId: PngDecoderId::<Impl, IMPL_OFFSET>,
            TiffDecoderId: TiffDecoderId::<Impl, IMPL_OFFSET>,
            GifDecoderId: GifDecoderId::<Impl, IMPL_OFFSET>,
            JpegXRDecoderId: JpegXRDecoderId::<Impl, IMPL_OFFSET>,
            IcoDecoderId: IcoDecoderId::<Impl, IMPL_OFFSET>,
            GetDecoderInformationEnumerator: GetDecoderInformationEnumerator::<Impl, IMPL_OFFSET>,
            CreateAsync: CreateAsync::<Impl, IMPL_OFFSET>,
            CreateWithIdAsync: CreateWithIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapDecoderStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapDecoderStatics2_Impl: Sized {
    fn HeifDecoderId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn WebpDecoderId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapDecoderStatics2 {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapDecoderStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapDecoderStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapDecoderStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapDecoderStatics2_Vtbl {
        unsafe extern "system" fn HeifDecoderId<Impl: IBitmapDecoderStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeifDecoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WebpDecoderId<Impl: IBitmapDecoderStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WebpDecoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapDecoderStatics2, BASE_OFFSET>(),
            HeifDecoderId: HeifDecoderId::<Impl, IMPL_OFFSET>,
            WebpDecoderId: WebpDecoderId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapDecoderStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBitmapEncoder_Impl: Sized {
    fn EncoderInformation(&mut self) -> ::windows::core::Result<BitmapCodecInformation>;
    fn BitmapProperties(&mut self) -> ::windows::core::Result<BitmapProperties>;
    fn BitmapContainerProperties(&mut self) -> ::windows::core::Result<BitmapProperties>;
    fn IsThumbnailGenerated(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsThumbnailGenerated(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GeneratedThumbnailWidth(&mut self) -> ::windows::core::Result<u32>;
    fn SetGeneratedThumbnailWidth(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn GeneratedThumbnailHeight(&mut self) -> ::windows::core::Result<u32>;
    fn SetGeneratedThumbnailHeight(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn BitmapTransform(&mut self) -> ::windows::core::Result<BitmapTransform>;
    fn SetPixelData(&mut self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, width: u32, height: u32, dpix: f64, dpiy: f64, pixels: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GoToNextFrameAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GoToNextFrameWithEncodingOptionsAsync(&mut self, encodingoptions: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn FlushAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBitmapEncoder {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapEncoder";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBitmapEncoder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapEncoder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapEncoder_Vtbl {
        unsafe extern "system" fn EncoderInformation<Impl: IBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncoderInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapProperties<Impl: IBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitmapProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapContainerProperties<Impl: IBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitmapContainerProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThumbnailGenerated<Impl: IBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsThumbnailGenerated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsThumbnailGenerated<Impl: IBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsThumbnailGenerated(value).into()
        }
        unsafe extern "system" fn GeneratedThumbnailWidth<Impl: IBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GeneratedThumbnailWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeneratedThumbnailWidth<Impl: IBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGeneratedThumbnailWidth(value).into()
        }
        unsafe extern "system" fn GeneratedThumbnailHeight<Impl: IBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GeneratedThumbnailHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeneratedThumbnailHeight<Impl: IBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGeneratedThumbnailHeight(value).into()
        }
        unsafe extern "system" fn BitmapTransform<Impl: IBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitmapTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPixelData<Impl: IBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, width: u32, height: u32, dpix: f64, dpiy: f64, pixels_array_size: u32, pixels: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPixelData(pixelformat, alphamode, width, height, dpix, dpiy, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&pixels), pixels_array_size as _)).into()
        }
        unsafe extern "system" fn GoToNextFrameAsync<Impl: IBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GoToNextFrameAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GoToNextFrameWithEncodingOptionsAsync<Impl: IBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GoToNextFrameWithEncodingOptionsAsync(&*(&encodingoptions as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushAsync<Impl: IBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlushAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapEncoder, BASE_OFFSET>(),
            EncoderInformation: EncoderInformation::<Impl, IMPL_OFFSET>,
            BitmapProperties: BitmapProperties::<Impl, IMPL_OFFSET>,
            BitmapContainerProperties: BitmapContainerProperties::<Impl, IMPL_OFFSET>,
            IsThumbnailGenerated: IsThumbnailGenerated::<Impl, IMPL_OFFSET>,
            SetIsThumbnailGenerated: SetIsThumbnailGenerated::<Impl, IMPL_OFFSET>,
            GeneratedThumbnailWidth: GeneratedThumbnailWidth::<Impl, IMPL_OFFSET>,
            SetGeneratedThumbnailWidth: SetGeneratedThumbnailWidth::<Impl, IMPL_OFFSET>,
            GeneratedThumbnailHeight: GeneratedThumbnailHeight::<Impl, IMPL_OFFSET>,
            SetGeneratedThumbnailHeight: SetGeneratedThumbnailHeight::<Impl, IMPL_OFFSET>,
            BitmapTransform: BitmapTransform::<Impl, IMPL_OFFSET>,
            SetPixelData: SetPixelData::<Impl, IMPL_OFFSET>,
            GoToNextFrameAsync: GoToNextFrameAsync::<Impl, IMPL_OFFSET>,
            GoToNextFrameWithEncodingOptionsAsync: GoToNextFrameWithEncodingOptionsAsync::<Impl, IMPL_OFFSET>,
            FlushAsync: FlushAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapEncoder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IBitmapEncoderStatics_Impl: Sized {
    fn BmpEncoderId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn JpegEncoderId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn PngEncoderId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TiffEncoderId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GifEncoderId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn JpegXREncoderId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetEncoderInformationEnumerator(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<BitmapCodecInformation>>;
    fn CreateAsync(&mut self, encoderid: &::windows::core::GUID, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>>;
    fn CreateWithEncodingOptionsAsync(&mut self, encoderid: &::windows::core::GUID, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, encodingoptions: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>>;
    fn CreateForTranscodingAsync(&mut self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, bitmapdecoder: &::core::option::Option<BitmapDecoder>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>>;
    fn CreateForInPlacePropertyEncodingAsync(&mut self, bitmapdecoder: &::core::option::Option<BitmapDecoder>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBitmapEncoderStatics {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapEncoderStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IBitmapEncoderStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapEncoderStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapEncoderStatics_Vtbl {
        unsafe extern "system" fn BmpEncoderId<Impl: IBitmapEncoderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BmpEncoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JpegEncoderId<Impl: IBitmapEncoderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JpegEncoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PngEncoderId<Impl: IBitmapEncoderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PngEncoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TiffEncoderId<Impl: IBitmapEncoderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TiffEncoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GifEncoderId<Impl: IBitmapEncoderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GifEncoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JpegXREncoderId<Impl: IBitmapEncoderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JpegXREncoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEncoderInformationEnumerator<Impl: IBitmapEncoderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEncoderInformationEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAsync<Impl: IBitmapEncoderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoderid: ::windows::core::GUID, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAsync(&*(&encoderid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithEncodingOptionsAsync<Impl: IBitmapEncoderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoderid: ::windows::core::GUID, stream: ::windows::core::RawPtr, encodingoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn CreateForTranscodingAsync<Impl: IBitmapEncoderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, bitmapdecoder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForTranscodingAsync(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType), &*(&bitmapdecoder as *const <BitmapDecoder as ::windows::core::Abi>::Abi as *const <BitmapDecoder as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForInPlacePropertyEncodingAsync<Impl: IBitmapEncoderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmapdecoder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForInPlacePropertyEncodingAsync(&*(&bitmapdecoder as *const <BitmapDecoder as ::windows::core::Abi>::Abi as *const <BitmapDecoder as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapEncoderStatics, BASE_OFFSET>(),
            BmpEncoderId: BmpEncoderId::<Impl, IMPL_OFFSET>,
            JpegEncoderId: JpegEncoderId::<Impl, IMPL_OFFSET>,
            PngEncoderId: PngEncoderId::<Impl, IMPL_OFFSET>,
            TiffEncoderId: TiffEncoderId::<Impl, IMPL_OFFSET>,
            GifEncoderId: GifEncoderId::<Impl, IMPL_OFFSET>,
            JpegXREncoderId: JpegXREncoderId::<Impl, IMPL_OFFSET>,
            GetEncoderInformationEnumerator: GetEncoderInformationEnumerator::<Impl, IMPL_OFFSET>,
            CreateAsync: CreateAsync::<Impl, IMPL_OFFSET>,
            CreateWithEncodingOptionsAsync: CreateWithEncodingOptionsAsync::<Impl, IMPL_OFFSET>,
            CreateForTranscodingAsync: CreateForTranscodingAsync::<Impl, IMPL_OFFSET>,
            CreateForInPlacePropertyEncodingAsync: CreateForInPlacePropertyEncodingAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapEncoderStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapEncoderStatics2_Impl: Sized {
    fn HeifEncoderId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapEncoderStatics2 {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapEncoderStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapEncoderStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapEncoderStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapEncoderStatics2_Vtbl {
        unsafe extern "system" fn HeifEncoderId<Impl: IBitmapEncoderStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeifEncoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapEncoderStatics2, BASE_OFFSET>(),
            HeifEncoderId: HeifEncoderId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapEncoderStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapEncoderWithSoftwareBitmap_Impl: Sized {
    fn SetSoftwareBitmap(&mut self, bitmap: &::core::option::Option<SoftwareBitmap>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapEncoderWithSoftwareBitmap {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapEncoderWithSoftwareBitmap";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapEncoderWithSoftwareBitmap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapEncoderWithSoftwareBitmap_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapEncoderWithSoftwareBitmap_Vtbl {
        unsafe extern "system" fn SetSoftwareBitmap<Impl: IBitmapEncoderWithSoftwareBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSoftwareBitmap(&*(&bitmap as *const <SoftwareBitmap as ::windows::core::Abi>::Abi as *const <SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapEncoderWithSoftwareBitmap, BASE_OFFSET>(),
            SetSoftwareBitmap: SetSoftwareBitmap::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapEncoderWithSoftwareBitmap as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IBitmapFrame_Impl: Sized {
    fn GetThumbnailAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageStream>>;
    fn BitmapProperties(&mut self) -> ::windows::core::Result<BitmapPropertiesView>;
    fn BitmapPixelFormat(&mut self) -> ::windows::core::Result<BitmapPixelFormat>;
    fn BitmapAlphaMode(&mut self) -> ::windows::core::Result<BitmapAlphaMode>;
    fn DpiX(&mut self) -> ::windows::core::Result<f64>;
    fn DpiY(&mut self) -> ::windows::core::Result<f64>;
    fn PixelWidth(&mut self) -> ::windows::core::Result<u32>;
    fn PixelHeight(&mut self) -> ::windows::core::Result<u32>;
    fn OrientedPixelWidth(&mut self) -> ::windows::core::Result<u32>;
    fn OrientedPixelHeight(&mut self) -> ::windows::core::Result<u32>;
    fn GetPixelDataAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>>;
    fn GetPixelDataTransformedAsync(&mut self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: &::core::option::Option<BitmapTransform>, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IBitmapFrame {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapFrame";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl IBitmapFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapFrame_Vtbl {
        unsafe extern "system" fn GetThumbnailAsync<Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThumbnailAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapProperties<Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitmapProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapPixelFormat<Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BitmapPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitmapPixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapAlphaMode<Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BitmapAlphaMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitmapAlphaMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DpiX<Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DpiX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DpiY<Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DpiY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PixelWidth<Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PixelWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PixelHeight<Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PixelHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OrientedPixelWidth<Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OrientedPixelWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OrientedPixelHeight<Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OrientedPixelHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelDataAsync<Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPixelDataAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelDataTransformedAsync<Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: ::windows::core::RawPtr, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPixelDataTransformedAsync(pixelformat, alphamode, &*(&transform as *const <BitmapTransform as ::windows::core::Abi>::Abi as *const <BitmapTransform as ::windows::core::DefaultType>::DefaultType), exiforientationmode, colormanagementmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapFrame, BASE_OFFSET>(),
            GetThumbnailAsync: GetThumbnailAsync::<Impl, IMPL_OFFSET>,
            BitmapProperties: BitmapProperties::<Impl, IMPL_OFFSET>,
            BitmapPixelFormat: BitmapPixelFormat::<Impl, IMPL_OFFSET>,
            BitmapAlphaMode: BitmapAlphaMode::<Impl, IMPL_OFFSET>,
            DpiX: DpiX::<Impl, IMPL_OFFSET>,
            DpiY: DpiY::<Impl, IMPL_OFFSET>,
            PixelWidth: PixelWidth::<Impl, IMPL_OFFSET>,
            PixelHeight: PixelHeight::<Impl, IMPL_OFFSET>,
            OrientedPixelWidth: OrientedPixelWidth::<Impl, IMPL_OFFSET>,
            OrientedPixelHeight: OrientedPixelHeight::<Impl, IMPL_OFFSET>,
            GetPixelDataAsync: GetPixelDataAsync::<Impl, IMPL_OFFSET>,
            GetPixelDataTransformedAsync: GetPixelDataTransformedAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IBitmapFrameWithSoftwareBitmap_Impl: Sized + IBitmapFrame_Impl {
    fn GetSoftwareBitmapAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
    fn GetSoftwareBitmapConvertedAsync(&mut self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
    fn GetSoftwareBitmapTransformedAsync(&mut self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: &::core::option::Option<BitmapTransform>, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IBitmapFrameWithSoftwareBitmap {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapFrameWithSoftwareBitmap";
}
#[cfg(feature = "Foundation")]
impl IBitmapFrameWithSoftwareBitmap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapFrameWithSoftwareBitmap_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapFrameWithSoftwareBitmap_Vtbl {
        unsafe extern "system" fn GetSoftwareBitmapAsync<Impl: IBitmapFrameWithSoftwareBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSoftwareBitmapAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSoftwareBitmapConvertedAsync<Impl: IBitmapFrameWithSoftwareBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSoftwareBitmapConvertedAsync(pixelformat, alphamode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSoftwareBitmapTransformedAsync<Impl: IBitmapFrameWithSoftwareBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: ::windows::core::RawPtr, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSoftwareBitmapTransformedAsync(pixelformat, alphamode, &*(&transform as *const <BitmapTransform as ::windows::core::Abi>::Abi as *const <BitmapTransform as ::windows::core::DefaultType>::DefaultType), exiforientationmode, colormanagementmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapFrameWithSoftwareBitmap, BASE_OFFSET>(),
            GetSoftwareBitmapAsync: GetSoftwareBitmapAsync::<Impl, IMPL_OFFSET>,
            GetSoftwareBitmapConvertedAsync: GetSoftwareBitmapConvertedAsync::<Impl, IMPL_OFFSET>,
            GetSoftwareBitmapTransformedAsync: GetSoftwareBitmapTransformedAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapFrameWithSoftwareBitmap as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBitmapProperties_Impl: Sized + IBitmapPropertiesView_Impl {
    fn SetPropertiesAsync(&mut self, propertiestoset: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBitmapProperties {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapProperties";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBitmapProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapProperties_Vtbl {
        unsafe extern "system" fn SetPropertiesAsync<Impl: IBitmapProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertiestoset: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPropertiesAsync(&*(&propertiestoset as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapProperties, BASE_OFFSET>(),
            SetPropertiesAsync: SetPropertiesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
pub trait IBitmapPropertiesView_Impl: Sized {
    fn GetPropertiesAsync(&mut self, propertiestoretrieve: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapPropertySet>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::windows::core::RuntimeName for IBitmapPropertiesView {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapPropertiesView";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl IBitmapPropertiesView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapPropertiesView_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapPropertiesView_Vtbl {
        unsafe extern "system" fn GetPropertiesAsync<Impl: IBitmapPropertiesView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertiestoretrieve: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertiesAsync(&*(&propertiestoretrieve as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapPropertiesView, BASE_OFFSET>(),
            GetPropertiesAsync: GetPropertiesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapPropertiesView as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapTransform_Impl: Sized {
    fn ScaledWidth(&mut self) -> ::windows::core::Result<u32>;
    fn SetScaledWidth(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ScaledHeight(&mut self) -> ::windows::core::Result<u32>;
    fn SetScaledHeight(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn InterpolationMode(&mut self) -> ::windows::core::Result<BitmapInterpolationMode>;
    fn SetInterpolationMode(&mut self, value: BitmapInterpolationMode) -> ::windows::core::Result<()>;
    fn Flip(&mut self) -> ::windows::core::Result<BitmapFlip>;
    fn SetFlip(&mut self, value: BitmapFlip) -> ::windows::core::Result<()>;
    fn Rotation(&mut self) -> ::windows::core::Result<BitmapRotation>;
    fn SetRotation(&mut self, value: BitmapRotation) -> ::windows::core::Result<()>;
    fn Bounds(&mut self) -> ::windows::core::Result<BitmapBounds>;
    fn SetBounds(&mut self, value: &BitmapBounds) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapTransform {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapTransform";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapTransform_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapTransform_Vtbl {
        unsafe extern "system" fn ScaledWidth<Impl: IBitmapTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaledWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaledWidth<Impl: IBitmapTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaledWidth(value).into()
        }
        unsafe extern "system" fn ScaledHeight<Impl: IBitmapTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaledHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaledHeight<Impl: IBitmapTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaledHeight(value).into()
        }
        unsafe extern "system" fn InterpolationMode<Impl: IBitmapTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BitmapInterpolationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterpolationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterpolationMode<Impl: IBitmapTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: BitmapInterpolationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterpolationMode(value).into()
        }
        unsafe extern "system" fn Flip<Impl: IBitmapTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BitmapFlip) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlip<Impl: IBitmapTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: BitmapFlip) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFlip(value).into()
        }
        unsafe extern "system" fn Rotation<Impl: IBitmapTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BitmapRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rotation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotation<Impl: IBitmapTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: BitmapRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotation(value).into()
        }
        unsafe extern "system" fn Bounds<Impl: IBitmapTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BitmapBounds) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBounds<Impl: IBitmapTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: BitmapBounds) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBounds(&*(&value as *const <BitmapBounds as ::windows::core::Abi>::Abi as *const <BitmapBounds as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapTransform, BASE_OFFSET>(),
            ScaledWidth: ScaledWidth::<Impl, IMPL_OFFSET>,
            SetScaledWidth: SetScaledWidth::<Impl, IMPL_OFFSET>,
            ScaledHeight: ScaledHeight::<Impl, IMPL_OFFSET>,
            SetScaledHeight: SetScaledHeight::<Impl, IMPL_OFFSET>,
            InterpolationMode: InterpolationMode::<Impl, IMPL_OFFSET>,
            SetInterpolationMode: SetInterpolationMode::<Impl, IMPL_OFFSET>,
            Flip: Flip::<Impl, IMPL_OFFSET>,
            SetFlip: SetFlip::<Impl, IMPL_OFFSET>,
            Rotation: Rotation::<Impl, IMPL_OFFSET>,
            SetRotation: SetRotation::<Impl, IMPL_OFFSET>,
            Bounds: Bounds::<Impl, IMPL_OFFSET>,
            SetBounds: SetBounds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBitmapTypedValue_Impl: Sized {
    fn Value(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Type(&mut self) -> ::windows::core::Result<super::super::Foundation::PropertyType>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBitmapTypedValue {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapTypedValue";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBitmapTypedValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapTypedValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapTypedValue_Vtbl {
        unsafe extern "system" fn Value<Impl: IBitmapTypedValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IBitmapTypedValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::PropertyType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapTypedValue, BASE_OFFSET>(),
            Value: Value::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapTypedValue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBitmapTypedValueFactory_Impl: Sized {
    fn Create(&mut self, value: &::core::option::Option<::windows::core::IInspectable>, r#type: super::super::Foundation::PropertyType) -> ::windows::core::Result<BitmapTypedValue>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBitmapTypedValueFactory {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapTypedValueFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBitmapTypedValueFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapTypedValueFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapTypedValueFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IBitmapTypedValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, r#type: super::super::Foundation::PropertyType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapTypedValueFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapTypedValueFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPixelDataProvider_Impl: Sized {
    fn DetachPixelData(&mut self) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPixelDataProvider {
    const NAME: &'static str = "Windows.Graphics.Imaging.IPixelDataProvider";
}
#[cfg(feature = "implement_exclusive")]
impl IPixelDataProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPixelDataProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPixelDataProvider_Vtbl {
        unsafe extern "system" fn DetachPixelData<Impl: IPixelDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPixelDataProvider, BASE_OFFSET>(),
            DetachPixelData: DetachPixelData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPixelDataProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISoftwareBitmap_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn BitmapPixelFormat(&mut self) -> ::windows::core::Result<BitmapPixelFormat>;
    fn BitmapAlphaMode(&mut self) -> ::windows::core::Result<BitmapAlphaMode>;
    fn PixelWidth(&mut self) -> ::windows::core::Result<i32>;
    fn PixelHeight(&mut self) -> ::windows::core::Result<i32>;
    fn IsReadOnly(&mut self) -> ::windows::core::Result<bool>;
    fn SetDpiX(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn DpiX(&mut self) -> ::windows::core::Result<f64>;
    fn SetDpiY(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn DpiY(&mut self) -> ::windows::core::Result<f64>;
    fn LockBuffer(&mut self, mode: BitmapBufferAccessMode) -> ::windows::core::Result<BitmapBuffer>;
    fn CopyTo(&mut self, bitmap: &::core::option::Option<SoftwareBitmap>) -> ::windows::core::Result<()>;
    fn CopyFromBuffer(&mut self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn CopyToBuffer(&mut self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn GetReadOnlyView(&mut self) -> ::windows::core::Result<SoftwareBitmap>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISoftwareBitmap {
    const NAME: &'static str = "Windows.Graphics.Imaging.ISoftwareBitmap";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISoftwareBitmap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISoftwareBitmap_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISoftwareBitmap_Vtbl {
        unsafe extern "system" fn BitmapPixelFormat<Impl: ISoftwareBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BitmapPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitmapPixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapAlphaMode<Impl: ISoftwareBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BitmapAlphaMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitmapAlphaMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PixelWidth<Impl: ISoftwareBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PixelWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PixelHeight<Impl: ISoftwareBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PixelHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadOnly<Impl: ISoftwareBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDpiX<Impl: ISoftwareBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDpiX(value).into()
        }
        unsafe extern "system" fn DpiX<Impl: ISoftwareBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DpiX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDpiY<Impl: ISoftwareBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDpiY(value).into()
        }
        unsafe extern "system" fn DpiY<Impl: ISoftwareBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DpiY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockBuffer<Impl: ISoftwareBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: BitmapBufferAccessMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockBuffer(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTo<Impl: ISoftwareBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyTo(&*(&bitmap as *const <SoftwareBitmap as ::windows::core::Abi>::Abi as *const <SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CopyFromBuffer<Impl: ISoftwareBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyFromBuffer(&*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CopyToBuffer<Impl: ISoftwareBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyToBuffer(&*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetReadOnlyView<Impl: ISoftwareBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReadOnlyView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISoftwareBitmap, BASE_OFFSET>(),
            BitmapPixelFormat: BitmapPixelFormat::<Impl, IMPL_OFFSET>,
            BitmapAlphaMode: BitmapAlphaMode::<Impl, IMPL_OFFSET>,
            PixelWidth: PixelWidth::<Impl, IMPL_OFFSET>,
            PixelHeight: PixelHeight::<Impl, IMPL_OFFSET>,
            IsReadOnly: IsReadOnly::<Impl, IMPL_OFFSET>,
            SetDpiX: SetDpiX::<Impl, IMPL_OFFSET>,
            DpiX: DpiX::<Impl, IMPL_OFFSET>,
            SetDpiY: SetDpiY::<Impl, IMPL_OFFSET>,
            DpiY: DpiY::<Impl, IMPL_OFFSET>,
            LockBuffer: LockBuffer::<Impl, IMPL_OFFSET>,
            CopyTo: CopyTo::<Impl, IMPL_OFFSET>,
            CopyFromBuffer: CopyFromBuffer::<Impl, IMPL_OFFSET>,
            CopyToBuffer: CopyToBuffer::<Impl, IMPL_OFFSET>,
            GetReadOnlyView: GetReadOnlyView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISoftwareBitmap as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISoftwareBitmapFactory_Impl: Sized {
    fn Create(&mut self, format: BitmapPixelFormat, width: i32, height: i32) -> ::windows::core::Result<SoftwareBitmap>;
    fn CreateWithAlpha(&mut self, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> ::windows::core::Result<SoftwareBitmap>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISoftwareBitmapFactory {
    const NAME: &'static str = "Windows.Graphics.Imaging.ISoftwareBitmapFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISoftwareBitmapFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISoftwareBitmapFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISoftwareBitmapFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: ISoftwareBitmapFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: BitmapPixelFormat, width: i32, height: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(format, width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithAlpha<Impl: ISoftwareBitmapFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithAlpha(format, width, height, alpha) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISoftwareBitmapFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithAlpha: CreateWithAlpha::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISoftwareBitmapFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISoftwareBitmapStatics_Impl: Sized {
    fn Copy(&mut self, source: &::core::option::Option<SoftwareBitmap>) -> ::windows::core::Result<SoftwareBitmap>;
    fn Convert(&mut self, source: &::core::option::Option<SoftwareBitmap>, format: BitmapPixelFormat) -> ::windows::core::Result<SoftwareBitmap>;
    fn ConvertWithAlpha(&mut self, source: &::core::option::Option<SoftwareBitmap>, format: BitmapPixelFormat, alpha: BitmapAlphaMode) -> ::windows::core::Result<SoftwareBitmap>;
    fn CreateCopyFromBuffer(&mut self, source: &::core::option::Option<super::super::Storage::Streams::IBuffer>, format: BitmapPixelFormat, width: i32, height: i32) -> ::windows::core::Result<SoftwareBitmap>;
    fn CreateCopyWithAlphaFromBuffer(&mut self, source: &::core::option::Option<super::super::Storage::Streams::IBuffer>, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> ::windows::core::Result<SoftwareBitmap>;
    fn CreateCopyFromSurfaceAsync(&mut self, surface: &::core::option::Option<super::DirectX::Direct3D11::IDirect3DSurface>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
    fn CreateCopyWithAlphaFromSurfaceAsync(&mut self, surface: &::core::option::Option<super::DirectX::Direct3D11::IDirect3DSurface>, alpha: BitmapAlphaMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISoftwareBitmapStatics {
    const NAME: &'static str = "Windows.Graphics.Imaging.ISoftwareBitmapStatics";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISoftwareBitmapStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISoftwareBitmapStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISoftwareBitmapStatics_Vtbl {
        unsafe extern "system" fn Copy<Impl: ISoftwareBitmapStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Copy(&*(&source as *const <SoftwareBitmap as ::windows::core::Abi>::Abi as *const <SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Convert<Impl: ISoftwareBitmapStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, format: BitmapPixelFormat, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Convert(&*(&source as *const <SoftwareBitmap as ::windows::core::Abi>::Abi as *const <SoftwareBitmap as ::windows::core::DefaultType>::DefaultType), format) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertWithAlpha<Impl: ISoftwareBitmapStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, format: BitmapPixelFormat, alpha: BitmapAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertWithAlpha(&*(&source as *const <SoftwareBitmap as ::windows::core::Abi>::Abi as *const <SoftwareBitmap as ::windows::core::DefaultType>::DefaultType), format, alpha) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCopyFromBuffer<Impl: ISoftwareBitmapStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, format: BitmapPixelFormat, width: i32, height: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCopyFromBuffer(&*(&source as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), format, width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCopyWithAlphaFromBuffer<Impl: ISoftwareBitmapStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCopyWithAlphaFromBuffer(&*(&source as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), format, width, height, alpha) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCopyFromSurfaceAsync<Impl: ISoftwareBitmapStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, surface: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCopyFromSurfaceAsync(&*(&surface as *const <super::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::Abi>::Abi as *const <super::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCopyWithAlphaFromSurfaceAsync<Impl: ISoftwareBitmapStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, surface: ::windows::core::RawPtr, alpha: BitmapAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCopyWithAlphaFromSurfaceAsync(&*(&surface as *const <super::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::Abi>::Abi as *const <super::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::DefaultType>::DefaultType), alpha) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISoftwareBitmapStatics, BASE_OFFSET>(),
            Copy: Copy::<Impl, IMPL_OFFSET>,
            Convert: Convert::<Impl, IMPL_OFFSET>,
            ConvertWithAlpha: ConvertWithAlpha::<Impl, IMPL_OFFSET>,
            CreateCopyFromBuffer: CreateCopyFromBuffer::<Impl, IMPL_OFFSET>,
            CreateCopyWithAlphaFromBuffer: CreateCopyWithAlphaFromBuffer::<Impl, IMPL_OFFSET>,
            CreateCopyFromSurfaceAsync: CreateCopyFromSurfaceAsync::<Impl, IMPL_OFFSET>,
            CreateCopyWithAlphaFromSurfaceAsync: CreateCopyWithAlphaFromSurfaceAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISoftwareBitmapStatics as ::windows::core::Interface>::IID
    }
}
