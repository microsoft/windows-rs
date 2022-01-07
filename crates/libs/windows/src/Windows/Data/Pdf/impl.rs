#[cfg(feature = "implement_exclusive")]
pub trait IPdfDocumentImpl: Sized {
    fn GetPage(&self, pageindex: u32) -> ::windows::core::Result<PdfPage>;
    fn PageCount(&self) -> ::windows::core::Result<u32>;
    fn IsPasswordProtected(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPdfDocument {
    const NAME: &'static str = "Windows.Data.Pdf.IPdfDocument";
}
#[cfg(feature = "implement_exclusive")]
impl IPdfDocumentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPdfDocumentImpl, const OFFSET: isize>() -> IPdfDocumentVtbl {
        unsafe extern "system" fn GetPage<Impl: IPdfDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pageindex: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPage(pageindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageCount<Impl: IPdfDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPasswordProtected<Impl: IPdfDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPasswordProtected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPdfDocument>, ::windows::core::GetTrustLevel, GetPage::<Impl, OFFSET>, PageCount::<Impl, OFFSET>, IsPasswordProtected::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPdfDocumentStaticsImpl: Sized {
    fn LoadFromFileAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PdfDocument>>;
    fn LoadFromFileWithPasswordAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>, password: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PdfDocument>>;
    fn LoadFromStreamAsync(&self, inputstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PdfDocument>>;
    fn LoadFromStreamWithPasswordAsync(&self, inputstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, password: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PdfDocument>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPdfDocumentStatics {
    const NAME: &'static str = "Windows.Data.Pdf.IPdfDocumentStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPdfDocumentStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPdfDocumentStaticsImpl, const OFFSET: isize>() -> IPdfDocumentStaticsVtbl {
        unsafe extern "system" fn LoadFromFileAsync<Impl: IPdfDocumentStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadFromFileAsync(&*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadFromFileWithPasswordAsync<Impl: IPdfDocumentStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadFromFileWithPasswordAsync(&*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType), &*(&password as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadFromStreamAsync<Impl: IPdfDocumentStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadFromStreamAsync(&*(&inputstream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadFromStreamWithPasswordAsync<Impl: IPdfDocumentStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadFromStreamWithPasswordAsync(&*(&inputstream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType), &*(&password as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPdfDocumentStatics>, ::windows::core::GetTrustLevel, LoadFromFileAsync::<Impl, OFFSET>, LoadFromFileWithPasswordAsync::<Impl, OFFSET>, LoadFromStreamAsync::<Impl, OFFSET>, LoadFromStreamWithPasswordAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPdfPageImpl: Sized {
    fn RenderToStreamAsync(&self, outputstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RenderWithOptionsToStreamAsync(&self, outputstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, options: &::core::option::Option<PdfPageRenderOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn PreparePageAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Index(&self) -> ::windows::core::Result<u32>;
    fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn Dimensions(&self) -> ::windows::core::Result<PdfPageDimensions>;
    fn Rotation(&self) -> ::windows::core::Result<PdfPageRotation>;
    fn PreferredZoom(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPdfPage {
    const NAME: &'static str = "Windows.Data.Pdf.IPdfPage";
}
#[cfg(feature = "implement_exclusive")]
impl IPdfPageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPdfPageImpl, const OFFSET: isize>() -> IPdfPageVtbl {
        unsafe extern "system" fn RenderToStreamAsync<Impl: IPdfPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderToStreamAsync(&*(&outputstream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenderWithOptionsToStreamAsync<Impl: IPdfPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderWithOptionsToStreamAsync(&*(&outputstream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <PdfPageRenderOptions as ::windows::core::Abi>::Abi as *const <PdfPageRenderOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreparePageAsync<Impl: IPdfPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreparePageAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Index<Impl: IPdfPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Index() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IPdfPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dimensions<Impl: IPdfPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Dimensions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rotation<Impl: IPdfPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PdfPageRotation) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PreferredZoom<Impl: IPdfPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredZoom() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPdfPage>,
            ::windows::core::GetTrustLevel,
            RenderToStreamAsync::<Impl, OFFSET>,
            RenderWithOptionsToStreamAsync::<Impl, OFFSET>,
            PreparePageAsync::<Impl, OFFSET>,
            Index::<Impl, OFFSET>,
            Size::<Impl, OFFSET>,
            Dimensions::<Impl, OFFSET>,
            Rotation::<Impl, OFFSET>,
            PreferredZoom::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPdfPageDimensionsImpl: Sized {
    fn MediaBox(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn CropBox(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn BleedBox(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn TrimBox(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn ArtBox(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPdfPageDimensions {
    const NAME: &'static str = "Windows.Data.Pdf.IPdfPageDimensions";
}
#[cfg(feature = "implement_exclusive")]
impl IPdfPageDimensionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPdfPageDimensionsImpl, const OFFSET: isize>() -> IPdfPageDimensionsVtbl {
        unsafe extern "system" fn MediaBox<Impl: IPdfPageDimensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaBox() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CropBox<Impl: IPdfPageDimensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CropBox() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BleedBox<Impl: IPdfPageDimensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BleedBox() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrimBox<Impl: IPdfPageDimensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrimBox() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArtBox<Impl: IPdfPageDimensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArtBox() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPdfPageDimensions>, ::windows::core::GetTrustLevel, MediaBox::<Impl, OFFSET>, CropBox::<Impl, OFFSET>, BleedBox::<Impl, OFFSET>, TrimBox::<Impl, OFFSET>, ArtBox::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPdfPageRenderOptionsImpl: Sized {
    fn SourceRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetSourceRect(&self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn DestinationWidth(&self) -> ::windows::core::Result<u32>;
    fn SetDestinationWidth(&self, value: u32) -> ::windows::core::Result<()>;
    fn DestinationHeight(&self) -> ::windows::core::Result<u32>;
    fn SetDestinationHeight(&self, value: u32) -> ::windows::core::Result<()>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetBackgroundColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn IsIgnoringHighContrast(&self) -> ::windows::core::Result<bool>;
    fn SetIsIgnoringHighContrast(&self, value: bool) -> ::windows::core::Result<()>;
    fn BitmapEncoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetBitmapEncoderId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPdfPageRenderOptions {
    const NAME: &'static str = "Windows.Data.Pdf.IPdfPageRenderOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IPdfPageRenderOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPdfPageRenderOptionsImpl, const OFFSET: isize>() -> IPdfPageRenderOptionsVtbl {
        unsafe extern "system" fn SourceRect<Impl: IPdfPageRenderOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceRect<Impl: IPdfPageRenderOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourceRect(&*(&value as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DestinationWidth<Impl: IPdfPageRenderOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationWidth<Impl: IPdfPageRenderOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDestinationWidth(value).into()
        }
        unsafe extern "system" fn DestinationHeight<Impl: IPdfPageRenderOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationHeight<Impl: IPdfPageRenderOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDestinationHeight(value).into()
        }
        unsafe extern "system" fn BackgroundColor<Impl: IPdfPageRenderOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Impl: IPdfPageRenderOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackgroundColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsIgnoringHighContrast<Impl: IPdfPageRenderOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsIgnoringHighContrast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsIgnoringHighContrast<Impl: IPdfPageRenderOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsIgnoringHighContrast(value).into()
        }
        unsafe extern "system" fn BitmapEncoderId<Impl: IPdfPageRenderOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitmapEncoderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitmapEncoderId<Impl: IPdfPageRenderOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitmapEncoderId(&*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPdfPageRenderOptions>,
            ::windows::core::GetTrustLevel,
            SourceRect::<Impl, OFFSET>,
            SetSourceRect::<Impl, OFFSET>,
            DestinationWidth::<Impl, OFFSET>,
            SetDestinationWidth::<Impl, OFFSET>,
            DestinationHeight::<Impl, OFFSET>,
            SetDestinationHeight::<Impl, OFFSET>,
            BackgroundColor::<Impl, OFFSET>,
            SetBackgroundColor::<Impl, OFFSET>,
            IsIgnoringHighContrast::<Impl, OFFSET>,
            SetIsIgnoringHighContrast::<Impl, OFFSET>,
            BitmapEncoderId::<Impl, OFFSET>,
            SetBitmapEncoderId::<Impl, OFFSET>,
        )
    }
}
