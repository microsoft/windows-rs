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
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IBitmapFrameWithSoftwareBitmap_Impl: Sized + IBitmapFrame_Impl {
    fn GetSoftwareBitmapAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
    fn GetSoftwareBitmapConvertedAsync(&mut self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
    fn GetSoftwareBitmapTransformedAsync(&mut self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: &::core::option::Option<BitmapTransform>, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IBitmapFrameWithSoftwareBitmap {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapFrameWithSoftwareBitmap";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
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
