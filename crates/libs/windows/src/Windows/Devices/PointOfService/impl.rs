pub trait ICashDrawerEventSourceEventArgs_Impl: Sized {
    fn CashDrawer(&self) -> ::windows::core::Result<CashDrawer>;
}
impl ::windows::core::RuntimeName for ICashDrawerEventSourceEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICashDrawerEventSourceEventArgs";
}
impl ICashDrawerEventSourceEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICashDrawerEventSourceEventArgs_Impl, const OFFSET: isize>() -> ICashDrawerEventSourceEventArgs_Vtbl {
        unsafe extern "system" fn CashDrawer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICashDrawerEventSourceEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CashDrawer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, ICashDrawerEventSourceEventArgs, OFFSET>(),
            CashDrawer: CashDrawer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICashDrawerEventSourceEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait ICommonClaimedPosPrinterStation_Impl: Sized {
    fn SetCharactersPerLine(&self, value: u32) -> ::windows::core::Result<()>;
    fn CharactersPerLine(&self) -> ::windows::core::Result<u32>;
    fn SetLineHeight(&self, value: u32) -> ::windows::core::Result<()>;
    fn LineHeight(&self) -> ::windows::core::Result<u32>;
    fn SetLineSpacing(&self, value: u32) -> ::windows::core::Result<()>;
    fn LineSpacing(&self) -> ::windows::core::Result<u32>;
    fn LineWidth(&self) -> ::windows::core::Result<u32>;
    fn SetIsLetterQuality(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsLetterQuality(&self) -> ::windows::core::Result<bool>;
    fn IsPaperNearEnd(&self) -> ::windows::core::Result<bool>;
    fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> ::windows::core::Result<()>;
    fn ColorCartridge(&self) -> ::windows::core::Result<PosPrinterColorCartridge>;
    fn IsCoverOpen(&self) -> ::windows::core::Result<bool>;
    fn IsCartridgeRemoved(&self) -> ::windows::core::Result<bool>;
    fn IsCartridgeEmpty(&self) -> ::windows::core::Result<bool>;
    fn IsHeadCleaning(&self) -> ::windows::core::Result<bool>;
    fn IsPaperEmpty(&self) -> ::windows::core::Result<bool>;
    fn IsReadyToPrint(&self) -> ::windows::core::Result<bool>;
    fn ValidateData(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for ICommonClaimedPosPrinterStation {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICommonClaimedPosPrinterStation";
}
impl ICommonClaimedPosPrinterStation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>() -> ICommonClaimedPosPrinterStation_Vtbl {
        unsafe extern "system" fn SetCharactersPerLine<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCharactersPerLine(value).into()
        }
        unsafe extern "system" fn CharactersPerLine<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CharactersPerLine() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineHeight<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLineHeight(value).into()
        }
        unsafe extern "system" fn LineHeight<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LineHeight() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineSpacing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLineSpacing(value).into()
        }
        unsafe extern "system" fn LineSpacing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LineSpacing() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineWidth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LineWidth() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsLetterQuality<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsLetterQuality(value).into()
        }
        unsafe extern "system" fn IsLetterQuality<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsLetterQuality() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperNearEnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPaperNearEnd() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorCartridge<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PosPrinterColorCartridge) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColorCartridge(value).into()
        }
        unsafe extern "system" fn ColorCartridge<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterColorCartridge) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ColorCartridge() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCoverOpen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCoverOpen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCartridgeRemoved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCartridgeRemoved() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCartridgeEmpty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCartridgeEmpty() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHeadCleaning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsHeadCleaning() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperEmpty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPaperEmpty() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadyToPrint<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsReadyToPrint() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ValidateData(::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, ICommonClaimedPosPrinterStation, OFFSET>(),
            SetCharactersPerLine: SetCharactersPerLine::<Identity, Impl, OFFSET>,
            CharactersPerLine: CharactersPerLine::<Identity, Impl, OFFSET>,
            SetLineHeight: SetLineHeight::<Identity, Impl, OFFSET>,
            LineHeight: LineHeight::<Identity, Impl, OFFSET>,
            SetLineSpacing: SetLineSpacing::<Identity, Impl, OFFSET>,
            LineSpacing: LineSpacing::<Identity, Impl, OFFSET>,
            LineWidth: LineWidth::<Identity, Impl, OFFSET>,
            SetIsLetterQuality: SetIsLetterQuality::<Identity, Impl, OFFSET>,
            IsLetterQuality: IsLetterQuality::<Identity, Impl, OFFSET>,
            IsPaperNearEnd: IsPaperNearEnd::<Identity, Impl, OFFSET>,
            SetColorCartridge: SetColorCartridge::<Identity, Impl, OFFSET>,
            ColorCartridge: ColorCartridge::<Identity, Impl, OFFSET>,
            IsCoverOpen: IsCoverOpen::<Identity, Impl, OFFSET>,
            IsCartridgeRemoved: IsCartridgeRemoved::<Identity, Impl, OFFSET>,
            IsCartridgeEmpty: IsCartridgeEmpty::<Identity, Impl, OFFSET>,
            IsHeadCleaning: IsHeadCleaning::<Identity, Impl, OFFSET>,
            IsPaperEmpty: IsPaperEmpty::<Identity, Impl, OFFSET>,
            IsReadyToPrint: IsReadyToPrint::<Identity, Impl, OFFSET>,
            ValidateData: ValidateData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommonClaimedPosPrinterStation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait ICommonPosPrintStationCapabilities_Impl: Sized {
    fn IsPrinterPresent(&self) -> ::windows::core::Result<bool>;
    fn IsDualColorSupported(&self) -> ::windows::core::Result<bool>;
    fn ColorCartridgeCapabilities(&self) -> ::windows::core::Result<PosPrinterColorCapabilities>;
    fn CartridgeSensors(&self) -> ::windows::core::Result<PosPrinterCartridgeSensors>;
    fn IsBoldSupported(&self) -> ::windows::core::Result<bool>;
    fn IsItalicSupported(&self) -> ::windows::core::Result<bool>;
    fn IsUnderlineSupported(&self) -> ::windows::core::Result<bool>;
    fn IsDoubleHighPrintSupported(&self) -> ::windows::core::Result<bool>;
    fn IsDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool>;
    fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool>;
    fn IsPaperEmptySensorSupported(&self) -> ::windows::core::Result<bool>;
    fn IsPaperNearEndSensorSupported(&self) -> ::windows::core::Result<bool>;
    fn SupportedCharactersPerLine(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ICommonPosPrintStationCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICommonPosPrintStationCapabilities";
}
#[cfg(feature = "Foundation_Collections")]
impl ICommonPosPrintStationCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>() -> ICommonPosPrintStationCapabilities_Vtbl {
        unsafe extern "system" fn IsPrinterPresent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPrinterPresent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDualColorSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDualColorSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorCartridgeCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterColorCapabilities) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ColorCartridgeCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CartridgeSensors<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterCartridgeSensors) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CartridgeSensors() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBoldSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsBoldSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsItalicSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsItalicSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUnderlineSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsUnderlineSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDoubleHighPrintSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDoubleHighPrintSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDoubleWidePrintSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDoubleWidePrintSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDoubleHighDoubleWidePrintSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDoubleHighDoubleWidePrintSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperEmptySensorSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPaperEmptySensorSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperNearEndSensorSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPaperNearEndSensorSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedCharactersPerLine<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportedCharactersPerLine() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, ICommonPosPrintStationCapabilities, OFFSET>(),
            IsPrinterPresent: IsPrinterPresent::<Identity, Impl, OFFSET>,
            IsDualColorSupported: IsDualColorSupported::<Identity, Impl, OFFSET>,
            ColorCartridgeCapabilities: ColorCartridgeCapabilities::<Identity, Impl, OFFSET>,
            CartridgeSensors: CartridgeSensors::<Identity, Impl, OFFSET>,
            IsBoldSupported: IsBoldSupported::<Identity, Impl, OFFSET>,
            IsItalicSupported: IsItalicSupported::<Identity, Impl, OFFSET>,
            IsUnderlineSupported: IsUnderlineSupported::<Identity, Impl, OFFSET>,
            IsDoubleHighPrintSupported: IsDoubleHighPrintSupported::<Identity, Impl, OFFSET>,
            IsDoubleWidePrintSupported: IsDoubleWidePrintSupported::<Identity, Impl, OFFSET>,
            IsDoubleHighDoubleWidePrintSupported: IsDoubleHighDoubleWidePrintSupported::<Identity, Impl, OFFSET>,
            IsPaperEmptySensorSupported: IsPaperEmptySensorSupported::<Identity, Impl, OFFSET>,
            IsPaperNearEndSensorSupported: IsPaperNearEndSensorSupported::<Identity, Impl, OFFSET>,
            SupportedCharactersPerLine: SupportedCharactersPerLine::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommonPosPrintStationCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait ICommonReceiptSlipCapabilities_Impl: Sized + ICommonPosPrintStationCapabilities_Impl {
    fn IsBarcodeSupported(&self) -> ::windows::core::Result<bool>;
    fn IsBitmapSupported(&self) -> ::windows::core::Result<bool>;
    fn IsLeft90RotationSupported(&self) -> ::windows::core::Result<bool>;
    fn IsRight90RotationSupported(&self) -> ::windows::core::Result<bool>;
    fn Is180RotationSupported(&self) -> ::windows::core::Result<bool>;
    fn IsPrintAreaSupported(&self) -> ::windows::core::Result<bool>;
    fn RuledLineCapabilities(&self) -> ::windows::core::Result<PosPrinterRuledLineCapabilities>;
    fn SupportedBarcodeRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>;
    fn SupportedBitmapRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ICommonReceiptSlipCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICommonReceiptSlipCapabilities";
}
#[cfg(feature = "Foundation_Collections")]
impl ICommonReceiptSlipCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>() -> ICommonReceiptSlipCapabilities_Vtbl {
        unsafe extern "system" fn IsBarcodeSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsBarcodeSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBitmapSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsBitmapSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLeft90RotationSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsLeft90RotationSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRight90RotationSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsRight90RotationSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Is180RotationSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Is180RotationSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPrintAreaSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPrintAreaSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RuledLineCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterRuledLineCapabilities) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RuledLineCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedBarcodeRotations<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportedBarcodeRotations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedBitmapRotations<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportedBitmapRotations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, ICommonReceiptSlipCapabilities, OFFSET>(),
            IsBarcodeSupported: IsBarcodeSupported::<Identity, Impl, OFFSET>,
            IsBitmapSupported: IsBitmapSupported::<Identity, Impl, OFFSET>,
            IsLeft90RotationSupported: IsLeft90RotationSupported::<Identity, Impl, OFFSET>,
            IsRight90RotationSupported: IsRight90RotationSupported::<Identity, Impl, OFFSET>,
            Is180RotationSupported: Is180RotationSupported::<Identity, Impl, OFFSET>,
            IsPrintAreaSupported: IsPrintAreaSupported::<Identity, Impl, OFFSET>,
            RuledLineCapabilities: RuledLineCapabilities::<Identity, Impl, OFFSET>,
            SupportedBarcodeRotations: SupportedBarcodeRotations::<Identity, Impl, OFFSET>,
            SupportedBitmapRotations: SupportedBitmapRotations::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommonReceiptSlipCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IPosPrinterJob_Impl: Sized {
    fn Print(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PrintLine(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PrintNewline(&self) -> ::windows::core::Result<()>;
    fn ExecuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IPosPrinterJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinterJob";
}
#[cfg(feature = "Foundation")]
impl IPosPrinterJob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPosPrinterJob_Impl, const OFFSET: isize>() -> IPosPrinterJob_Vtbl {
        unsafe extern "system" fn Print<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPosPrinterJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Print(::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn PrintLine<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPosPrinterJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrintLine(::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn PrintNewline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPosPrinterJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrintNewline().into()
        }
        unsafe extern "system" fn ExecuteAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPosPrinterJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExecuteAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IPosPrinterJob, OFFSET>(),
            Print: Print::<Identity, Impl, OFFSET>,
            PrintLine: PrintLine::<Identity, Impl, OFFSET>,
            PrintNewline: PrintNewline::<Identity, Impl, OFFSET>,
            ExecuteAsync: ExecuteAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPosPrinterJob as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
pub trait IReceiptOrSlipJob_Impl: Sized + IPosPrinterJob_Impl {
    fn SetBarcodeRotation(&self, value: PosPrinterRotation) -> ::windows::core::Result<()>;
    fn SetPrintRotation(&self, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::core::Result<()>;
    fn SetPrintArea(&self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn SetBitmap(&self, bitmapnumber: u32, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment) -> ::windows::core::Result<()>;
    fn SetBitmapCustomWidthStandardAlign(&self, bitmapnumber: u32, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()>;
    fn SetCustomAlignedBitmap(&self, bitmapnumber: u32, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32) -> ::windows::core::Result<()>;
    fn SetBitmapCustomWidthCustomAlign(&self, bitmapnumber: u32, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()>;
    fn PrintSavedBitmap(&self, bitmapnumber: u32) -> ::windows::core::Result<()>;
    fn DrawRuledLine(&self, positionlist: &::windows::core::HSTRING, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::core::Result<()>;
    fn PrintBarcode(&self, data: &::windows::core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::core::Result<()>;
    fn PrintBarcodeCustomAlign(&self, data: &::windows::core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::core::Result<()>;
    fn PrintBitmap(&self, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment) -> ::windows::core::Result<()>;
    fn PrintBitmapCustomWidthStandardAlign(&self, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()>;
    fn PrintCustomAlignedBitmap(&self, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32) -> ::windows::core::Result<()>;
    fn PrintBitmapCustomWidthCustomAlign(&self, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
impl ::windows::core::RuntimeName for IReceiptOrSlipJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.IReceiptOrSlipJob";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
impl IReceiptOrSlipJob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>() -> IReceiptOrSlipJob_Vtbl {
        unsafe extern "system" fn SetBarcodeRotation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PosPrinterRotation) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBarcodeRotation(value).into()
        }
        unsafe extern "system" fn SetPrintRotation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrintRotation(value, includebitmaps).into()
        }
        unsafe extern "system" fn SetPrintArea<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrintArea(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn SetBitmap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: *mut ::core::ffi::c_void, alignment: PosPrinterAlignment) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBitmap(bitmapnumber, ::core::mem::transmute(&bitmap), alignment).into()
        }
        unsafe extern "system" fn SetBitmapCustomWidthStandardAlign<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: *mut ::core::ffi::c_void, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBitmapCustomWidthStandardAlign(bitmapnumber, ::core::mem::transmute(&bitmap), alignment, width).into()
        }
        unsafe extern "system" fn SetCustomAlignedBitmap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: *mut ::core::ffi::c_void, alignmentdistance: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCustomAlignedBitmap(bitmapnumber, ::core::mem::transmute(&bitmap), alignmentdistance).into()
        }
        unsafe extern "system" fn SetBitmapCustomWidthCustomAlign<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: *mut ::core::ffi::c_void, alignmentdistance: u32, width: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBitmapCustomWidthCustomAlign(bitmapnumber, ::core::mem::transmute(&bitmap), alignmentdistance, width).into()
        }
        unsafe extern "system" fn PrintSavedBitmap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrintSavedBitmap(bitmapnumber).into()
        }
        unsafe extern "system" fn DrawRuledLine<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, positionlist: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawRuledLine(::core::mem::transmute(&positionlist), linedirection, linewidth, linestyle, linecolor).into()
        }
        unsafe extern "system" fn PrintBarcode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrintBarcode(::core::mem::transmute(&data), symbology, height, width, textposition, alignment).into()
        }
        unsafe extern "system" fn PrintBarcodeCustomAlign<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrintBarcodeCustomAlign(::core::mem::transmute(&data), symbology, height, width, textposition, alignmentdistance).into()
        }
        unsafe extern "system" fn PrintBitmap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, alignment: PosPrinterAlignment) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrintBitmap(::core::mem::transmute(&bitmap), alignment).into()
        }
        unsafe extern "system" fn PrintBitmapCustomWidthStandardAlign<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrintBitmapCustomWidthStandardAlign(::core::mem::transmute(&bitmap), alignment, width).into()
        }
        unsafe extern "system" fn PrintCustomAlignedBitmap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, alignmentdistance: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrintCustomAlignedBitmap(::core::mem::transmute(&bitmap), alignmentdistance).into()
        }
        unsafe extern "system" fn PrintBitmapCustomWidthCustomAlign<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, alignmentdistance: u32, width: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrintBitmapCustomWidthCustomAlign(::core::mem::transmute(&bitmap), alignmentdistance, width).into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IReceiptOrSlipJob, OFFSET>(),
            SetBarcodeRotation: SetBarcodeRotation::<Identity, Impl, OFFSET>,
            SetPrintRotation: SetPrintRotation::<Identity, Impl, OFFSET>,
            SetPrintArea: SetPrintArea::<Identity, Impl, OFFSET>,
            SetBitmap: SetBitmap::<Identity, Impl, OFFSET>,
            SetBitmapCustomWidthStandardAlign: SetBitmapCustomWidthStandardAlign::<Identity, Impl, OFFSET>,
            SetCustomAlignedBitmap: SetCustomAlignedBitmap::<Identity, Impl, OFFSET>,
            SetBitmapCustomWidthCustomAlign: SetBitmapCustomWidthCustomAlign::<Identity, Impl, OFFSET>,
            PrintSavedBitmap: PrintSavedBitmap::<Identity, Impl, OFFSET>,
            DrawRuledLine: DrawRuledLine::<Identity, Impl, OFFSET>,
            PrintBarcode: PrintBarcode::<Identity, Impl, OFFSET>,
            PrintBarcodeCustomAlign: PrintBarcodeCustomAlign::<Identity, Impl, OFFSET>,
            PrintBitmap: PrintBitmap::<Identity, Impl, OFFSET>,
            PrintBitmapCustomWidthStandardAlign: PrintBitmapCustomWidthStandardAlign::<Identity, Impl, OFFSET>,
            PrintCustomAlignedBitmap: PrintCustomAlignedBitmap::<Identity, Impl, OFFSET>,
            PrintBitmapCustomWidthCustomAlign: PrintBitmapCustomWidthCustomAlign::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReceiptOrSlipJob as ::windows::core::Interface>::IID
    }
}
