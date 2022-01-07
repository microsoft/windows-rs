#[cfg(feature = "implement_exclusive")]
pub trait IBackPressedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackPressedEventArgs {
    const NAME: &'static str = "Windows.Phone.UI.Input.IBackPressedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IBackPressedEventArgsVtbl {
    pub const fn new<Impl: IBackPressedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackPressedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IBackPressedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IBackPressedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackPressedEventArgs>, base.5, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICameraEventArgs {
    const NAME: &'static str = "Windows.Phone.UI.Input.ICameraEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICameraEventArgsVtbl {
    pub const fn new<Impl: ICameraEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICameraEventArgsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICameraEventArgs>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHardwareButtonsStaticsImpl: Sized {
    fn BackPressed(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<BackPressedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBackPressed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHardwareButtonsStatics {
    const NAME: &'static str = "Windows.Phone.UI.Input.IHardwareButtonsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHardwareButtonsStaticsVtbl {
    pub const fn new<Impl: IHardwareButtonsStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHardwareButtonsStaticsVtbl {
        unsafe extern "system" fn BackPressed<Impl: IHardwareButtonsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BackPressed(&*(&handler as *const <super::super::super::Foundation::EventHandler<BackPressedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<BackPressedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBackPressed<Impl: IHardwareButtonsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveBackPressed(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHardwareButtonsStatics>, base.5, BackPressed::<Impl, OFFSET>, RemoveBackPressed::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHardwareButtonsStatics2Impl: Sized {
    fn CameraHalfPressed(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<CameraEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraHalfPressed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CameraPressed(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<CameraEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraPressed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CameraReleased(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<CameraEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraReleased(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHardwareButtonsStatics2 {
    const NAME: &'static str = "Windows.Phone.UI.Input.IHardwareButtonsStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IHardwareButtonsStatics2Vtbl {
    pub const fn new<Impl: IHardwareButtonsStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHardwareButtonsStatics2Vtbl {
        unsafe extern "system" fn CameraHalfPressed<Impl: IHardwareButtonsStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CameraHalfPressed(&*(&handler as *const <super::super::super::Foundation::EventHandler<CameraEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<CameraEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCameraHalfPressed<Impl: IHardwareButtonsStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCameraHalfPressed(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CameraPressed<Impl: IHardwareButtonsStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CameraPressed(&*(&handler as *const <super::super::super::Foundation::EventHandler<CameraEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<CameraEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCameraPressed<Impl: IHardwareButtonsStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCameraPressed(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CameraReleased<Impl: IHardwareButtonsStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CameraReleased(&*(&handler as *const <super::super::super::Foundation::EventHandler<CameraEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<CameraEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCameraReleased<Impl: IHardwareButtonsStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCameraReleased(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHardwareButtonsStatics2>, base.5, CameraHalfPressed::<Impl, OFFSET>, RemoveCameraHalfPressed::<Impl, OFFSET>, CameraPressed::<Impl, OFFSET>, RemoveCameraPressed::<Impl, OFFSET>, CameraReleased::<Impl, OFFSET>, RemoveCameraReleased::<Impl, OFFSET>)
    }
}
