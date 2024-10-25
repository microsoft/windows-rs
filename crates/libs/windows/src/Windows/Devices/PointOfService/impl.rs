pub trait ICashDrawerEventSourceEventArgs_Impl: Sized + windows_core::IUnknownImpl {
    fn CashDrawer(&self) -> windows_core::Result<CashDrawer>;
}
impl windows_core::RuntimeName for ICashDrawerEventSourceEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICashDrawerEventSourceEventArgs";
}
impl ICashDrawerEventSourceEventArgs_Vtbl {
    pub const fn new<Identity: ICashDrawerEventSourceEventArgs_Impl, const OFFSET: isize>() -> ICashDrawerEventSourceEventArgs_Vtbl {
        unsafe extern "system" fn CashDrawer<Identity: ICashDrawerEventSourceEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICashDrawerEventSourceEventArgs_Impl::CashDrawer(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ICashDrawerEventSourceEventArgs, OFFSET>(), CashDrawer: CashDrawer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICashDrawerEventSourceEventArgs as windows_core::Interface>::IID
    }
}
pub trait ICommonClaimedPosPrinterStation_Impl: Sized + windows_core::IUnknownImpl {
    fn SetCharactersPerLine(&self, value: u32) -> windows_core::Result<()>;
    fn CharactersPerLine(&self) -> windows_core::Result<u32>;
    fn SetLineHeight(&self, value: u32) -> windows_core::Result<()>;
    fn LineHeight(&self) -> windows_core::Result<u32>;
    fn SetLineSpacing(&self, value: u32) -> windows_core::Result<()>;
    fn LineSpacing(&self) -> windows_core::Result<u32>;
    fn LineWidth(&self) -> windows_core::Result<u32>;
    fn SetIsLetterQuality(&self, value: bool) -> windows_core::Result<()>;
    fn IsLetterQuality(&self) -> windows_core::Result<bool>;
    fn IsPaperNearEnd(&self) -> windows_core::Result<bool>;
    fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> windows_core::Result<()>;
    fn ColorCartridge(&self) -> windows_core::Result<PosPrinterColorCartridge>;
    fn IsCoverOpen(&self) -> windows_core::Result<bool>;
    fn IsCartridgeRemoved(&self) -> windows_core::Result<bool>;
    fn IsCartridgeEmpty(&self) -> windows_core::Result<bool>;
    fn IsHeadCleaning(&self) -> windows_core::Result<bool>;
    fn IsPaperEmpty(&self) -> windows_core::Result<bool>;
    fn IsReadyToPrint(&self) -> windows_core::Result<bool>;
    fn ValidateData(&self, data: &windows_core::HSTRING) -> windows_core::Result<bool>;
}
impl windows_core::RuntimeName for ICommonClaimedPosPrinterStation {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICommonClaimedPosPrinterStation";
}
impl ICommonClaimedPosPrinterStation_Vtbl {
    pub const fn new<Identity: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>() -> ICommonClaimedPosPrinterStation_Vtbl {
        unsafe extern "system" fn SetCharactersPerLine<Identity: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommonClaimedPosPrinterStation_Impl::SetCharactersPerLine(this, value).into()
        }
        unsafe extern "system" fn CharactersPerLine<Identity: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::CharactersPerLine(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineHeight<Identity: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommonClaimedPosPrinterStation_Impl::SetLineHeight(this, value).into()
        }
        unsafe extern "system" fn LineHeight<Identity: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::LineHeight(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineSpacing<Identity: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommonClaimedPosPrinterStation_Impl::SetLineSpacing(this, value).into()
        }
        unsafe extern "system" fn LineSpacing<Identity: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::LineSpacing(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineWidth<Identity: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::LineWidth(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsLetterQuality<Identity: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommonClaimedPosPrinterStation_Impl::SetIsLetterQuality(this, value).into()
        }
        unsafe extern "system" fn IsLetterQuality<Identity: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::IsLetterQuality(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperNearEnd<Identity: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::IsPaperNearEnd(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorCartridge<Identity: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: PosPrinterColorCartridge) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommonClaimedPosPrinterStation_Impl::SetColorCartridge(this, value).into()
        }
        unsafe extern "system" fn ColorCartridge<Identity: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PosPrinterColorCartridge) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::ColorCartridge(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCoverOpen<Identity: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::IsCoverOpen(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCartridgeRemoved<Identity: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::IsCartridgeRemoved(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCartridgeEmpty<Identity: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::IsCartridgeEmpty(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHeadCleaning<Identity: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::IsHeadCleaning(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperEmpty<Identity: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::IsPaperEmpty(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadyToPrint<Identity: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::IsReadyToPrint(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateData<Identity: ICommonClaimedPosPrinterStation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::ValidateData(this, core::mem::transmute(&data)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICommonClaimedPosPrinterStation, OFFSET>(),
            SetCharactersPerLine: SetCharactersPerLine::<Identity, OFFSET>,
            CharactersPerLine: CharactersPerLine::<Identity, OFFSET>,
            SetLineHeight: SetLineHeight::<Identity, OFFSET>,
            LineHeight: LineHeight::<Identity, OFFSET>,
            SetLineSpacing: SetLineSpacing::<Identity, OFFSET>,
            LineSpacing: LineSpacing::<Identity, OFFSET>,
            LineWidth: LineWidth::<Identity, OFFSET>,
            SetIsLetterQuality: SetIsLetterQuality::<Identity, OFFSET>,
            IsLetterQuality: IsLetterQuality::<Identity, OFFSET>,
            IsPaperNearEnd: IsPaperNearEnd::<Identity, OFFSET>,
            SetColorCartridge: SetColorCartridge::<Identity, OFFSET>,
            ColorCartridge: ColorCartridge::<Identity, OFFSET>,
            IsCoverOpen: IsCoverOpen::<Identity, OFFSET>,
            IsCartridgeRemoved: IsCartridgeRemoved::<Identity, OFFSET>,
            IsCartridgeEmpty: IsCartridgeEmpty::<Identity, OFFSET>,
            IsHeadCleaning: IsHeadCleaning::<Identity, OFFSET>,
            IsPaperEmpty: IsPaperEmpty::<Identity, OFFSET>,
            IsReadyToPrint: IsReadyToPrint::<Identity, OFFSET>,
            ValidateData: ValidateData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommonClaimedPosPrinterStation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait ICommonPosPrintStationCapabilities_Impl: Sized + windows_core::IUnknownImpl {
    fn IsPrinterPresent(&self) -> windows_core::Result<bool>;
    fn IsDualColorSupported(&self) -> windows_core::Result<bool>;
    fn ColorCartridgeCapabilities(&self) -> windows_core::Result<PosPrinterColorCapabilities>;
    fn CartridgeSensors(&self) -> windows_core::Result<PosPrinterCartridgeSensors>;
    fn IsBoldSupported(&self) -> windows_core::Result<bool>;
    fn IsItalicSupported(&self) -> windows_core::Result<bool>;
    fn IsUnderlineSupported(&self) -> windows_core::Result<bool>;
    fn IsDoubleHighPrintSupported(&self) -> windows_core::Result<bool>;
    fn IsDoubleWidePrintSupported(&self) -> windows_core::Result<bool>;
    fn IsDoubleHighDoubleWidePrintSupported(&self) -> windows_core::Result<bool>;
    fn IsPaperEmptySensorSupported(&self) -> windows_core::Result<bool>;
    fn IsPaperNearEndSensorSupported(&self) -> windows_core::Result<bool>;
    fn SupportedCharactersPerLine(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for ICommonPosPrintStationCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICommonPosPrintStationCapabilities";
}
#[cfg(feature = "Foundation_Collections")]
impl ICommonPosPrintStationCapabilities_Vtbl {
    pub const fn new<Identity: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>() -> ICommonPosPrintStationCapabilities_Vtbl {
        unsafe extern "system" fn IsPrinterPresent<Identity: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::IsPrinterPresent(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDualColorSupported<Identity: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::IsDualColorSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorCartridgeCapabilities<Identity: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PosPrinterColorCapabilities) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::ColorCartridgeCapabilities(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CartridgeSensors<Identity: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PosPrinterCartridgeSensors) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::CartridgeSensors(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBoldSupported<Identity: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::IsBoldSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsItalicSupported<Identity: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::IsItalicSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUnderlineSupported<Identity: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::IsUnderlineSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDoubleHighPrintSupported<Identity: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::IsDoubleHighPrintSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDoubleWidePrintSupported<Identity: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::IsDoubleWidePrintSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDoubleHighDoubleWidePrintSupported<Identity: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::IsDoubleHighDoubleWidePrintSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperEmptySensorSupported<Identity: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::IsPaperEmptySensorSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperNearEndSensorSupported<Identity: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::IsPaperNearEndSensorSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedCharactersPerLine<Identity: ICommonPosPrintStationCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::SupportedCharactersPerLine(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICommonPosPrintStationCapabilities, OFFSET>(),
            IsPrinterPresent: IsPrinterPresent::<Identity, OFFSET>,
            IsDualColorSupported: IsDualColorSupported::<Identity, OFFSET>,
            ColorCartridgeCapabilities: ColorCartridgeCapabilities::<Identity, OFFSET>,
            CartridgeSensors: CartridgeSensors::<Identity, OFFSET>,
            IsBoldSupported: IsBoldSupported::<Identity, OFFSET>,
            IsItalicSupported: IsItalicSupported::<Identity, OFFSET>,
            IsUnderlineSupported: IsUnderlineSupported::<Identity, OFFSET>,
            IsDoubleHighPrintSupported: IsDoubleHighPrintSupported::<Identity, OFFSET>,
            IsDoubleWidePrintSupported: IsDoubleWidePrintSupported::<Identity, OFFSET>,
            IsDoubleHighDoubleWidePrintSupported: IsDoubleHighDoubleWidePrintSupported::<Identity, OFFSET>,
            IsPaperEmptySensorSupported: IsPaperEmptySensorSupported::<Identity, OFFSET>,
            IsPaperNearEndSensorSupported: IsPaperNearEndSensorSupported::<Identity, OFFSET>,
            SupportedCharactersPerLine: SupportedCharactersPerLine::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommonPosPrintStationCapabilities as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait ICommonReceiptSlipCapabilities_Impl: Sized + windows_core::IUnknownImpl + ICommonPosPrintStationCapabilities_Impl {
    fn IsBarcodeSupported(&self) -> windows_core::Result<bool>;
    fn IsBitmapSupported(&self) -> windows_core::Result<bool>;
    fn IsLeft90RotationSupported(&self) -> windows_core::Result<bool>;
    fn IsRight90RotationSupported(&self) -> windows_core::Result<bool>;
    fn Is180RotationSupported(&self) -> windows_core::Result<bool>;
    fn IsPrintAreaSupported(&self) -> windows_core::Result<bool>;
    fn RuledLineCapabilities(&self) -> windows_core::Result<PosPrinterRuledLineCapabilities>;
    fn SupportedBarcodeRotations(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>;
    fn SupportedBitmapRotations(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for ICommonReceiptSlipCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICommonReceiptSlipCapabilities";
}
#[cfg(feature = "Foundation_Collections")]
impl ICommonReceiptSlipCapabilities_Vtbl {
    pub const fn new<Identity: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>() -> ICommonReceiptSlipCapabilities_Vtbl {
        unsafe extern "system" fn IsBarcodeSupported<Identity: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonReceiptSlipCapabilities_Impl::IsBarcodeSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBitmapSupported<Identity: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonReceiptSlipCapabilities_Impl::IsBitmapSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLeft90RotationSupported<Identity: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonReceiptSlipCapabilities_Impl::IsLeft90RotationSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRight90RotationSupported<Identity: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonReceiptSlipCapabilities_Impl::IsRight90RotationSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Is180RotationSupported<Identity: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonReceiptSlipCapabilities_Impl::Is180RotationSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPrintAreaSupported<Identity: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonReceiptSlipCapabilities_Impl::IsPrintAreaSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RuledLineCapabilities<Identity: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PosPrinterRuledLineCapabilities) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonReceiptSlipCapabilities_Impl::RuledLineCapabilities(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedBarcodeRotations<Identity: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonReceiptSlipCapabilities_Impl::SupportedBarcodeRotations(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedBitmapRotations<Identity: ICommonReceiptSlipCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonReceiptSlipCapabilities_Impl::SupportedBitmapRotations(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICommonReceiptSlipCapabilities, OFFSET>(),
            IsBarcodeSupported: IsBarcodeSupported::<Identity, OFFSET>,
            IsBitmapSupported: IsBitmapSupported::<Identity, OFFSET>,
            IsLeft90RotationSupported: IsLeft90RotationSupported::<Identity, OFFSET>,
            IsRight90RotationSupported: IsRight90RotationSupported::<Identity, OFFSET>,
            Is180RotationSupported: Is180RotationSupported::<Identity, OFFSET>,
            IsPrintAreaSupported: IsPrintAreaSupported::<Identity, OFFSET>,
            RuledLineCapabilities: RuledLineCapabilities::<Identity, OFFSET>,
            SupportedBarcodeRotations: SupportedBarcodeRotations::<Identity, OFFSET>,
            SupportedBitmapRotations: SupportedBitmapRotations::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommonReceiptSlipCapabilities as windows_core::Interface>::IID
    }
}
pub trait IPosPrinterJob_Impl: Sized + windows_core::IUnknownImpl {
    fn Print(&self, data: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn PrintLine(&self, data: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn PrintNewline(&self) -> windows_core::Result<()>;
    fn ExecuteAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
impl windows_core::RuntimeName for IPosPrinterJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinterJob";
}
impl IPosPrinterJob_Vtbl {
    pub const fn new<Identity: IPosPrinterJob_Impl, const OFFSET: isize>() -> IPosPrinterJob_Vtbl {
        unsafe extern "system" fn Print<Identity: IPosPrinterJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPosPrinterJob_Impl::Print(this, core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn PrintLine<Identity: IPosPrinterJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPosPrinterJob_Impl::PrintLine(this, core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn PrintNewline<Identity: IPosPrinterJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPosPrinterJob_Impl::PrintNewline(this).into()
        }
        unsafe extern "system" fn ExecuteAsync<Identity: IPosPrinterJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPosPrinterJob_Impl::ExecuteAsync(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPosPrinterJob, OFFSET>(),
            Print: Print::<Identity, OFFSET>,
            PrintLine: PrintLine::<Identity, OFFSET>,
            PrintNewline: PrintNewline::<Identity, OFFSET>,
            ExecuteAsync: ExecuteAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPosPrinterJob as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Graphics_Imaging")]
pub trait IReceiptOrSlipJob_Impl: Sized + windows_core::IUnknownImpl + IPosPrinterJob_Impl {
    fn SetBarcodeRotation(&self, value: PosPrinterRotation) -> windows_core::Result<()>;
    fn SetPrintRotation(&self, value: PosPrinterRotation, includebitmaps: bool) -> windows_core::Result<()>;
    fn SetPrintArea(&self, value: &super::super::Foundation::Rect) -> windows_core::Result<()>;
    fn SetBitmap(&self, bitmapnumber: u32, bitmap: Option<&super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment) -> windows_core::Result<()>;
    fn SetBitmapCustomWidthStandardAlign(&self, bitmapnumber: u32, bitmap: Option<&super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment, width: u32) -> windows_core::Result<()>;
    fn SetCustomAlignedBitmap(&self, bitmapnumber: u32, bitmap: Option<&super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32) -> windows_core::Result<()>;
    fn SetBitmapCustomWidthCustomAlign(&self, bitmapnumber: u32, bitmap: Option<&super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32, width: u32) -> windows_core::Result<()>;
    fn PrintSavedBitmap(&self, bitmapnumber: u32) -> windows_core::Result<()>;
    fn DrawRuledLine(&self, positionlist: &windows_core::HSTRING, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> windows_core::Result<()>;
    fn PrintBarcode(&self, data: &windows_core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> windows_core::Result<()>;
    fn PrintBarcodeCustomAlign(&self, data: &windows_core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> windows_core::Result<()>;
    fn PrintBitmap(&self, bitmap: Option<&super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment) -> windows_core::Result<()>;
    fn PrintBitmapCustomWidthStandardAlign(&self, bitmap: Option<&super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment, width: u32) -> windows_core::Result<()>;
    fn PrintCustomAlignedBitmap(&self, bitmap: Option<&super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32) -> windows_core::Result<()>;
    fn PrintBitmapCustomWidthCustomAlign(&self, bitmap: Option<&super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32, width: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Graphics_Imaging")]
impl windows_core::RuntimeName for IReceiptOrSlipJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.IReceiptOrSlipJob";
}
#[cfg(feature = "Graphics_Imaging")]
impl IReceiptOrSlipJob_Vtbl {
    pub const fn new<Identity: IReceiptOrSlipJob_Impl, const OFFSET: isize>() -> IReceiptOrSlipJob_Vtbl {
        unsafe extern "system" fn SetBarcodeRotation<Identity: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: PosPrinterRotation) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::SetBarcodeRotation(this, value).into()
        }
        unsafe extern "system" fn SetPrintRotation<Identity: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: PosPrinterRotation, includebitmaps: bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::SetPrintRotation(this, value, includebitmaps).into()
        }
        unsafe extern "system" fn SetPrintArea<Identity: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::Rect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::SetPrintArea(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn SetBitmap<Identity: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmapnumber: u32, bitmap: *mut core::ffi::c_void, alignment: PosPrinterAlignment) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::SetBitmap(this, bitmapnumber, windows_core::from_raw_borrowed(&bitmap), alignment).into()
        }
        unsafe extern "system" fn SetBitmapCustomWidthStandardAlign<Identity: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmapnumber: u32, bitmap: *mut core::ffi::c_void, alignment: PosPrinterAlignment, width: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::SetBitmapCustomWidthStandardAlign(this, bitmapnumber, windows_core::from_raw_borrowed(&bitmap), alignment, width).into()
        }
        unsafe extern "system" fn SetCustomAlignedBitmap<Identity: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmapnumber: u32, bitmap: *mut core::ffi::c_void, alignmentdistance: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::SetCustomAlignedBitmap(this, bitmapnumber, windows_core::from_raw_borrowed(&bitmap), alignmentdistance).into()
        }
        unsafe extern "system" fn SetBitmapCustomWidthCustomAlign<Identity: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmapnumber: u32, bitmap: *mut core::ffi::c_void, alignmentdistance: u32, width: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::SetBitmapCustomWidthCustomAlign(this, bitmapnumber, windows_core::from_raw_borrowed(&bitmap), alignmentdistance, width).into()
        }
        unsafe extern "system" fn PrintSavedBitmap<Identity: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmapnumber: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::PrintSavedBitmap(this, bitmapnumber).into()
        }
        unsafe extern "system" fn DrawRuledLine<Identity: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, positionlist: core::mem::MaybeUninit<windows_core::HSTRING>, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::DrawRuledLine(this, core::mem::transmute(&positionlist), linedirection, linewidth, linestyle, linecolor).into()
        }
        unsafe extern "system" fn PrintBarcode<Identity: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: core::mem::MaybeUninit<windows_core::HSTRING>, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::PrintBarcode(this, core::mem::transmute(&data), symbology, height, width, textposition, alignment).into()
        }
        unsafe extern "system" fn PrintBarcodeCustomAlign<Identity: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: core::mem::MaybeUninit<windows_core::HSTRING>, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::PrintBarcodeCustomAlign(this, core::mem::transmute(&data), symbology, height, width, textposition, alignmentdistance).into()
        }
        unsafe extern "system" fn PrintBitmap<Identity: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut core::ffi::c_void, alignment: PosPrinterAlignment) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::PrintBitmap(this, windows_core::from_raw_borrowed(&bitmap), alignment).into()
        }
        unsafe extern "system" fn PrintBitmapCustomWidthStandardAlign<Identity: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut core::ffi::c_void, alignment: PosPrinterAlignment, width: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::PrintBitmapCustomWidthStandardAlign(this, windows_core::from_raw_borrowed(&bitmap), alignment, width).into()
        }
        unsafe extern "system" fn PrintCustomAlignedBitmap<Identity: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut core::ffi::c_void, alignmentdistance: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::PrintCustomAlignedBitmap(this, windows_core::from_raw_borrowed(&bitmap), alignmentdistance).into()
        }
        unsafe extern "system" fn PrintBitmapCustomWidthCustomAlign<Identity: IReceiptOrSlipJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut core::ffi::c_void, alignmentdistance: u32, width: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::PrintBitmapCustomWidthCustomAlign(this, windows_core::from_raw_borrowed(&bitmap), alignmentdistance, width).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IReceiptOrSlipJob, OFFSET>(),
            SetBarcodeRotation: SetBarcodeRotation::<Identity, OFFSET>,
            SetPrintRotation: SetPrintRotation::<Identity, OFFSET>,
            SetPrintArea: SetPrintArea::<Identity, OFFSET>,
            SetBitmap: SetBitmap::<Identity, OFFSET>,
            SetBitmapCustomWidthStandardAlign: SetBitmapCustomWidthStandardAlign::<Identity, OFFSET>,
            SetCustomAlignedBitmap: SetCustomAlignedBitmap::<Identity, OFFSET>,
            SetBitmapCustomWidthCustomAlign: SetBitmapCustomWidthCustomAlign::<Identity, OFFSET>,
            PrintSavedBitmap: PrintSavedBitmap::<Identity, OFFSET>,
            DrawRuledLine: DrawRuledLine::<Identity, OFFSET>,
            PrintBarcode: PrintBarcode::<Identity, OFFSET>,
            PrintBarcodeCustomAlign: PrintBarcodeCustomAlign::<Identity, OFFSET>,
            PrintBitmap: PrintBitmap::<Identity, OFFSET>,
            PrintBitmapCustomWidthStandardAlign: PrintBitmapCustomWidthStandardAlign::<Identity, OFFSET>,
            PrintCustomAlignedBitmap: PrintCustomAlignedBitmap::<Identity, OFFSET>,
            PrintBitmapCustomWidthCustomAlign: PrintBitmapCustomWidthCustomAlign::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReceiptOrSlipJob as windows_core::Interface>::IID
    }
}
