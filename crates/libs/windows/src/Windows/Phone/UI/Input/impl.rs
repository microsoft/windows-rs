#[cfg(feature = "implement_exclusive")]
pub trait IBackPressedEventArgsImpl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackPressedEventArgs {
    const NAME: &'static str = "Windows.Phone.UI.Input.IBackPressedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IBackPressedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackPressedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackPressedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IBackPressedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IBackPressedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackPressedEventArgs, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackPressedEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICameraEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICameraEventArgsVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICameraEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICameraEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHardwareButtonsStaticsImpl: Sized {
    fn BackPressed(&mut self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<BackPressedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBackPressed(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHardwareButtonsStatics {
    const NAME: &'static str = "Windows.Phone.UI.Input.IHardwareButtonsStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHardwareButtonsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHardwareButtonsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHardwareButtonsStaticsVtbl {
        unsafe extern "system" fn BackPressed<Impl: IHardwareButtonsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackPressed(&*(&handler as *const <super::super::super::Foundation::EventHandler<BackPressedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<BackPressedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBackPressed<Impl: IHardwareButtonsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBackPressed(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHardwareButtonsStatics, BASE_OFFSET>(),
            BackPressed: BackPressed::<Impl, IMPL_OFFSET>,
            RemoveBackPressed: RemoveBackPressed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHardwareButtonsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHardwareButtonsStatics2Impl: Sized {
    fn CameraHalfPressed(&mut self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<CameraEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraHalfPressed(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CameraPressed(&mut self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<CameraEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraPressed(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CameraReleased(&mut self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<CameraEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraReleased(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHardwareButtonsStatics2 {
    const NAME: &'static str = "Windows.Phone.UI.Input.IHardwareButtonsStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHardwareButtonsStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHardwareButtonsStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHardwareButtonsStatics2Vtbl {
        unsafe extern "system" fn CameraHalfPressed<Impl: IHardwareButtonsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraHalfPressed(&*(&handler as *const <super::super::super::Foundation::EventHandler<CameraEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<CameraEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCameraHalfPressed<Impl: IHardwareButtonsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCameraHalfPressed(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CameraPressed<Impl: IHardwareButtonsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraPressed(&*(&handler as *const <super::super::super::Foundation::EventHandler<CameraEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<CameraEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCameraPressed<Impl: IHardwareButtonsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCameraPressed(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CameraReleased<Impl: IHardwareButtonsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraReleased(&*(&handler as *const <super::super::super::Foundation::EventHandler<CameraEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<CameraEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCameraReleased<Impl: IHardwareButtonsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCameraReleased(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHardwareButtonsStatics2, BASE_OFFSET>(),
            CameraHalfPressed: CameraHalfPressed::<Impl, IMPL_OFFSET>,
            RemoveCameraHalfPressed: RemoveCameraHalfPressed::<Impl, IMPL_OFFSET>,
            CameraPressed: CameraPressed::<Impl, IMPL_OFFSET>,
            RemoveCameraPressed: RemoveCameraPressed::<Impl, IMPL_OFFSET>,
            CameraReleased: CameraReleased::<Impl, IMPL_OFFSET>,
            RemoveCameraReleased: RemoveCameraReleased::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHardwareButtonsStatics2 as ::windows::core::Interface>::IID
    }
}
