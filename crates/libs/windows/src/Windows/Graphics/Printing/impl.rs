#[doc = "*Required features: `\"Graphics_Printing\"`, `\"implement\"`*"]
pub trait IPrintDocumentSource_Impl: Sized {}
impl ::windows::core::RuntimeName for IPrintDocumentSource {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintDocumentSource";
}
impl IPrintDocumentSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintDocumentSource_Impl, const OFFSET: isize>() -> IPrintDocumentSource_Vtbl {
        Self { base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IPrintDocumentSource, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDocumentSource as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IPrintTaskOptionsCore_Impl: Sized {
    fn GetPageDescription(&self, jobpagenumber: u32) -> ::windows::core::Result<PrintPageDescription>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IPrintTaskOptionsCore {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskOptionsCore";
}
#[cfg(feature = "Foundation")]
impl IPrintTaskOptionsCore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCore_Impl, const OFFSET: isize>() -> IPrintTaskOptionsCore_Vtbl {
        unsafe extern "system" fn GetPageDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobpagenumber: u32, result__: *mut PrintPageDescription) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPageDescription(jobpagenumber) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IPrintTaskOptionsCore, OFFSET>(),
            GetPageDescription: GetPageDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTaskOptionsCore as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`, `\"implement\"`*"]
pub trait IPrintTaskOptionsCoreProperties_Impl: Sized {
    fn SetMediaSize(&self, value: PrintMediaSize) -> ::windows::core::Result<()>;
    fn MediaSize(&self) -> ::windows::core::Result<PrintMediaSize>;
    fn SetMediaType(&self, value: PrintMediaType) -> ::windows::core::Result<()>;
    fn MediaType(&self) -> ::windows::core::Result<PrintMediaType>;
    fn SetOrientation(&self, value: PrintOrientation) -> ::windows::core::Result<()>;
    fn Orientation(&self) -> ::windows::core::Result<PrintOrientation>;
    fn SetPrintQuality(&self, value: PrintQuality) -> ::windows::core::Result<()>;
    fn PrintQuality(&self) -> ::windows::core::Result<PrintQuality>;
    fn SetColorMode(&self, value: PrintColorMode) -> ::windows::core::Result<()>;
    fn ColorMode(&self) -> ::windows::core::Result<PrintColorMode>;
    fn SetDuplex(&self, value: PrintDuplex) -> ::windows::core::Result<()>;
    fn Duplex(&self) -> ::windows::core::Result<PrintDuplex>;
    fn SetCollation(&self, value: PrintCollation) -> ::windows::core::Result<()>;
    fn Collation(&self) -> ::windows::core::Result<PrintCollation>;
    fn SetStaple(&self, value: PrintStaple) -> ::windows::core::Result<()>;
    fn Staple(&self) -> ::windows::core::Result<PrintStaple>;
    fn SetHolePunch(&self, value: PrintHolePunch) -> ::windows::core::Result<()>;
    fn HolePunch(&self) -> ::windows::core::Result<PrintHolePunch>;
    fn SetBinding(&self, value: PrintBinding) -> ::windows::core::Result<()>;
    fn Binding(&self) -> ::windows::core::Result<PrintBinding>;
    fn MinCopies(&self) -> ::windows::core::Result<u32>;
    fn MaxCopies(&self) -> ::windows::core::Result<u32>;
    fn SetNumberOfCopies(&self, value: u32) -> ::windows::core::Result<()>;
    fn NumberOfCopies(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IPrintTaskOptionsCoreProperties {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskOptionsCoreProperties";
}
impl IPrintTaskOptionsCoreProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>() -> IPrintTaskOptionsCoreProperties_Vtbl {
        unsafe extern "system" fn SetMediaSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintMediaSize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMediaSize(value).into()
        }
        unsafe extern "system" fn MediaSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintMediaSize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MediaSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMediaType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintMediaType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMediaType(value).into()
        }
        unsafe extern "system" fn MediaType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintMediaType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MediaType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrientation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintOrientation) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOrientation(value).into()
        }
        unsafe extern "system" fn Orientation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintOrientation) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Orientation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintQuality<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintQuality) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrintQuality(value).into()
        }
        unsafe extern "system" fn PrintQuality<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintQuality) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrintQuality() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintColorMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColorMode(value).into()
        }
        unsafe extern "system" fn ColorMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintColorMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ColorMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuplex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintDuplex) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDuplex(value).into()
        }
        unsafe extern "system" fn Duplex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintDuplex) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Duplex() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCollation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintCollation) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCollation(value).into()
        }
        unsafe extern "system" fn Collation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintCollation) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Collation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStaple<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintStaple) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStaple(value).into()
        }
        unsafe extern "system" fn Staple<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintStaple) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Staple() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHolePunch<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintHolePunch) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHolePunch(value).into()
        }
        unsafe extern "system" fn HolePunch<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintHolePunch) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HolePunch() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBinding<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintBinding) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBinding(value).into()
        }
        unsafe extern "system" fn Binding<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintBinding) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Binding() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinCopies<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MinCopies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxCopies<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxCopies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumberOfCopies<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNumberOfCopies(value).into()
        }
        unsafe extern "system" fn NumberOfCopies<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NumberOfCopies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IPrintTaskOptionsCoreProperties, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTaskOptionsCoreProperties as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`, `\"Foundation_Collections\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation_Collections")]
pub trait IPrintTaskOptionsCoreUIConfiguration_Impl: Sized {
    fn DisplayedOptions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IPrintTaskOptionsCoreUIConfiguration {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskOptionsCoreUIConfiguration";
}
#[cfg(feature = "Foundation_Collections")]
impl IPrintTaskOptionsCoreUIConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreUIConfiguration_Impl, const OFFSET: isize>() -> IPrintTaskOptionsCoreUIConfiguration_Vtbl {
        unsafe extern "system" fn DisplayedOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintTaskOptionsCoreUIConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayedOptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IPrintTaskOptionsCoreUIConfiguration, OFFSET>(),
            DisplayedOptions: DisplayedOptions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTaskOptionsCoreUIConfiguration as ::windows::core::ComInterface>::IID
    }
}
