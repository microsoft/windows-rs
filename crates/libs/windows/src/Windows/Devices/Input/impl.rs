#[cfg(feature = "implement_exclusive")]
pub trait IKeyboardCapabilities_Impl: Sized {
    fn KeyboardPresent(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyboardCapabilities {
    const NAME: &'static str = "Windows.Devices.Input.IKeyboardCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyboardCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyboardCapabilities_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyboardCapabilities_Vtbl {
        unsafe extern "system" fn KeyboardPresent<Impl: IKeyboardCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyboardPresent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyboardCapabilities, BASE_OFFSET>(),
            KeyboardPresent: KeyboardPresent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyboardCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMouseCapabilities_Impl: Sized {
    fn MousePresent(&mut self) -> ::windows::core::Result<i32>;
    fn VerticalWheelPresent(&mut self) -> ::windows::core::Result<i32>;
    fn HorizontalWheelPresent(&mut self) -> ::windows::core::Result<i32>;
    fn SwapButtons(&mut self) -> ::windows::core::Result<i32>;
    fn NumberOfButtons(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMouseCapabilities {
    const NAME: &'static str = "Windows.Devices.Input.IMouseCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IMouseCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMouseCapabilities_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMouseCapabilities_Vtbl {
        unsafe extern "system" fn MousePresent<Impl: IMouseCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MousePresent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalWheelPresent<Impl: IMouseCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalWheelPresent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalWheelPresent<Impl: IMouseCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalWheelPresent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SwapButtons<Impl: IMouseCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SwapButtons() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfButtons<Impl: IMouseCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfButtons() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMouseCapabilities, BASE_OFFSET>(),
            MousePresent: MousePresent::<Impl, IMPL_OFFSET>,
            VerticalWheelPresent: VerticalWheelPresent::<Impl, IMPL_OFFSET>,
            HorizontalWheelPresent: HorizontalWheelPresent::<Impl, IMPL_OFFSET>,
            SwapButtons: SwapButtons::<Impl, IMPL_OFFSET>,
            NumberOfButtons: NumberOfButtons::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMouseCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMouseDevice_Impl: Sized {
    fn MouseMoved(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MouseDevice, MouseEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMouseMoved(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMouseDevice {
    const NAME: &'static str = "Windows.Devices.Input.IMouseDevice";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMouseDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMouseDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMouseDevice_Vtbl {
        unsafe extern "system" fn MouseMoved<Impl: IMouseDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MouseMoved(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MouseDevice, MouseEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MouseDevice, MouseEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMouseMoved<Impl: IMouseDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMouseMoved(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMouseDevice, BASE_OFFSET>(),
            MouseMoved: MouseMoved::<Impl, IMPL_OFFSET>,
            RemoveMouseMoved: RemoveMouseMoved::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMouseDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMouseDeviceStatics_Impl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<MouseDevice>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMouseDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Input.IMouseDeviceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMouseDeviceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMouseDeviceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMouseDeviceStatics_Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IMouseDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMouseDeviceStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMouseDeviceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMouseEventArgs_Impl: Sized {
    fn MouseDelta(&mut self) -> ::windows::core::Result<MouseDelta>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMouseEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.IMouseEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMouseEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMouseEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMouseEventArgs_Vtbl {
        unsafe extern "system" fn MouseDelta<Impl: IMouseEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MouseDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MouseDelta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMouseEventArgs, BASE_OFFSET>(), MouseDelta: MouseDelta::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMouseEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPenButtonListener_Impl: Sized {
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsSupportedChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PenButtonListener, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsSupportedChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TailButtonClicked(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonClickedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTailButtonClicked(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TailButtonDoubleClicked(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonDoubleClickedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTailButtonDoubleClicked(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TailButtonLongPressed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonLongPressedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTailButtonLongPressed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPenButtonListener {
    const NAME: &'static str = "Windows.Devices.Input.IPenButtonListener";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPenButtonListener_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPenButtonListener_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPenButtonListener_Vtbl {
        unsafe extern "system" fn IsSupported<Impl: IPenButtonListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupportedChanged<Impl: IPenButtonListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupportedChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PenButtonListener, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PenButtonListener, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIsSupportedChanged<Impl: IPenButtonListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveIsSupportedChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TailButtonClicked<Impl: IPenButtonListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TailButtonClicked(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonClickedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonClickedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTailButtonClicked<Impl: IPenButtonListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTailButtonClicked(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TailButtonDoubleClicked<Impl: IPenButtonListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TailButtonDoubleClicked(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonDoubleClickedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonDoubleClickedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTailButtonDoubleClicked<Impl: IPenButtonListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTailButtonDoubleClicked(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TailButtonLongPressed<Impl: IPenButtonListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TailButtonLongPressed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonLongPressedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonLongPressedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTailButtonLongPressed<Impl: IPenButtonListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTailButtonLongPressed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPenButtonListener, BASE_OFFSET>(),
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
            IsSupportedChanged: IsSupportedChanged::<Impl, IMPL_OFFSET>,
            RemoveIsSupportedChanged: RemoveIsSupportedChanged::<Impl, IMPL_OFFSET>,
            TailButtonClicked: TailButtonClicked::<Impl, IMPL_OFFSET>,
            RemoveTailButtonClicked: RemoveTailButtonClicked::<Impl, IMPL_OFFSET>,
            TailButtonDoubleClicked: TailButtonDoubleClicked::<Impl, IMPL_OFFSET>,
            RemoveTailButtonDoubleClicked: RemoveTailButtonDoubleClicked::<Impl, IMPL_OFFSET>,
            TailButtonLongPressed: TailButtonLongPressed::<Impl, IMPL_OFFSET>,
            RemoveTailButtonLongPressed: RemoveTailButtonLongPressed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPenButtonListener as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenButtonListenerStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<PenButtonListener>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenButtonListenerStatics {
    const NAME: &'static str = "Windows.Devices.Input.IPenButtonListenerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPenButtonListenerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPenButtonListenerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPenButtonListenerStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IPenButtonListenerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPenButtonListenerStatics, BASE_OFFSET>(), GetDefault: GetDefault::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPenButtonListenerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenDevice_Impl: Sized {
    fn PenId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenDevice {
    const NAME: &'static str = "Windows.Devices.Input.IPenDevice";
}
#[cfg(feature = "implement_exclusive")]
impl IPenDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPenDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPenDevice_Vtbl {
        unsafe extern "system" fn PenId<Impl: IPenDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PenId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPenDevice, BASE_OFFSET>(), PenId: PenId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPenDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
pub trait IPenDevice2_Impl: Sized {
    fn SimpleHapticsController(&mut self) -> ::windows::core::Result<super::Haptics::SimpleHapticsController>;
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPenDevice2 {
    const NAME: &'static str = "Windows.Devices.Input.IPenDevice2";
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
impl IPenDevice2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPenDevice2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPenDevice2_Vtbl {
        unsafe extern "system" fn SimpleHapticsController<Impl: IPenDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimpleHapticsController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPenDevice2, BASE_OFFSET>(),
            SimpleHapticsController: SimpleHapticsController::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPenDevice2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenDeviceStatics_Impl: Sized {
    fn GetFromPointerId(&mut self, pointerid: u32) -> ::windows::core::Result<PenDevice>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Input.IPenDeviceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPenDeviceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPenDeviceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPenDeviceStatics_Vtbl {
        unsafe extern "system" fn GetFromPointerId<Impl: IPenDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFromPointerId(pointerid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPenDeviceStatics, BASE_OFFSET>(),
            GetFromPointerId: GetFromPointerId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPenDeviceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPenDockListener_Impl: Sized {
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsSupportedChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PenDockListener, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsSupportedChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Docked(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PenDockListener, PenDockedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDocked(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Undocked(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PenDockListener, PenUndockedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUndocked(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPenDockListener {
    const NAME: &'static str = "Windows.Devices.Input.IPenDockListener";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPenDockListener_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPenDockListener_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPenDockListener_Vtbl {
        unsafe extern "system" fn IsSupported<Impl: IPenDockListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupportedChanged<Impl: IPenDockListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupportedChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PenDockListener, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PenDockListener, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIsSupportedChanged<Impl: IPenDockListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveIsSupportedChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Docked<Impl: IPenDockListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Docked(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PenDockListener, PenDockedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PenDockListener, PenDockedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDocked<Impl: IPenDockListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDocked(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Undocked<Impl: IPenDockListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Undocked(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PenDockListener, PenUndockedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PenDockListener, PenUndockedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUndocked<Impl: IPenDockListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUndocked(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPenDockListener, BASE_OFFSET>(),
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
            IsSupportedChanged: IsSupportedChanged::<Impl, IMPL_OFFSET>,
            RemoveIsSupportedChanged: RemoveIsSupportedChanged::<Impl, IMPL_OFFSET>,
            Docked: Docked::<Impl, IMPL_OFFSET>,
            RemoveDocked: RemoveDocked::<Impl, IMPL_OFFSET>,
            Undocked: Undocked::<Impl, IMPL_OFFSET>,
            RemoveUndocked: RemoveUndocked::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPenDockListener as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenDockListenerStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<PenDockListener>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenDockListenerStatics {
    const NAME: &'static str = "Windows.Devices.Input.IPenDockListenerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPenDockListenerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPenDockListenerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPenDockListenerStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IPenDockListenerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPenDockListenerStatics, BASE_OFFSET>(), GetDefault: GetDefault::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPenDockListenerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenDockedEventArgs_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenDockedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.IPenDockedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPenDockedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPenDockedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPenDockedEventArgs_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPenDockedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPenDockedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenTailButtonClickedEventArgs_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenTailButtonClickedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.IPenTailButtonClickedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPenTailButtonClickedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPenTailButtonClickedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPenTailButtonClickedEventArgs_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPenTailButtonClickedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPenTailButtonClickedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenTailButtonDoubleClickedEventArgs_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenTailButtonDoubleClickedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.IPenTailButtonDoubleClickedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPenTailButtonDoubleClickedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPenTailButtonDoubleClickedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPenTailButtonDoubleClickedEventArgs_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPenTailButtonDoubleClickedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPenTailButtonDoubleClickedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenTailButtonLongPressedEventArgs_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenTailButtonLongPressedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.IPenTailButtonLongPressedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPenTailButtonLongPressedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPenTailButtonLongPressedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPenTailButtonLongPressedEventArgs_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPenTailButtonLongPressedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPenTailButtonLongPressedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenUndockedEventArgs_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenUndockedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.IPenUndockedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPenUndockedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPenUndockedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPenUndockedEventArgs_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPenUndockedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPenUndockedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPointerDevice_Impl: Sized {
    fn PointerDeviceType(&mut self) -> ::windows::core::Result<PointerDeviceType>;
    fn IsIntegrated(&mut self) -> ::windows::core::Result<bool>;
    fn MaxContacts(&mut self) -> ::windows::core::Result<u32>;
    fn PhysicalDeviceRect(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn ScreenRect(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SupportedUsages(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PointerDeviceUsage>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPointerDevice {
    const NAME: &'static str = "Windows.Devices.Input.IPointerDevice";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPointerDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerDevice_Vtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IPointerDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIntegrated<Impl: IPointerDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsIntegrated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxContacts<Impl: IPointerDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxContacts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhysicalDeviceRect<Impl: IPointerDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhysicalDeviceRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScreenRect<Impl: IPointerDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScreenRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedUsages<Impl: IPointerDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedUsages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointerDevice, BASE_OFFSET>(),
            PointerDeviceType: PointerDeviceType::<Impl, IMPL_OFFSET>,
            IsIntegrated: IsIntegrated::<Impl, IMPL_OFFSET>,
            MaxContacts: MaxContacts::<Impl, IMPL_OFFSET>,
            PhysicalDeviceRect: PhysicalDeviceRect::<Impl, IMPL_OFFSET>,
            ScreenRect: ScreenRect::<Impl, IMPL_OFFSET>,
            SupportedUsages: SupportedUsages::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerDevice2_Impl: Sized {
    fn MaxPointersWithZDistance(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerDevice2 {
    const NAME: &'static str = "Windows.Devices.Input.IPointerDevice2";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerDevice2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerDevice2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerDevice2_Vtbl {
        unsafe extern "system" fn MaxPointersWithZDistance<Impl: IPointerDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPointersWithZDistance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointerDevice2, BASE_OFFSET>(),
            MaxPointersWithZDistance: MaxPointersWithZDistance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerDevice2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPointerDeviceStatics_Impl: Sized {
    fn GetPointerDevice(&mut self, pointerid: u32) -> ::windows::core::Result<PointerDevice>;
    fn GetPointerDevices(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PointerDevice>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPointerDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Input.IPointerDeviceStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPointerDeviceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerDeviceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerDeviceStatics_Vtbl {
        unsafe extern "system" fn GetPointerDevice<Impl: IPointerDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPointerDevice(pointerid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPointerDevices<Impl: IPointerDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPointerDevices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointerDeviceStatics, BASE_OFFSET>(),
            GetPointerDevice: GetPointerDevice::<Impl, IMPL_OFFSET>,
            GetPointerDevices: GetPointerDevices::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerDeviceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITouchCapabilities_Impl: Sized {
    fn TouchPresent(&mut self) -> ::windows::core::Result<i32>;
    fn Contacts(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITouchCapabilities {
    const NAME: &'static str = "Windows.Devices.Input.ITouchCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl ITouchCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITouchCapabilities_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITouchCapabilities_Vtbl {
        unsafe extern "system" fn TouchPresent<Impl: ITouchCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TouchPresent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contacts<Impl: ITouchCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contacts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITouchCapabilities, BASE_OFFSET>(),
            TouchPresent: TouchPresent::<Impl, IMPL_OFFSET>,
            Contacts: Contacts::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITouchCapabilities as ::windows::core::Interface>::IID
    }
}
