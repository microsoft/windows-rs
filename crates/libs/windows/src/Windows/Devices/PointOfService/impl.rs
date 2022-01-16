pub trait ICashDrawerEventSourceEventArgs_Impl: Sized {
    fn CashDrawer(&mut self) -> ::windows::core::Result<CashDrawer>;
}
impl ::windows::core::RuntimeName for ICashDrawerEventSourceEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICashDrawerEventSourceEventArgs";
}
impl ICashDrawerEventSourceEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICashDrawerEventSourceEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICashDrawerEventSourceEventArgs_Vtbl {
        unsafe extern "system" fn CashDrawer<Impl: ICashDrawerEventSourceEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CashDrawer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICashDrawerEventSourceEventArgs, BASE_OFFSET>(),
            CashDrawer: CashDrawer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICashDrawerEventSourceEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait ICommonClaimedPosPrinterStation_Impl: Sized {
    fn SetCharactersPerLine(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn CharactersPerLine(&mut self) -> ::windows::core::Result<u32>;
    fn SetLineHeight(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn LineHeight(&mut self) -> ::windows::core::Result<u32>;
    fn SetLineSpacing(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn LineSpacing(&mut self) -> ::windows::core::Result<u32>;
    fn LineWidth(&mut self) -> ::windows::core::Result<u32>;
    fn SetIsLetterQuality(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsLetterQuality(&mut self) -> ::windows::core::Result<bool>;
    fn IsPaperNearEnd(&mut self) -> ::windows::core::Result<bool>;
    fn SetColorCartridge(&mut self, value: PosPrinterColorCartridge) -> ::windows::core::Result<()>;
    fn ColorCartridge(&mut self) -> ::windows::core::Result<PosPrinterColorCartridge>;
    fn IsCoverOpen(&mut self) -> ::windows::core::Result<bool>;
    fn IsCartridgeRemoved(&mut self) -> ::windows::core::Result<bool>;
    fn IsCartridgeEmpty(&mut self) -> ::windows::core::Result<bool>;
    fn IsHeadCleaning(&mut self) -> ::windows::core::Result<bool>;
    fn IsPaperEmpty(&mut self) -> ::windows::core::Result<bool>;
    fn IsReadyToPrint(&mut self) -> ::windows::core::Result<bool>;
    fn ValidateData(&mut self, data: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for ICommonClaimedPosPrinterStation {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICommonClaimedPosPrinterStation";
}
impl ICommonClaimedPosPrinterStation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommonClaimedPosPrinterStation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommonClaimedPosPrinterStation_Vtbl {
        unsafe extern "system" fn SetCharactersPerLine<Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCharactersPerLine(value).into()
        }
        unsafe extern "system" fn CharactersPerLine<Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharactersPerLine() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineHeight<Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLineHeight(value).into()
        }
        unsafe extern "system" fn LineHeight<Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineSpacing<Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLineSpacing(value).into()
        }
        unsafe extern "system" fn LineSpacing<Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineSpacing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineWidth<Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsLetterQuality<Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsLetterQuality(value).into()
        }
        unsafe extern "system" fn IsLetterQuality<Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLetterQuality() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperNearEnd<Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPaperNearEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorCartridge<Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PosPrinterColorCartridge) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorCartridge(value).into()
        }
        unsafe extern "system" fn ColorCartridge<Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterColorCartridge) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorCartridge() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCoverOpen<Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCoverOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCartridgeRemoved<Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCartridgeRemoved() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCartridgeEmpty<Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCartridgeEmpty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHeadCleaning<Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHeadCleaning() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperEmpty<Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPaperEmpty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadyToPrint<Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReadyToPrint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateData<Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidateData(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommonClaimedPosPrinterStation, BASE_OFFSET>(),
            SetCharactersPerLine: SetCharactersPerLine::<Impl, IMPL_OFFSET>,
            CharactersPerLine: CharactersPerLine::<Impl, IMPL_OFFSET>,
            SetLineHeight: SetLineHeight::<Impl, IMPL_OFFSET>,
            LineHeight: LineHeight::<Impl, IMPL_OFFSET>,
            SetLineSpacing: SetLineSpacing::<Impl, IMPL_OFFSET>,
            LineSpacing: LineSpacing::<Impl, IMPL_OFFSET>,
            LineWidth: LineWidth::<Impl, IMPL_OFFSET>,
            SetIsLetterQuality: SetIsLetterQuality::<Impl, IMPL_OFFSET>,
            IsLetterQuality: IsLetterQuality::<Impl, IMPL_OFFSET>,
            IsPaperNearEnd: IsPaperNearEnd::<Impl, IMPL_OFFSET>,
            SetColorCartridge: SetColorCartridge::<Impl, IMPL_OFFSET>,
            ColorCartridge: ColorCartridge::<Impl, IMPL_OFFSET>,
            IsCoverOpen: IsCoverOpen::<Impl, IMPL_OFFSET>,
            IsCartridgeRemoved: IsCartridgeRemoved::<Impl, IMPL_OFFSET>,
            IsCartridgeEmpty: IsCartridgeEmpty::<Impl, IMPL_OFFSET>,
            IsHeadCleaning: IsHeadCleaning::<Impl, IMPL_OFFSET>,
            IsPaperEmpty: IsPaperEmpty::<Impl, IMPL_OFFSET>,
            IsReadyToPrint: IsReadyToPrint::<Impl, IMPL_OFFSET>,
            ValidateData: ValidateData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommonClaimedPosPrinterStation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait ICommonPosPrintStationCapabilities_Impl: Sized {
    fn IsPrinterPresent(&mut self) -> ::windows::core::Result<bool>;
    fn IsDualColorSupported(&mut self) -> ::windows::core::Result<bool>;
    fn ColorCartridgeCapabilities(&mut self) -> ::windows::core::Result<PosPrinterColorCapabilities>;
    fn CartridgeSensors(&mut self) -> ::windows::core::Result<PosPrinterCartridgeSensors>;
    fn IsBoldSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsItalicSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsUnderlineSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsDoubleHighPrintSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsDoubleWidePrintSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsDoubleHighDoubleWidePrintSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsPaperEmptySensorSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsPaperNearEndSensorSupported(&mut self) -> ::windows::core::Result<bool>;
    fn SupportedCharactersPerLine(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ICommonPosPrintStationCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICommonPosPrintStationCapabilities";
}
#[cfg(feature = "Foundation_Collections")]
impl ICommonPosPrintStationCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommonPosPrintStationCapabilities_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommonPosPrintStationCapabilities_Vtbl {
        unsafe extern "system" fn IsPrinterPresent<Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPrinterPresent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDualColorSupported<Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDualColorSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorCartridgeCapabilities<Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterColorCapabilities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorCartridgeCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CartridgeSensors<Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterCartridgeSensors) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CartridgeSensors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBoldSupported<Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBoldSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsItalicSupported<Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsItalicSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUnderlineSupported<Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUnderlineSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDoubleHighPrintSupported<Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDoubleHighPrintSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDoubleWidePrintSupported<Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDoubleWidePrintSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDoubleHighDoubleWidePrintSupported<Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDoubleHighDoubleWidePrintSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperEmptySensorSupported<Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPaperEmptySensorSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperNearEndSensorSupported<Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPaperNearEndSensorSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedCharactersPerLine<Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedCharactersPerLine() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommonPosPrintStationCapabilities, BASE_OFFSET>(),
            IsPrinterPresent: IsPrinterPresent::<Impl, IMPL_OFFSET>,
            IsDualColorSupported: IsDualColorSupported::<Impl, IMPL_OFFSET>,
            ColorCartridgeCapabilities: ColorCartridgeCapabilities::<Impl, IMPL_OFFSET>,
            CartridgeSensors: CartridgeSensors::<Impl, IMPL_OFFSET>,
            IsBoldSupported: IsBoldSupported::<Impl, IMPL_OFFSET>,
            IsItalicSupported: IsItalicSupported::<Impl, IMPL_OFFSET>,
            IsUnderlineSupported: IsUnderlineSupported::<Impl, IMPL_OFFSET>,
            IsDoubleHighPrintSupported: IsDoubleHighPrintSupported::<Impl, IMPL_OFFSET>,
            IsDoubleWidePrintSupported: IsDoubleWidePrintSupported::<Impl, IMPL_OFFSET>,
            IsDoubleHighDoubleWidePrintSupported: IsDoubleHighDoubleWidePrintSupported::<Impl, IMPL_OFFSET>,
            IsPaperEmptySensorSupported: IsPaperEmptySensorSupported::<Impl, IMPL_OFFSET>,
            IsPaperNearEndSensorSupported: IsPaperNearEndSensorSupported::<Impl, IMPL_OFFSET>,
            SupportedCharactersPerLine: SupportedCharactersPerLine::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommonPosPrintStationCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait ICommonReceiptSlipCapabilities_Impl: Sized + ICommonPosPrintStationCapabilities_Impl {
    fn IsBarcodeSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsBitmapSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsLeft90RotationSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsRight90RotationSupported(&mut self) -> ::windows::core::Result<bool>;
    fn Is180RotationSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsPrintAreaSupported(&mut self) -> ::windows::core::Result<bool>;
    fn RuledLineCapabilities(&mut self) -> ::windows::core::Result<PosPrinterRuledLineCapabilities>;
    fn SupportedBarcodeRotations(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>;
    fn SupportedBitmapRotations(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ICommonReceiptSlipCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICommonReceiptSlipCapabilities";
}
#[cfg(feature = "Foundation_Collections")]
impl ICommonReceiptSlipCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommonReceiptSlipCapabilities_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommonReceiptSlipCapabilities_Vtbl {
        unsafe extern "system" fn IsBarcodeSupported<Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBarcodeSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBitmapSupported<Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBitmapSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLeft90RotationSupported<Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLeft90RotationSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRight90RotationSupported<Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRight90RotationSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Is180RotationSupported<Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Is180RotationSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPrintAreaSupported<Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPrintAreaSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RuledLineCapabilities<Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterRuledLineCapabilities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RuledLineCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedBarcodeRotations<Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedBarcodeRotations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedBitmapRotations<Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedBitmapRotations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommonReceiptSlipCapabilities, BASE_OFFSET>(),
            IsBarcodeSupported: IsBarcodeSupported::<Impl, IMPL_OFFSET>,
            IsBitmapSupported: IsBitmapSupported::<Impl, IMPL_OFFSET>,
            IsLeft90RotationSupported: IsLeft90RotationSupported::<Impl, IMPL_OFFSET>,
            IsRight90RotationSupported: IsRight90RotationSupported::<Impl, IMPL_OFFSET>,
            Is180RotationSupported: Is180RotationSupported::<Impl, IMPL_OFFSET>,
            IsPrintAreaSupported: IsPrintAreaSupported::<Impl, IMPL_OFFSET>,
            RuledLineCapabilities: RuledLineCapabilities::<Impl, IMPL_OFFSET>,
            SupportedBarcodeRotations: SupportedBarcodeRotations::<Impl, IMPL_OFFSET>,
            SupportedBitmapRotations: SupportedBitmapRotations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommonReceiptSlipCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IPosPrinterJob_Impl: Sized {
    fn Print(&mut self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PrintLine(&mut self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PrintNewline(&mut self) -> ::windows::core::Result<()>;
    fn ExecuteAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IPosPrinterJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinterJob";
}
#[cfg(feature = "Foundation")]
impl IPosPrinterJob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPosPrinterJob_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPosPrinterJob_Vtbl {
        unsafe extern "system" fn Print<Impl: IPosPrinterJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Print(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PrintLine<Impl: IPosPrinterJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrintLine(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PrintNewline<Impl: IPosPrinterJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrintNewline().into()
        }
        unsafe extern "system" fn ExecuteAsync<Impl: IPosPrinterJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecuteAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPosPrinterJob, BASE_OFFSET>(),
            Print: Print::<Impl, IMPL_OFFSET>,
            PrintLine: PrintLine::<Impl, IMPL_OFFSET>,
            PrintNewline: PrintNewline::<Impl, IMPL_OFFSET>,
            ExecuteAsync: ExecuteAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPosPrinterJob as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
pub trait IReceiptOrSlipJob_Impl: Sized + IPosPrinterJob_Impl {
    fn SetBarcodeRotation(&mut self, value: PosPrinterRotation) -> ::windows::core::Result<()>;
    fn SetPrintRotation(&mut self, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::core::Result<()>;
    fn SetPrintArea(&mut self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn SetBitmap(&mut self, bitmapnumber: u32, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment) -> ::windows::core::Result<()>;
    fn SetBitmapCustomWidthStandardAlign(&mut self, bitmapnumber: u32, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()>;
    fn SetCustomAlignedBitmap(&mut self, bitmapnumber: u32, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32) -> ::windows::core::Result<()>;
    fn SetBitmapCustomWidthCustomAlign(&mut self, bitmapnumber: u32, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()>;
    fn PrintSavedBitmap(&mut self, bitmapnumber: u32) -> ::windows::core::Result<()>;
    fn DrawRuledLine(&mut self, positionlist: &::windows::core::HSTRING, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::core::Result<()>;
    fn PrintBarcode(&mut self, data: &::windows::core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::core::Result<()>;
    fn PrintBarcodeCustomAlign(&mut self, data: &::windows::core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::core::Result<()>;
    fn PrintBitmap(&mut self, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment) -> ::windows::core::Result<()>;
    fn PrintBitmapCustomWidthStandardAlign(&mut self, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()>;
    fn PrintCustomAlignedBitmap(&mut self, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32) -> ::windows::core::Result<()>;
    fn PrintBitmapCustomWidthCustomAlign(&mut self, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
impl ::windows::core::RuntimeName for IReceiptOrSlipJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.IReceiptOrSlipJob";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
impl IReceiptOrSlipJob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReceiptOrSlipJob_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReceiptOrSlipJob_Vtbl {
        unsafe extern "system" fn SetBarcodeRotation<Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PosPrinterRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBarcodeRotation(value).into()
        }
        unsafe extern "system" fn SetPrintRotation<Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrintRotation(value, includebitmaps).into()
        }
        unsafe extern "system" fn SetPrintArea<Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrintArea(&*(&value as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetBitmap<Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: ::windows::core::RawPtr, alignment: PosPrinterAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitmap(bitmapnumber, &*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignment).into()
        }
        unsafe extern "system" fn SetBitmapCustomWidthStandardAlign<Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: ::windows::core::RawPtr, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitmapCustomWidthStandardAlign(bitmapnumber, &*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignment, width).into()
        }
        unsafe extern "system" fn SetCustomAlignedBitmap<Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: ::windows::core::RawPtr, alignmentdistance: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCustomAlignedBitmap(bitmapnumber, &*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignmentdistance).into()
        }
        unsafe extern "system" fn SetBitmapCustomWidthCustomAlign<Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: ::windows::core::RawPtr, alignmentdistance: u32, width: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitmapCustomWidthCustomAlign(bitmapnumber, &*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignmentdistance, width).into()
        }
        unsafe extern "system" fn PrintSavedBitmap<Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrintSavedBitmap(bitmapnumber).into()
        }
        unsafe extern "system" fn DrawRuledLine<Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, positionlist: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawRuledLine(&*(&positionlist as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), linedirection, linewidth, linestyle, linecolor).into()
        }
        unsafe extern "system" fn PrintBarcode<Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrintBarcode(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), symbology, height, width, textposition, alignment).into()
        }
        unsafe extern "system" fn PrintBarcodeCustomAlign<Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrintBarcodeCustomAlign(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), symbology, height, width, textposition, alignmentdistance).into()
        }
        unsafe extern "system" fn PrintBitmap<Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, alignment: PosPrinterAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrintBitmap(&*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignment).into()
        }
        unsafe extern "system" fn PrintBitmapCustomWidthStandardAlign<Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrintBitmapCustomWidthStandardAlign(&*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignment, width).into()
        }
        unsafe extern "system" fn PrintCustomAlignedBitmap<Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, alignmentdistance: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrintCustomAlignedBitmap(&*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignmentdistance).into()
        }
        unsafe extern "system" fn PrintBitmapCustomWidthCustomAlign<Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, alignmentdistance: u32, width: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrintBitmapCustomWidthCustomAlign(&*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignmentdistance, width).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IReceiptOrSlipJob, BASE_OFFSET>(),
            SetBarcodeRotation: SetBarcodeRotation::<Impl, IMPL_OFFSET>,
            SetPrintRotation: SetPrintRotation::<Impl, IMPL_OFFSET>,
            SetPrintArea: SetPrintArea::<Impl, IMPL_OFFSET>,
            SetBitmap: SetBitmap::<Impl, IMPL_OFFSET>,
            SetBitmapCustomWidthStandardAlign: SetBitmapCustomWidthStandardAlign::<Impl, IMPL_OFFSET>,
            SetCustomAlignedBitmap: SetCustomAlignedBitmap::<Impl, IMPL_OFFSET>,
            SetBitmapCustomWidthCustomAlign: SetBitmapCustomWidthCustomAlign::<Impl, IMPL_OFFSET>,
            PrintSavedBitmap: PrintSavedBitmap::<Impl, IMPL_OFFSET>,
            DrawRuledLine: DrawRuledLine::<Impl, IMPL_OFFSET>,
            PrintBarcode: PrintBarcode::<Impl, IMPL_OFFSET>,
            PrintBarcodeCustomAlign: PrintBarcodeCustomAlign::<Impl, IMPL_OFFSET>,
            PrintBitmap: PrintBitmap::<Impl, IMPL_OFFSET>,
            PrintBitmapCustomWidthStandardAlign: PrintBitmapCustomWidthStandardAlign::<Impl, IMPL_OFFSET>,
            PrintCustomAlignedBitmap: PrintCustomAlignedBitmap::<Impl, IMPL_OFFSET>,
            PrintBitmapCustomWidthCustomAlign: PrintBitmapCustomWidthCustomAlign::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReceiptOrSlipJob as ::windows::core::Interface>::IID
    }
}
