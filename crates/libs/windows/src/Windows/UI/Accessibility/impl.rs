#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IScreenReaderPositionChangedEventArgsImpl: Sized {
    fn ScreenPositionInRawPixels(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn IsReadingText(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IScreenReaderPositionChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Accessibility.IScreenReaderPositionChangedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IScreenReaderPositionChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScreenReaderPositionChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScreenReaderPositionChangedEventArgsVtbl {
        unsafe extern "system" fn ScreenPositionInRawPixels<Impl: IScreenReaderPositionChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScreenPositionInRawPixels() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadingText<Impl: IScreenReaderPositionChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReadingText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScreenReaderPositionChangedEventArgs, BASE_OFFSET>(),
            ScreenPositionInRawPixels: ScreenPositionInRawPixels::<Impl, IMPL_OFFSET>,
            IsReadingText: IsReadingText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScreenReaderPositionChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IScreenReaderServiceImpl: Sized {
    fn CurrentScreenReaderPosition(&self) -> ::windows::core::Result<ScreenReaderPositionChangedEventArgs>;
    fn ScreenReaderPositionChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ScreenReaderService, ScreenReaderPositionChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScreenReaderPositionChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IScreenReaderService {
    const NAME: &'static str = "Windows.UI.Accessibility.IScreenReaderService";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IScreenReaderServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScreenReaderServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScreenReaderServiceVtbl {
        unsafe extern "system" fn CurrentScreenReaderPosition<Impl: IScreenReaderServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentScreenReaderPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScreenReaderPositionChanged<Impl: IScreenReaderServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScreenReaderPositionChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ScreenReaderService, ScreenReaderPositionChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ScreenReaderService, ScreenReaderPositionChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveScreenReaderPositionChanged<Impl: IScreenReaderServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveScreenReaderPositionChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScreenReaderService, BASE_OFFSET>(),
            CurrentScreenReaderPosition: CurrentScreenReaderPosition::<Impl, IMPL_OFFSET>,
            ScreenReaderPositionChanged: ScreenReaderPositionChanged::<Impl, IMPL_OFFSET>,
            RemoveScreenReaderPositionChanged: RemoveScreenReaderPositionChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScreenReaderService as ::windows::core::Interface>::IID
    }
}
