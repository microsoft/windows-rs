#[cfg(feature = "implement_exclusive")]
pub trait IPdfDocumentImpl: Sized {
    fn GetPage(&mut self, pageindex: u32) -> ::windows::core::Result<PdfPage>;
    fn PageCount(&mut self) -> ::windows::core::Result<u32>;
    fn IsPasswordProtected(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPdfDocument {
    const NAME: &'static str = "Windows.Data.Pdf.IPdfDocument";
}
#[cfg(feature = "implement_exclusive")]
impl IPdfDocumentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPdfDocumentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPdfDocumentVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPdfDocument, BASE_OFFSET>(),
            GetPage: GetPage::<Impl, IMPL_OFFSET>,
            PageCount: PageCount::<Impl, IMPL_OFFSET>,
            IsPasswordProtected: IsPasswordProtected::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPdfDocument as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPdfDocumentStaticsImpl: Sized {
    fn LoadFromFileAsync(&mut self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PdfDocument>>;
    fn LoadFromFileWithPasswordAsync(&mut self, file: &::core::option::Option<super::super::Storage::IStorageFile>, password: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PdfDocument>>;
    fn LoadFromStreamAsync(&mut self, inputstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PdfDocument>>;
    fn LoadFromStreamWithPasswordAsync(&mut self, inputstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, password: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PdfDocument>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPdfDocumentStatics {
    const NAME: &'static str = "Windows.Data.Pdf.IPdfDocumentStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPdfDocumentStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPdfDocumentStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPdfDocumentStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPdfDocumentStatics, BASE_OFFSET>(),
            LoadFromFileAsync: LoadFromFileAsync::<Impl, IMPL_OFFSET>,
            LoadFromFileWithPasswordAsync: LoadFromFileWithPasswordAsync::<Impl, IMPL_OFFSET>,
            LoadFromStreamAsync: LoadFromStreamAsync::<Impl, IMPL_OFFSET>,
            LoadFromStreamWithPasswordAsync: LoadFromStreamWithPasswordAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPdfDocumentStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPdfPageImpl: Sized {
    fn RenderToStreamAsync(&mut self, outputstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RenderWithOptionsToStreamAsync(&mut self, outputstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, options: &::core::option::Option<PdfPageRenderOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn PreparePageAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Index(&mut self) -> ::windows::core::Result<u32>;
    fn Size(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn Dimensions(&mut self) -> ::windows::core::Result<PdfPageDimensions>;
    fn Rotation(&mut self) -> ::windows::core::Result<PdfPageRotation>;
    fn PreferredZoom(&mut self) -> ::windows::core::Result<f32>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPdfPage {
    const NAME: &'static str = "Windows.Data.Pdf.IPdfPage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPdfPageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPdfPageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPdfPageVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPdfPage, BASE_OFFSET>(),
            RenderToStreamAsync: RenderToStreamAsync::<Impl, IMPL_OFFSET>,
            RenderWithOptionsToStreamAsync: RenderWithOptionsToStreamAsync::<Impl, IMPL_OFFSET>,
            PreparePageAsync: PreparePageAsync::<Impl, IMPL_OFFSET>,
            Index: Index::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
            Dimensions: Dimensions::<Impl, IMPL_OFFSET>,
            Rotation: Rotation::<Impl, IMPL_OFFSET>,
            PreferredZoom: PreferredZoom::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPdfPage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPdfPageDimensionsImpl: Sized {
    fn MediaBox(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn CropBox(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn BleedBox(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn TrimBox(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn ArtBox(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPdfPageDimensions {
    const NAME: &'static str = "Windows.Data.Pdf.IPdfPageDimensions";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPdfPageDimensionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPdfPageDimensionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPdfPageDimensionsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPdfPageDimensions, BASE_OFFSET>(),
            MediaBox: MediaBox::<Impl, IMPL_OFFSET>,
            CropBox: CropBox::<Impl, IMPL_OFFSET>,
            BleedBox: BleedBox::<Impl, IMPL_OFFSET>,
            TrimBox: TrimBox::<Impl, IMPL_OFFSET>,
            ArtBox: ArtBox::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPdfPageDimensions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI", feature = "implement_exclusive"))]
pub trait IPdfPageRenderOptionsImpl: Sized {
    fn SourceRect(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetSourceRect(&mut self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn DestinationWidth(&mut self) -> ::windows::core::Result<u32>;
    fn SetDestinationWidth(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn DestinationHeight(&mut self) -> ::windows::core::Result<u32>;
    fn SetDestinationHeight(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn BackgroundColor(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetBackgroundColor(&mut self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn IsIgnoringHighContrast(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsIgnoringHighContrast(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn BitmapEncoderId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetBitmapEncoderId(&mut self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPdfPageRenderOptions {
    const NAME: &'static str = "Windows.Data.Pdf.IPdfPageRenderOptions";
}
#[cfg(all(feature = "Foundation", feature = "UI", feature = "implement_exclusive"))]
impl IPdfPageRenderOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPdfPageRenderOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPdfPageRenderOptionsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPdfPageRenderOptions, BASE_OFFSET>(),
            SourceRect: SourceRect::<Impl, IMPL_OFFSET>,
            SetSourceRect: SetSourceRect::<Impl, IMPL_OFFSET>,
            DestinationWidth: DestinationWidth::<Impl, IMPL_OFFSET>,
            SetDestinationWidth: SetDestinationWidth::<Impl, IMPL_OFFSET>,
            DestinationHeight: DestinationHeight::<Impl, IMPL_OFFSET>,
            SetDestinationHeight: SetDestinationHeight::<Impl, IMPL_OFFSET>,
            BackgroundColor: BackgroundColor::<Impl, IMPL_OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Impl, IMPL_OFFSET>,
            IsIgnoringHighContrast: IsIgnoringHighContrast::<Impl, IMPL_OFFSET>,
            SetIsIgnoringHighContrast: SetIsIgnoringHighContrast::<Impl, IMPL_OFFSET>,
            BitmapEncoderId: BitmapEncoderId::<Impl, IMPL_OFFSET>,
            SetBitmapEncoderId: SetBitmapEncoderId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPdfPageRenderOptions as ::windows::core::Interface>::IID
    }
}
