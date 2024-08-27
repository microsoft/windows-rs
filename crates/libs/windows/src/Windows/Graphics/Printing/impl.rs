pub trait IPrintDocumentSource_Impl: Sized + windows_core::IUnknownImpl {}
impl windows_core::RuntimeName for IPrintDocumentSource {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintDocumentSource";
}
impl IPrintDocumentSource_Vtbl {
    pub const fn new<Identity: IPrintDocumentSource_Impl, const OFFSET: isize>() -> IPrintDocumentSource_Vtbl {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrintDocumentSource, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintDocumentSource as windows_core::Interface>::IID
    }
}
pub trait IPrintTaskOptionsCore_Impl: Sized + windows_core::IUnknownImpl {
    fn GetPageDescription(&self, jobpagenumber: u32) -> windows_core::Result<PrintPageDescription>;
}
impl windows_core::RuntimeName for IPrintTaskOptionsCore {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskOptionsCore";
}
impl IPrintTaskOptionsCore_Vtbl {
    pub const fn new<Identity: IPrintTaskOptionsCore_Impl, const OFFSET: isize>() -> IPrintTaskOptionsCore_Vtbl {
        unsafe extern "system" fn GetPageDescription<Identity: IPrintTaskOptionsCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, jobpagenumber: u32, result__: *mut PrintPageDescription) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintTaskOptionsCore_Impl::GetPageDescription(this, jobpagenumber) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrintTaskOptionsCore, OFFSET>(),
            GetPageDescription: GetPageDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintTaskOptionsCore as windows_core::Interface>::IID
    }
}
pub trait IPrintTaskOptionsCoreProperties_Impl: Sized + windows_core::IUnknownImpl {
    fn SetMediaSize(&self, value: PrintMediaSize) -> windows_core::Result<()>;
    fn MediaSize(&self) -> windows_core::Result<PrintMediaSize>;
    fn SetMediaType(&self, value: PrintMediaType) -> windows_core::Result<()>;
    fn MediaType(&self) -> windows_core::Result<PrintMediaType>;
    fn SetOrientation(&self, value: PrintOrientation) -> windows_core::Result<()>;
    fn Orientation(&self) -> windows_core::Result<PrintOrientation>;
    fn SetPrintQuality(&self, value: PrintQuality) -> windows_core::Result<()>;
    fn PrintQuality(&self) -> windows_core::Result<PrintQuality>;
    fn SetColorMode(&self, value: PrintColorMode) -> windows_core::Result<()>;
    fn ColorMode(&self) -> windows_core::Result<PrintColorMode>;
    fn SetDuplex(&self, value: PrintDuplex) -> windows_core::Result<()>;
    fn Duplex(&self) -> windows_core::Result<PrintDuplex>;
    fn SetCollation(&self, value: PrintCollation) -> windows_core::Result<()>;
    fn Collation(&self) -> windows_core::Result<PrintCollation>;
    fn SetStaple(&self, value: PrintStaple) -> windows_core::Result<()>;
    fn Staple(&self) -> windows_core::Result<PrintStaple>;
    fn SetHolePunch(&self, value: PrintHolePunch) -> windows_core::Result<()>;
    fn HolePunch(&self) -> windows_core::Result<PrintHolePunch>;
    fn SetBinding(&self, value: PrintBinding) -> windows_core::Result<()>;
    fn Binding(&self) -> windows_core::Result<PrintBinding>;
    fn MinCopies(&self) -> windows_core::Result<u32>;
    fn MaxCopies(&self) -> windows_core::Result<u32>;
    fn SetNumberOfCopies(&self, value: u32) -> windows_core::Result<()>;
    fn NumberOfCopies(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IPrintTaskOptionsCoreProperties {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskOptionsCoreProperties";
}
impl IPrintTaskOptionsCoreProperties_Vtbl {
    pub const fn new<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>() -> IPrintTaskOptionsCoreProperties_Vtbl {
        unsafe extern "system" fn SetMediaSize<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: PrintMediaSize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintTaskOptionsCoreProperties_Impl::SetMediaSize(this, value).into()
        }
        unsafe extern "system" fn MediaSize<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PrintMediaSize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintTaskOptionsCoreProperties_Impl::MediaSize(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMediaType<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: PrintMediaType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintTaskOptionsCoreProperties_Impl::SetMediaType(this, value).into()
        }
        unsafe extern "system" fn MediaType<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PrintMediaType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintTaskOptionsCoreProperties_Impl::MediaType(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrientation<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: PrintOrientation) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintTaskOptionsCoreProperties_Impl::SetOrientation(this, value).into()
        }
        unsafe extern "system" fn Orientation<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PrintOrientation) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintTaskOptionsCoreProperties_Impl::Orientation(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintQuality<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: PrintQuality) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintTaskOptionsCoreProperties_Impl::SetPrintQuality(this, value).into()
        }
        unsafe extern "system" fn PrintQuality<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PrintQuality) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintTaskOptionsCoreProperties_Impl::PrintQuality(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorMode<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: PrintColorMode) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintTaskOptionsCoreProperties_Impl::SetColorMode(this, value).into()
        }
        unsafe extern "system" fn ColorMode<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PrintColorMode) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintTaskOptionsCoreProperties_Impl::ColorMode(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuplex<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: PrintDuplex) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintTaskOptionsCoreProperties_Impl::SetDuplex(this, value).into()
        }
        unsafe extern "system" fn Duplex<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PrintDuplex) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintTaskOptionsCoreProperties_Impl::Duplex(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCollation<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: PrintCollation) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintTaskOptionsCoreProperties_Impl::SetCollation(this, value).into()
        }
        unsafe extern "system" fn Collation<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PrintCollation) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintTaskOptionsCoreProperties_Impl::Collation(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStaple<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: PrintStaple) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintTaskOptionsCoreProperties_Impl::SetStaple(this, value).into()
        }
        unsafe extern "system" fn Staple<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PrintStaple) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintTaskOptionsCoreProperties_Impl::Staple(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHolePunch<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: PrintHolePunch) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintTaskOptionsCoreProperties_Impl::SetHolePunch(this, value).into()
        }
        unsafe extern "system" fn HolePunch<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PrintHolePunch) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintTaskOptionsCoreProperties_Impl::HolePunch(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBinding<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: PrintBinding) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintTaskOptionsCoreProperties_Impl::SetBinding(this, value).into()
        }
        unsafe extern "system" fn Binding<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PrintBinding) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintTaskOptionsCoreProperties_Impl::Binding(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinCopies<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintTaskOptionsCoreProperties_Impl::MinCopies(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxCopies<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintTaskOptionsCoreProperties_Impl::MaxCopies(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumberOfCopies<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintTaskOptionsCoreProperties_Impl::SetNumberOfCopies(this, value).into()
        }
        unsafe extern "system" fn NumberOfCopies<Identity: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintTaskOptionsCoreProperties_Impl::NumberOfCopies(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrintTaskOptionsCoreProperties, OFFSET>(),
            SetMediaSize: SetMediaSize::<Identity, OFFSET>,
            MediaSize: MediaSize::<Identity, OFFSET>,
            SetMediaType: SetMediaType::<Identity, OFFSET>,
            MediaType: MediaType::<Identity, OFFSET>,
            SetOrientation: SetOrientation::<Identity, OFFSET>,
            Orientation: Orientation::<Identity, OFFSET>,
            SetPrintQuality: SetPrintQuality::<Identity, OFFSET>,
            PrintQuality: PrintQuality::<Identity, OFFSET>,
            SetColorMode: SetColorMode::<Identity, OFFSET>,
            ColorMode: ColorMode::<Identity, OFFSET>,
            SetDuplex: SetDuplex::<Identity, OFFSET>,
            Duplex: Duplex::<Identity, OFFSET>,
            SetCollation: SetCollation::<Identity, OFFSET>,
            Collation: Collation::<Identity, OFFSET>,
            SetStaple: SetStaple::<Identity, OFFSET>,
            Staple: Staple::<Identity, OFFSET>,
            SetHolePunch: SetHolePunch::<Identity, OFFSET>,
            HolePunch: HolePunch::<Identity, OFFSET>,
            SetBinding: SetBinding::<Identity, OFFSET>,
            Binding: Binding::<Identity, OFFSET>,
            MinCopies: MinCopies::<Identity, OFFSET>,
            MaxCopies: MaxCopies::<Identity, OFFSET>,
            SetNumberOfCopies: SetNumberOfCopies::<Identity, OFFSET>,
            NumberOfCopies: NumberOfCopies::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintTaskOptionsCoreProperties as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IPrintTaskOptionsCoreUIConfiguration_Impl: Sized + windows_core::IUnknownImpl {
    fn DisplayedOptions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IPrintTaskOptionsCoreUIConfiguration {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskOptionsCoreUIConfiguration";
}
#[cfg(feature = "Foundation_Collections")]
impl IPrintTaskOptionsCoreUIConfiguration_Vtbl {
    pub const fn new<Identity: IPrintTaskOptionsCoreUIConfiguration_Impl, const OFFSET: isize>() -> IPrintTaskOptionsCoreUIConfiguration_Vtbl {
        unsafe extern "system" fn DisplayedOptions<Identity: IPrintTaskOptionsCoreUIConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintTaskOptionsCoreUIConfiguration_Impl::DisplayedOptions(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrintTaskOptionsCoreUIConfiguration, OFFSET>(),
            DisplayedOptions: DisplayedOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintTaskOptionsCoreUIConfiguration as windows_core::Interface>::IID
    }
}
