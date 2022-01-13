#[cfg(all(feature = "Gaming_Input", feature = "implement_exclusive"))]
pub trait IInjectedInputGamepadInfoImpl: Sized {
    fn Buttons(&mut self) -> ::windows::core::Result<super::super::super::super::Gaming::Input::GamepadButtons>;
    fn SetButtons(&mut self, value: super::super::super::super::Gaming::Input::GamepadButtons) -> ::windows::core::Result<()>;
    fn LeftThumbstickX(&mut self) -> ::windows::core::Result<f64>;
    fn SetLeftThumbstickX(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn LeftThumbstickY(&mut self) -> ::windows::core::Result<f64>;
    fn SetLeftThumbstickY(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn LeftTrigger(&mut self) -> ::windows::core::Result<f64>;
    fn SetLeftTrigger(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn RightThumbstickX(&mut self) -> ::windows::core::Result<f64>;
    fn SetRightThumbstickX(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn RightThumbstickY(&mut self) -> ::windows::core::Result<f64>;
    fn SetRightThumbstickY(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn RightTrigger(&mut self) -> ::windows::core::Result<f64>;
    fn SetRightTrigger(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Gaming_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInjectedInputGamepadInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInjectedInputGamepadInfo";
}
#[cfg(all(feature = "Gaming_Input", feature = "implement_exclusive"))]
impl IInjectedInputGamepadInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInjectedInputGamepadInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInjectedInputGamepadInfoVtbl {
        unsafe extern "system" fn Buttons<Impl: IInjectedInputGamepadInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Gaming::Input::GamepadButtons) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buttons() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtons<Impl: IInjectedInputGamepadInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Gaming::Input::GamepadButtons) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtons(value).into()
        }
        unsafe extern "system" fn LeftThumbstickX<Impl: IInjectedInputGamepadInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LeftThumbstickX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeftThumbstickX<Impl: IInjectedInputGamepadInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeftThumbstickX(value).into()
        }
        unsafe extern "system" fn LeftThumbstickY<Impl: IInjectedInputGamepadInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LeftThumbstickY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeftThumbstickY<Impl: IInjectedInputGamepadInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeftThumbstickY(value).into()
        }
        unsafe extern "system" fn LeftTrigger<Impl: IInjectedInputGamepadInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LeftTrigger() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeftTrigger<Impl: IInjectedInputGamepadInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeftTrigger(value).into()
        }
        unsafe extern "system" fn RightThumbstickX<Impl: IInjectedInputGamepadInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RightThumbstickX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRightThumbstickX<Impl: IInjectedInputGamepadInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRightThumbstickX(value).into()
        }
        unsafe extern "system" fn RightThumbstickY<Impl: IInjectedInputGamepadInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RightThumbstickY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRightThumbstickY<Impl: IInjectedInputGamepadInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRightThumbstickY(value).into()
        }
        unsafe extern "system" fn RightTrigger<Impl: IInjectedInputGamepadInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RightTrigger() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRightTrigger<Impl: IInjectedInputGamepadInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRightTrigger(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInjectedInputGamepadInfo, BASE_OFFSET>(),
            Buttons: Buttons::<Impl, IMPL_OFFSET>,
            SetButtons: SetButtons::<Impl, IMPL_OFFSET>,
            LeftThumbstickX: LeftThumbstickX::<Impl, IMPL_OFFSET>,
            SetLeftThumbstickX: SetLeftThumbstickX::<Impl, IMPL_OFFSET>,
            LeftThumbstickY: LeftThumbstickY::<Impl, IMPL_OFFSET>,
            SetLeftThumbstickY: SetLeftThumbstickY::<Impl, IMPL_OFFSET>,
            LeftTrigger: LeftTrigger::<Impl, IMPL_OFFSET>,
            SetLeftTrigger: SetLeftTrigger::<Impl, IMPL_OFFSET>,
            RightThumbstickX: RightThumbstickX::<Impl, IMPL_OFFSET>,
            SetRightThumbstickX: SetRightThumbstickX::<Impl, IMPL_OFFSET>,
            RightThumbstickY: RightThumbstickY::<Impl, IMPL_OFFSET>,
            SetRightThumbstickY: SetRightThumbstickY::<Impl, IMPL_OFFSET>,
            RightTrigger: RightTrigger::<Impl, IMPL_OFFSET>,
            SetRightTrigger: SetRightTrigger::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInjectedInputGamepadInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Gaming_Input", feature = "implement_exclusive"))]
pub trait IInjectedInputGamepadInfoFactoryImpl: Sized {
    fn CreateInstanceFromGamepadReading(&mut self, reading: &super::super::super::super::Gaming::Input::GamepadReading) -> ::windows::core::Result<InjectedInputGamepadInfo>;
}
#[cfg(all(feature = "Gaming_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInjectedInputGamepadInfoFactory {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInjectedInputGamepadInfoFactory";
}
#[cfg(all(feature = "Gaming_Input", feature = "implement_exclusive"))]
impl IInjectedInputGamepadInfoFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInjectedInputGamepadInfoFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInjectedInputGamepadInfoFactoryVtbl {
        unsafe extern "system" fn CreateInstanceFromGamepadReading<Impl: IInjectedInputGamepadInfoFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reading: super::super::super::super::Gaming::Input::GamepadReading, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceFromGamepadReading(&*(&reading as *const <super::super::super::super::Gaming::Input::GamepadReading as ::windows::core::Abi>::Abi as *const <super::super::super::super::Gaming::Input::GamepadReading as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInjectedInputGamepadInfoFactory, BASE_OFFSET>(),
            CreateInstanceFromGamepadReading: CreateInstanceFromGamepadReading::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInjectedInputGamepadInfoFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInjectedInputKeyboardInfoImpl: Sized {
    fn KeyOptions(&mut self) -> ::windows::core::Result<InjectedInputKeyOptions>;
    fn SetKeyOptions(&mut self, value: InjectedInputKeyOptions) -> ::windows::core::Result<()>;
    fn ScanCode(&mut self) -> ::windows::core::Result<u16>;
    fn SetScanCode(&mut self, value: u16) -> ::windows::core::Result<()>;
    fn VirtualKey(&mut self) -> ::windows::core::Result<u16>;
    fn SetVirtualKey(&mut self, value: u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInjectedInputKeyboardInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInjectedInputKeyboardInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IInjectedInputKeyboardInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInjectedInputKeyboardInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInjectedInputKeyboardInfoVtbl {
        unsafe extern "system" fn KeyOptions<Impl: IInjectedInputKeyboardInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputKeyOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyOptions<Impl: IInjectedInputKeyboardInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InjectedInputKeyOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyOptions(value).into()
        }
        unsafe extern "system" fn ScanCode<Impl: IInjectedInputKeyboardInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScanCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScanCode<Impl: IInjectedInputKeyboardInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScanCode(value).into()
        }
        unsafe extern "system" fn VirtualKey<Impl: IInjectedInputKeyboardInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VirtualKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVirtualKey<Impl: IInjectedInputKeyboardInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVirtualKey(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInjectedInputKeyboardInfo, BASE_OFFSET>(),
            KeyOptions: KeyOptions::<Impl, IMPL_OFFSET>,
            SetKeyOptions: SetKeyOptions::<Impl, IMPL_OFFSET>,
            ScanCode: ScanCode::<Impl, IMPL_OFFSET>,
            SetScanCode: SetScanCode::<Impl, IMPL_OFFSET>,
            VirtualKey: VirtualKey::<Impl, IMPL_OFFSET>,
            SetVirtualKey: SetVirtualKey::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInjectedInputKeyboardInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInjectedInputMouseInfoImpl: Sized {
    fn MouseOptions(&mut self) -> ::windows::core::Result<InjectedInputMouseOptions>;
    fn SetMouseOptions(&mut self, value: InjectedInputMouseOptions) -> ::windows::core::Result<()>;
    fn MouseData(&mut self) -> ::windows::core::Result<u32>;
    fn SetMouseData(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn DeltaY(&mut self) -> ::windows::core::Result<i32>;
    fn SetDeltaY(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn DeltaX(&mut self) -> ::windows::core::Result<i32>;
    fn SetDeltaX(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn TimeOffsetInMilliseconds(&mut self) -> ::windows::core::Result<u32>;
    fn SetTimeOffsetInMilliseconds(&mut self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInjectedInputMouseInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInjectedInputMouseInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IInjectedInputMouseInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInjectedInputMouseInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInjectedInputMouseInfoVtbl {
        unsafe extern "system" fn MouseOptions<Impl: IInjectedInputMouseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputMouseOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MouseOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMouseOptions<Impl: IInjectedInputMouseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InjectedInputMouseOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMouseOptions(value).into()
        }
        unsafe extern "system" fn MouseData<Impl: IInjectedInputMouseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MouseData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMouseData<Impl: IInjectedInputMouseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMouseData(value).into()
        }
        unsafe extern "system" fn DeltaY<Impl: IInjectedInputMouseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeltaY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeltaY<Impl: IInjectedInputMouseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeltaY(value).into()
        }
        unsafe extern "system" fn DeltaX<Impl: IInjectedInputMouseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeltaX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeltaX<Impl: IInjectedInputMouseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeltaX(value).into()
        }
        unsafe extern "system" fn TimeOffsetInMilliseconds<Impl: IInjectedInputMouseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeOffsetInMilliseconds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimeOffsetInMilliseconds<Impl: IInjectedInputMouseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimeOffsetInMilliseconds(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInjectedInputMouseInfo, BASE_OFFSET>(),
            MouseOptions: MouseOptions::<Impl, IMPL_OFFSET>,
            SetMouseOptions: SetMouseOptions::<Impl, IMPL_OFFSET>,
            MouseData: MouseData::<Impl, IMPL_OFFSET>,
            SetMouseData: SetMouseData::<Impl, IMPL_OFFSET>,
            DeltaY: DeltaY::<Impl, IMPL_OFFSET>,
            SetDeltaY: SetDeltaY::<Impl, IMPL_OFFSET>,
            DeltaX: DeltaX::<Impl, IMPL_OFFSET>,
            SetDeltaX: SetDeltaX::<Impl, IMPL_OFFSET>,
            TimeOffsetInMilliseconds: TimeOffsetInMilliseconds::<Impl, IMPL_OFFSET>,
            SetTimeOffsetInMilliseconds: SetTimeOffsetInMilliseconds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInjectedInputMouseInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInjectedInputPenInfoImpl: Sized {
    fn PointerInfo(&mut self) -> ::windows::core::Result<InjectedInputPointerInfo>;
    fn SetPointerInfo(&mut self, value: &InjectedInputPointerInfo) -> ::windows::core::Result<()>;
    fn PenButtons(&mut self) -> ::windows::core::Result<InjectedInputPenButtons>;
    fn SetPenButtons(&mut self, value: InjectedInputPenButtons) -> ::windows::core::Result<()>;
    fn PenParameters(&mut self) -> ::windows::core::Result<InjectedInputPenParameters>;
    fn SetPenParameters(&mut self, value: InjectedInputPenParameters) -> ::windows::core::Result<()>;
    fn Pressure(&mut self) -> ::windows::core::Result<f64>;
    fn SetPressure(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Rotation(&mut self) -> ::windows::core::Result<f64>;
    fn SetRotation(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn TiltX(&mut self) -> ::windows::core::Result<i32>;
    fn SetTiltX(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn TiltY(&mut self) -> ::windows::core::Result<i32>;
    fn SetTiltY(&mut self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInjectedInputPenInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInjectedInputPenInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IInjectedInputPenInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInjectedInputPenInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInjectedInputPenInfoVtbl {
        unsafe extern "system" fn PointerInfo<Impl: IInjectedInputPenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputPointerInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerInfo<Impl: IInjectedInputPenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InjectedInputPointerInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointerInfo(&*(&value as *const <InjectedInputPointerInfo as ::windows::core::Abi>::Abi as *const <InjectedInputPointerInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PenButtons<Impl: IInjectedInputPenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputPenButtons) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PenButtons() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPenButtons<Impl: IInjectedInputPenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InjectedInputPenButtons) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPenButtons(value).into()
        }
        unsafe extern "system" fn PenParameters<Impl: IInjectedInputPenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputPenParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PenParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPenParameters<Impl: IInjectedInputPenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InjectedInputPenParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPenParameters(value).into()
        }
        unsafe extern "system" fn Pressure<Impl: IInjectedInputPenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pressure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPressure<Impl: IInjectedInputPenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPressure(value).into()
        }
        unsafe extern "system" fn Rotation<Impl: IInjectedInputPenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rotation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotation<Impl: IInjectedInputPenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotation(value).into()
        }
        unsafe extern "system" fn TiltX<Impl: IInjectedInputPenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TiltX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTiltX<Impl: IInjectedInputPenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTiltX(value).into()
        }
        unsafe extern "system" fn TiltY<Impl: IInjectedInputPenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TiltY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTiltY<Impl: IInjectedInputPenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTiltY(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInjectedInputPenInfo, BASE_OFFSET>(),
            PointerInfo: PointerInfo::<Impl, IMPL_OFFSET>,
            SetPointerInfo: SetPointerInfo::<Impl, IMPL_OFFSET>,
            PenButtons: PenButtons::<Impl, IMPL_OFFSET>,
            SetPenButtons: SetPenButtons::<Impl, IMPL_OFFSET>,
            PenParameters: PenParameters::<Impl, IMPL_OFFSET>,
            SetPenParameters: SetPenParameters::<Impl, IMPL_OFFSET>,
            Pressure: Pressure::<Impl, IMPL_OFFSET>,
            SetPressure: SetPressure::<Impl, IMPL_OFFSET>,
            Rotation: Rotation::<Impl, IMPL_OFFSET>,
            SetRotation: SetRotation::<Impl, IMPL_OFFSET>,
            TiltX: TiltX::<Impl, IMPL_OFFSET>,
            SetTiltX: SetTiltX::<Impl, IMPL_OFFSET>,
            TiltY: TiltY::<Impl, IMPL_OFFSET>,
            SetTiltY: SetTiltY::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInjectedInputPenInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInjectedInputTouchInfoImpl: Sized {
    fn Contact(&mut self) -> ::windows::core::Result<InjectedInputRectangle>;
    fn SetContact(&mut self, value: &InjectedInputRectangle) -> ::windows::core::Result<()>;
    fn Orientation(&mut self) -> ::windows::core::Result<i32>;
    fn SetOrientation(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn PointerInfo(&mut self) -> ::windows::core::Result<InjectedInputPointerInfo>;
    fn SetPointerInfo(&mut self, value: &InjectedInputPointerInfo) -> ::windows::core::Result<()>;
    fn Pressure(&mut self) -> ::windows::core::Result<f64>;
    fn SetPressure(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn TouchParameters(&mut self) -> ::windows::core::Result<InjectedInputTouchParameters>;
    fn SetTouchParameters(&mut self, value: InjectedInputTouchParameters) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInjectedInputTouchInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInjectedInputTouchInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IInjectedInputTouchInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInjectedInputTouchInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInjectedInputTouchInfoVtbl {
        unsafe extern "system" fn Contact<Impl: IInjectedInputTouchInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputRectangle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContact<Impl: IInjectedInputTouchInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InjectedInputRectangle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContact(&*(&value as *const <InjectedInputRectangle as ::windows::core::Abi>::Abi as *const <InjectedInputRectangle as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Orientation<Impl: IInjectedInputTouchInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Orientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrientation<Impl: IInjectedInputTouchInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOrientation(value).into()
        }
        unsafe extern "system" fn PointerInfo<Impl: IInjectedInputTouchInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputPointerInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerInfo<Impl: IInjectedInputTouchInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InjectedInputPointerInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointerInfo(&*(&value as *const <InjectedInputPointerInfo as ::windows::core::Abi>::Abi as *const <InjectedInputPointerInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Pressure<Impl: IInjectedInputTouchInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pressure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPressure<Impl: IInjectedInputTouchInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPressure(value).into()
        }
        unsafe extern "system" fn TouchParameters<Impl: IInjectedInputTouchInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputTouchParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TouchParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTouchParameters<Impl: IInjectedInputTouchInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InjectedInputTouchParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTouchParameters(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInjectedInputTouchInfo, BASE_OFFSET>(),
            Contact: Contact::<Impl, IMPL_OFFSET>,
            SetContact: SetContact::<Impl, IMPL_OFFSET>,
            Orientation: Orientation::<Impl, IMPL_OFFSET>,
            SetOrientation: SetOrientation::<Impl, IMPL_OFFSET>,
            PointerInfo: PointerInfo::<Impl, IMPL_OFFSET>,
            SetPointerInfo: SetPointerInfo::<Impl, IMPL_OFFSET>,
            Pressure: Pressure::<Impl, IMPL_OFFSET>,
            SetPressure: SetPressure::<Impl, IMPL_OFFSET>,
            TouchParameters: TouchParameters::<Impl, IMPL_OFFSET>,
            SetTouchParameters: SetTouchParameters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInjectedInputTouchInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInputInjectorImpl: Sized {
    fn InjectKeyboardInput(&mut self, input: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<InjectedInputKeyboardInfo>>) -> ::windows::core::Result<()>;
    fn InjectMouseInput(&mut self, input: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<InjectedInputMouseInfo>>) -> ::windows::core::Result<()>;
    fn InitializeTouchInjection(&mut self, visualmode: InjectedInputVisualizationMode) -> ::windows::core::Result<()>;
    fn InjectTouchInput(&mut self, input: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<InjectedInputTouchInfo>>) -> ::windows::core::Result<()>;
    fn UninitializeTouchInjection(&mut self) -> ::windows::core::Result<()>;
    fn InitializePenInjection(&mut self, visualmode: InjectedInputVisualizationMode) -> ::windows::core::Result<()>;
    fn InjectPenInput(&mut self, input: &::core::option::Option<InjectedInputPenInfo>) -> ::windows::core::Result<()>;
    fn UninitializePenInjection(&mut self) -> ::windows::core::Result<()>;
    fn InjectShortcut(&mut self, shortcut: InjectedInputShortcut) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInputInjector {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInputInjector";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInputInjectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputInjectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputInjectorVtbl {
        unsafe extern "system" fn InjectKeyboardInput<Impl: IInputInjectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InjectKeyboardInput(&*(&input as *const <super::super::super::super::Foundation::Collections::IIterable<InjectedInputKeyboardInfo> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IIterable<InjectedInputKeyboardInfo> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InjectMouseInput<Impl: IInputInjectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InjectMouseInput(&*(&input as *const <super::super::super::super::Foundation::Collections::IIterable<InjectedInputMouseInfo> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IIterable<InjectedInputMouseInfo> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InitializeTouchInjection<Impl: IInputInjectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visualmode: InjectedInputVisualizationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeTouchInjection(visualmode).into()
        }
        unsafe extern "system" fn InjectTouchInput<Impl: IInputInjectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InjectTouchInput(&*(&input as *const <super::super::super::super::Foundation::Collections::IIterable<InjectedInputTouchInfo> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IIterable<InjectedInputTouchInfo> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UninitializeTouchInjection<Impl: IInputInjectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UninitializeTouchInjection().into()
        }
        unsafe extern "system" fn InitializePenInjection<Impl: IInputInjectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visualmode: InjectedInputVisualizationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializePenInjection(visualmode).into()
        }
        unsafe extern "system" fn InjectPenInput<Impl: IInputInjectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InjectPenInput(&*(&input as *const <InjectedInputPenInfo as ::windows::core::Abi>::Abi as *const <InjectedInputPenInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UninitializePenInjection<Impl: IInputInjectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UninitializePenInjection().into()
        }
        unsafe extern "system" fn InjectShortcut<Impl: IInputInjectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortcut: InjectedInputShortcut) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InjectShortcut(shortcut).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInputInjector, BASE_OFFSET>(),
            InjectKeyboardInput: InjectKeyboardInput::<Impl, IMPL_OFFSET>,
            InjectMouseInput: InjectMouseInput::<Impl, IMPL_OFFSET>,
            InitializeTouchInjection: InitializeTouchInjection::<Impl, IMPL_OFFSET>,
            InjectTouchInput: InjectTouchInput::<Impl, IMPL_OFFSET>,
            UninitializeTouchInjection: UninitializeTouchInjection::<Impl, IMPL_OFFSET>,
            InitializePenInjection: InitializePenInjection::<Impl, IMPL_OFFSET>,
            InjectPenInput: InjectPenInput::<Impl, IMPL_OFFSET>,
            UninitializePenInjection: UninitializePenInjection::<Impl, IMPL_OFFSET>,
            InjectShortcut: InjectShortcut::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputInjector as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInputInjector2Impl: Sized + IInputInjectorImpl {
    fn InitializeGamepadInjection(&mut self) -> ::windows::core::Result<()>;
    fn InjectGamepadInput(&mut self, input: &::core::option::Option<InjectedInputGamepadInfo>) -> ::windows::core::Result<()>;
    fn UninitializeGamepadInjection(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInputInjector2 {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInputInjector2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInputInjector2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputInjector2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputInjector2Vtbl {
        unsafe extern "system" fn InitializeGamepadInjection<Impl: IInputInjector2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeGamepadInjection().into()
        }
        unsafe extern "system" fn InjectGamepadInput<Impl: IInputInjector2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InjectGamepadInput(&*(&input as *const <InjectedInputGamepadInfo as ::windows::core::Abi>::Abi as *const <InjectedInputGamepadInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UninitializeGamepadInjection<Impl: IInputInjector2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UninitializeGamepadInjection().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInputInjector2, BASE_OFFSET>(),
            InitializeGamepadInjection: InitializeGamepadInjection::<Impl, IMPL_OFFSET>,
            InjectGamepadInput: InjectGamepadInput::<Impl, IMPL_OFFSET>,
            UninitializeGamepadInjection: UninitializeGamepadInjection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputInjector2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputInjectorStaticsImpl: Sized {
    fn TryCreate(&mut self) -> ::windows::core::Result<InputInjector>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputInjectorStatics {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInputInjectorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInputInjectorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputInjectorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputInjectorStaticsVtbl {
        unsafe extern "system" fn TryCreate<Impl: IInputInjectorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInputInjectorStatics, BASE_OFFSET>(), TryCreate: TryCreate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputInjectorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputInjectorStatics2Impl: Sized + IInputInjectorStaticsImpl {
    fn TryCreateForAppBroadcastOnly(&mut self) -> ::windows::core::Result<InputInjector>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputInjectorStatics2 {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInputInjectorStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IInputInjectorStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputInjectorStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputInjectorStatics2Vtbl {
        unsafe extern "system" fn TryCreateForAppBroadcastOnly<Impl: IInputInjectorStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateForAppBroadcastOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInputInjectorStatics2, BASE_OFFSET>(),
            TryCreateForAppBroadcastOnly: TryCreateForAppBroadcastOnly::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputInjectorStatics2 as ::windows::core::Interface>::IID
    }
}
