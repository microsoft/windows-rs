#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBitmapImage_Impl: Sized {
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
impl IBitmapImage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapImage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapImage_Vtbl {
        unsafe extern "system" fn CreateOptions<Impl: IBitmapImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BitmapCreateOptions) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCreateOptions<Impl: IBitmapImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: BitmapCreateOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCreateOptions(value).into()
        }
        unsafe extern "system" fn UriSource<Impl: IBitmapImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetUriSource<Impl: IBitmapImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUriSource(&*(&value as *const <super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DecodePixelWidth<Impl: IBitmapImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDecodePixelWidth<Impl: IBitmapImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDecodePixelWidth(value).into()
        }
        unsafe extern "system" fn DecodePixelHeight<Impl: IBitmapImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDecodePixelHeight<Impl: IBitmapImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDecodePixelHeight(value).into()
        }
        unsafe extern "system" fn DownloadProgress<Impl: IBitmapImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveDownloadProgress<Impl: IBitmapImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDownloadProgress(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ImageOpened<Impl: IBitmapImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveImageOpened<Impl: IBitmapImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveImageOpened(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ImageFailed<Impl: IBitmapImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveImageFailed<Impl: IBitmapImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IBitmapImage2_Impl: Sized {
    fn DecodePixelType(&mut self) -> ::windows::core::Result<DecodePixelType>;
    fn SetDecodePixelType(&mut self, value: DecodePixelType) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapImage2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapImage2";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapImage2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapImage2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapImage2_Vtbl {
        unsafe extern "system" fn DecodePixelType<Impl: IBitmapImage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DecodePixelType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDecodePixelType<Impl: IBitmapImage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DecodePixelType) -> ::windows::core::HRESULT {
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
pub trait IBitmapImage3_Impl: Sized {
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
impl IBitmapImage3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapImage3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapImage3_Vtbl {
        unsafe extern "system" fn IsAnimatedBitmap<Impl: IBitmapImage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsPlaying<Impl: IBitmapImage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AutoPlay<Impl: IBitmapImage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAutoPlay<Impl: IBitmapImage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoPlay(value).into()
        }
        unsafe extern "system" fn Play<Impl: IBitmapImage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Play().into()
        }
        unsafe extern "system" fn Stop<Impl: IBitmapImage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IBitmapImageFactory_Impl: Sized {
    fn CreateInstanceWithUriSource(&mut self, urisource: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<BitmapImage>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBitmapImageFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapImageFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBitmapImageFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapImageFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapImageFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithUriSource<Impl: IBitmapImageFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, urisource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IBitmapImageStatics_Impl: Sized {
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
impl IBitmapImageStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapImageStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapImageStatics_Vtbl {
        unsafe extern "system" fn CreateOptionsProperty<Impl: IBitmapImageStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UriSourceProperty<Impl: IBitmapImageStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DecodePixelWidthProperty<Impl: IBitmapImageStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DecodePixelHeightProperty<Impl: IBitmapImageStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IBitmapImageStatics2_Impl: Sized {
    fn DecodePixelTypeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapImageStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapImageStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapImageStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapImageStatics2_Vtbl {
        unsafe extern "system" fn DecodePixelTypeProperty<Impl: IBitmapImageStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IBitmapImageStatics3_Impl: Sized {
    fn IsAnimatedBitmapProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsPlayingProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn AutoPlayProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapImageStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapImageStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapImageStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapImageStatics3_Vtbl {
        unsafe extern "system" fn IsAnimatedBitmapProperty<Impl: IBitmapImageStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsPlayingProperty<Impl: IBitmapImageStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AutoPlayProperty<Impl: IBitmapImageStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IBitmapSource_Impl: Sized {
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
impl IBitmapSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapSource_Vtbl {
        unsafe extern "system" fn PixelWidth<Impl: IBitmapSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PixelHeight<Impl: IBitmapSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSource<Impl: IBitmapSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamsource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSource(&*(&streamsource as *const <super::super::super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetSourceAsync<Impl: IBitmapSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamsource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IBitmapSourceFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<BitmapSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapSourceFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapSourceFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapSourceFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IBitmapSourceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IBitmapSourceStatics_Impl: Sized {
    fn PixelWidthProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PixelHeightProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapSourceStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IBitmapSourceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapSourceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapSourceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapSourceStatics_Vtbl {
        unsafe extern "system" fn PixelWidthProperty<Impl: IBitmapSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PixelHeightProperty<Impl: IBitmapSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IDownloadProgressEventArgs_Impl: Sized {
    fn Progress(&mut self) -> ::windows::core::Result<i32>;
    fn SetProgress(&mut self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDownloadProgressEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IDownloadProgressEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDownloadProgressEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadProgressEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDownloadProgressEventArgs_Vtbl {
        unsafe extern "system" fn Progress<Impl: IDownloadProgressEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetProgress<Impl: IDownloadProgressEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
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
pub trait IRenderTargetBitmap_Impl: Sized {
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
impl IRenderTargetBitmap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRenderTargetBitmap_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRenderTargetBitmap_Vtbl {
        unsafe extern "system" fn PixelWidth<Impl: IRenderTargetBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PixelHeight<Impl: IRenderTargetBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RenderAsync<Impl: IRenderTargetBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RenderToSizeAsync<Impl: IRenderTargetBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scaledwidth: i32, scaledheight: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetPixelsAsync<Impl: IRenderTargetBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRenderTargetBitmapStatics_Impl: Sized {
    fn PixelWidthProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PixelHeightProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRenderTargetBitmapStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IRenderTargetBitmapStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRenderTargetBitmapStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRenderTargetBitmapStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRenderTargetBitmapStatics_Vtbl {
        unsafe extern "system" fn PixelWidthProperty<Impl: IRenderTargetBitmapStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PixelHeightProperty<Impl: IRenderTargetBitmapStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISoftwareBitmapSource_Impl: Sized {
    fn SetBitmapAsync(&mut self, softwarebitmap: &::core::option::Option<super::super::super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISoftwareBitmapSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.ISoftwareBitmapSource";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ISoftwareBitmapSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISoftwareBitmapSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISoftwareBitmapSource_Vtbl {
        unsafe extern "system" fn SetBitmapAsync<Impl: ISoftwareBitmapSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, softwarebitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISurfaceImageSource_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISurfaceImageSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.ISurfaceImageSource";
}
#[cfg(feature = "implement_exclusive")]
impl ISurfaceImageSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISurfaceImageSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISurfaceImageSource_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISurfaceImageSource, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISurfaceImageSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISurfaceImageSourceFactory_Impl: Sized {
    fn CreateInstanceWithDimensions(&mut self, pixelwidth: i32, pixelheight: i32, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SurfaceImageSource>;
    fn CreateInstanceWithDimensionsAndOpacity(&mut self, pixelwidth: i32, pixelheight: i32, isopaque: bool, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SurfaceImageSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISurfaceImageSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.ISurfaceImageSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISurfaceImageSourceFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISurfaceImageSourceFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISurfaceImageSourceFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithDimensions<Impl: ISurfaceImageSourceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelwidth: i32, pixelheight: i32, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateInstanceWithDimensionsAndOpacity<Impl: ISurfaceImageSourceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelwidth: i32, pixelheight: i32, isopaque: bool, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISvgImageSource_Impl: Sized {
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
impl ISvgImageSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISvgImageSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISvgImageSource_Vtbl {
        unsafe extern "system" fn UriSource<Impl: ISvgImageSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetUriSource<Impl: ISvgImageSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUriSource(&*(&value as *const <super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RasterizePixelWidth<Impl: ISvgImageSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRasterizePixelWidth<Impl: ISvgImageSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRasterizePixelWidth(value).into()
        }
        unsafe extern "system" fn RasterizePixelHeight<Impl: ISvgImageSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRasterizePixelHeight<Impl: ISvgImageSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRasterizePixelHeight(value).into()
        }
        unsafe extern "system" fn Opened<Impl: ISvgImageSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveOpened<Impl: ISvgImageSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOpened(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OpenFailed<Impl: ISvgImageSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveOpenFailed<Impl: ISvgImageSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOpenFailed(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetSourceAsync<Impl: ISvgImageSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamsource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISvgImageSourceFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SvgImageSource>;
    fn CreateInstanceWithUriSource(&mut self, urisource: &::core::option::Option<super::super::super::super::Foundation::Uri>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SvgImageSource>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISvgImageSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.ISvgImageSourceFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISvgImageSourceFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISvgImageSourceFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISvgImageSourceFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: ISvgImageSourceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateInstanceWithUriSource<Impl: ISvgImageSourceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, urisource: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISvgImageSourceFailedEventArgs_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<SvgImageSourceLoadStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISvgImageSourceFailedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.ISvgImageSourceFailedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISvgImageSourceFailedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISvgImageSourceFailedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISvgImageSourceFailedEventArgs_Vtbl {
        unsafe extern "system" fn Status<Impl: ISvgImageSourceFailedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SvgImageSourceLoadStatus) -> ::windows::core::HRESULT {
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
pub trait ISvgImageSourceOpenedEventArgs_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISvgImageSourceOpenedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.ISvgImageSourceOpenedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISvgImageSourceOpenedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISvgImageSourceOpenedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISvgImageSourceOpenedEventArgs_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISvgImageSourceOpenedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISvgImageSourceOpenedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISvgImageSourceStatics_Impl: Sized {
    fn UriSourceProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RasterizePixelWidthProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RasterizePixelHeightProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISvgImageSourceStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.ISvgImageSourceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISvgImageSourceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISvgImageSourceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISvgImageSourceStatics_Vtbl {
        unsafe extern "system" fn UriSourceProperty<Impl: ISvgImageSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RasterizePixelWidthProperty<Impl: ISvgImageSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RasterizePixelHeightProperty<Impl: ISvgImageSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVirtualSurfaceImageSource_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVirtualSurfaceImageSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IVirtualSurfaceImageSource";
}
#[cfg(feature = "implement_exclusive")]
impl IVirtualSurfaceImageSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualSurfaceImageSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVirtualSurfaceImageSource_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVirtualSurfaceImageSource, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVirtualSurfaceImageSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVirtualSurfaceImageSourceFactory_Impl: Sized {
    fn CreateInstanceWithDimensions(&mut self, pixelwidth: i32, pixelheight: i32) -> ::windows::core::Result<VirtualSurfaceImageSource>;
    fn CreateInstanceWithDimensionsAndOpacity(&mut self, pixelwidth: i32, pixelheight: i32, isopaque: bool) -> ::windows::core::Result<VirtualSurfaceImageSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVirtualSurfaceImageSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IVirtualSurfaceImageSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVirtualSurfaceImageSourceFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualSurfaceImageSourceFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVirtualSurfaceImageSourceFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithDimensions<Impl: IVirtualSurfaceImageSourceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelwidth: i32, pixelheight: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateInstanceWithDimensionsAndOpacity<Impl: IVirtualSurfaceImageSourceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelwidth: i32, pixelheight: i32, isopaque: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IWriteableBitmap_Impl: Sized {
    fn PixelBuffer(&mut self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer>;
    fn Invalidate(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWriteableBitmap {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IWriteableBitmap";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IWriteableBitmap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWriteableBitmap_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWriteableBitmap_Vtbl {
        unsafe extern "system" fn PixelBuffer<Impl: IWriteableBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Invalidate<Impl: IWriteableBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IWriteableBitmapFactory_Impl: Sized {
    fn CreateInstanceWithDimensions(&mut self, pixelwidth: i32, pixelheight: i32) -> ::windows::core::Result<WriteableBitmap>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWriteableBitmapFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IWriteableBitmapFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWriteableBitmapFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWriteableBitmapFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWriteableBitmapFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithDimensions<Impl: IWriteableBitmapFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelwidth: i32, pixelheight: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXamlRenderingBackgroundTask_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlRenderingBackgroundTask {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTask";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlRenderingBackgroundTask_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlRenderingBackgroundTask_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlRenderingBackgroundTask_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlRenderingBackgroundTask, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlRenderingBackgroundTask as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlRenderingBackgroundTaskFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<XamlRenderingBackgroundTask>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlRenderingBackgroundTaskFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTaskFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlRenderingBackgroundTaskFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlRenderingBackgroundTaskFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlRenderingBackgroundTaskFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IXamlRenderingBackgroundTaskFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXamlRenderingBackgroundTaskOverrides_Impl: Sized {
    fn OnRun(&mut self, taskinstance: &::core::option::Option<super::super::super::super::ApplicationModel::Background::IBackgroundTaskInstance>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_Background", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXamlRenderingBackgroundTaskOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTaskOverrides";
}
#[cfg(all(feature = "ApplicationModel_Background", feature = "implement_exclusive"))]
impl IXamlRenderingBackgroundTaskOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlRenderingBackgroundTaskOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlRenderingBackgroundTaskOverrides_Vtbl {
        unsafe extern "system" fn OnRun<Impl: IXamlRenderingBackgroundTaskOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskinstance: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
