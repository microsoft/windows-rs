#[cfg(feature = "implement_exclusive")]
pub trait IInjectedInputGamepadInfoImpl: Sized {
    fn Buttons(&self) -> ::windows::core::Result<super::super::super::super::Gaming::Input::GamepadButtons>;
    fn SetButtons(&self, value: super::super::super::super::Gaming::Input::GamepadButtons) -> ::windows::core::Result<()>;
    fn LeftThumbstickX(&self) -> ::windows::core::Result<f64>;
    fn SetLeftThumbstickX(&self, value: f64) -> ::windows::core::Result<()>;
    fn LeftThumbstickY(&self) -> ::windows::core::Result<f64>;
    fn SetLeftThumbstickY(&self, value: f64) -> ::windows::core::Result<()>;
    fn LeftTrigger(&self) -> ::windows::core::Result<f64>;
    fn SetLeftTrigger(&self, value: f64) -> ::windows::core::Result<()>;
    fn RightThumbstickX(&self) -> ::windows::core::Result<f64>;
    fn SetRightThumbstickX(&self, value: f64) -> ::windows::core::Result<()>;
    fn RightThumbstickY(&self) -> ::windows::core::Result<f64>;
    fn SetRightThumbstickY(&self, value: f64) -> ::windows::core::Result<()>;
    fn RightTrigger(&self) -> ::windows::core::Result<f64>;
    fn SetRightTrigger(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInjectedInputGamepadInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInjectedInputGamepadInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IInjectedInputGamepadInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInjectedInputGamepadInfoImpl, const OFFSET: isize>() -> IInjectedInputGamepadInfoVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IInjectedInputGamepadInfo>,
            ::windows::core::GetTrustLevel,
            Buttons::<Impl, OFFSET>,
            SetButtons::<Impl, OFFSET>,
            LeftThumbstickX::<Impl, OFFSET>,
            SetLeftThumbstickX::<Impl, OFFSET>,
            LeftThumbstickY::<Impl, OFFSET>,
            SetLeftThumbstickY::<Impl, OFFSET>,
            LeftTrigger::<Impl, OFFSET>,
            SetLeftTrigger::<Impl, OFFSET>,
            RightThumbstickX::<Impl, OFFSET>,
            SetRightThumbstickX::<Impl, OFFSET>,
            RightThumbstickY::<Impl, OFFSET>,
            SetRightThumbstickY::<Impl, OFFSET>,
            RightTrigger::<Impl, OFFSET>,
            SetRightTrigger::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInjectedInputGamepadInfoFactoryImpl: Sized {
    fn CreateInstanceFromGamepadReading(&self, reading: &super::super::super::super::Gaming::Input::GamepadReading) -> ::windows::core::Result<InjectedInputGamepadInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInjectedInputGamepadInfoFactory {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInjectedInputGamepadInfoFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IInjectedInputGamepadInfoFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInjectedInputGamepadInfoFactoryImpl, const OFFSET: isize>() -> IInjectedInputGamepadInfoFactoryVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInjectedInputGamepadInfoFactory>, ::windows::core::GetTrustLevel, CreateInstanceFromGamepadReading::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInjectedInputKeyboardInfoImpl: Sized {
    fn KeyOptions(&self) -> ::windows::core::Result<InjectedInputKeyOptions>;
    fn SetKeyOptions(&self, value: InjectedInputKeyOptions) -> ::windows::core::Result<()>;
    fn ScanCode(&self) -> ::windows::core::Result<u16>;
    fn SetScanCode(&self, value: u16) -> ::windows::core::Result<()>;
    fn VirtualKey(&self) -> ::windows::core::Result<u16>;
    fn SetVirtualKey(&self, value: u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInjectedInputKeyboardInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInjectedInputKeyboardInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IInjectedInputKeyboardInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInjectedInputKeyboardInfoImpl, const OFFSET: isize>() -> IInjectedInputKeyboardInfoVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInjectedInputKeyboardInfo>, ::windows::core::GetTrustLevel, KeyOptions::<Impl, OFFSET>, SetKeyOptions::<Impl, OFFSET>, ScanCode::<Impl, OFFSET>, SetScanCode::<Impl, OFFSET>, VirtualKey::<Impl, OFFSET>, SetVirtualKey::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInjectedInputMouseInfoImpl: Sized {
    fn MouseOptions(&self) -> ::windows::core::Result<InjectedInputMouseOptions>;
    fn SetMouseOptions(&self, value: InjectedInputMouseOptions) -> ::windows::core::Result<()>;
    fn MouseData(&self) -> ::windows::core::Result<u32>;
    fn SetMouseData(&self, value: u32) -> ::windows::core::Result<()>;
    fn DeltaY(&self) -> ::windows::core::Result<i32>;
    fn SetDeltaY(&self, value: i32) -> ::windows::core::Result<()>;
    fn DeltaX(&self) -> ::windows::core::Result<i32>;
    fn SetDeltaX(&self, value: i32) -> ::windows::core::Result<()>;
    fn TimeOffsetInMilliseconds(&self) -> ::windows::core::Result<u32>;
    fn SetTimeOffsetInMilliseconds(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInjectedInputMouseInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInjectedInputMouseInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IInjectedInputMouseInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInjectedInputMouseInfoImpl, const OFFSET: isize>() -> IInjectedInputMouseInfoVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IInjectedInputMouseInfo>,
            ::windows::core::GetTrustLevel,
            MouseOptions::<Impl, OFFSET>,
            SetMouseOptions::<Impl, OFFSET>,
            MouseData::<Impl, OFFSET>,
            SetMouseData::<Impl, OFFSET>,
            DeltaY::<Impl, OFFSET>,
            SetDeltaY::<Impl, OFFSET>,
            DeltaX::<Impl, OFFSET>,
            SetDeltaX::<Impl, OFFSET>,
            TimeOffsetInMilliseconds::<Impl, OFFSET>,
            SetTimeOffsetInMilliseconds::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInjectedInputPenInfoImpl: Sized {
    fn PointerInfo(&self) -> ::windows::core::Result<InjectedInputPointerInfo>;
    fn SetPointerInfo(&self, value: &InjectedInputPointerInfo) -> ::windows::core::Result<()>;
    fn PenButtons(&self) -> ::windows::core::Result<InjectedInputPenButtons>;
    fn SetPenButtons(&self, value: InjectedInputPenButtons) -> ::windows::core::Result<()>;
    fn PenParameters(&self) -> ::windows::core::Result<InjectedInputPenParameters>;
    fn SetPenParameters(&self, value: InjectedInputPenParameters) -> ::windows::core::Result<()>;
    fn Pressure(&self) -> ::windows::core::Result<f64>;
    fn SetPressure(&self, value: f64) -> ::windows::core::Result<()>;
    fn Rotation(&self) -> ::windows::core::Result<f64>;
    fn SetRotation(&self, value: f64) -> ::windows::core::Result<()>;
    fn TiltX(&self) -> ::windows::core::Result<i32>;
    fn SetTiltX(&self, value: i32) -> ::windows::core::Result<()>;
    fn TiltY(&self) -> ::windows::core::Result<i32>;
    fn SetTiltY(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInjectedInputPenInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInjectedInputPenInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IInjectedInputPenInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInjectedInputPenInfoImpl, const OFFSET: isize>() -> IInjectedInputPenInfoVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IInjectedInputPenInfo>,
            ::windows::core::GetTrustLevel,
            PointerInfo::<Impl, OFFSET>,
            SetPointerInfo::<Impl, OFFSET>,
            PenButtons::<Impl, OFFSET>,
            SetPenButtons::<Impl, OFFSET>,
            PenParameters::<Impl, OFFSET>,
            SetPenParameters::<Impl, OFFSET>,
            Pressure::<Impl, OFFSET>,
            SetPressure::<Impl, OFFSET>,
            Rotation::<Impl, OFFSET>,
            SetRotation::<Impl, OFFSET>,
            TiltX::<Impl, OFFSET>,
            SetTiltX::<Impl, OFFSET>,
            TiltY::<Impl, OFFSET>,
            SetTiltY::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInjectedInputTouchInfoImpl: Sized {
    fn Contact(&self) -> ::windows::core::Result<InjectedInputRectangle>;
    fn SetContact(&self, value: &InjectedInputRectangle) -> ::windows::core::Result<()>;
    fn Orientation(&self) -> ::windows::core::Result<i32>;
    fn SetOrientation(&self, value: i32) -> ::windows::core::Result<()>;
    fn PointerInfo(&self) -> ::windows::core::Result<InjectedInputPointerInfo>;
    fn SetPointerInfo(&self, value: &InjectedInputPointerInfo) -> ::windows::core::Result<()>;
    fn Pressure(&self) -> ::windows::core::Result<f64>;
    fn SetPressure(&self, value: f64) -> ::windows::core::Result<()>;
    fn TouchParameters(&self) -> ::windows::core::Result<InjectedInputTouchParameters>;
    fn SetTouchParameters(&self, value: InjectedInputTouchParameters) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInjectedInputTouchInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInjectedInputTouchInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IInjectedInputTouchInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInjectedInputTouchInfoImpl, const OFFSET: isize>() -> IInjectedInputTouchInfoVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IInjectedInputTouchInfo>,
            ::windows::core::GetTrustLevel,
            Contact::<Impl, OFFSET>,
            SetContact::<Impl, OFFSET>,
            Orientation::<Impl, OFFSET>,
            SetOrientation::<Impl, OFFSET>,
            PointerInfo::<Impl, OFFSET>,
            SetPointerInfo::<Impl, OFFSET>,
            Pressure::<Impl, OFFSET>,
            SetPressure::<Impl, OFFSET>,
            TouchParameters::<Impl, OFFSET>,
            SetTouchParameters::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputInjectorImpl: Sized {
    fn InjectKeyboardInput(&self, input: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<InjectedInputKeyboardInfo>>) -> ::windows::core::Result<()>;
    fn InjectMouseInput(&self, input: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<InjectedInputMouseInfo>>) -> ::windows::core::Result<()>;
    fn InitializeTouchInjection(&self, visualmode: InjectedInputVisualizationMode) -> ::windows::core::Result<()>;
    fn InjectTouchInput(&self, input: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<InjectedInputTouchInfo>>) -> ::windows::core::Result<()>;
    fn UninitializeTouchInjection(&self) -> ::windows::core::Result<()>;
    fn InitializePenInjection(&self, visualmode: InjectedInputVisualizationMode) -> ::windows::core::Result<()>;
    fn InjectPenInput(&self, input: &::core::option::Option<InjectedInputPenInfo>) -> ::windows::core::Result<()>;
    fn UninitializePenInjection(&self) -> ::windows::core::Result<()>;
    fn InjectShortcut(&self, shortcut: InjectedInputShortcut) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputInjector {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInputInjector";
}
#[cfg(feature = "implement_exclusive")]
impl IInputInjectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputInjectorImpl, const OFFSET: isize>() -> IInputInjectorVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IInputInjector>,
            ::windows::core::GetTrustLevel,
            InjectKeyboardInput::<Impl, OFFSET>,
            InjectMouseInput::<Impl, OFFSET>,
            InitializeTouchInjection::<Impl, OFFSET>,
            InjectTouchInput::<Impl, OFFSET>,
            UninitializeTouchInjection::<Impl, OFFSET>,
            InitializePenInjection::<Impl, OFFSET>,
            InjectPenInput::<Impl, OFFSET>,
            UninitializePenInjection::<Impl, OFFSET>,
            InjectShortcut::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputInjector2Impl: Sized + IInputInjectorImpl {
    fn InitializeGamepadInjection(&self) -> ::windows::core::Result<()>;
    fn InjectGamepadInput(&self, input: &::core::option::Option<InjectedInputGamepadInfo>) -> ::windows::core::Result<()>;
    fn UninitializeGamepadInjection(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputInjector2 {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInputInjector2";
}
#[cfg(feature = "implement_exclusive")]
impl IInputInjector2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputInjector2Impl, const OFFSET: isize>() -> IInputInjector2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInputInjector2>, ::windows::core::GetTrustLevel, InitializeGamepadInjection::<Impl, OFFSET>, InjectGamepadInput::<Impl, OFFSET>, UninitializeGamepadInjection::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputInjectorStaticsImpl: Sized {
    fn TryCreate(&self) -> ::windows::core::Result<InputInjector>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputInjectorStatics {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInputInjectorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInputInjectorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputInjectorStaticsImpl, const OFFSET: isize>() -> IInputInjectorStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInputInjectorStatics>, ::windows::core::GetTrustLevel, TryCreate::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputInjectorStatics2Impl: Sized + IInputInjectorStaticsImpl {
    fn TryCreateForAppBroadcastOnly(&self) -> ::windows::core::Result<InputInjector>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputInjectorStatics2 {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.IInputInjectorStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IInputInjectorStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputInjectorStatics2Impl, const OFFSET: isize>() -> IInputInjectorStatics2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInputInjectorStatics2>, ::windows::core::GetTrustLevel, TryCreateForAppBroadcastOnly::<Impl, OFFSET>)
    }
}
