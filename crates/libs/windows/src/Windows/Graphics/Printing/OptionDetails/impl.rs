pub trait IPrintCustomOptionDetails_Impl: Sized + IPrintOptionDetails_Impl {
    fn SetDisplayName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IPrintCustomOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintCustomOptionDetails";
}
impl IPrintCustomOptionDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCustomOptionDetails_Impl, const OFFSET: isize>() -> IPrintCustomOptionDetails_Vtbl {
        unsafe extern "system" fn SetDisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCustomOptionDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCustomOptionDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintCustomOptionDetails, OFFSET>(),
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintCustomOptionDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IPrintItemListOptionDetails_Impl: Sized + IPrintOptionDetails_Impl {
    fn Items(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IPrintItemListOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails";
}
#[cfg(feature = "Foundation_Collections")]
impl IPrintItemListOptionDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintItemListOptionDetails_Impl, const OFFSET: isize>() -> IPrintItemListOptionDetails_Vtbl {
        unsafe extern "system" fn Items<Identity: ::windows::core::IUnknownImpl, Impl: IPrintItemListOptionDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Items() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintItemListOptionDetails, OFFSET>(), Items: Items::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintItemListOptionDetails as ::windows::core::Interface>::IID
    }
}
pub trait IPrintNumberOptionDetails_Impl: Sized + IPrintOptionDetails_Impl {
    fn MinValue(&mut self) -> ::windows::core::Result<u32>;
    fn MaxValue(&mut self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IPrintNumberOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintNumberOptionDetails";
}
impl IPrintNumberOptionDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintNumberOptionDetails_Impl, const OFFSET: isize>() -> IPrintNumberOptionDetails_Vtbl {
        unsafe extern "system" fn MinValue<Identity: ::windows::core::IUnknownImpl, Impl: IPrintNumberOptionDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MinValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxValue<Identity: ::windows::core::IUnknownImpl, Impl: IPrintNumberOptionDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintNumberOptionDetails, OFFSET>(),
            MinValue: MinValue::<Identity, Impl, OFFSET>,
            MaxValue: MaxValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintNumberOptionDetails as ::windows::core::Interface>::IID
    }
}
pub trait IPrintOptionDetails_Impl: Sized {
    fn OptionId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OptionType(&mut self) -> ::windows::core::Result<PrintOptionType>;
    fn SetErrorText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ErrorText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetState(&mut self, value: PrintOptionStates) -> ::windows::core::Result<()>;
    fn State(&mut self) -> ::windows::core::Result<PrintOptionStates>;
    fn Value(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn TrySetValue(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IPrintOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails";
}
impl IPrintOptionDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOptionDetails_Impl, const OFFSET: isize>() -> IPrintOptionDetails_Vtbl {
        unsafe extern "system" fn OptionId<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OptionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OptionType<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintOptionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OptionType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorText<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetErrorText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ErrorText<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ErrorText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintOptionStates) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetState(value).into()
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintOptionStates) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetValue<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TrySetValue(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintOptionDetails, OFFSET>(),
            OptionId: OptionId::<Identity, Impl, OFFSET>,
            OptionType: OptionType::<Identity, Impl, OFFSET>,
            SetErrorText: SetErrorText::<Identity, Impl, OFFSET>,
            ErrorText: ErrorText::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            TrySetValue: TrySetValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintOptionDetails as ::windows::core::Interface>::IID
    }
}
pub trait IPrintTextOptionDetails_Impl: Sized + IPrintOptionDetails_Impl {
    fn MaxCharacters(&mut self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IPrintTextOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintTextOptionDetails";
}
impl IPrintTextOptionDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTextOptionDetails_Impl, const OFFSET: isize>() -> IPrintTextOptionDetails_Vtbl {
        unsafe extern "system" fn MaxCharacters<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTextOptionDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxCharacters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintTextOptionDetails, OFFSET>(),
            MaxCharacters: MaxCharacters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTextOptionDetails as ::windows::core::Interface>::IID
    }
}
