#[cfg(all(feature = "Gaming_Input", feature = "implement_exclusive"))]
pub trait IInjectedInputGamepadInfo_Impl: Sized {
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
impl IInjectedInputGamepadInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInjectedInputGamepadInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInjectedInputGamepadInfo_Vtbl {
        unsafe extern "system" fn Buttons<Impl: IInjectedInputGamepadInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Gaming::Input::GamepadButtons) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtons<Impl: IInjectedInputGamepadInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Gaming::Input::GamepadButtons) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtons(value).into()
        }
        unsafe extern "system" fn LeftThumbstickX<Impl: IInjectedInputGamepadInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLeftThumbstickX<Impl: IInjectedInputGamepadInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeftThumbstickX(value).into()
        }
        unsafe extern "system" fn LeftThumbstickY<Impl: IInjectedInputGamepadInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLeftThumbstickY<Impl: IInjectedInputGamepadInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeftThumbstickY(value).into()
        }
        unsafe extern "system" fn LeftTrigger<Impl: IInjectedInputGamepadInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLeftTrigger<Impl: IInjectedInputGamepadInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeftTrigger(value).into()
        }
        unsafe extern "system" fn RightThumbstickX<Impl: IInjectedInputGamepadInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRightThumbstickX<Impl: IInjectedInputGamepadInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRightThumbstickX(value).into()
        }
        unsafe extern "system" fn RightThumbstickY<Impl: IInjectedInputGamepadInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRightThumbstickY<Impl: IInjectedInputGamepadInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRightThumbstickY(value).into()
        }
        unsafe extern "system" fn RightTrigger<Impl: IInjectedInputGamepadInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRightTrigger<Impl: IInjectedInputGamepadInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
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
pub trait IInjectedInputGamepadInfoFactory_Impl: Sized {
    fn CreateInstanceFromGamepadReading(&mut self, reading: &super::super::super::super::Gaming::Input::GamepadReading) -> ::windows::core::Result<InjectedInputGamepadInfo>;
}
#[cfg(all(feature = "Gaming_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInjectedInputGamepadInfoFactory {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInjectedInputGamepadInfoFactory";
}
#[cfg(all(feature = "Gaming_Input", feature = "implement_exclusive"))]
impl IInjectedInputGamepadInfoFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInjectedInputGamepadInfoFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInjectedInputGamepadInfoFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceFromGamepadReading<Impl: IInjectedInputGamepadInfoFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reading: super::super::super::super::Gaming::Input::GamepadReading, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInjectedInputKeyboardInfo_Impl: Sized {
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
impl IInjectedInputKeyboardInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInjectedInputKeyboardInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInjectedInputKeyboardInfo_Vtbl {
        unsafe extern "system" fn KeyOptions<Impl: IInjectedInputKeyboardInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputKeyOptions) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetKeyOptions<Impl: IInjectedInputKeyboardInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InjectedInputKeyOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyOptions(value).into()
        }
        unsafe extern "system" fn ScanCode<Impl: IInjectedInputKeyboardInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetScanCode<Impl: IInjectedInputKeyboardInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScanCode(value).into()
        }
        unsafe extern "system" fn VirtualKey<Impl: IInjectedInputKeyboardInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetVirtualKey<Impl: IInjectedInputKeyboardInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT {
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
pub trait IInjectedInputMouseInfo_Impl: Sized {
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
impl IInjectedInputMouseInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInjectedInputMouseInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInjectedInputMouseInfo_Vtbl {
        unsafe extern "system" fn MouseOptions<Impl: IInjectedInputMouseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputMouseOptions) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMouseOptions<Impl: IInjectedInputMouseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InjectedInputMouseOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMouseOptions(value).into()
        }
        unsafe extern "system" fn MouseData<Impl: IInjectedInputMouseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMouseData<Impl: IInjectedInputMouseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMouseData(value).into()
        }
        unsafe extern "system" fn DeltaY<Impl: IInjectedInputMouseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDeltaY<Impl: IInjectedInputMouseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeltaY(value).into()
        }
        unsafe extern "system" fn DeltaX<Impl: IInjectedInputMouseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDeltaX<Impl: IInjectedInputMouseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeltaX(value).into()
        }
        unsafe extern "system" fn TimeOffsetInMilliseconds<Impl: IInjectedInputMouseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTimeOffsetInMilliseconds<Impl: IInjectedInputMouseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
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
pub trait IInjectedInputPenInfo_Impl: Sized {
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
impl IInjectedInputPenInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInjectedInputPenInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInjectedInputPenInfo_Vtbl {
        unsafe extern "system" fn PointerInfo<Impl: IInjectedInputPenInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputPointerInfo) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPointerInfo<Impl: IInjectedInputPenInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InjectedInputPointerInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointerInfo(&*(&value as *const <InjectedInputPointerInfo as ::windows::core::Abi>::Abi as *const <InjectedInputPointerInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PenButtons<Impl: IInjectedInputPenInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputPenButtons) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPenButtons<Impl: IInjectedInputPenInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InjectedInputPenButtons) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPenButtons(value).into()
        }
        unsafe extern "system" fn PenParameters<Impl: IInjectedInputPenInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputPenParameters) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPenParameters<Impl: IInjectedInputPenInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InjectedInputPenParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPenParameters(value).into()
        }
        unsafe extern "system" fn Pressure<Impl: IInjectedInputPenInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPressure<Impl: IInjectedInputPenInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPressure(value).into()
        }
        unsafe extern "system" fn Rotation<Impl: IInjectedInputPenInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRotation<Impl: IInjectedInputPenInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotation(value).into()
        }
        unsafe extern "system" fn TiltX<Impl: IInjectedInputPenInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTiltX<Impl: IInjectedInputPenInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTiltX(value).into()
        }
        unsafe extern "system" fn TiltY<Impl: IInjectedInputPenInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTiltY<Impl: IInjectedInputPenInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
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
pub trait IInjectedInputTouchInfo_Impl: Sized {
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
impl IInjectedInputTouchInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInjectedInputTouchInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInjectedInputTouchInfo_Vtbl {
        unsafe extern "system" fn Contact<Impl: IInjectedInputTouchInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputRectangle) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContact<Impl: IInjectedInputTouchInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InjectedInputRectangle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContact(&*(&value as *const <InjectedInputRectangle as ::windows::core::Abi>::Abi as *const <InjectedInputRectangle as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Orientation<Impl: IInjectedInputTouchInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOrientation<Impl: IInjectedInputTouchInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOrientation(value).into()
        }
        unsafe extern "system" fn PointerInfo<Impl: IInjectedInputTouchInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputPointerInfo) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPointerInfo<Impl: IInjectedInputTouchInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InjectedInputPointerInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointerInfo(&*(&value as *const <InjectedInputPointerInfo as ::windows::core::Abi>::Abi as *const <InjectedInputPointerInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Pressure<Impl: IInjectedInputTouchInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPressure<Impl: IInjectedInputTouchInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPressure(value).into()
        }
        unsafe extern "system" fn TouchParameters<Impl: IInjectedInputTouchInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputTouchParameters) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTouchParameters<Impl: IInjectedInputTouchInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InjectedInputTouchParameters) -> ::windows::core::HRESULT {
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
pub trait IInputInjector_Impl: Sized {
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
impl IInputInjector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputInjector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputInjector_Vtbl {
        unsafe extern "system" fn InjectKeyboardInput<Impl: IInputInjector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InjectKeyboardInput(&*(&input as *const <super::super::super::super::Foundation::Collections::IIterable<InjectedInputKeyboardInfo> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IIterable<InjectedInputKeyboardInfo> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InjectMouseInput<Impl: IInputInjector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InjectMouseInput(&*(&input as *const <super::super::super::super::Foundation::Collections::IIterable<InjectedInputMouseInfo> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IIterable<InjectedInputMouseInfo> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InitializeTouchInjection<Impl: IInputInjector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visualmode: InjectedInputVisualizationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeTouchInjection(visualmode).into()
        }
        unsafe extern "system" fn InjectTouchInput<Impl: IInputInjector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InjectTouchInput(&*(&input as *const <super::super::super::super::Foundation::Collections::IIterable<InjectedInputTouchInfo> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IIterable<InjectedInputTouchInfo> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UninitializeTouchInjection<Impl: IInputInjector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UninitializeTouchInjection().into()
        }
        unsafe extern "system" fn InitializePenInjection<Impl: IInputInjector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visualmode: InjectedInputVisualizationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializePenInjection(visualmode).into()
        }
        unsafe extern "system" fn InjectPenInput<Impl: IInputInjector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InjectPenInput(&*(&input as *const <InjectedInputPenInfo as ::windows::core::Abi>::Abi as *const <InjectedInputPenInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UninitializePenInjection<Impl: IInputInjector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UninitializePenInjection().into()
        }
        unsafe extern "system" fn InjectShortcut<Impl: IInputInjector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortcut: InjectedInputShortcut) -> ::windows::core::HRESULT {
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
pub trait IInputInjector2_Impl: Sized + IInputInjector_Impl {
    fn InitializeGamepadInjection(&mut self) -> ::windows::core::Result<()>;
    fn InjectGamepadInput(&mut self, input: &::core::option::Option<InjectedInputGamepadInfo>) -> ::windows::core::Result<()>;
    fn UninitializeGamepadInjection(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInputInjector2 {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInputInjector2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInputInjector2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputInjector2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputInjector2_Vtbl {
        unsafe extern "system" fn InitializeGamepadInjection<Impl: IInputInjector2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeGamepadInjection().into()
        }
        unsafe extern "system" fn InjectGamepadInput<Impl: IInputInjector2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InjectGamepadInput(&*(&input as *const <InjectedInputGamepadInfo as ::windows::core::Abi>::Abi as *const <InjectedInputGamepadInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UninitializeGamepadInjection<Impl: IInputInjector2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IInputInjectorStatics_Impl: Sized {
    fn TryCreate(&mut self) -> ::windows::core::Result<InputInjector>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputInjectorStatics {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInputInjectorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInputInjectorStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputInjectorStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputInjectorStatics_Vtbl {
        unsafe extern "system" fn TryCreate<Impl: IInputInjectorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInputInjectorStatics2_Impl: Sized + IInputInjectorStatics_Impl {
    fn TryCreateForAppBroadcastOnly(&mut self) -> ::windows::core::Result<InputInjector>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputInjectorStatics2 {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInputInjectorStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IInputInjectorStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputInjectorStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputInjectorStatics2_Vtbl {
        unsafe extern "system" fn TryCreateForAppBroadcastOnly<Impl: IInputInjectorStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
