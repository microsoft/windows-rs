#[cfg(feature = "implement_exclusive")]
pub trait IBitmapImageImpl: Sized {
    fn CreateOptions(&self) -> ::windows::core::Result<BitmapCreateOptions>;
    fn SetCreateOptions(&self, value: BitmapCreateOptions) -> ::windows::core::Result<()>;
    fn UriSource(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn SetUriSource(&self, value: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn DecodePixelWidth(&self) -> ::windows::core::Result<i32>;
    fn SetDecodePixelWidth(&self, value: i32) -> ::windows::core::Result<()>;
    fn DecodePixelHeight(&self) -> ::windows::core::Result<i32>;
    fn SetDecodePixelHeight(&self, value: i32) -> ::windows::core::Result<()>;
    fn DownloadProgress(&self, handler: &::core::option::Option<DownloadProgressEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDownloadProgress(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ImageOpened(&self, handler: &::core::option::Option<super::super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveImageOpened(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ImageFailed(&self, handler: &::core::option::Option<super::super::ExceptionRoutedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveImageFailed(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapImage {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapImage";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapImageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapImageImpl, const OFFSET: isize>() -> IBitmapImageVtbl {
        unsafe extern "system" fn CreateOptions<Impl: IBitmapImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BitmapCreateOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreateOptions<Impl: IBitmapImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: BitmapCreateOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCreateOptions(value).into()
        }
        unsafe extern "system" fn UriSource<Impl: IBitmapImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UriSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUriSource<Impl: IBitmapImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUriSource(&*(&value as *const <super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DecodePixelWidth<Impl: IBitmapImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecodePixelWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDecodePixelWidth<Impl: IBitmapImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDecodePixelWidth(value).into()
        }
        unsafe extern "system" fn DecodePixelHeight<Impl: IBitmapImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecodePixelHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDecodePixelHeight<Impl: IBitmapImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDecodePixelHeight(value).into()
        }
        unsafe extern "system" fn DownloadProgress<Impl: IBitmapImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadProgress(&*(&handler as *const <DownloadProgressEventHandler as ::windows::core::Abi>::Abi as *const <DownloadProgressEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDownloadProgress<Impl: IBitmapImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDownloadProgress(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ImageOpened<Impl: IBitmapImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageOpened(&*(&handler as *const <super::super::RoutedEventHandler as ::windows::core::Abi>::Abi as *const <super::super::RoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveImageOpened<Impl: IBitmapImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveImageOpened(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ImageFailed<Impl: IBitmapImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageFailed(&*(&handler as *const <super::super::ExceptionRoutedEventHandler as ::windows::core::Abi>::Abi as *const <super::super::ExceptionRoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveImageFailed<Impl: IBitmapImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveImageFailed(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IBitmapImage>,
            ::windows::core::GetTrustLevel,
            CreateOptions::<Impl, OFFSET>,
            SetCreateOptions::<Impl, OFFSET>,
            UriSource::<Impl, OFFSET>,
            SetUriSource::<Impl, OFFSET>,
            DecodePixelWidth::<Impl, OFFSET>,
            SetDecodePixelWidth::<Impl, OFFSET>,
            DecodePixelHeight::<Impl, OFFSET>,
            SetDecodePixelHeight::<Impl, OFFSET>,
            DownloadProgress::<Impl, OFFSET>,
            RemoveDownloadProgress::<Impl, OFFSET>,
            ImageOpened::<Impl, OFFSET>,
            RemoveImageOpened::<Impl, OFFSET>,
            ImageFailed::<Impl, OFFSET>,
            RemoveImageFailed::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapImage2Impl: Sized {
    fn DecodePixelType(&self) -> ::windows::core::Result<DecodePixelType>;
    fn SetDecodePixelType(&self, value: DecodePixelType) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapImage2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapImage2";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapImage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapImage2Impl, const OFFSET: isize>() -> IBitmapImage2Vtbl {
        unsafe extern "system" fn DecodePixelType<Impl: IBitmapImage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DecodePixelType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecodePixelType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDecodePixelType<Impl: IBitmapImage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DecodePixelType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDecodePixelType(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBitmapImage2>, ::windows::core::GetTrustLevel, DecodePixelType::<Impl, OFFSET>, SetDecodePixelType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapImage3Impl: Sized {
    fn IsAnimatedBitmap(&self) -> ::windows::core::Result<bool>;
    fn IsPlaying(&self) -> ::windows::core::Result<bool>;
    fn AutoPlay(&self) -> ::windows::core::Result<bool>;
    fn SetAutoPlay(&self, value: bool) -> ::windows::core::Result<()>;
    fn Play(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapImage3 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapImage3";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapImage3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapImage3Impl, const OFFSET: isize>() -> IBitmapImage3Vtbl {
        unsafe extern "system" fn IsAnimatedBitmap<Impl: IBitmapImage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAnimatedBitmap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPlaying<Impl: IBitmapImage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPlaying() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoPlay<Impl: IBitmapImage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoPlay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoPlay<Impl: IBitmapImage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoPlay(value).into()
        }
        unsafe extern "system" fn Play<Impl: IBitmapImage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Play().into()
        }
        unsafe extern "system" fn Stop<Impl: IBitmapImage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBitmapImage3>, ::windows::core::GetTrustLevel, IsAnimatedBitmap::<Impl, OFFSET>, IsPlaying::<Impl, OFFSET>, AutoPlay::<Impl, OFFSET>, SetAutoPlay::<Impl, OFFSET>, Play::<Impl, OFFSET>, Stop::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapImageFactoryImpl: Sized {
    fn CreateInstanceWithUriSource(&self, urisource: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<BitmapImage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapImageFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapImageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapImageFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapImageFactoryImpl, const OFFSET: isize>() -> IBitmapImageFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithUriSource<Impl: IBitmapImageFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, urisource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithUriSource(&*(&urisource as *const <super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBitmapImageFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithUriSource::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapImageStaticsImpl: Sized {
    fn CreateOptionsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn UriSourceProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DecodePixelWidthProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DecodePixelHeightProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapImageStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapImageStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapImageStaticsImpl, const OFFSET: isize>() -> IBitmapImageStaticsVtbl {
        unsafe extern "system" fn CreateOptionsProperty<Impl: IBitmapImageStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateOptionsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UriSourceProperty<Impl: IBitmapImageStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UriSourceProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecodePixelWidthProperty<Impl: IBitmapImageStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecodePixelWidthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecodePixelHeightProperty<Impl: IBitmapImageStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecodePixelHeightProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBitmapImageStatics>, ::windows::core::GetTrustLevel, CreateOptionsProperty::<Impl, OFFSET>, UriSourceProperty::<Impl, OFFSET>, DecodePixelWidthProperty::<Impl, OFFSET>, DecodePixelHeightProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapImageStatics2Impl: Sized {
    fn DecodePixelTypeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapImageStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapImageStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapImageStatics2Impl, const OFFSET: isize>() -> IBitmapImageStatics2Vtbl {
        unsafe extern "system" fn DecodePixelTypeProperty<Impl: IBitmapImageStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecodePixelTypeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBitmapImageStatics2>, ::windows::core::GetTrustLevel, DecodePixelTypeProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapImageStatics3Impl: Sized {
    fn IsAnimatedBitmapProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsPlayingProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn AutoPlayProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapImageStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapImageStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapImageStatics3Impl, const OFFSET: isize>() -> IBitmapImageStatics3Vtbl {
        unsafe extern "system" fn IsAnimatedBitmapProperty<Impl: IBitmapImageStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAnimatedBitmapProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPlayingProperty<Impl: IBitmapImageStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPlayingProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoPlayProperty<Impl: IBitmapImageStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoPlayProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBitmapImageStatics3>, ::windows::core::GetTrustLevel, IsAnimatedBitmapProperty::<Impl, OFFSET>, IsPlayingProperty::<Impl, OFFSET>, AutoPlayProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapSourceImpl: Sized {
    fn PixelWidth(&self) -> ::windows::core::Result<i32>;
    fn PixelHeight(&self) -> ::windows::core::Result<i32>;
    fn SetSource(&self, streamsource: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn SetSourceAsync(&self, streamsource: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapSource";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapSourceImpl, const OFFSET: isize>() -> IBitmapSourceVtbl {
        unsafe extern "system" fn PixelWidth<Impl: IBitmapSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PixelHeight<Impl: IBitmapSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSource<Impl: IBitmapSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamsource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSource(&*(&streamsource as *const <super::super::super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetSourceAsync<Impl: IBitmapSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamsource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSourceAsync(&*(&streamsource as *const <super::super::super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBitmapSource>, ::windows::core::GetTrustLevel, PixelWidth::<Impl, OFFSET>, PixelHeight::<Impl, OFFSET>, SetSource::<Impl, OFFSET>, SetSourceAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapSourceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<BitmapSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapSourceFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapSourceFactoryImpl, const OFFSET: isize>() -> IBitmapSourceFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IBitmapSourceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBitmapSourceFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapSourceStaticsImpl: Sized {
    fn PixelWidthProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PixelHeightProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapSourceStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapSourceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapSourceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapSourceStaticsImpl, const OFFSET: isize>() -> IBitmapSourceStaticsVtbl {
        unsafe extern "system" fn PixelWidthProperty<Impl: IBitmapSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PixelWidthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PixelHeightProperty<Impl: IBitmapSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PixelHeightProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBitmapSourceStatics>, ::windows::core::GetTrustLevel, PixelWidthProperty::<Impl, OFFSET>, PixelHeightProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDownloadProgressEventArgsImpl: Sized {
    fn Progress(&self) -> ::windows::core::Result<i32>;
    fn SetProgress(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDownloadProgressEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IDownloadProgressEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDownloadProgressEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadProgressEventArgsImpl, const OFFSET: isize>() -> IDownloadProgressEventArgsVtbl {
        unsafe extern "system" fn Progress<Impl: IDownloadProgressEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Progress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProgress<Impl: IDownloadProgressEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProgress(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDownloadProgressEventArgs>, ::windows::core::GetTrustLevel, Progress::<Impl, OFFSET>, SetProgress::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRenderTargetBitmapImpl: Sized {
    fn PixelWidth(&self) -> ::windows::core::Result<i32>;
    fn PixelHeight(&self) -> ::windows::core::Result<i32>;
    fn RenderAsync(&self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn RenderToSizeAsync(&self, element: &::core::option::Option<super::super::UIElement>, scaledwidth: i32, scaledheight: i32) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn GetPixelsAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Storage::Streams::IBuffer>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRenderTargetBitmap {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IRenderTargetBitmap";
}
#[cfg(feature = "implement_exclusive")]
impl IRenderTargetBitmapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRenderTargetBitmapImpl, const OFFSET: isize>() -> IRenderTargetBitmapVtbl {
        unsafe extern "system" fn PixelWidth<Impl: IRenderTargetBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PixelHeight<Impl: IRenderTargetBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RenderAsync<Impl: IRenderTargetBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderAsync(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenderToSizeAsync<Impl: IRenderTargetBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scaledwidth: i32, scaledheight: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderToSizeAsync(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType), scaledwidth, scaledheight) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelsAsync<Impl: IRenderTargetBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPixelsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRenderTargetBitmap>, ::windows::core::GetTrustLevel, PixelWidth::<Impl, OFFSET>, PixelHeight::<Impl, OFFSET>, RenderAsync::<Impl, OFFSET>, RenderToSizeAsync::<Impl, OFFSET>, GetPixelsAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRenderTargetBitmapStaticsImpl: Sized {
    fn PixelWidthProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PixelHeightProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRenderTargetBitmapStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IRenderTargetBitmapStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRenderTargetBitmapStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRenderTargetBitmapStaticsImpl, const OFFSET: isize>() -> IRenderTargetBitmapStaticsVtbl {
        unsafe extern "system" fn PixelWidthProperty<Impl: IRenderTargetBitmapStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PixelWidthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PixelHeightProperty<Impl: IRenderTargetBitmapStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PixelHeightProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRenderTargetBitmapStatics>, ::windows::core::GetTrustLevel, PixelWidthProperty::<Impl, OFFSET>, PixelHeightProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISoftwareBitmapSourceImpl: Sized {
    fn SetBitmapAsync(&self, softwarebitmap: &::core::option::Option<super::super::super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISoftwareBitmapSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.ISoftwareBitmapSource";
}
#[cfg(feature = "implement_exclusive")]
impl ISoftwareBitmapSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISoftwareBitmapSourceImpl, const OFFSET: isize>() -> ISoftwareBitmapSourceVtbl {
        unsafe extern "system" fn SetBitmapAsync<Impl: ISoftwareBitmapSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, softwarebitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBitmapAsync(&*(&softwarebitmap as *const <super::super::super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISoftwareBitmapSource>, ::windows::core::GetTrustLevel, SetBitmapAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISurfaceImageSourceImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISurfaceImageSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.ISurfaceImageSource";
}
#[cfg(feature = "implement_exclusive")]
impl ISurfaceImageSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISurfaceImageSourceImpl, const OFFSET: isize>() -> ISurfaceImageSourceVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISurfaceImageSource>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISurfaceImageSourceFactoryImpl: Sized {
    fn CreateInstanceWithDimensions(&self, pixelwidth: i32, pixelheight: i32, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SurfaceImageSource>;
    fn CreateInstanceWithDimensionsAndOpacity(&self, pixelwidth: i32, pixelheight: i32, isopaque: bool, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SurfaceImageSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISurfaceImageSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.ISurfaceImageSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISurfaceImageSourceFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISurfaceImageSourceFactoryImpl, const OFFSET: isize>() -> ISurfaceImageSourceFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithDimensions<Impl: ISurfaceImageSourceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelwidth: i32, pixelheight: i32, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithDimensions(pixelwidth, pixelheight, &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithDimensionsAndOpacity<Impl: ISurfaceImageSourceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelwidth: i32, pixelheight: i32, isopaque: bool, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithDimensionsAndOpacity(pixelwidth, pixelheight, isopaque, &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISurfaceImageSourceFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithDimensions::<Impl, OFFSET>, CreateInstanceWithDimensionsAndOpacity::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISvgImageSourceImpl: Sized {
    fn UriSource(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn SetUriSource(&self, value: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn RasterizePixelWidth(&self) -> ::windows::core::Result<f64>;
    fn SetRasterizePixelWidth(&self, value: f64) -> ::windows::core::Result<()>;
    fn RasterizePixelHeight(&self) -> ::windows::core::Result<f64>;
    fn SetRasterizePixelHeight(&self, value: f64) -> ::windows::core::Result<()>;
    fn Opened(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<SvgImageSource, SvgImageSourceOpenedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpened(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn OpenFailed(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<SvgImageSource, SvgImageSourceFailedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpenFailed(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetSourceAsync(&self, streamsource: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SvgImageSourceLoadStatus>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISvgImageSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.ISvgImageSource";
}
#[cfg(feature = "implement_exclusive")]
impl ISvgImageSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISvgImageSourceImpl, const OFFSET: isize>() -> ISvgImageSourceVtbl {
        unsafe extern "system" fn UriSource<Impl: ISvgImageSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UriSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUriSource<Impl: ISvgImageSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUriSource(&*(&value as *const <super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RasterizePixelWidth<Impl: ISvgImageSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RasterizePixelWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRasterizePixelWidth<Impl: ISvgImageSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRasterizePixelWidth(value).into()
        }
        unsafe extern "system" fn RasterizePixelHeight<Impl: ISvgImageSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RasterizePixelHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRasterizePixelHeight<Impl: ISvgImageSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRasterizePixelHeight(value).into()
        }
        unsafe extern "system" fn Opened<Impl: ISvgImageSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Opened(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<SvgImageSource, SvgImageSourceOpenedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<SvgImageSource, SvgImageSourceOpenedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOpened<Impl: ISvgImageSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOpened(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OpenFailed<Impl: ISvgImageSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenFailed(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<SvgImageSource, SvgImageSourceFailedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<SvgImageSource, SvgImageSourceFailedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOpenFailed<Impl: ISvgImageSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOpenFailed(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetSourceAsync<Impl: ISvgImageSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamsource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSourceAsync(&*(&streamsource as *const <super::super::super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<ISvgImageSource>,
            ::windows::core::GetTrustLevel,
            UriSource::<Impl, OFFSET>,
            SetUriSource::<Impl, OFFSET>,
            RasterizePixelWidth::<Impl, OFFSET>,
            SetRasterizePixelWidth::<Impl, OFFSET>,
            RasterizePixelHeight::<Impl, OFFSET>,
            SetRasterizePixelHeight::<Impl, OFFSET>,
            Opened::<Impl, OFFSET>,
            RemoveOpened::<Impl, OFFSET>,
            OpenFailed::<Impl, OFFSET>,
            RemoveOpenFailed::<Impl, OFFSET>,
            SetSourceAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISvgImageSourceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SvgImageSource>;
    fn CreateInstanceWithUriSource(&self, urisource: &::core::option::Option<super::super::super::super::Foundation::Uri>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SvgImageSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISvgImageSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.ISvgImageSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISvgImageSourceFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISvgImageSourceFactoryImpl, const OFFSET: isize>() -> ISvgImageSourceFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ISvgImageSourceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithUriSource<Impl: ISvgImageSourceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, urisource: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithUriSource(&*(&urisource as *const <super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISvgImageSourceFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>, CreateInstanceWithUriSource::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISvgImageSourceFailedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<SvgImageSourceLoadStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISvgImageSourceFailedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.ISvgImageSourceFailedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISvgImageSourceFailedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISvgImageSourceFailedEventArgsImpl, const OFFSET: isize>() -> ISvgImageSourceFailedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: ISvgImageSourceFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SvgImageSourceLoadStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISvgImageSourceFailedEventArgs>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISvgImageSourceOpenedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISvgImageSourceOpenedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.ISvgImageSourceOpenedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISvgImageSourceOpenedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISvgImageSourceOpenedEventArgsImpl, const OFFSET: isize>() -> ISvgImageSourceOpenedEventArgsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISvgImageSourceOpenedEventArgs>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISvgImageSourceStaticsImpl: Sized {
    fn UriSourceProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RasterizePixelWidthProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RasterizePixelHeightProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISvgImageSourceStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.ISvgImageSourceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISvgImageSourceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISvgImageSourceStaticsImpl, const OFFSET: isize>() -> ISvgImageSourceStaticsVtbl {
        unsafe extern "system" fn UriSourceProperty<Impl: ISvgImageSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UriSourceProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RasterizePixelWidthProperty<Impl: ISvgImageSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RasterizePixelWidthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RasterizePixelHeightProperty<Impl: ISvgImageSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RasterizePixelHeightProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISvgImageSourceStatics>, ::windows::core::GetTrustLevel, UriSourceProperty::<Impl, OFFSET>, RasterizePixelWidthProperty::<Impl, OFFSET>, RasterizePixelHeightProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVirtualSurfaceImageSourceImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVirtualSurfaceImageSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IVirtualSurfaceImageSource";
}
#[cfg(feature = "implement_exclusive")]
impl IVirtualSurfaceImageSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualSurfaceImageSourceImpl, const OFFSET: isize>() -> IVirtualSurfaceImageSourceVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVirtualSurfaceImageSource>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVirtualSurfaceImageSourceFactoryImpl: Sized {
    fn CreateInstanceWithDimensions(&self, pixelwidth: i32, pixelheight: i32) -> ::windows::core::Result<VirtualSurfaceImageSource>;
    fn CreateInstanceWithDimensionsAndOpacity(&self, pixelwidth: i32, pixelheight: i32, isopaque: bool) -> ::windows::core::Result<VirtualSurfaceImageSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVirtualSurfaceImageSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IVirtualSurfaceImageSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVirtualSurfaceImageSourceFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualSurfaceImageSourceFactoryImpl, const OFFSET: isize>() -> IVirtualSurfaceImageSourceFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithDimensions<Impl: IVirtualSurfaceImageSourceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelwidth: i32, pixelheight: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithDimensions(pixelwidth, pixelheight) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithDimensionsAndOpacity<Impl: IVirtualSurfaceImageSourceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelwidth: i32, pixelheight: i32, isopaque: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithDimensionsAndOpacity(pixelwidth, pixelheight, isopaque) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVirtualSurfaceImageSourceFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithDimensions::<Impl, OFFSET>, CreateInstanceWithDimensionsAndOpacity::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWriteableBitmapImpl: Sized {
    fn PixelBuffer(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer>;
    fn Invalidate(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWriteableBitmap {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IWriteableBitmap";
}
#[cfg(feature = "implement_exclusive")]
impl IWriteableBitmapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWriteableBitmapImpl, const OFFSET: isize>() -> IWriteableBitmapVtbl {
        unsafe extern "system" fn PixelBuffer<Impl: IWriteableBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PixelBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invalidate<Impl: IWriteableBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Invalidate().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWriteableBitmap>, ::windows::core::GetTrustLevel, PixelBuffer::<Impl, OFFSET>, Invalidate::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWriteableBitmapFactoryImpl: Sized {
    fn CreateInstanceWithDimensions(&self, pixelwidth: i32, pixelheight: i32) -> ::windows::core::Result<WriteableBitmap>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWriteableBitmapFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IWriteableBitmapFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWriteableBitmapFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWriteableBitmapFactoryImpl, const OFFSET: isize>() -> IWriteableBitmapFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithDimensions<Impl: IWriteableBitmapFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelwidth: i32, pixelheight: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithDimensions(pixelwidth, pixelheight) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWriteableBitmapFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithDimensions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlRenderingBackgroundTaskImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlRenderingBackgroundTask {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTask";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlRenderingBackgroundTaskVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlRenderingBackgroundTaskImpl, const OFFSET: isize>() -> IXamlRenderingBackgroundTaskVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXamlRenderingBackgroundTask>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlRenderingBackgroundTaskFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<XamlRenderingBackgroundTask>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlRenderingBackgroundTaskFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTaskFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlRenderingBackgroundTaskFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlRenderingBackgroundTaskFactoryImpl, const OFFSET: isize>() -> IXamlRenderingBackgroundTaskFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IXamlRenderingBackgroundTaskFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXamlRenderingBackgroundTaskFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlRenderingBackgroundTaskOverridesImpl: Sized {
    fn OnRun(&self, taskinstance: &::core::option::Option<super::super::super::super::ApplicationModel::Background::IBackgroundTaskInstance>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlRenderingBackgroundTaskOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTaskOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlRenderingBackgroundTaskOverridesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlRenderingBackgroundTaskOverridesImpl, const OFFSET: isize>() -> IXamlRenderingBackgroundTaskOverridesVtbl {
        unsafe extern "system" fn OnRun<Impl: IXamlRenderingBackgroundTaskOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskinstance: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnRun(&*(&taskinstance as *const <super::super::super::super::ApplicationModel::Background::IBackgroundTaskInstance as ::windows::core::Abi>::Abi as *const <super::super::super::super::ApplicationModel::Background::IBackgroundTaskInstance as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXamlRenderingBackgroundTaskOverrides>, ::windows::core::GetTrustLevel, OnRun::<Impl, OFFSET>)
    }
}
