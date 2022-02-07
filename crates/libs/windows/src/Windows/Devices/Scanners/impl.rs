pub trait IImageScannerFormatConfiguration_Impl: Sized {
    fn DefaultFormat(&self) -> ::windows::core::Result<ImageScannerFormat>;
    fn Format(&self) -> ::windows::core::Result<ImageScannerFormat>;
    fn SetFormat(&self, value: ImageScannerFormat) -> ::windows::core::Result<()>;
    fn IsFormatSupported(&self, value: ImageScannerFormat) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IImageScannerFormatConfiguration {
    const NAME: &'static str = "Windows.Devices.Scanners.IImageScannerFormatConfiguration";
}
impl IImageScannerFormatConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerFormatConfiguration_Impl, const OFFSET: isize>() -> IImageScannerFormatConfiguration_Vtbl {
        unsafe extern "system" fn DefaultFormat<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerFormatConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Format<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerFormatConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerFormatConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ImageScannerFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFormat(value).into()
        }
        unsafe extern "system" fn IsFormatSupported<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerFormatConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ImageScannerFormat, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsFormatSupported(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IImageScannerFormatConfiguration, OFFSET>(),
            DefaultFormat: DefaultFormat::<Identity, Impl, OFFSET>,
            Format: Format::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
            IsFormatSupported: IsFormatSupported::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageScannerFormatConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IImageScannerSourceConfiguration_Impl: Sized + IImageScannerFormatConfiguration_Impl {
    fn MinScanArea(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn MaxScanArea(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SelectedScanRegion(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetSelectedScanRegion(&self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn AutoCroppingMode(&self) -> ::windows::core::Result<ImageScannerAutoCroppingMode>;
    fn SetAutoCroppingMode(&self, value: ImageScannerAutoCroppingMode) -> ::windows::core::Result<()>;
    fn IsAutoCroppingModeSupported(&self, value: ImageScannerAutoCroppingMode) -> ::windows::core::Result<bool>;
    fn MinResolution(&self) -> ::windows::core::Result<ImageScannerResolution>;
    fn MaxResolution(&self) -> ::windows::core::Result<ImageScannerResolution>;
    fn OpticalResolution(&self) -> ::windows::core::Result<ImageScannerResolution>;
    fn DesiredResolution(&self) -> ::windows::core::Result<ImageScannerResolution>;
    fn SetDesiredResolution(&self, value: &ImageScannerResolution) -> ::windows::core::Result<()>;
    fn ActualResolution(&self) -> ::windows::core::Result<ImageScannerResolution>;
    fn DefaultColorMode(&self) -> ::windows::core::Result<ImageScannerColorMode>;
    fn ColorMode(&self) -> ::windows::core::Result<ImageScannerColorMode>;
    fn SetColorMode(&self, value: ImageScannerColorMode) -> ::windows::core::Result<()>;
    fn IsColorModeSupported(&self, value: ImageScannerColorMode) -> ::windows::core::Result<bool>;
    fn MinBrightness(&self) -> ::windows::core::Result<i32>;
    fn MaxBrightness(&self) -> ::windows::core::Result<i32>;
    fn BrightnessStep(&self) -> ::windows::core::Result<u32>;
    fn DefaultBrightness(&self) -> ::windows::core::Result<i32>;
    fn Brightness(&self) -> ::windows::core::Result<i32>;
    fn SetBrightness(&self, value: i32) -> ::windows::core::Result<()>;
    fn MinContrast(&self) -> ::windows::core::Result<i32>;
    fn MaxContrast(&self) -> ::windows::core::Result<i32>;
    fn ContrastStep(&self) -> ::windows::core::Result<u32>;
    fn DefaultContrast(&self) -> ::windows::core::Result<i32>;
    fn Contrast(&self) -> ::windows::core::Result<i32>;
    fn SetContrast(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IImageScannerSourceConfiguration {
    const NAME: &'static str = "Windows.Devices.Scanners.IImageScannerSourceConfiguration";
}
#[cfg(feature = "Foundation")]
impl IImageScannerSourceConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>() -> IImageScannerSourceConfiguration_Vtbl {
        unsafe extern "system" fn MinScanArea<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MinScanArea() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxScanArea<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxScanArea() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedScanRegion<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelectedScanRegion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedScanRegion<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSelectedScanRegion(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn AutoCroppingMode<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerAutoCroppingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoCroppingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoCroppingMode<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ImageScannerAutoCroppingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoCroppingMode(value).into()
        }
        unsafe extern "system" fn IsAutoCroppingModeSupported<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ImageScannerAutoCroppingMode, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsAutoCroppingModeSupported(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinResolution<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MinResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxResolution<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpticalResolution<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpticalResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredResolution<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DesiredResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredResolution<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ImageScannerResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDesiredResolution(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ActualResolution<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ActualResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultColorMode<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerColorMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultColorMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorMode<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerColorMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ColorMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorMode<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ImageScannerColorMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetColorMode(value).into()
        }
        unsafe extern "system" fn IsColorModeSupported<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ImageScannerColorMode, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsColorModeSupported(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinBrightness<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MinBrightness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxBrightness<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxBrightness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BrightnessStep<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BrightnessStep() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultBrightness<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultBrightness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Brightness<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Brightness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBrightness<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBrightness(value).into()
        }
        unsafe extern "system" fn MinContrast<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MinContrast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxContrast<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxContrast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContrastStep<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ContrastStep() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultContrast<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultContrast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contrast<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Contrast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContrast<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetContrast(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IImageScannerSourceConfiguration, OFFSET>(),
            MinScanArea: MinScanArea::<Identity, Impl, OFFSET>,
            MaxScanArea: MaxScanArea::<Identity, Impl, OFFSET>,
            SelectedScanRegion: SelectedScanRegion::<Identity, Impl, OFFSET>,
            SetSelectedScanRegion: SetSelectedScanRegion::<Identity, Impl, OFFSET>,
            AutoCroppingMode: AutoCroppingMode::<Identity, Impl, OFFSET>,
            SetAutoCroppingMode: SetAutoCroppingMode::<Identity, Impl, OFFSET>,
            IsAutoCroppingModeSupported: IsAutoCroppingModeSupported::<Identity, Impl, OFFSET>,
            MinResolution: MinResolution::<Identity, Impl, OFFSET>,
            MaxResolution: MaxResolution::<Identity, Impl, OFFSET>,
            OpticalResolution: OpticalResolution::<Identity, Impl, OFFSET>,
            DesiredResolution: DesiredResolution::<Identity, Impl, OFFSET>,
            SetDesiredResolution: SetDesiredResolution::<Identity, Impl, OFFSET>,
            ActualResolution: ActualResolution::<Identity, Impl, OFFSET>,
            DefaultColorMode: DefaultColorMode::<Identity, Impl, OFFSET>,
            ColorMode: ColorMode::<Identity, Impl, OFFSET>,
            SetColorMode: SetColorMode::<Identity, Impl, OFFSET>,
            IsColorModeSupported: IsColorModeSupported::<Identity, Impl, OFFSET>,
            MinBrightness: MinBrightness::<Identity, Impl, OFFSET>,
            MaxBrightness: MaxBrightness::<Identity, Impl, OFFSET>,
            BrightnessStep: BrightnessStep::<Identity, Impl, OFFSET>,
            DefaultBrightness: DefaultBrightness::<Identity, Impl, OFFSET>,
            Brightness: Brightness::<Identity, Impl, OFFSET>,
            SetBrightness: SetBrightness::<Identity, Impl, OFFSET>,
            MinContrast: MinContrast::<Identity, Impl, OFFSET>,
            MaxContrast: MaxContrast::<Identity, Impl, OFFSET>,
            ContrastStep: ContrastStep::<Identity, Impl, OFFSET>,
            DefaultContrast: DefaultContrast::<Identity, Impl, OFFSET>,
            Contrast: Contrast::<Identity, Impl, OFFSET>,
            SetContrast: SetContrast::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageScannerSourceConfiguration as ::windows::core::Interface>::IID
    }
}
