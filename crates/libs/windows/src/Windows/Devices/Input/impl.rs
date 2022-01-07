#[cfg(feature = "implement_exclusive")]
pub trait IKeyboardCapabilitiesImpl: Sized {
    fn KeyboardPresent(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyboardCapabilities {
    const NAME: &'static str = "Windows.Devices.Input.IKeyboardCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyboardCapabilitiesVtbl {
    pub const fn new<Impl: IKeyboardCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyboardCapabilitiesVtbl {
        unsafe extern "system" fn KeyboardPresent<Impl: IKeyboardCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyboardPresent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyboardCapabilities>, base.5, KeyboardPresent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMouseCapabilitiesImpl: Sized {
    fn MousePresent(&self) -> ::windows::core::Result<i32>;
    fn VerticalWheelPresent(&self) -> ::windows::core::Result<i32>;
    fn HorizontalWheelPresent(&self) -> ::windows::core::Result<i32>;
    fn SwapButtons(&self) -> ::windows::core::Result<i32>;
    fn NumberOfButtons(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMouseCapabilities {
    const NAME: &'static str = "Windows.Devices.Input.IMouseCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IMouseCapabilitiesVtbl {
    pub const fn new<Impl: IMouseCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMouseCapabilitiesVtbl {
        unsafe extern "system" fn MousePresent<Impl: IMouseCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MousePresent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalWheelPresent<Impl: IMouseCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VerticalWheelPresent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalWheelPresent<Impl: IMouseCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HorizontalWheelPresent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SwapButtons<Impl: IMouseCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SwapButtons() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfButtons<Impl: IMouseCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NumberOfButtons() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMouseCapabilities>, base.5, MousePresent::<Impl, OFFSET>, VerticalWheelPresent::<Impl, OFFSET>, HorizontalWheelPresent::<Impl, OFFSET>, SwapButtons::<Impl, OFFSET>, NumberOfButtons::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMouseDeviceImpl: Sized {
    fn MouseMoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MouseDevice, MouseEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMouseMoved(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMouseDevice {
    const NAME: &'static str = "Windows.Devices.Input.IMouseDevice";
}
#[cfg(feature = "implement_exclusive")]
impl IMouseDeviceVtbl {
    pub const fn new<Impl: IMouseDeviceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMouseDeviceVtbl {
        unsafe extern "system" fn MouseMoved<Impl: IMouseDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MouseMoved(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MouseDevice, MouseEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MouseDevice, MouseEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMouseMoved<Impl: IMouseDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveMouseMoved(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMouseDevice>, base.5, MouseMoved::<Impl, OFFSET>, RemoveMouseMoved::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMouseDeviceStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<MouseDevice>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMouseDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Input.IMouseDeviceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMouseDeviceStaticsVtbl {
    pub const fn new<Impl: IMouseDeviceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMouseDeviceStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IMouseDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMouseDeviceStatics>, base.5, GetForCurrentView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMouseEventArgsImpl: Sized {
    fn MouseDelta(&self) -> ::windows::core::Result<MouseDelta>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMouseEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.IMouseEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMouseEventArgsVtbl {
    pub const fn new<Impl: IMouseEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMouseEventArgsVtbl {
        unsafe extern "system" fn MouseDelta<Impl: IMouseEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MouseDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MouseDelta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMouseEventArgs>, base.5, MouseDelta::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenButtonListenerImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn IsSupportedChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PenButtonListener, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsSupportedChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TailButtonClicked(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonClickedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTailButtonClicked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TailButtonDoubleClicked(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonDoubleClickedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTailButtonDoubleClicked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TailButtonLongPressed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonLongPressedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTailButtonLongPressed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenButtonListener {
    const NAME: &'static str = "Windows.Devices.Input.IPenButtonListener";
}
#[cfg(feature = "implement_exclusive")]
impl IPenButtonListenerVtbl {
    pub const fn new<Impl: IPenButtonListenerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPenButtonListenerVtbl {
        unsafe extern "system" fn IsSupported<Impl: IPenButtonListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupportedChanged<Impl: IPenButtonListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSupportedChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PenButtonListener, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PenButtonListener, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIsSupportedChanged<Impl: IPenButtonListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveIsSupportedChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TailButtonClicked<Impl: IPenButtonListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TailButtonClicked(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonClickedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonClickedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTailButtonClicked<Impl: IPenButtonListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveTailButtonClicked(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TailButtonDoubleClicked<Impl: IPenButtonListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TailButtonDoubleClicked(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonDoubleClickedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonDoubleClickedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTailButtonDoubleClicked<Impl: IPenButtonListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveTailButtonDoubleClicked(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TailButtonLongPressed<Impl: IPenButtonListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TailButtonLongPressed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonLongPressedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonLongPressedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTailButtonLongPressed<Impl: IPenButtonListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveTailButtonLongPressed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPenButtonListener>, base.5, IsSupported::<Impl, OFFSET>, IsSupportedChanged::<Impl, OFFSET>, RemoveIsSupportedChanged::<Impl, OFFSET>, TailButtonClicked::<Impl, OFFSET>, RemoveTailButtonClicked::<Impl, OFFSET>, TailButtonDoubleClicked::<Impl, OFFSET>, RemoveTailButtonDoubleClicked::<Impl, OFFSET>, TailButtonLongPressed::<Impl, OFFSET>, RemoveTailButtonLongPressed::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenButtonListenerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<PenButtonListener>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenButtonListenerStatics {
    const NAME: &'static str = "Windows.Devices.Input.IPenButtonListenerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPenButtonListenerStaticsVtbl {
    pub const fn new<Impl: IPenButtonListenerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPenButtonListenerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IPenButtonListenerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPenButtonListenerStatics>, base.5, GetDefault::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenDeviceImpl: Sized {
    fn PenId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenDevice {
    const NAME: &'static str = "Windows.Devices.Input.IPenDevice";
}
#[cfg(feature = "implement_exclusive")]
impl IPenDeviceVtbl {
    pub const fn new<Impl: IPenDeviceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPenDeviceVtbl {
        unsafe extern "system" fn PenId<Impl: IPenDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PenId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPenDevice>, base.5, PenId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenDevice2Impl: Sized {
    fn SimpleHapticsController(&self) -> ::windows::core::Result<super::Haptics::SimpleHapticsController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenDevice2 {
    const NAME: &'static str = "Windows.Devices.Input.IPenDevice2";
}
#[cfg(feature = "implement_exclusive")]
impl IPenDevice2Vtbl {
    pub const fn new<Impl: IPenDevice2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPenDevice2Vtbl {
        unsafe extern "system" fn SimpleHapticsController<Impl: IPenDevice2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SimpleHapticsController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPenDevice2>, base.5, SimpleHapticsController::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenDeviceStaticsImpl: Sized {
    fn GetFromPointerId(&self, pointerid: u32) -> ::windows::core::Result<PenDevice>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Input.IPenDeviceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPenDeviceStaticsVtbl {
    pub const fn new<Impl: IPenDeviceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPenDeviceStaticsVtbl {
        unsafe extern "system" fn GetFromPointerId<Impl: IPenDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointerid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFromPointerId(pointerid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPenDeviceStatics>, base.5, GetFromPointerId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenDockListenerImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn IsSupportedChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PenDockListener, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsSupportedChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Docked(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PenDockListener, PenDockedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDocked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Undocked(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PenDockListener, PenUndockedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUndocked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenDockListener {
    const NAME: &'static str = "Windows.Devices.Input.IPenDockListener";
}
#[cfg(feature = "implement_exclusive")]
impl IPenDockListenerVtbl {
    pub const fn new<Impl: IPenDockListenerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPenDockListenerVtbl {
        unsafe extern "system" fn IsSupported<Impl: IPenDockListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupportedChanged<Impl: IPenDockListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSupportedChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PenDockListener, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PenDockListener, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIsSupportedChanged<Impl: IPenDockListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveIsSupportedChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Docked<Impl: IPenDockListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Docked(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PenDockListener, PenDockedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PenDockListener, PenDockedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDocked<Impl: IPenDockListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDocked(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Undocked<Impl: IPenDockListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Undocked(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PenDockListener, PenUndockedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PenDockListener, PenUndockedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUndocked<Impl: IPenDockListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveUndocked(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPenDockListener>, base.5, IsSupported::<Impl, OFFSET>, IsSupportedChanged::<Impl, OFFSET>, RemoveIsSupportedChanged::<Impl, OFFSET>, Docked::<Impl, OFFSET>, RemoveDocked::<Impl, OFFSET>, Undocked::<Impl, OFFSET>, RemoveUndocked::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenDockListenerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<PenDockListener>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenDockListenerStatics {
    const NAME: &'static str = "Windows.Devices.Input.IPenDockListenerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPenDockListenerStaticsVtbl {
    pub const fn new<Impl: IPenDockListenerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPenDockListenerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IPenDockListenerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPenDockListenerStatics>, base.5, GetDefault::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenDockedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenDockedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.IPenDockedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPenDockedEventArgsVtbl {
    pub const fn new<Impl: IPenDockedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPenDockedEventArgsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPenDockedEventArgs>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenTailButtonClickedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenTailButtonClickedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.IPenTailButtonClickedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPenTailButtonClickedEventArgsVtbl {
    pub const fn new<Impl: IPenTailButtonClickedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPenTailButtonClickedEventArgsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPenTailButtonClickedEventArgs>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenTailButtonDoubleClickedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenTailButtonDoubleClickedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.IPenTailButtonDoubleClickedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPenTailButtonDoubleClickedEventArgsVtbl {
    pub const fn new<Impl: IPenTailButtonDoubleClickedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPenTailButtonDoubleClickedEventArgsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPenTailButtonDoubleClickedEventArgs>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenTailButtonLongPressedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenTailButtonLongPressedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.IPenTailButtonLongPressedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPenTailButtonLongPressedEventArgsVtbl {
    pub const fn new<Impl: IPenTailButtonLongPressedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPenTailButtonLongPressedEventArgsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPenTailButtonLongPressedEventArgs>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenUndockedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenUndockedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.IPenUndockedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPenUndockedEventArgsVtbl {
    pub const fn new<Impl: IPenUndockedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPenUndockedEventArgsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPenUndockedEventArgs>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerDeviceImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<PointerDeviceType>;
    fn IsIntegrated(&self) -> ::windows::core::Result<bool>;
    fn MaxContacts(&self) -> ::windows::core::Result<u32>;
    fn PhysicalDeviceRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn ScreenRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SupportedUsages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PointerDeviceUsage>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerDevice {
    const NAME: &'static str = "Windows.Devices.Input.IPointerDevice";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerDeviceVtbl {
    pub const fn new<Impl: IPointerDeviceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointerDeviceVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IPointerDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIntegrated<Impl: IPointerDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsIntegrated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxContacts<Impl: IPointerDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxContacts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhysicalDeviceRect<Impl: IPointerDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PhysicalDeviceRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScreenRect<Impl: IPointerDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScreenRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedUsages<Impl: IPointerDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedUsages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointerDevice>, base.5, PointerDeviceType::<Impl, OFFSET>, IsIntegrated::<Impl, OFFSET>, MaxContacts::<Impl, OFFSET>, PhysicalDeviceRect::<Impl, OFFSET>, ScreenRect::<Impl, OFFSET>, SupportedUsages::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerDevice2Impl: Sized {
    fn MaxPointersWithZDistance(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerDevice2 {
    const NAME: &'static str = "Windows.Devices.Input.IPointerDevice2";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerDevice2Vtbl {
    pub const fn new<Impl: IPointerDevice2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointerDevice2Vtbl {
        unsafe extern "system" fn MaxPointersWithZDistance<Impl: IPointerDevice2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxPointersWithZDistance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointerDevice2>, base.5, MaxPointersWithZDistance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerDeviceStaticsImpl: Sized {
    fn GetPointerDevice(&self, pointerid: u32) -> ::windows::core::Result<PointerDevice>;
    fn GetPointerDevices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PointerDevice>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Input.IPointerDeviceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerDeviceStaticsVtbl {
    pub const fn new<Impl: IPointerDeviceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointerDeviceStaticsVtbl {
        unsafe extern "system" fn GetPointerDevice<Impl: IPointerDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointerid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPointerDevice(pointerid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPointerDevices<Impl: IPointerDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPointerDevices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointerDeviceStatics>, base.5, GetPointerDevice::<Impl, OFFSET>, GetPointerDevices::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITouchCapabilitiesImpl: Sized {
    fn TouchPresent(&self) -> ::windows::core::Result<i32>;
    fn Contacts(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITouchCapabilities {
    const NAME: &'static str = "Windows.Devices.Input.ITouchCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl ITouchCapabilitiesVtbl {
    pub const fn new<Impl: ITouchCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITouchCapabilitiesVtbl {
        unsafe extern "system" fn TouchPresent<Impl: ITouchCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TouchPresent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contacts<Impl: ITouchCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Contacts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITouchCapabilities>, base.5, TouchPresent::<Impl, OFFSET>, Contacts::<Impl, OFFSET>)
    }
}
