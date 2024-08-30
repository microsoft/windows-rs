pub trait IPrintCustomOptionDetails_Impl: Sized + windows_core::IUnknownImpl + IPrintOptionDetails_Impl {
    fn SetDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IPrintCustomOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintCustomOptionDetails";
}
impl IPrintCustomOptionDetails_Vtbl {
    pub const fn new<Identity: IPrintCustomOptionDetails_Impl, const OFFSET: isize>() -> IPrintCustomOptionDetails_Vtbl {
        unsafe extern "system" fn SetDisplayName<Identity: IPrintCustomOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintCustomOptionDetails_Impl::SetDisplayName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn DisplayName<Identity: IPrintCustomOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintCustomOptionDetails_Impl::DisplayName(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrintCustomOptionDetails, OFFSET>(),
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintCustomOptionDetails as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IPrintItemListOptionDetails_Impl: Sized + windows_core::IUnknownImpl + IPrintOptionDetails_Impl {
    fn Items(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<windows_core::IInspectable>>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IPrintItemListOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails";
}
#[cfg(feature = "Foundation_Collections")]
impl IPrintItemListOptionDetails_Vtbl {
    pub const fn new<Identity: IPrintItemListOptionDetails_Impl, const OFFSET: isize>() -> IPrintItemListOptionDetails_Vtbl {
        unsafe extern "system" fn Items<Identity: IPrintItemListOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintItemListOptionDetails_Impl::Items(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrintItemListOptionDetails, OFFSET>(), Items: Items::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintItemListOptionDetails as windows_core::Interface>::IID
    }
}
pub trait IPrintNumberOptionDetails_Impl: Sized + windows_core::IUnknownImpl + IPrintOptionDetails_Impl {
    fn MinValue(&self) -> windows_core::Result<u32>;
    fn MaxValue(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IPrintNumberOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintNumberOptionDetails";
}
impl IPrintNumberOptionDetails_Vtbl {
    pub const fn new<Identity: IPrintNumberOptionDetails_Impl, const OFFSET: isize>() -> IPrintNumberOptionDetails_Vtbl {
        unsafe extern "system" fn MinValue<Identity: IPrintNumberOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintNumberOptionDetails_Impl::MinValue(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxValue<Identity: IPrintNumberOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintNumberOptionDetails_Impl::MaxValue(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrintNumberOptionDetails, OFFSET>(),
            MinValue: MinValue::<Identity, OFFSET>,
            MaxValue: MaxValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintNumberOptionDetails as windows_core::Interface>::IID
    }
}
pub trait IPrintOptionDetails_Impl: Sized + windows_core::IUnknownImpl {
    fn OptionId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn OptionType(&self) -> windows_core::Result<PrintOptionType>;
    fn SetErrorText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn ErrorText(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetState(&self, value: PrintOptionStates) -> windows_core::Result<()>;
    fn State(&self) -> windows_core::Result<PrintOptionStates>;
    fn Value(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn TrySetValue(&self, value: Option<&windows_core::IInspectable>) -> windows_core::Result<bool>;
}
impl windows_core::RuntimeName for IPrintOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails";
}
impl IPrintOptionDetails_Vtbl {
    pub const fn new<Identity: IPrintOptionDetails_Impl, const OFFSET: isize>() -> IPrintOptionDetails_Vtbl {
        unsafe extern "system" fn OptionId<Identity: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintOptionDetails_Impl::OptionId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OptionType<Identity: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PrintOptionType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintOptionDetails_Impl::OptionType(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorText<Identity: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOptionDetails_Impl::SetErrorText(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ErrorText<Identity: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintOptionDetails_Impl::ErrorText(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: PrintOptionStates) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOptionDetails_Impl::SetState(this, value).into()
        }
        unsafe extern "system" fn State<Identity: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PrintOptionStates) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintOptionDetails_Impl::State(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Identity: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintOptionDetails_Impl::Value(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetValue<Identity: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintOptionDetails_Impl::TrySetValue(this, windows_core::from_raw_borrowed(&value)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrintOptionDetails, OFFSET>(),
            OptionId: OptionId::<Identity, OFFSET>,
            OptionType: OptionType::<Identity, OFFSET>,
            SetErrorText: SetErrorText::<Identity, OFFSET>,
            ErrorText: ErrorText::<Identity, OFFSET>,
            SetState: SetState::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            TrySetValue: TrySetValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintOptionDetails as windows_core::Interface>::IID
    }
}
pub trait IPrintTextOptionDetails_Impl: Sized + windows_core::IUnknownImpl + IPrintOptionDetails_Impl {
    fn MaxCharacters(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IPrintTextOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintTextOptionDetails";
}
impl IPrintTextOptionDetails_Vtbl {
    pub const fn new<Identity: IPrintTextOptionDetails_Impl, const OFFSET: isize>() -> IPrintTextOptionDetails_Vtbl {
        unsafe extern "system" fn MaxCharacters<Identity: IPrintTextOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintTextOptionDetails_Impl::MaxCharacters(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrintTextOptionDetails, OFFSET>(), MaxCharacters: MaxCharacters::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintTextOptionDetails as windows_core::Interface>::IID
    }
}
