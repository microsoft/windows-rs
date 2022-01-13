#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBitmapImageImpl: Sized {
    fn CreateOptions(&mut self) -> ::windows::core::Result<BitmapCreateOptions>;
    fn SetCreateOptions(&mut self, value: BitmapCreateOptions) -> ::windows::core::Result<()>;
    fn UriSource(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn SetUriSource(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn DecodePixelWidth(&mut self) -> ::windows::core::Result<i32>;
    fn SetDecodePixelWidth(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn DecodePixelHeight(&mut self) -> ::windows::core::Result<i32>;
    fn SetDecodePixelHeight(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn DownloadProgress(&mut self, handler: &::core::option::Option<DownloadProgressEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDownloadProgress(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ImageOpened(&mut self, handler: &::core::option::Option<super::super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveImageOpened(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ImageFailed(&mut self, handler: &::core::option::Option<super::super::ExceptionRoutedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveImageFailed(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBitmapImage {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapImage";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBitmapImageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapImageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapImageVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapImage, BASE_OFFSET>(),
            CreateOptions: CreateOptions::<Impl, IMPL_OFFSET>,
            SetCreateOptions: SetCreateOptions::<Impl, IMPL_OFFSET>,
            UriSource: UriSource::<Impl, IMPL_OFFSET>,
            SetUriSource: SetUriSource::<Impl, IMPL_OFFSET>,
            DecodePixelWidth: DecodePixelWidth::<Impl, IMPL_OFFSET>,
            SetDecodePixelWidth: SetDecodePixelWidth::<Impl, IMPL_OFFSET>,
            DecodePixelHeight: DecodePixelHeight::<Impl, IMPL_OFFSET>,
            SetDecodePixelHeight: SetDecodePixelHeight::<Impl, IMPL_OFFSET>,
            DownloadProgress: DownloadProgress::<Impl, IMPL_OFFSET>,
            RemoveDownloadProgress: RemoveDownloadProgress::<Impl, IMPL_OFFSET>,
            ImageOpened: ImageOpened::<Impl, IMPL_OFFSET>,
            RemoveImageOpened: RemoveImageOpened::<Impl, IMPL_OFFSET>,
            ImageFailed: ImageFailed::<Impl, IMPL_OFFSET>,
            RemoveImageFailed: RemoveImageFailed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapImage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapImage2Impl: Sized {
    fn DecodePixelType(&mut self) -> ::windows::core::Result<DecodePixelType>;
    fn SetDecodePixelType(&mut self, value: DecodePixelType) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapImage2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapImage2";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapImage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapImage2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapImage2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapImage2, BASE_OFFSET>(),
            DecodePixelType: DecodePixelType::<Impl, IMPL_OFFSET>,
            SetDecodePixelType: SetDecodePixelType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapImage2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapImage3Impl: Sized {
    fn IsAnimatedBitmap(&mut self) -> ::windows::core::Result<bool>;
    fn IsPlaying(&mut self) -> ::windows::core::Result<bool>;
    fn AutoPlay(&mut self) -> ::windows::core::Result<bool>;
    fn SetAutoPlay(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Play(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapImage3 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapImage3";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapImage3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapImage3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapImage3Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapImage3, BASE_OFFSET>(),
            IsAnimatedBitmap: IsAnimatedBitmap::<Impl, IMPL_OFFSET>,
            IsPlaying: IsPlaying::<Impl, IMPL_OFFSET>,
            AutoPlay: AutoPlay::<Impl, IMPL_OFFSET>,
            SetAutoPlay: SetAutoPlay::<Impl, IMPL_OFFSET>,
            Play: Play::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapImage3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBitmapImageFactoryImpl: Sized {
    fn CreateInstanceWithUriSource(&mut self, urisource: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<BitmapImage>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBitmapImageFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapImageFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBitmapImageFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapImageFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapImageFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapImageFactory, BASE_OFFSET>(),
            CreateInstanceWithUriSource: CreateInstanceWithUriSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapImageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapImageStaticsImpl: Sized {
    fn CreateOptionsProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn UriSourceProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DecodePixelWidthProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DecodePixelHeightProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapImageStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapImageStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapImageStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapImageStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapImageStatics, BASE_OFFSET>(),
            CreateOptionsProperty: CreateOptionsProperty::<Impl, IMPL_OFFSET>,
            UriSourceProperty: UriSourceProperty::<Impl, IMPL_OFFSET>,
            DecodePixelWidthProperty: DecodePixelWidthProperty::<Impl, IMPL_OFFSET>,
            DecodePixelHeightProperty: DecodePixelHeightProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapImageStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapImageStatics2Impl: Sized {
    fn DecodePixelTypeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapImageStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapImageStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapImageStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapImageStatics2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapImageStatics2, BASE_OFFSET>(),
            DecodePixelTypeProperty: DecodePixelTypeProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapImageStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapImageStatics3Impl: Sized {
    fn IsAnimatedBitmapProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsPlayingProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn AutoPlayProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapImageStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapImageStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapImageStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapImageStatics3Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapImageStatics3, BASE_OFFSET>(),
            IsAnimatedBitmapProperty: IsAnimatedBitmapProperty::<Impl, IMPL_OFFSET>,
            IsPlayingProperty: IsPlayingProperty::<Impl, IMPL_OFFSET>,
            AutoPlayProperty: AutoPlayProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapImageStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IBitmapSourceImpl: Sized {
    fn PixelWidth(&mut self) -> ::windows::core::Result<i32>;
    fn PixelHeight(&mut self) -> ::windows::core::Result<i32>;
    fn SetSource(&mut self, streamsource: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn SetSourceAsync(&mut self, streamsource: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBitmapSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapSource";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IBitmapSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapSourceVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapSource, BASE_OFFSET>(),
            PixelWidth: PixelWidth::<Impl, IMPL_OFFSET>,
            PixelHeight: PixelHeight::<Impl, IMPL_OFFSET>,
            SetSource: SetSource::<Impl, IMPL_OFFSET>,
            SetSourceAsync: SetSourceAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapSourceFactoryImpl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<BitmapSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapSourceFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapSourceFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapSourceFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapSourceFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapSourceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapSourceStaticsImpl: Sized {
    fn PixelWidthProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PixelHeightProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapSourceStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapSourceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapSourceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapSourceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapSourceStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapSourceStatics, BASE_OFFSET>(),
            PixelWidthProperty: PixelWidthProperty::<Impl, IMPL_OFFSET>,
            PixelHeightProperty: PixelHeightProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapSourceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDownloadProgressEventArgsImpl: Sized {
    fn Progress(&mut self) -> ::windows::core::Result<i32>;
    fn SetProgress(&mut self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDownloadProgressEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IDownloadProgressEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDownloadProgressEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadProgressEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDownloadProgressEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDownloadProgressEventArgs, BASE_OFFSET>(),
            Progress: Progress::<Impl, IMPL_OFFSET>,
            SetProgress: SetProgress::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadProgressEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IRenderTargetBitmapImpl: Sized {
    fn PixelWidth(&mut self) -> ::windows::core::Result<i32>;
    fn PixelHeight(&mut self) -> ::windows::core::Result<i32>;
    fn RenderAsync(&mut self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn RenderToSizeAsync(&mut self, element: &::core::option::Option<super::super::UIElement>, scaledwidth: i32, scaledheight: i32) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn GetPixelsAsync(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Storage::Streams::IBuffer>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRenderTargetBitmap {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IRenderTargetBitmap";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IRenderTargetBitmapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRenderTargetBitmapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRenderTargetBitmapVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRenderTargetBitmap, BASE_OFFSET>(),
            PixelWidth: PixelWidth::<Impl, IMPL_OFFSET>,
            PixelHeight: PixelHeight::<Impl, IMPL_OFFSET>,
            RenderAsync: RenderAsync::<Impl, IMPL_OFFSET>,
            RenderToSizeAsync: RenderToSizeAsync::<Impl, IMPL_OFFSET>,
            GetPixelsAsync: GetPixelsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRenderTargetBitmap as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRenderTargetBitmapStaticsImpl: Sized {
    fn PixelWidthProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PixelHeightProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRenderTargetBitmapStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IRenderTargetBitmapStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRenderTargetBitmapStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRenderTargetBitmapStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRenderTargetBitmapStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRenderTargetBitmapStatics, BASE_OFFSET>(),
            PixelWidthProperty: PixelWidthProperty::<Impl, IMPL_OFFSET>,
            PixelHeightProperty: PixelHeightProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRenderTargetBitmapStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait ISoftwareBitmapSourceImpl: Sized {
    fn SetBitmapAsync(&mut self, softwarebitmap: &::core::option::Option<super::super::super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISoftwareBitmapSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.ISoftwareBitmapSource";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ISoftwareBitmapSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISoftwareBitmapSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISoftwareBitmapSourceVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISoftwareBitmapSource, BASE_OFFSET>(),
            SetBitmapAsync: SetBitmapAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISoftwareBitmapSource as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISurfaceImageSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISurfaceImageSourceVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISurfaceImageSource, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISurfaceImageSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISurfaceImageSourceFactoryImpl: Sized {
    fn CreateInstanceWithDimensions(&mut self, pixelwidth: i32, pixelheight: i32, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SurfaceImageSource>;
    fn CreateInstanceWithDimensionsAndOpacity(&mut self, pixelwidth: i32, pixelheight: i32, isopaque: bool, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SurfaceImageSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISurfaceImageSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.ISurfaceImageSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISurfaceImageSourceFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISurfaceImageSourceFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISurfaceImageSourceFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISurfaceImageSourceFactory, BASE_OFFSET>(),
            CreateInstanceWithDimensions: CreateInstanceWithDimensions::<Impl, IMPL_OFFSET>,
            CreateInstanceWithDimensionsAndOpacity: CreateInstanceWithDimensionsAndOpacity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISurfaceImageSourceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISvgImageSourceImpl: Sized {
    fn UriSource(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn SetUriSource(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn RasterizePixelWidth(&mut self) -> ::windows::core::Result<f64>;
    fn SetRasterizePixelWidth(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn RasterizePixelHeight(&mut self) -> ::windows::core::Result<f64>;
    fn SetRasterizePixelHeight(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Opened(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<SvgImageSource, SvgImageSourceOpenedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpened(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn OpenFailed(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<SvgImageSource, SvgImageSourceFailedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpenFailed(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetSourceAsync(&mut self, streamsource: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SvgImageSourceLoadStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISvgImageSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.ISvgImageSource";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISvgImageSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISvgImageSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISvgImageSourceVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISvgImageSource, BASE_OFFSET>(),
            UriSource: UriSource::<Impl, IMPL_OFFSET>,
            SetUriSource: SetUriSource::<Impl, IMPL_OFFSET>,
            RasterizePixelWidth: RasterizePixelWidth::<Impl, IMPL_OFFSET>,
            SetRasterizePixelWidth: SetRasterizePixelWidth::<Impl, IMPL_OFFSET>,
            RasterizePixelHeight: RasterizePixelHeight::<Impl, IMPL_OFFSET>,
            SetRasterizePixelHeight: SetRasterizePixelHeight::<Impl, IMPL_OFFSET>,
            Opened: Opened::<Impl, IMPL_OFFSET>,
            RemoveOpened: RemoveOpened::<Impl, IMPL_OFFSET>,
            OpenFailed: OpenFailed::<Impl, IMPL_OFFSET>,
            RemoveOpenFailed: RemoveOpenFailed::<Impl, IMPL_OFFSET>,
            SetSourceAsync: SetSourceAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISvgImageSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISvgImageSourceFactoryImpl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SvgImageSource>;
    fn CreateInstanceWithUriSource(&mut self, urisource: &::core::option::Option<super::super::super::super::Foundation::Uri>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SvgImageSource>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISvgImageSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.ISvgImageSourceFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISvgImageSourceFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISvgImageSourceFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISvgImageSourceFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISvgImageSourceFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
            CreateInstanceWithUriSource: CreateInstanceWithUriSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISvgImageSourceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISvgImageSourceFailedEventArgsImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<SvgImageSourceLoadStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISvgImageSourceFailedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.ISvgImageSourceFailedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISvgImageSourceFailedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISvgImageSourceFailedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISvgImageSourceFailedEventArgsVtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISvgImageSourceFailedEventArgs, BASE_OFFSET>(), Status: Status::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISvgImageSourceFailedEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISvgImageSourceOpenedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISvgImageSourceOpenedEventArgsVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISvgImageSourceOpenedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISvgImageSourceOpenedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISvgImageSourceStaticsImpl: Sized {
    fn UriSourceProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RasterizePixelWidthProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RasterizePixelHeightProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISvgImageSourceStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.ISvgImageSourceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISvgImageSourceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISvgImageSourceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISvgImageSourceStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISvgImageSourceStatics, BASE_OFFSET>(),
            UriSourceProperty: UriSourceProperty::<Impl, IMPL_OFFSET>,
            RasterizePixelWidthProperty: RasterizePixelWidthProperty::<Impl, IMPL_OFFSET>,
            RasterizePixelHeightProperty: RasterizePixelHeightProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISvgImageSourceStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualSurfaceImageSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVirtualSurfaceImageSourceVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVirtualSurfaceImageSource, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVirtualSurfaceImageSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVirtualSurfaceImageSourceFactoryImpl: Sized {
    fn CreateInstanceWithDimensions(&mut self, pixelwidth: i32, pixelheight: i32) -> ::windows::core::Result<VirtualSurfaceImageSource>;
    fn CreateInstanceWithDimensionsAndOpacity(&mut self, pixelwidth: i32, pixelheight: i32, isopaque: bool) -> ::windows::core::Result<VirtualSurfaceImageSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVirtualSurfaceImageSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IVirtualSurfaceImageSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVirtualSurfaceImageSourceFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualSurfaceImageSourceFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVirtualSurfaceImageSourceFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVirtualSurfaceImageSourceFactory, BASE_OFFSET>(),
            CreateInstanceWithDimensions: CreateInstanceWithDimensions::<Impl, IMPL_OFFSET>,
            CreateInstanceWithDimensionsAndOpacity: CreateInstanceWithDimensionsAndOpacity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVirtualSurfaceImageSourceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IWriteableBitmapImpl: Sized {
    fn PixelBuffer(&mut self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer>;
    fn Invalidate(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWriteableBitmap {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IWriteableBitmap";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IWriteableBitmapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWriteableBitmapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWriteableBitmapVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWriteableBitmap, BASE_OFFSET>(),
            PixelBuffer: PixelBuffer::<Impl, IMPL_OFFSET>,
            Invalidate: Invalidate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWriteableBitmap as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWriteableBitmapFactoryImpl: Sized {
    fn CreateInstanceWithDimensions(&mut self, pixelwidth: i32, pixelheight: i32) -> ::windows::core::Result<WriteableBitmap>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWriteableBitmapFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IWriteableBitmapFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWriteableBitmapFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWriteableBitmapFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWriteableBitmapFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWriteableBitmapFactory, BASE_OFFSET>(),
            CreateInstanceWithDimensions: CreateInstanceWithDimensions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWriteableBitmapFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlRenderingBackgroundTaskImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlRenderingBackgroundTaskVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlRenderingBackgroundTask, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlRenderingBackgroundTask as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlRenderingBackgroundTaskFactoryImpl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<XamlRenderingBackgroundTask>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlRenderingBackgroundTaskFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTaskFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlRenderingBackgroundTaskFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlRenderingBackgroundTaskFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlRenderingBackgroundTaskFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlRenderingBackgroundTaskFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlRenderingBackgroundTaskFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Background", feature = "implement_exclusive"))]
pub trait IXamlRenderingBackgroundTaskOverridesImpl: Sized {
    fn OnRun(&mut self, taskinstance: &::core::option::Option<super::super::super::super::ApplicationModel::Background::IBackgroundTaskInstance>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_Background", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXamlRenderingBackgroundTaskOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTaskOverrides";
}
#[cfg(all(feature = "ApplicationModel_Background", feature = "implement_exclusive"))]
impl IXamlRenderingBackgroundTaskOverridesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlRenderingBackgroundTaskOverridesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlRenderingBackgroundTaskOverridesVtbl {
        unsafe extern "system" fn OnRun<Impl: IXamlRenderingBackgroundTaskOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskinstance: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnRun(&*(&taskinstance as *const <super::super::super::super::ApplicationModel::Background::IBackgroundTaskInstance as ::windows::core::Abi>::Abi as *const <super::super::super::super::ApplicationModel::Background::IBackgroundTaskInstance as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlRenderingBackgroundTaskOverrides, BASE_OFFSET>(),
            OnRun: OnRun::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlRenderingBackgroundTaskOverrides as ::windows::core::Interface>::IID
    }
}
