#[doc = "*Required features: `\"Graphics_Printing\"`, `\"implement\"`*"]
pub trait IPrintDocumentSource_Impl: Sized {}
impl ::windows_core::RuntimeName for IPrintDocumentSource {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintDocumentSource";
}
impl IPrintDocumentSource_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintDocumentSource_Impl, const OFFSET: isize>() -> IPrintDocumentSource_Vtbl {
        Self { base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IPrintDocumentSource, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPrintDocumentSource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IPrintTaskOptionsCore_Impl: Sized {
    fn GetPageDescription(&self, jobpagenumber: u32) -> ::windows_core::Result<PrintPageDescription>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for IPrintTaskOptionsCore {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskOptionsCore";
}
#[cfg(feature = "Foundation")]
impl IPrintTaskOptionsCore_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCore_Impl, const OFFSET: isize>() -> IPrintTaskOptionsCore_Vtbl {
        unsafe extern "system" fn GetPageDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobpagenumber: u32, result__: *mut PrintPageDescription) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPageDescription(jobpagenumber) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IPrintTaskOptionsCore, OFFSET>(),
            GetPageDescription: GetPageDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPrintTaskOptionsCore as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`, `\"implement\"`*"]
pub trait IPrintTaskOptionsCoreProperties_Impl: Sized {
    fn SetMediaSize(&self, value: PrintMediaSize) -> ::windows_core::Result<()>;
    fn MediaSize(&self) -> ::windows_core::Result<PrintMediaSize>;
    fn SetMediaType(&self, value: PrintMediaType) -> ::windows_core::Result<()>;
    fn MediaType(&self) -> ::windows_core::Result<PrintMediaType>;
    fn SetOrientation(&self, value: PrintOrientation) -> ::windows_core::Result<()>;
    fn Orientation(&self) -> ::windows_core::Result<PrintOrientation>;
    fn SetPrintQuality(&self, value: PrintQuality) -> ::windows_core::Result<()>;
    fn PrintQuality(&self) -> ::windows_core::Result<PrintQuality>;
    fn SetColorMode(&self, value: PrintColorMode) -> ::windows_core::Result<()>;
    fn ColorMode(&self) -> ::windows_core::Result<PrintColorMode>;
    fn SetDuplex(&self, value: PrintDuplex) -> ::windows_core::Result<()>;
    fn Duplex(&self) -> ::windows_core::Result<PrintDuplex>;
    fn SetCollation(&self, value: PrintCollation) -> ::windows_core::Result<()>;
    fn Collation(&self) -> ::windows_core::Result<PrintCollation>;
    fn SetStaple(&self, value: PrintStaple) -> ::windows_core::Result<()>;
    fn Staple(&self) -> ::windows_core::Result<PrintStaple>;
    fn SetHolePunch(&self, value: PrintHolePunch) -> ::windows_core::Result<()>;
    fn HolePunch(&self) -> ::windows_core::Result<PrintHolePunch>;
    fn SetBinding(&self, value: PrintBinding) -> ::windows_core::Result<()>;
    fn Binding(&self) -> ::windows_core::Result<PrintBinding>;
    fn MinCopies(&self) -> ::windows_core::Result<u32>;
    fn MaxCopies(&self) -> ::windows_core::Result<u32>;
    fn SetNumberOfCopies(&self, value: u32) -> ::windows_core::Result<()>;
    fn NumberOfCopies(&self) -> ::windows_core::Result<u32>;
}
impl ::windows_core::RuntimeName for IPrintTaskOptionsCoreProperties {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskOptionsCoreProperties";
}
impl IPrintTaskOptionsCoreProperties_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>() -> IPrintTaskOptionsCoreProperties_Vtbl {
        unsafe extern "system" fn SetMediaSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintMediaSize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMediaSize(value).into()
        }
        unsafe extern "system" fn MediaSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintMediaSize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MediaSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMediaType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintMediaType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMediaType(value).into()
        }
        unsafe extern "system" fn MediaType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintMediaType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MediaType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrientation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintOrientation) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOrientation(value).into()
        }
        unsafe extern "system" fn Orientation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintOrientation) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Orientation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintQuality<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintQuality) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrintQuality(value).into()
        }
        unsafe extern "system" fn PrintQuality<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintQuality) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrintQuality() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintColorMode) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColorMode(value).into()
        }
        unsafe extern "system" fn ColorMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintColorMode) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ColorMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuplex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintDuplex) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDuplex(value).into()
        }
        unsafe extern "system" fn Duplex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintDuplex) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Duplex() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCollation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintCollation) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCollation(value).into()
        }
        unsafe extern "system" fn Collation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintCollation) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Collation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStaple<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintStaple) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStaple(value).into()
        }
        unsafe extern "system" fn Staple<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintStaple) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Staple() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHolePunch<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintHolePunch) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHolePunch(value).into()
        }
        unsafe extern "system" fn HolePunch<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintHolePunch) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HolePunch() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBinding<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintBinding) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBinding(value).into()
        }
        unsafe extern "system" fn Binding<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintBinding) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Binding() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinCopies<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MinCopies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxCopies<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxCopies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumberOfCopies<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNumberOfCopies(value).into()
        }
        unsafe extern "system" fn NumberOfCopies<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NumberOfCopies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IPrintTaskOptionsCoreProperties, OFFSET>(),
            SetMediaSize: SetMediaSize::<Identity, Impl, OFFSET>,
            MediaSize: MediaSize::<Identity, Impl, OFFSET>,
            SetMediaType: SetMediaType::<Identity, Impl, OFFSET>,
            MediaType: MediaType::<Identity, Impl, OFFSET>,
            SetOrientation: SetOrientation::<Identity, Impl, OFFSET>,
            Orientation: Orientation::<Identity, Impl, OFFSET>,
            SetPrintQuality: SetPrintQuality::<Identity, Impl, OFFSET>,
            PrintQuality: PrintQuality::<Identity, Impl, OFFSET>,
            SetColorMode: SetColorMode::<Identity, Impl, OFFSET>,
            ColorMode: ColorMode::<Identity, Impl, OFFSET>,
            SetDuplex: SetDuplex::<Identity, Impl, OFFSET>,
            Duplex: Duplex::<Identity, Impl, OFFSET>,
            SetCollation: SetCollation::<Identity, Impl, OFFSET>,
            Collation: Collation::<Identity, Impl, OFFSET>,
            SetStaple: SetStaple::<Identity, Impl, OFFSET>,
            Staple: Staple::<Identity, Impl, OFFSET>,
            SetHolePunch: SetHolePunch::<Identity, Impl, OFFSET>,
            HolePunch: HolePunch::<Identity, Impl, OFFSET>,
            SetBinding: SetBinding::<Identity, Impl, OFFSET>,
            Binding: Binding::<Identity, Impl, OFFSET>,
            MinCopies: MinCopies::<Identity, Impl, OFFSET>,
            MaxCopies: MaxCopies::<Identity, Impl, OFFSET>,
            SetNumberOfCopies: SetNumberOfCopies::<Identity, Impl, OFFSET>,
            NumberOfCopies: NumberOfCopies::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPrintTaskOptionsCoreProperties as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`, `\"Foundation_Collections\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation_Collections")]
pub trait IPrintTaskOptionsCoreUIConfiguration_Impl: Sized {
    fn DisplayedOptions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for IPrintTaskOptionsCoreUIConfiguration {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskOptionsCoreUIConfiguration";
}
#[cfg(feature = "Foundation_Collections")]
impl IPrintTaskOptionsCoreUIConfiguration_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreUIConfiguration_Impl, const OFFSET: isize>() -> IPrintTaskOptionsCoreUIConfiguration_Vtbl {
        unsafe extern "system" fn DisplayedOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreUIConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayedOptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IPrintTaskOptionsCoreUIConfiguration, OFFSET>(),
            DisplayedOptions: DisplayedOptions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPrintTaskOptionsCoreUIConfiguration as ::windows_core::ComInterface>::IID
    }
}
