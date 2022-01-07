#[cfg(feature = "implement_exclusive")]
pub trait IScreenReaderPositionChangedEventArgsImpl: Sized {
    fn ScreenPositionInRawPixels(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn IsReadingText(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScreenReaderPositionChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Accessibility.IScreenReaderPositionChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IScreenReaderPositionChangedEventArgsVtbl {
    pub const fn new<Impl: IScreenReaderPositionChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScreenReaderPositionChangedEventArgsVtbl {
        unsafe extern "system" fn ScreenPositionInRawPixels<Impl: IScreenReaderPositionChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScreenPositionInRawPixels() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadingText<Impl: IScreenReaderPositionChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsReadingText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScreenReaderPositionChangedEventArgs>, base.5, ScreenPositionInRawPixels::<Impl, OFFSET>, IsReadingText::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScreenReaderServiceImpl: Sized {
    fn CurrentScreenReaderPosition(&self) -> ::windows::core::Result<ScreenReaderPositionChangedEventArgs>;
    fn ScreenReaderPositionChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ScreenReaderService, ScreenReaderPositionChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScreenReaderPositionChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScreenReaderService {
    const NAME: &'static str = "Windows.UI.Accessibility.IScreenReaderService";
}
#[cfg(feature = "implement_exclusive")]
impl IScreenReaderServiceVtbl {
    pub const fn new<Impl: IScreenReaderServiceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScreenReaderServiceVtbl {
        unsafe extern "system" fn CurrentScreenReaderPosition<Impl: IScreenReaderServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentScreenReaderPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScreenReaderPositionChanged<Impl: IScreenReaderServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScreenReaderPositionChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ScreenReaderService, ScreenReaderPositionChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ScreenReaderService, ScreenReaderPositionChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveScreenReaderPositionChanged<Impl: IScreenReaderServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveScreenReaderPositionChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScreenReaderService>, base.5, CurrentScreenReaderPosition::<Impl, OFFSET>, ScreenReaderPositionChanged::<Impl, OFFSET>, RemoveScreenReaderPositionChanged::<Impl, OFFSET>)
    }
}
