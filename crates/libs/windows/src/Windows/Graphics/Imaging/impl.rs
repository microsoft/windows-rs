#[doc = "*Required features: `\"Graphics_Imaging\"`, `\"Foundation\"`, `\"Storage_Streams\"`, `\"implement\"`*"]
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IBitmapFrame_Impl: Sized {
    fn GetThumbnailAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ImageStream>>;
    fn BitmapProperties(&self) -> ::windows_core::Result<BitmapPropertiesView>;
    fn BitmapPixelFormat(&self) -> ::windows_core::Result<BitmapPixelFormat>;
    fn BitmapAlphaMode(&self) -> ::windows_core::Result<BitmapAlphaMode>;
    fn DpiX(&self) -> ::windows_core::Result<f64>;
    fn DpiY(&self) -> ::windows_core::Result<f64>;
    fn PixelWidth(&self) -> ::windows_core::Result<u32>;
    fn PixelHeight(&self) -> ::windows_core::Result<u32>;
    fn OrientedPixelWidth(&self) -> ::windows_core::Result<u32>;
    fn OrientedPixelHeight(&self) -> ::windows_core::Result<u32>;
    fn GetPixelDataAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>>;
    fn GetPixelDataTransformedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: ::core::option::Option<&BitmapTransform>, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows_core::RuntimeName for IBitmapFrame {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapFrame";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl IBitmapFrame_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>() -> IBitmapFrame_Vtbl {
        unsafe extern "system" fn GetThumbnailAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetThumbnailAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BitmapProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapPixelFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BitmapPixelFormat) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BitmapPixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapAlphaMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BitmapAlphaMode) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BitmapAlphaMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DpiX<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DpiX() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DpiY<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DpiY() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PixelWidth<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PixelWidth() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PixelHeight<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PixelHeight() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OrientedPixelWidth<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OrientedPixelWidth() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OrientedPixelHeight<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OrientedPixelHeight() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelDataAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPixelDataAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelDataTransformedAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: *mut ::core::ffi::c_void, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPixelDataTransformedAsync(pixelformat, alphamode, ::windows_core::from_raw_borrowed(&transform), exiforientationmode, colormanagementmode) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IBitmapFrame, OFFSET>(),
            GetThumbnailAsync: GetThumbnailAsync::<Identity, Impl, OFFSET>,
            BitmapProperties: BitmapProperties::<Identity, Impl, OFFSET>,
            BitmapPixelFormat: BitmapPixelFormat::<Identity, Impl, OFFSET>,
            BitmapAlphaMode: BitmapAlphaMode::<Identity, Impl, OFFSET>,
            DpiX: DpiX::<Identity, Impl, OFFSET>,
            DpiY: DpiY::<Identity, Impl, OFFSET>,
            PixelWidth: PixelWidth::<Identity, Impl, OFFSET>,
            PixelHeight: PixelHeight::<Identity, Impl, OFFSET>,
            OrientedPixelWidth: OrientedPixelWidth::<Identity, Impl, OFFSET>,
            OrientedPixelHeight: OrientedPixelHeight::<Identity, Impl, OFFSET>,
            GetPixelDataAsync: GetPixelDataAsync::<Identity, Impl, OFFSET>,
            GetPixelDataTransformedAsync: GetPixelDataTransformedAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IBitmapFrame as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Graphics_Imaging\"`, `\"Foundation\"`, `\"Storage_Streams\"`, `\"implement\"`*"]
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IBitmapFrameWithSoftwareBitmap_Impl: Sized + IBitmapFrame_Impl {
    fn GetSoftwareBitmapAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
    fn GetSoftwareBitmapConvertedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
    fn GetSoftwareBitmapTransformedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: ::core::option::Option<&BitmapTransform>, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows_core::RuntimeName for IBitmapFrameWithSoftwareBitmap {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapFrameWithSoftwareBitmap";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl IBitmapFrameWithSoftwareBitmap_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrameWithSoftwareBitmap_Impl, const OFFSET: isize>() -> IBitmapFrameWithSoftwareBitmap_Vtbl {
        unsafe extern "system" fn GetSoftwareBitmapAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrameWithSoftwareBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSoftwareBitmapAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSoftwareBitmapConvertedAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrameWithSoftwareBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSoftwareBitmapConvertedAsync(pixelformat, alphamode) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSoftwareBitmapTransformedAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrameWithSoftwareBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: *mut ::core::ffi::c_void, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSoftwareBitmapTransformedAsync(pixelformat, alphamode, ::windows_core::from_raw_borrowed(&transform), exiforientationmode, colormanagementmode) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IBitmapFrameWithSoftwareBitmap, OFFSET>(),
            GetSoftwareBitmapAsync: GetSoftwareBitmapAsync::<Identity, Impl, OFFSET>,
            GetSoftwareBitmapConvertedAsync: GetSoftwareBitmapConvertedAsync::<Identity, Impl, OFFSET>,
            GetSoftwareBitmapTransformedAsync: GetSoftwareBitmapTransformedAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IBitmapFrameWithSoftwareBitmap as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Graphics_Imaging\"`, `\"Foundation_Collections\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation_Collections")]
pub trait IBitmapPropertiesView_Impl: Sized {
    fn GetPropertiesAsync(&self, propertiestoretrieve: ::core::option::Option<&super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<BitmapPropertySet>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for IBitmapPropertiesView {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapPropertiesView";
}
#[cfg(feature = "Foundation_Collections")]
impl IBitmapPropertiesView_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapPropertiesView_Impl, const OFFSET: isize>() -> IBitmapPropertiesView_Vtbl {
        unsafe extern "system" fn GetPropertiesAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapPropertiesView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertiestoretrieve: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertiesAsync(::windows_core::from_raw_borrowed(&propertiestoretrieve)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IBitmapPropertiesView, OFFSET>(),
            GetPropertiesAsync: GetPropertiesAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IBitmapPropertiesView as ::windows_core::ComInterface>::IID
    }
}
