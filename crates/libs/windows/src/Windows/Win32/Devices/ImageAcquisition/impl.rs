#[cfg(feature = "Win32_Foundation")]
pub trait IEnumWIA_DEV_CAPS_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut WIA_DEV_CAP, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumWIA_DEV_CAPS>;
    fn GetCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IEnumWIA_DEV_CAPS {}
#[cfg(feature = "Win32_Foundation")]
impl IEnumWIA_DEV_CAPS_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWIA_DEV_CAPS_Impl, const OFFSET: isize>() -> IEnumWIA_DEV_CAPS_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWIA_DEV_CAPS_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut WIA_DEV_CAP, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWIA_DEV_CAPS_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWIA_DEV_CAPS_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWIA_DEV_CAPS_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWIA_DEV_CAPS_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcelt, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumWIA_DEV_CAPS as ::windows::core::Interface>::IID
    }
}
pub trait IEnumWIA_DEV_INFO_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IWiaPropertyStorage>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumWIA_DEV_INFO>;
    fn GetCount(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IEnumWIA_DEV_INFO {}
impl IEnumWIA_DEV_INFO_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWIA_DEV_INFO_Impl, const OFFSET: isize>() -> IEnumWIA_DEV_INFO_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWIA_DEV_INFO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWIA_DEV_INFO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWIA_DEV_INFO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWIA_DEV_INFO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWIA_DEV_INFO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(celt, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumWIA_DEV_INFO as ::windows::core::Interface>::IID
    }
}
pub trait IEnumWIA_FORMAT_INFO_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut WIA_FORMAT_INFO, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumWIA_FORMAT_INFO>;
    fn GetCount(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IEnumWIA_FORMAT_INFO {}
impl IEnumWIA_FORMAT_INFO_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: isize>() -> IEnumWIA_FORMAT_INFO_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut WIA_FORMAT_INFO, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcelt, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumWIA_FORMAT_INFO as ::windows::core::Interface>::IID
    }
}
pub trait IEnumWiaItem_Impl: Sized {
    fn Next(&self, celt: u32, ppiwiaitem: *mut ::core::option::Option<IWiaItem>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumWiaItem>;
    fn GetCount(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IEnumWiaItem {}
impl IEnumWiaItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWiaItem_Impl, const OFFSET: isize>() -> IEnumWiaItem_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWiaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppiwiaitem: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppiwiaitem), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWiaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWiaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWiaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWiaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(celt, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumWiaItem as ::windows::core::Interface>::IID
    }
}
pub trait IEnumWiaItem2_Impl: Sized {
    fn Next(&self, celt: u32, ppiwiaitem2: *mut ::core::option::Option<IWiaItem2>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumWiaItem2>;
    fn GetCount(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IEnumWiaItem2 {}
impl IEnumWiaItem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWiaItem2_Impl, const OFFSET: isize>() -> IEnumWiaItem2_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWiaItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppiwiaitem2: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppiwiaitem2), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWiaItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWiaItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWiaItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWiaItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(celt, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumWiaItem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaAppErrorHandler_Impl: Sized {
    fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND>;
    fn ReportStatus(&self, lflags: i32, pwiaitem2: &::core::option::Option<IWiaItem2>, hrstatus: ::windows::core::HRESULT, lpercentcomplete: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWiaAppErrorHandler {}
#[cfg(feature = "Win32_Foundation")]
impl IWiaAppErrorHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaAppErrorHandler_Impl, const OFFSET: isize>() -> IWiaAppErrorHandler_Vtbl {
        unsafe extern "system" fn GetWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaAppErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWindow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaAppErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiaitem2: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT, lpercentcomplete: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportStatus(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pwiaitem2), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&lpercentcomplete)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetWindow: GetWindow::<Identity, Impl, OFFSET>,
            ReportStatus: ReportStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaAppErrorHandler as ::windows::core::Interface>::IID
    }
}
pub trait IWiaDataCallback_Impl: Sized {
    fn BandedDataCallback(&self, lmessage: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, lreserved: i32, lreslength: i32, pbbuffer: *mut u8) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWiaDataCallback {}
impl IWiaDataCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDataCallback_Impl, const OFFSET: isize>() -> IWiaDataCallback_Vtbl {
        unsafe extern "system" fn BandedDataCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDataCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmessage: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, lreserved: i32, lreslength: i32, pbbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BandedDataCallback(::core::mem::transmute_copy(&lmessage), ::core::mem::transmute_copy(&lstatus), ::core::mem::transmute_copy(&lpercentcomplete), ::core::mem::transmute_copy(&loffset), ::core::mem::transmute_copy(&llength), ::core::mem::transmute_copy(&lreserved), ::core::mem::transmute_copy(&lreslength), ::core::mem::transmute_copy(&pbbuffer)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), BandedDataCallback: BandedDataCallback::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaDataCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWiaDataTransfer_Impl: Sized {
    fn idtGetData(&self, pmedium: *mut super::super::System::Com::STGMEDIUM, piwiadatacallback: &::core::option::Option<IWiaDataCallback>) -> ::windows::core::Result<()>;
    fn idtGetBandedData(&self, pwiadatatransinfo: *mut WIA_DATA_TRANSFER_INFO, piwiadatacallback: &::core::option::Option<IWiaDataCallback>) -> ::windows::core::Result<()>;
    fn idtQueryGetData(&self, pfe: *const WIA_FORMAT_INFO) -> ::windows::core::Result<()>;
    fn idtEnumWIA_FORMAT_INFO(&self) -> ::windows::core::Result<IEnumWIA_FORMAT_INFO>;
    fn idtGetExtendedTransferInfo(&self) -> ::windows::core::Result<WIA_EXTENDED_TRANSFER_INFO>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for IWiaDataTransfer {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IWiaDataTransfer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDataTransfer_Impl, const OFFSET: isize>() -> IWiaDataTransfer_Vtbl {
        unsafe extern "system" fn idtGetData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDataTransfer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmedium: *mut super::super::System::Com::STGMEDIUM, piwiadatacallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.idtGetData(::core::mem::transmute_copy(&pmedium), ::core::mem::transmute(&piwiadatacallback)).into()
        }
        unsafe extern "system" fn idtGetBandedData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDataTransfer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwiadatatransinfo: *mut WIA_DATA_TRANSFER_INFO, piwiadatacallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.idtGetBandedData(::core::mem::transmute_copy(&pwiadatatransinfo), ::core::mem::transmute(&piwiadatacallback)).into()
        }
        unsafe extern "system" fn idtQueryGetData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDataTransfer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfe: *const WIA_FORMAT_INFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.idtQueryGetData(::core::mem::transmute_copy(&pfe)).into()
        }
        unsafe extern "system" fn idtEnumWIA_FORMAT_INFO<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDataTransfer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.idtEnumWIA_FORMAT_INFO() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn idtGetExtendedTransferInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDataTransfer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextendedtransferinfo: *mut WIA_EXTENDED_TRANSFER_INFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.idtGetExtendedTransferInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pextendedtransferinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            idtGetData: idtGetData::<Identity, Impl, OFFSET>,
            idtGetBandedData: idtGetBandedData::<Identity, Impl, OFFSET>,
            idtQueryGetData: idtQueryGetData::<Identity, Impl, OFFSET>,
            idtEnumWIA_FORMAT_INFO: idtEnumWIA_FORMAT_INFO::<Identity, Impl, OFFSET>,
            idtGetExtendedTransferInfo: idtGetExtendedTransferInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaDataTransfer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaDevMgr_Impl: Sized {
    fn EnumDeviceInfo(&self, lflag: i32) -> ::windows::core::Result<IEnumWIA_DEV_INFO>;
    fn CreateDevice(&self, bstrdeviceid: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWiaItem>;
    fn SelectDeviceDlg(&self, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR, ppitemroot: *mut ::core::option::Option<IWiaItem>) -> ::windows::core::Result<()>;
    fn SelectDeviceDlgID(&self, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetImageDlg(&self, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, lintent: i32, pitemroot: &::core::option::Option<IWiaItem>, bstrfilename: &super::super::Foundation::BSTR, pguidformat: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RegisterEventCallbackProgram(&self, lflags: i32, bstrdeviceid: &super::super::Foundation::BSTR, peventguid: *const ::windows::core::GUID, bstrcommandline: &super::super::Foundation::BSTR, bstrname: &super::super::Foundation::BSTR, bstrdescription: &super::super::Foundation::BSTR, bstricon: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RegisterEventCallbackInterface(&self, lflags: i32, bstrdeviceid: &super::super::Foundation::BSTR, peventguid: *const ::windows::core::GUID, piwiaeventcallback: &::core::option::Option<IWiaEventCallback>) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn RegisterEventCallbackCLSID(&self, lflags: i32, bstrdeviceid: &super::super::Foundation::BSTR, peventguid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, bstrname: &super::super::Foundation::BSTR, bstrdescription: &super::super::Foundation::BSTR, bstricon: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddDeviceDlg(&self, hwndparent: super::super::Foundation::HWND, lflags: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWiaDevMgr {}
#[cfg(feature = "Win32_Foundation")]
impl IWiaDevMgr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDevMgr_Impl, const OFFSET: isize>() -> IWiaDevMgr_Vtbl {
        unsafe extern "system" fn EnumDeviceInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflag: i32, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumDeviceInfo(::core::mem::transmute_copy(&lflag)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwiaitemroot: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDevice(::core::mem::transmute(&bstrdeviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwiaitemroot, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectDeviceDlg<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR, ppitemroot: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SelectDeviceDlg(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&ldevicetype), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pbstrdeviceid), ::core::mem::transmute_copy(&ppitemroot)).into()
        }
        unsafe extern "system" fn SelectDeviceDlgID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SelectDeviceDlgID(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&ldevicetype), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pbstrdeviceid)).into()
        }
        unsafe extern "system" fn GetImageDlg<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, lintent: i32, pitemroot: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pguidformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetImageDlg(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&ldevicetype), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&lintent), ::core::mem::transmute(&pitemroot), ::core::mem::transmute(&bstrfilename), ::core::mem::transmute_copy(&pguidformat)).into()
        }
        unsafe extern "system" fn RegisterEventCallbackProgram<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, bstrcommandline: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstricon: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterEventCallbackProgram(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute_copy(&peventguid), ::core::mem::transmute(&bstrcommandline), ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrdescription), ::core::mem::transmute(&bstricon)).into()
        }
        unsafe extern "system" fn RegisterEventCallbackInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, piwiaeventcallback: *mut ::core::ffi::c_void, peventobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterEventCallbackInterface(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute_copy(&peventguid), ::core::mem::transmute(&piwiaeventcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peventobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterEventCallbackCLSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstricon: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterEventCallbackCLSID(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute_copy(&peventguid), ::core::mem::transmute_copy(&pclsid), ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrdescription), ::core::mem::transmute(&bstricon)).into()
        }
        unsafe extern "system" fn AddDeviceDlg<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDeviceDlg(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&lflags)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            EnumDeviceInfo: EnumDeviceInfo::<Identity, Impl, OFFSET>,
            CreateDevice: CreateDevice::<Identity, Impl, OFFSET>,
            SelectDeviceDlg: SelectDeviceDlg::<Identity, Impl, OFFSET>,
            SelectDeviceDlgID: SelectDeviceDlgID::<Identity, Impl, OFFSET>,
            GetImageDlg: GetImageDlg::<Identity, Impl, OFFSET>,
            RegisterEventCallbackProgram: RegisterEventCallbackProgram::<Identity, Impl, OFFSET>,
            RegisterEventCallbackInterface: RegisterEventCallbackInterface::<Identity, Impl, OFFSET>,
            RegisterEventCallbackCLSID: RegisterEventCallbackCLSID::<Identity, Impl, OFFSET>,
            AddDeviceDlg: AddDeviceDlg::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaDevMgr as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaDevMgr2_Impl: Sized {
    fn EnumDeviceInfo(&self, lflags: i32) -> ::windows::core::Result<IEnumWIA_DEV_INFO>;
    fn CreateDevice(&self, lflags: i32, bstrdeviceid: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWiaItem2>;
    fn SelectDeviceDlg(&self, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR, ppitemroot: *mut ::core::option::Option<IWiaItem2>) -> ::windows::core::Result<()>;
    fn SelectDeviceDlgID(&self, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RegisterEventCallbackInterface(&self, lflags: i32, bstrdeviceid: &super::super::Foundation::BSTR, peventguid: *const ::windows::core::GUID, piwiaeventcallback: &::core::option::Option<IWiaEventCallback>) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn RegisterEventCallbackProgram(&self, lflags: i32, bstrdeviceid: &super::super::Foundation::BSTR, peventguid: *const ::windows::core::GUID, bstrfullappname: &super::super::Foundation::BSTR, bstrcommandlinearg: &super::super::Foundation::BSTR, bstrname: &super::super::Foundation::BSTR, bstrdescription: &super::super::Foundation::BSTR, bstricon: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RegisterEventCallbackCLSID(&self, lflags: i32, bstrdeviceid: &super::super::Foundation::BSTR, peventguid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, bstrname: &super::super::Foundation::BSTR, bstrdescription: &super::super::Foundation::BSTR, bstricon: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetImageDlg(&self, lflags: i32, bstrdeviceid: &super::super::Foundation::BSTR, hwndparent: super::super::Foundation::HWND, bstrfoldername: &super::super::Foundation::BSTR, bstrfilename: &super::super::Foundation::BSTR, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut super::super::Foundation::BSTR, ppitem: *mut ::core::option::Option<IWiaItem2>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWiaDevMgr2 {}
#[cfg(feature = "Win32_Foundation")]
impl IWiaDevMgr2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDevMgr2_Impl, const OFFSET: isize>() -> IWiaDevMgr2_Vtbl {
        unsafe extern "system" fn EnumDeviceInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumDeviceInfo(::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwiaitem2root: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDevice(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrdeviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwiaitem2root, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectDeviceDlg<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR, ppitemroot: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SelectDeviceDlg(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&ldevicetype), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pbstrdeviceid), ::core::mem::transmute_copy(&ppitemroot)).into()
        }
        unsafe extern "system" fn SelectDeviceDlgID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SelectDeviceDlgID(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&ldevicetype), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pbstrdeviceid)).into()
        }
        unsafe extern "system" fn RegisterEventCallbackInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, piwiaeventcallback: *mut ::core::ffi::c_void, peventobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterEventCallbackInterface(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute_copy(&peventguid), ::core::mem::transmute(&piwiaeventcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peventobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterEventCallbackProgram<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, bstrfullappname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrcommandlinearg: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstricon: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterEventCallbackProgram(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute_copy(&peventguid), ::core::mem::transmute(&bstrfullappname), ::core::mem::transmute(&bstrcommandlinearg), ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrdescription), ::core::mem::transmute(&bstricon)).into()
        }
        unsafe extern "system" fn RegisterEventCallbackCLSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstricon: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterEventCallbackCLSID(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute_copy(&peventguid), ::core::mem::transmute_copy(&pclsid), ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrdescription), ::core::mem::transmute(&bstricon)).into()
        }
        unsafe extern "system" fn GetImageDlg<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hwndparent: super::super::Foundation::HWND, bstrfoldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut super::super::Foundation::BSTR, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetImageDlg(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&bstrfoldername), ::core::mem::transmute(&bstrfilename), ::core::mem::transmute_copy(&plnumfiles), ::core::mem::transmute_copy(&ppbstrfilepaths), ::core::mem::transmute_copy(&ppitem)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            EnumDeviceInfo: EnumDeviceInfo::<Identity, Impl, OFFSET>,
            CreateDevice: CreateDevice::<Identity, Impl, OFFSET>,
            SelectDeviceDlg: SelectDeviceDlg::<Identity, Impl, OFFSET>,
            SelectDeviceDlgID: SelectDeviceDlgID::<Identity, Impl, OFFSET>,
            RegisterEventCallbackInterface: RegisterEventCallbackInterface::<Identity, Impl, OFFSET>,
            RegisterEventCallbackProgram: RegisterEventCallbackProgram::<Identity, Impl, OFFSET>,
            RegisterEventCallbackCLSID: RegisterEventCallbackCLSID::<Identity, Impl, OFFSET>,
            GetImageDlg: GetImageDlg::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaDevMgr2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaDrvItem_Impl: Sized {
    fn GetItemFlags(&self) -> ::windows::core::Result<i32>;
    fn GetDeviceSpecContext(&self) -> ::windows::core::Result<*mut u8>;
    fn GetFullItemName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetItemName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AddItemToFolder(&self, __midl__iwiadrvitem0004: &::core::option::Option<IWiaDrvItem>) -> ::windows::core::Result<()>;
    fn UnlinkItemTree(&self, __midl__iwiadrvitem0005: i32) -> ::windows::core::Result<()>;
    fn RemoveItemFromFolder(&self, __midl__iwiadrvitem0006: i32) -> ::windows::core::Result<()>;
    fn FindItemByName(&self, __midl__iwiadrvitem0007: i32, __midl__iwiadrvitem0008: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWiaDrvItem>;
    fn FindChildItemByName(&self, __midl__iwiadrvitem0010: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWiaDrvItem>;
    fn GetParentItem(&self) -> ::windows::core::Result<IWiaDrvItem>;
    fn GetFirstChildItem(&self) -> ::windows::core::Result<IWiaDrvItem>;
    fn GetNextSiblingItem(&self) -> ::windows::core::Result<IWiaDrvItem>;
    fn DumpItemData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWiaDrvItem {}
#[cfg(feature = "Win32_Foundation")]
impl IWiaDrvItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: isize>() -> IWiaDrvItem_Vtbl {
        unsafe extern "system" fn GetItemFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0000: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiadrvitem0000, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSpecContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0001: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceSpecContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiadrvitem0001, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFullItemName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0002: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFullItemName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiadrvitem0002, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0003: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiadrvitem0003, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItemToFolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0004: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddItemToFolder(::core::mem::transmute(&__midl__iwiadrvitem0004)).into()
        }
        unsafe extern "system" fn UnlinkItemTree<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0005: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnlinkItemTree(::core::mem::transmute_copy(&__midl__iwiadrvitem0005)).into()
        }
        unsafe extern "system" fn RemoveItemFromFolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0006: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveItemFromFolder(::core::mem::transmute_copy(&__midl__iwiadrvitem0006)).into()
        }
        unsafe extern "system" fn FindItemByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0007: i32, __midl__iwiadrvitem0008: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, __midl__iwiadrvitem0009: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindItemByName(::core::mem::transmute_copy(&__midl__iwiadrvitem0007), ::core::mem::transmute(&__midl__iwiadrvitem0008)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiadrvitem0009, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindChildItemByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0010: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, __midl__iwiadrvitem0011: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindChildItemByName(::core::mem::transmute(&__midl__iwiadrvitem0010)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiadrvitem0011, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0012: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParentItem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiadrvitem0012, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstChildItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0013: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFirstChildItem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiadrvitem0013, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextSiblingItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0014: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNextSiblingItem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiadrvitem0014, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DumpItemData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0015: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DumpItemData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiadrvitem0015, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetItemFlags: GetItemFlags::<Identity, Impl, OFFSET>,
            GetDeviceSpecContext: GetDeviceSpecContext::<Identity, Impl, OFFSET>,
            GetFullItemName: GetFullItemName::<Identity, Impl, OFFSET>,
            GetItemName: GetItemName::<Identity, Impl, OFFSET>,
            AddItemToFolder: AddItemToFolder::<Identity, Impl, OFFSET>,
            UnlinkItemTree: UnlinkItemTree::<Identity, Impl, OFFSET>,
            RemoveItemFromFolder: RemoveItemFromFolder::<Identity, Impl, OFFSET>,
            FindItemByName: FindItemByName::<Identity, Impl, OFFSET>,
            FindChildItemByName: FindChildItemByName::<Identity, Impl, OFFSET>,
            GetParentItem: GetParentItem::<Identity, Impl, OFFSET>,
            GetFirstChildItem: GetFirstChildItem::<Identity, Impl, OFFSET>,
            GetNextSiblingItem: GetNextSiblingItem::<Identity, Impl, OFFSET>,
            DumpItemData: DumpItemData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaDrvItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaErrorHandler_Impl: Sized {
    fn ReportStatus(&self, lflags: i32, hwndparent: super::super::Foundation::HWND, pwiaitem2: &::core::option::Option<IWiaItem2>, hrstatus: ::windows::core::HRESULT, lpercentcomplete: i32) -> ::windows::core::Result<()>;
    fn GetStatusDescription(&self, lflags: i32, pwiaitem2: &::core::option::Option<IWiaItem2>, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWiaErrorHandler {}
#[cfg(feature = "Win32_Foundation")]
impl IWiaErrorHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaErrorHandler_Impl, const OFFSET: isize>() -> IWiaErrorHandler_Vtbl {
        unsafe extern "system" fn ReportStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, hwndparent: super::super::Foundation::HWND, pwiaitem2: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT, lpercentcomplete: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportStatus(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&pwiaitem2), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&lpercentcomplete)).into()
        }
        unsafe extern "system" fn GetStatusDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiaitem2: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatusDescription(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pwiaitem2), ::core::mem::transmute_copy(&hrstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ReportStatus: ReportStatus::<Identity, Impl, OFFSET>,
            GetStatusDescription: GetStatusDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaErrorHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaEventCallback_Impl: Sized {
    fn ImageEventCallback(&self, peventguid: *const ::windows::core::GUID, bstreventdescription: &super::super::Foundation::BSTR, bstrdeviceid: &super::super::Foundation::BSTR, bstrdevicedescription: &super::super::Foundation::BSTR, dwdevicetype: u32, bstrfullitemname: &super::super::Foundation::BSTR, puleventtype: *mut u32, ulreserved: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWiaEventCallback {}
#[cfg(feature = "Win32_Foundation")]
impl IWiaEventCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaEventCallback_Impl, const OFFSET: isize>() -> IWiaEventCallback_Vtbl {
        unsafe extern "system" fn ImageEventCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaEventCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventguid: *const ::windows::core::GUID, bstreventdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdevicedescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwdevicetype: u32, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, puleventtype: *mut u32, ulreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ImageEventCallback(::core::mem::transmute_copy(&peventguid), ::core::mem::transmute(&bstreventdescription), ::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute(&bstrdevicedescription), ::core::mem::transmute_copy(&dwdevicetype), ::core::mem::transmute(&bstrfullitemname), ::core::mem::transmute_copy(&puleventtype), ::core::mem::transmute_copy(&ulreserved)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ImageEventCallback: ImageEventCallback::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaEventCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWiaImageFilter_Impl: Sized {
    fn InitializeFilter(&self, pwiaitem2: &::core::option::Option<IWiaItem2>, pwiatransfercallback: &::core::option::Option<IWiaTransferCallback>) -> ::windows::core::Result<()>;
    fn SetNewCallback(&self, pwiatransfercallback: &::core::option::Option<IWiaTransferCallback>) -> ::windows::core::Result<()>;
    fn FilterPreviewImage(&self, lflags: i32, pwiachilditem2: &::core::option::Option<IWiaItem2>, inputimageextents: &super::super::Foundation::RECT, pinputstream: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn ApplyProperties(&self, pwiapropertystorage: &::core::option::Option<IWiaPropertyStorage>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IWiaImageFilter {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWiaImageFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaImageFilter_Impl, const OFFSET: isize>() -> IWiaImageFilter_Vtbl {
        unsafe extern "system" fn InitializeFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaImageFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwiaitem2: *mut ::core::ffi::c_void, pwiatransfercallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeFilter(::core::mem::transmute(&pwiaitem2), ::core::mem::transmute(&pwiatransfercallback)).into()
        }
        unsafe extern "system" fn SetNewCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaImageFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwiatransfercallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNewCallback(::core::mem::transmute(&pwiatransfercallback)).into()
        }
        unsafe extern "system" fn FilterPreviewImage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaImageFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiachilditem2: *mut ::core::ffi::c_void, inputimageextents: super::super::Foundation::RECT, pinputstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FilterPreviewImage(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pwiachilditem2), ::core::mem::transmute(&inputimageextents), ::core::mem::transmute(&pinputstream)).into()
        }
        unsafe extern "system" fn ApplyProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaImageFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwiapropertystorage: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ApplyProperties(::core::mem::transmute(&pwiapropertystorage)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            InitializeFilter: InitializeFilter::<Identity, Impl, OFFSET>,
            SetNewCallback: SetNewCallback::<Identity, Impl, OFFSET>,
            FilterPreviewImage: FilterPreviewImage::<Identity, Impl, OFFSET>,
            ApplyProperties: ApplyProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaImageFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaItem_Impl: Sized {
    fn GetItemType(&self) -> ::windows::core::Result<i32>;
    fn AnalyzeItem(&self, lflags: i32) -> ::windows::core::Result<()>;
    fn EnumChildItems(&self) -> ::windows::core::Result<IEnumWiaItem>;
    fn DeleteItem(&self, lflags: i32) -> ::windows::core::Result<()>;
    fn CreateChildItem(&self, lflags: i32, bstritemname: &super::super::Foundation::BSTR, bstrfullitemname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWiaItem>;
    fn EnumRegisterEventInfo(&self, lflags: i32, peventguid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumWIA_DEV_CAPS>;
    fn FindItemByName(&self, lflags: i32, bstrfullitemname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWiaItem>;
    fn DeviceDlg(&self, hwndparent: super::super::Foundation::HWND, lflags: i32, lintent: i32, plitemcount: *mut i32, ppiwiaitem: *mut *mut ::core::option::Option<IWiaItem>) -> ::windows::core::Result<()>;
    fn DeviceCommand(&self, lflags: i32, pcmdguid: *const ::windows::core::GUID, piwiaitem: *mut ::core::option::Option<IWiaItem>) -> ::windows::core::Result<()>;
    fn GetRootItem(&self) -> ::windows::core::Result<IWiaItem>;
    fn EnumDeviceCapabilities(&self, lflags: i32) -> ::windows::core::Result<IEnumWIA_DEV_CAPS>;
    fn DumpItemData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DumpDrvItemData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DumpTreeItemData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Diagnostic(&self, ulsize: u32, pbuffer: *const u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWiaItem {}
#[cfg(feature = "Win32_Foundation")]
impl IWiaItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: isize>() -> IWiaItem_Vtbl {
        unsafe extern "system" fn GetItemType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitemtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnalyzeItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AnalyzeItem(::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn EnumChildItems<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienumwiaitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumChildItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienumwiaitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteItem(::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn CreateChildItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppiwiaitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateChildItem(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstritemname), ::core::mem::transmute(&bstrfullitemname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwiaitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRegisterEventInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, peventguid: *const ::windows::core::GUID, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumRegisterEventInfo(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&peventguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindItemByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppiwiaitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindItemByName(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrfullitemname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwiaitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceDlg<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, lflags: i32, lintent: i32, plitemcount: *mut i32, ppiwiaitem: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceDlg(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&lintent), ::core::mem::transmute_copy(&plitemcount), ::core::mem::transmute_copy(&ppiwiaitem)).into()
        }
        unsafe extern "system" fn DeviceCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pcmdguid: *const ::windows::core::GUID, piwiaitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceCommand(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pcmdguid), ::core::mem::transmute_copy(&piwiaitem)).into()
        }
        unsafe extern "system" fn GetRootItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwiaitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRootItem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwiaitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDeviceCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, ppienumwia_dev_caps: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumDeviceCapabilities(::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienumwia_dev_caps, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DumpItemData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DumpItemData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DumpDrvItemData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DumpDrvItemData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DumpTreeItemData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DumpTreeItemData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Diagnostic<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsize: u32, pbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Diagnostic(::core::mem::transmute_copy(&ulsize), ::core::mem::transmute_copy(&pbuffer)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetItemType: GetItemType::<Identity, Impl, OFFSET>,
            AnalyzeItem: AnalyzeItem::<Identity, Impl, OFFSET>,
            EnumChildItems: EnumChildItems::<Identity, Impl, OFFSET>,
            DeleteItem: DeleteItem::<Identity, Impl, OFFSET>,
            CreateChildItem: CreateChildItem::<Identity, Impl, OFFSET>,
            EnumRegisterEventInfo: EnumRegisterEventInfo::<Identity, Impl, OFFSET>,
            FindItemByName: FindItemByName::<Identity, Impl, OFFSET>,
            DeviceDlg: DeviceDlg::<Identity, Impl, OFFSET>,
            DeviceCommand: DeviceCommand::<Identity, Impl, OFFSET>,
            GetRootItem: GetRootItem::<Identity, Impl, OFFSET>,
            EnumDeviceCapabilities: EnumDeviceCapabilities::<Identity, Impl, OFFSET>,
            DumpItemData: DumpItemData::<Identity, Impl, OFFSET>,
            DumpDrvItemData: DumpDrvItemData::<Identity, Impl, OFFSET>,
            DumpTreeItemData: DumpTreeItemData::<Identity, Impl, OFFSET>,
            Diagnostic: Diagnostic::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaItem2_Impl: Sized {
    fn CreateChildItem(&self, litemflags: i32, lcreationflags: i32, bstritemname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWiaItem2>;
    fn DeleteItem(&self, lflags: i32) -> ::windows::core::Result<()>;
    fn EnumChildItems(&self, pcategoryguid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumWiaItem2>;
    fn FindItemByName(&self, lflags: i32, bstrfullitemname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWiaItem2>;
    fn GetItemCategory(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetItemType(&self) -> ::windows::core::Result<i32>;
    fn DeviceDlg(&self, lflags: i32, hwndparent: super::super::Foundation::HWND, bstrfoldername: &super::super::Foundation::BSTR, bstrfilename: &super::super::Foundation::BSTR, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut super::super::Foundation::BSTR, ppitem: *mut ::core::option::Option<IWiaItem2>) -> ::windows::core::Result<()>;
    fn DeviceCommand(&self, lflags: i32, pcmdguid: *const ::windows::core::GUID, ppiwiaitem2: *mut ::core::option::Option<IWiaItem2>) -> ::windows::core::Result<()>;
    fn EnumDeviceCapabilities(&self, lflags: i32) -> ::windows::core::Result<IEnumWIA_DEV_CAPS>;
    fn CheckExtension(&self, lflags: i32, bstrname: &super::super::Foundation::BSTR, riidextensioninterface: *const ::windows::core::GUID, pbextensionexists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetExtension(&self, lflags: i32, bstrname: &super::super::Foundation::BSTR, riidextensioninterface: *const ::windows::core::GUID, ppout: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetParentItem(&self) -> ::windows::core::Result<IWiaItem2>;
    fn GetRootItem(&self) -> ::windows::core::Result<IWiaItem2>;
    fn GetPreviewComponent(&self, lflags: i32) -> ::windows::core::Result<IWiaPreview>;
    fn EnumRegisterEventInfo(&self, lflags: i32, peventguid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumWIA_DEV_CAPS>;
    fn Diagnostic(&self, ulsize: u32, pbuffer: *const u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWiaItem2 {}
#[cfg(feature = "Win32_Foundation")]
impl IWiaItem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: isize>() -> IWiaItem2_Vtbl {
        unsafe extern "system" fn CreateChildItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, litemflags: i32, lcreationflags: i32, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppiwiaitem2: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateChildItem(::core::mem::transmute_copy(&litemflags), ::core::mem::transmute_copy(&lcreationflags), ::core::mem::transmute(&bstritemname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwiaitem2, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteItem(::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn EnumChildItems<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcategoryguid: *const ::windows::core::GUID, ppienumwiaitem2: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumChildItems(::core::mem::transmute_copy(&pcategoryguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienumwiaitem2, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindItemByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppiwiaitem2: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindItemByName(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrfullitemname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwiaitem2, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemCategory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemcategoryguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemCategory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitemcategoryguid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitemtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceDlg<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, hwndparent: super::super::Foundation::HWND, bstrfoldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut super::super::Foundation::BSTR, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceDlg(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&bstrfoldername), ::core::mem::transmute(&bstrfilename), ::core::mem::transmute_copy(&plnumfiles), ::core::mem::transmute_copy(&ppbstrfilepaths), ::core::mem::transmute_copy(&ppitem)).into()
        }
        unsafe extern "system" fn DeviceCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pcmdguid: *const ::windows::core::GUID, ppiwiaitem2: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceCommand(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pcmdguid), ::core::mem::transmute_copy(&ppiwiaitem2)).into()
        }
        unsafe extern "system" fn EnumDeviceCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, ppienumwia_dev_caps: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumDeviceCapabilities(::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienumwia_dev_caps, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckExtension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, riidextensioninterface: *const ::windows::core::GUID, pbextensionexists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CheckExtension(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&riidextensioninterface), ::core::mem::transmute_copy(&pbextensionexists)).into()
        }
        unsafe extern "system" fn GetExtension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, riidextensioninterface: *const ::windows::core::GUID, ppout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetExtension(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&riidextensioninterface), ::core::mem::transmute_copy(&ppout)).into()
        }
        unsafe extern "system" fn GetParentItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwiaitem2: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParentItem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwiaitem2, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRootItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwiaitem2: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRootItem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwiaitem2, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviewComponent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, ppwiapreview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPreviewComponent(::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwiapreview, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRegisterEventInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, peventguid: *const ::windows::core::GUID, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumRegisterEventInfo(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&peventguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Diagnostic<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsize: u32, pbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Diagnostic(::core::mem::transmute_copy(&ulsize), ::core::mem::transmute_copy(&pbuffer)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateChildItem: CreateChildItem::<Identity, Impl, OFFSET>,
            DeleteItem: DeleteItem::<Identity, Impl, OFFSET>,
            EnumChildItems: EnumChildItems::<Identity, Impl, OFFSET>,
            FindItemByName: FindItemByName::<Identity, Impl, OFFSET>,
            GetItemCategory: GetItemCategory::<Identity, Impl, OFFSET>,
            GetItemType: GetItemType::<Identity, Impl, OFFSET>,
            DeviceDlg: DeviceDlg::<Identity, Impl, OFFSET>,
            DeviceCommand: DeviceCommand::<Identity, Impl, OFFSET>,
            EnumDeviceCapabilities: EnumDeviceCapabilities::<Identity, Impl, OFFSET>,
            CheckExtension: CheckExtension::<Identity, Impl, OFFSET>,
            GetExtension: GetExtension::<Identity, Impl, OFFSET>,
            GetParentItem: GetParentItem::<Identity, Impl, OFFSET>,
            GetRootItem: GetRootItem::<Identity, Impl, OFFSET>,
            GetPreviewComponent: GetPreviewComponent::<Identity, Impl, OFFSET>,
            EnumRegisterEventInfo: EnumRegisterEventInfo::<Identity, Impl, OFFSET>,
            Diagnostic: Diagnostic::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaItem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaItemExtras_Impl: Sized {
    fn GetExtendedErrorInfo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Escape(&self, dwescapecode: u32, lpindata: *const u8, cbindatasize: u32, poutdata: *mut u8, dwoutdatasize: u32, pdwactualdatasize: *mut u32) -> ::windows::core::Result<()>;
    fn CancelPendingIO(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWiaItemExtras {}
#[cfg(feature = "Win32_Foundation")]
impl IWiaItemExtras_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItemExtras_Impl, const OFFSET: isize>() -> IWiaItemExtras_Vtbl {
        unsafe extern "system" fn GetExtendedErrorInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItemExtras_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrerrortext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetExtendedErrorInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrerrortext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Escape<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItemExtras_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwescapecode: u32, lpindata: *const u8, cbindatasize: u32, poutdata: *mut u8, dwoutdatasize: u32, pdwactualdatasize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Escape(::core::mem::transmute_copy(&dwescapecode), ::core::mem::transmute_copy(&lpindata), ::core::mem::transmute_copy(&cbindatasize), ::core::mem::transmute_copy(&poutdata), ::core::mem::transmute_copy(&dwoutdatasize), ::core::mem::transmute_copy(&pdwactualdatasize)).into()
        }
        unsafe extern "system" fn CancelPendingIO<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaItemExtras_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelPendingIO().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetExtendedErrorInfo: GetExtendedErrorInfo::<Identity, Impl, OFFSET>,
            Escape: Escape::<Identity, Impl, OFFSET>,
            CancelPendingIO: CancelPendingIO::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaItemExtras as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaLog_Impl: Sized {
    fn InitializeLog(&self, hinstance: i32) -> ::windows::core::Result<()>;
    fn hResult(&self, hresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn Log(&self, lflags: i32, lresid: i32, ldetail: i32, bstrtext: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWiaLog {}
#[cfg(feature = "Win32_Foundation")]
impl IWiaLog_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaLog_Impl, const OFFSET: isize>() -> IWiaLog_Vtbl {
        unsafe extern "system" fn InitializeLog<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaLog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hinstance: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeLog(::core::mem::transmute_copy(&hinstance)).into()
        }
        unsafe extern "system" fn hResult<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaLog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.hResult(::core::mem::transmute_copy(&hresult)).into()
        }
        unsafe extern "system" fn Log<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaLog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, lresid: i32, ldetail: i32, bstrtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Log(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&lresid), ::core::mem::transmute_copy(&ldetail), ::core::mem::transmute(&bstrtext)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            InitializeLog: InitializeLog::<Identity, Impl, OFFSET>,
            hResult: hResult::<Identity, Impl, OFFSET>,
            Log: Log::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaLog as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaLogEx_Impl: Sized {
    fn InitializeLogEx(&self, hinstance: *const u8) -> ::windows::core::Result<()>;
    fn hResult(&self, hresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn Log(&self, lflags: i32, lresid: i32, ldetail: i32, bstrtext: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn hResultEx(&self, lmethodid: i32, hresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn LogEx(&self, lmethodid: i32, lflags: i32, lresid: i32, ldetail: i32, bstrtext: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWiaLogEx {}
#[cfg(feature = "Win32_Foundation")]
impl IWiaLogEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaLogEx_Impl, const OFFSET: isize>() -> IWiaLogEx_Vtbl {
        unsafe extern "system" fn InitializeLogEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaLogEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hinstance: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeLogEx(::core::mem::transmute_copy(&hinstance)).into()
        }
        unsafe extern "system" fn hResult<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaLogEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.hResult(::core::mem::transmute_copy(&hresult)).into()
        }
        unsafe extern "system" fn Log<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaLogEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, lresid: i32, ldetail: i32, bstrtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Log(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&lresid), ::core::mem::transmute_copy(&ldetail), ::core::mem::transmute(&bstrtext)).into()
        }
        unsafe extern "system" fn hResultEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaLogEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmethodid: i32, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.hResultEx(::core::mem::transmute_copy(&lmethodid), ::core::mem::transmute_copy(&hresult)).into()
        }
        unsafe extern "system" fn LogEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaLogEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmethodid: i32, lflags: i32, lresid: i32, ldetail: i32, bstrtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LogEx(::core::mem::transmute_copy(&lmethodid), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&lresid), ::core::mem::transmute_copy(&ldetail), ::core::mem::transmute(&bstrtext)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            InitializeLogEx: InitializeLogEx::<Identity, Impl, OFFSET>,
            hResult: hResult::<Identity, Impl, OFFSET>,
            Log: Log::<Identity, Impl, OFFSET>,
            hResultEx: hResultEx::<Identity, Impl, OFFSET>,
            LogEx: LogEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaLogEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWiaMiniDrv_Impl: Sized {
    fn drvInitializeWia(&self, __midl__iwiaminidrv0000: *const u8, __midl__iwiaminidrv0001: i32, __midl__iwiaminidrv0002: &super::super::Foundation::BSTR, __midl__iwiaminidrv0003: &super::super::Foundation::BSTR, __midl__iwiaminidrv0004: &::core::option::Option<::windows::core::IUnknown>, __midl__iwiaminidrv0005: &::core::option::Option<::windows::core::IUnknown>, __midl__iwiaminidrv0006: *mut ::core::option::Option<IWiaDrvItem>, __midl__iwiaminidrv0007: *mut ::core::option::Option<::windows::core::IUnknown>, __midl__iwiaminidrv0008: *mut i32) -> ::windows::core::Result<()>;
    fn drvAcquireItemData(&self, __midl__iwiaminidrv0009: *const u8, __midl__iwiaminidrv0010: i32, __midl__iwiaminidrv0011: *mut MINIDRV_TRANSFER_CONTEXT, __midl__iwiaminidrv0012: *mut i32) -> ::windows::core::Result<()>;
    fn drvInitItemProperties(&self, __midl__iwiaminidrv0013: *const u8, __midl__iwiaminidrv0014: i32) -> ::windows::core::Result<i32>;
    fn drvValidateItemProperties(&self, __midl__iwiaminidrv0016: *const u8, __midl__iwiaminidrv0017: i32, __midl__iwiaminidrv0018: u32, __midl__iwiaminidrv0019: *const super::super::System::Com::StructuredStorage::PROPSPEC) -> ::windows::core::Result<i32>;
    fn drvWriteItemProperties(&self, __midl__iwiaminidrv0021: *const u8, __midl__iwiaminidrv0022: i32, __midl__iwiaminidrv0023: *const MINIDRV_TRANSFER_CONTEXT) -> ::windows::core::Result<i32>;
    fn drvReadItemProperties(&self, __midl__iwiaminidrv0025: *const u8, __midl__iwiaminidrv0026: i32, __midl__iwiaminidrv0027: u32, __midl__iwiaminidrv0028: *const super::super::System::Com::StructuredStorage::PROPSPEC) -> ::windows::core::Result<i32>;
    fn drvLockWiaDevice(&self, __midl__iwiaminidrv0030: *const u8, __midl__iwiaminidrv0031: i32) -> ::windows::core::Result<i32>;
    fn drvUnLockWiaDevice(&self, __midl__iwiaminidrv0033: *const u8, __midl__iwiaminidrv0034: i32) -> ::windows::core::Result<i32>;
    fn drvAnalyzeItem(&self, __midl__iwiaminidrv0036: *const u8, __midl__iwiaminidrv0037: i32, __midl__iwiaminidrv0038: *const i32) -> ::windows::core::Result<()>;
    fn drvGetDeviceErrorStr(&self, __midl__iwiaminidrv0039: i32, __midl__iwiaminidrv0040: i32, __midl__iwiaminidrv0041: *mut ::windows::core::PWSTR, __midl__iwiaminidrv0042: *mut i32) -> ::windows::core::Result<()>;
    fn drvDeviceCommand(&self, __midl__iwiaminidrv0043: *const u8, __midl__iwiaminidrv0044: i32, __midl__iwiaminidrv0045: *const ::windows::core::GUID, __midl__iwiaminidrv0046: *mut ::core::option::Option<IWiaDrvItem>, __midl__iwiaminidrv0047: *mut i32) -> ::windows::core::Result<()>;
    fn drvGetCapabilities(&self, __midl__iwiaminidrv0048: *const u8, __midl__iwiaminidrv0049: i32, __midl__iwiaminidrv0050: *mut i32, __midl__iwiaminidrv0051: *mut *mut WIA_DEV_CAP_DRV, __midl__iwiaminidrv0052: *mut i32) -> ::windows::core::Result<()>;
    fn drvDeleteItem(&self, __midl__iwiaminidrv0053: *const u8, __midl__iwiaminidrv0054: i32) -> ::windows::core::Result<i32>;
    fn drvFreeDrvItemContext(&self, __midl__iwiaminidrv0056: i32, __midl__iwiaminidrv0057: *const u8) -> ::windows::core::Result<i32>;
    fn drvGetWiaFormatInfo(&self, __midl__iwiaminidrv0059: *const u8, __midl__iwiaminidrv0060: i32, __midl__iwiaminidrv0061: *mut i32, __midl__iwiaminidrv0062: *mut *mut WIA_FORMAT_INFO, __midl__iwiaminidrv0063: *mut i32) -> ::windows::core::Result<()>;
    fn drvNotifyPnpEvent(&self, peventguid: *const ::windows::core::GUID, bstrdeviceid: &super::super::Foundation::BSTR, ulreserved: u32) -> ::windows::core::Result<()>;
    fn drvUnInitializeWia(&self, __midl__iwiaminidrv0064: *const u8) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for IWiaMiniDrv {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IWiaMiniDrv_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: isize>() -> IWiaMiniDrv_Vtbl {
        unsafe extern "system" fn drvInitializeWia<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0000: *const u8, __midl__iwiaminidrv0001: i32, __midl__iwiaminidrv0002: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, __midl__iwiaminidrv0003: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, __midl__iwiaminidrv0004: *mut ::core::ffi::c_void, __midl__iwiaminidrv0005: *mut ::core::ffi::c_void, __midl__iwiaminidrv0006: *mut *mut ::core::ffi::c_void, __midl__iwiaminidrv0007: *mut *mut ::core::ffi::c_void, __midl__iwiaminidrv0008: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.drvInitializeWia(::core::mem::transmute_copy(&__midl__iwiaminidrv0000), ::core::mem::transmute_copy(&__midl__iwiaminidrv0001), ::core::mem::transmute(&__midl__iwiaminidrv0002), ::core::mem::transmute(&__midl__iwiaminidrv0003), ::core::mem::transmute(&__midl__iwiaminidrv0004), ::core::mem::transmute(&__midl__iwiaminidrv0005), ::core::mem::transmute_copy(&__midl__iwiaminidrv0006), ::core::mem::transmute_copy(&__midl__iwiaminidrv0007), ::core::mem::transmute_copy(&__midl__iwiaminidrv0008))
                .into()
        }
        unsafe extern "system" fn drvAcquireItemData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0009: *const u8, __midl__iwiaminidrv0010: i32, __midl__iwiaminidrv0011: *mut MINIDRV_TRANSFER_CONTEXT, __midl__iwiaminidrv0012: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.drvAcquireItemData(::core::mem::transmute_copy(&__midl__iwiaminidrv0009), ::core::mem::transmute_copy(&__midl__iwiaminidrv0010), ::core::mem::transmute_copy(&__midl__iwiaminidrv0011), ::core::mem::transmute_copy(&__midl__iwiaminidrv0012)).into()
        }
        unsafe extern "system" fn drvInitItemProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0013: *const u8, __midl__iwiaminidrv0014: i32, __midl__iwiaminidrv0015: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.drvInitItemProperties(::core::mem::transmute_copy(&__midl__iwiaminidrv0013), ::core::mem::transmute_copy(&__midl__iwiaminidrv0014)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiaminidrv0015, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvValidateItemProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0016: *const u8, __midl__iwiaminidrv0017: i32, __midl__iwiaminidrv0018: u32, __midl__iwiaminidrv0019: *const super::super::System::Com::StructuredStorage::PROPSPEC, __midl__iwiaminidrv0020: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.drvValidateItemProperties(::core::mem::transmute_copy(&__midl__iwiaminidrv0016), ::core::mem::transmute_copy(&__midl__iwiaminidrv0017), ::core::mem::transmute_copy(&__midl__iwiaminidrv0018), ::core::mem::transmute_copy(&__midl__iwiaminidrv0019)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiaminidrv0020, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvWriteItemProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0021: *const u8, __midl__iwiaminidrv0022: i32, __midl__iwiaminidrv0023: *const MINIDRV_TRANSFER_CONTEXT, __midl__iwiaminidrv0024: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.drvWriteItemProperties(::core::mem::transmute_copy(&__midl__iwiaminidrv0021), ::core::mem::transmute_copy(&__midl__iwiaminidrv0022), ::core::mem::transmute_copy(&__midl__iwiaminidrv0023)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiaminidrv0024, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvReadItemProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0025: *const u8, __midl__iwiaminidrv0026: i32, __midl__iwiaminidrv0027: u32, __midl__iwiaminidrv0028: *const super::super::System::Com::StructuredStorage::PROPSPEC, __midl__iwiaminidrv0029: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.drvReadItemProperties(::core::mem::transmute_copy(&__midl__iwiaminidrv0025), ::core::mem::transmute_copy(&__midl__iwiaminidrv0026), ::core::mem::transmute_copy(&__midl__iwiaminidrv0027), ::core::mem::transmute_copy(&__midl__iwiaminidrv0028)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiaminidrv0029, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvLockWiaDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0030: *const u8, __midl__iwiaminidrv0031: i32, __midl__iwiaminidrv0032: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.drvLockWiaDevice(::core::mem::transmute_copy(&__midl__iwiaminidrv0030), ::core::mem::transmute_copy(&__midl__iwiaminidrv0031)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiaminidrv0032, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvUnLockWiaDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0033: *const u8, __midl__iwiaminidrv0034: i32, __midl__iwiaminidrv0035: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.drvUnLockWiaDevice(::core::mem::transmute_copy(&__midl__iwiaminidrv0033), ::core::mem::transmute_copy(&__midl__iwiaminidrv0034)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiaminidrv0035, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvAnalyzeItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0036: *const u8, __midl__iwiaminidrv0037: i32, __midl__iwiaminidrv0038: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.drvAnalyzeItem(::core::mem::transmute_copy(&__midl__iwiaminidrv0036), ::core::mem::transmute_copy(&__midl__iwiaminidrv0037), ::core::mem::transmute_copy(&__midl__iwiaminidrv0038)).into()
        }
        unsafe extern "system" fn drvGetDeviceErrorStr<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0039: i32, __midl__iwiaminidrv0040: i32, __midl__iwiaminidrv0041: *mut ::windows::core::PWSTR, __midl__iwiaminidrv0042: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.drvGetDeviceErrorStr(::core::mem::transmute_copy(&__midl__iwiaminidrv0039), ::core::mem::transmute_copy(&__midl__iwiaminidrv0040), ::core::mem::transmute_copy(&__midl__iwiaminidrv0041), ::core::mem::transmute_copy(&__midl__iwiaminidrv0042)).into()
        }
        unsafe extern "system" fn drvDeviceCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0043: *const u8, __midl__iwiaminidrv0044: i32, __midl__iwiaminidrv0045: *const ::windows::core::GUID, __midl__iwiaminidrv0046: *mut *mut ::core::ffi::c_void, __midl__iwiaminidrv0047: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.drvDeviceCommand(::core::mem::transmute_copy(&__midl__iwiaminidrv0043), ::core::mem::transmute_copy(&__midl__iwiaminidrv0044), ::core::mem::transmute_copy(&__midl__iwiaminidrv0045), ::core::mem::transmute_copy(&__midl__iwiaminidrv0046), ::core::mem::transmute_copy(&__midl__iwiaminidrv0047)).into()
        }
        unsafe extern "system" fn drvGetCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0048: *const u8, __midl__iwiaminidrv0049: i32, __midl__iwiaminidrv0050: *mut i32, __midl__iwiaminidrv0051: *mut *mut WIA_DEV_CAP_DRV, __midl__iwiaminidrv0052: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.drvGetCapabilities(::core::mem::transmute_copy(&__midl__iwiaminidrv0048), ::core::mem::transmute_copy(&__midl__iwiaminidrv0049), ::core::mem::transmute_copy(&__midl__iwiaminidrv0050), ::core::mem::transmute_copy(&__midl__iwiaminidrv0051), ::core::mem::transmute_copy(&__midl__iwiaminidrv0052)).into()
        }
        unsafe extern "system" fn drvDeleteItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0053: *const u8, __midl__iwiaminidrv0054: i32, __midl__iwiaminidrv0055: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.drvDeleteItem(::core::mem::transmute_copy(&__midl__iwiaminidrv0053), ::core::mem::transmute_copy(&__midl__iwiaminidrv0054)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiaminidrv0055, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvFreeDrvItemContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0056: i32, __midl__iwiaminidrv0057: *const u8, __midl__iwiaminidrv0058: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.drvFreeDrvItemContext(::core::mem::transmute_copy(&__midl__iwiaminidrv0056), ::core::mem::transmute_copy(&__midl__iwiaminidrv0057)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiaminidrv0058, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvGetWiaFormatInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0059: *const u8, __midl__iwiaminidrv0060: i32, __midl__iwiaminidrv0061: *mut i32, __midl__iwiaminidrv0062: *mut *mut WIA_FORMAT_INFO, __midl__iwiaminidrv0063: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.drvGetWiaFormatInfo(::core::mem::transmute_copy(&__midl__iwiaminidrv0059), ::core::mem::transmute_copy(&__midl__iwiaminidrv0060), ::core::mem::transmute_copy(&__midl__iwiaminidrv0061), ::core::mem::transmute_copy(&__midl__iwiaminidrv0062), ::core::mem::transmute_copy(&__midl__iwiaminidrv0063)).into()
        }
        unsafe extern "system" fn drvNotifyPnpEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventguid: *const ::windows::core::GUID, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ulreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.drvNotifyPnpEvent(::core::mem::transmute_copy(&peventguid), ::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute_copy(&ulreserved)).into()
        }
        unsafe extern "system" fn drvUnInitializeWia<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0064: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.drvUnInitializeWia(::core::mem::transmute_copy(&__midl__iwiaminidrv0064)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            drvInitializeWia: drvInitializeWia::<Identity, Impl, OFFSET>,
            drvAcquireItemData: drvAcquireItemData::<Identity, Impl, OFFSET>,
            drvInitItemProperties: drvInitItemProperties::<Identity, Impl, OFFSET>,
            drvValidateItemProperties: drvValidateItemProperties::<Identity, Impl, OFFSET>,
            drvWriteItemProperties: drvWriteItemProperties::<Identity, Impl, OFFSET>,
            drvReadItemProperties: drvReadItemProperties::<Identity, Impl, OFFSET>,
            drvLockWiaDevice: drvLockWiaDevice::<Identity, Impl, OFFSET>,
            drvUnLockWiaDevice: drvUnLockWiaDevice::<Identity, Impl, OFFSET>,
            drvAnalyzeItem: drvAnalyzeItem::<Identity, Impl, OFFSET>,
            drvGetDeviceErrorStr: drvGetDeviceErrorStr::<Identity, Impl, OFFSET>,
            drvDeviceCommand: drvDeviceCommand::<Identity, Impl, OFFSET>,
            drvGetCapabilities: drvGetCapabilities::<Identity, Impl, OFFSET>,
            drvDeleteItem: drvDeleteItem::<Identity, Impl, OFFSET>,
            drvFreeDrvItemContext: drvFreeDrvItemContext::<Identity, Impl, OFFSET>,
            drvGetWiaFormatInfo: drvGetWiaFormatInfo::<Identity, Impl, OFFSET>,
            drvNotifyPnpEvent: drvNotifyPnpEvent::<Identity, Impl, OFFSET>,
            drvUnInitializeWia: drvUnInitializeWia::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaMiniDrv as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaMiniDrvCallBack_Impl: Sized {
    fn MiniDrvCallback(&self, lreason: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, ptranctx: *const MINIDRV_TRANSFER_CONTEXT, lreserved: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWiaMiniDrvCallBack {}
#[cfg(feature = "Win32_Foundation")]
impl IWiaMiniDrvCallBack_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaMiniDrvCallBack_Impl, const OFFSET: isize>() -> IWiaMiniDrvCallBack_Vtbl {
        unsafe extern "system" fn MiniDrvCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaMiniDrvCallBack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lreason: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, ptranctx: *const MINIDRV_TRANSFER_CONTEXT, lreserved: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MiniDrvCallback(::core::mem::transmute_copy(&lreason), ::core::mem::transmute_copy(&lstatus), ::core::mem::transmute_copy(&lpercentcomplete), ::core::mem::transmute_copy(&loffset), ::core::mem::transmute_copy(&llength), ::core::mem::transmute_copy(&ptranctx), ::core::mem::transmute_copy(&lreserved)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), MiniDrvCallback: MiniDrvCallback::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaMiniDrvCallBack as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWiaMiniDrvTransferCallback_Impl: Sized {
    fn GetNextStream(&self, lflags: i32, bstritemname: &super::super::Foundation::BSTR, bstrfullitemname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn SendMessage(&self, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IWiaMiniDrvTransferCallback {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWiaMiniDrvTransferCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaMiniDrvTransferCallback_Impl, const OFFSET: isize>() -> IWiaMiniDrvTransferCallback_Vtbl {
        unsafe extern "system" fn GetNextStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaMiniDrvTransferCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppistream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNextStream(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstritemname), ::core::mem::transmute(&bstrfullitemname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppistream, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaMiniDrvTransferCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendMessage(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pwiatransferparams)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetNextStream: GetNextStream::<Identity, Impl, OFFSET>,
            SendMessage: SendMessage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaMiniDrvTransferCallback as ::windows::core::Interface>::IID
    }
}
pub trait IWiaNotifyDevMgr_Impl: Sized {
    fn NewDeviceArrival(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWiaNotifyDevMgr {}
impl IWiaNotifyDevMgr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaNotifyDevMgr_Impl, const OFFSET: isize>() -> IWiaNotifyDevMgr_Vtbl {
        unsafe extern "system" fn NewDeviceArrival<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaNotifyDevMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NewDeviceArrival().into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), NewDeviceArrival: NewDeviceArrival::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaNotifyDevMgr as ::windows::core::Interface>::IID
    }
}
pub trait IWiaPreview_Impl: Sized {
    fn GetNewPreview(&self, lflags: i32, pwiaitem2: &::core::option::Option<IWiaItem2>, pwiatransfercallback: &::core::option::Option<IWiaTransferCallback>) -> ::windows::core::Result<()>;
    fn UpdatePreview(&self, lflags: i32, pchildwiaitem2: &::core::option::Option<IWiaItem2>, pwiatransfercallback: &::core::option::Option<IWiaTransferCallback>) -> ::windows::core::Result<()>;
    fn DetectRegions(&self, lflags: i32) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWiaPreview {}
impl IWiaPreview_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaPreview_Impl, const OFFSET: isize>() -> IWiaPreview_Vtbl {
        unsafe extern "system" fn GetNewPreview<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiaitem2: *mut ::core::ffi::c_void, pwiatransfercallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNewPreview(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pwiaitem2), ::core::mem::transmute(&pwiatransfercallback)).into()
        }
        unsafe extern "system" fn UpdatePreview<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pchildwiaitem2: *mut ::core::ffi::c_void, pwiatransfercallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdatePreview(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pchildwiaitem2), ::core::mem::transmute(&pwiatransfercallback)).into()
        }
        unsafe extern "system" fn DetectRegions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DetectRegions(::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetNewPreview: GetNewPreview::<Identity, Impl, OFFSET>,
            UpdatePreview: UpdatePreview::<Identity, Impl, OFFSET>,
            DetectRegions: DetectRegions::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaPreview as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWiaPropertyStorage_Impl: Sized {
    fn ReadMultiple(&self, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn WriteMultiple(&self, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT, propidnamefirst: u32) -> ::windows::core::Result<()>;
    fn DeleteMultiple(&self, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC) -> ::windows::core::Result<()>;
    fn ReadPropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn WritePropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn DeletePropertyNames(&self, cpropid: u32, rgpropid: *const u32) -> ::windows::core::Result<()>;
    fn Commit(&self, grfcommitflags: u32) -> ::windows::core::Result<()>;
    fn Revert(&self) -> ::windows::core::Result<()>;
    fn Enum(&self) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::IEnumSTATPROPSTG>;
    fn SetTimes(&self, pctime: *const super::super::Foundation::FILETIME, patime: *const super::super::Foundation::FILETIME, pmtime: *const super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn SetClass(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Stat(&self) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::STATPROPSETSTG>;
    fn GetPropertyAttributes(&self, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgflags: *mut u32, rgpropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetCount(&self) -> ::windows::core::Result<u32>;
    fn GetPropertyStream(&self, pcompatibilityid: *mut ::windows::core::GUID, ppistream: *mut ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn SetPropertyStream(&self, pcompatibilityid: *mut ::windows::core::GUID, pistream: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for IWiaPropertyStorage {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IWiaPropertyStorage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: isize>() -> IWiaPropertyStorage_Vtbl {
        unsafe extern "system" fn ReadMultiple<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadMultiple(::core::mem::transmute_copy(&cpspec), ::core::mem::transmute_copy(&rgpspec), ::core::mem::transmute_copy(&rgpropvar)).into()
        }
        unsafe extern "system" fn WriteMultiple<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT, propidnamefirst: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteMultiple(::core::mem::transmute_copy(&cpspec), ::core::mem::transmute_copy(&rgpspec), ::core::mem::transmute_copy(&rgpropvar), ::core::mem::transmute_copy(&propidnamefirst)).into()
        }
        unsafe extern "system" fn DeleteMultiple<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteMultiple(::core::mem::transmute_copy(&cpspec), ::core::mem::transmute_copy(&rgpspec)).into()
        }
        unsafe extern "system" fn ReadPropertyNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadPropertyNames(::core::mem::transmute_copy(&cpropid), ::core::mem::transmute_copy(&rgpropid), ::core::mem::transmute_copy(&rglpwstrname)).into()
        }
        unsafe extern "system" fn WritePropertyNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WritePropertyNames(::core::mem::transmute_copy(&cpropid), ::core::mem::transmute_copy(&rgpropid), ::core::mem::transmute_copy(&rglpwstrname)).into()
        }
        unsafe extern "system" fn DeletePropertyNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePropertyNames(::core::mem::transmute_copy(&cpropid), ::core::mem::transmute_copy(&rgpropid)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfcommitflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit(::core::mem::transmute_copy(&grfcommitflags)).into()
        }
        unsafe extern "system" fn Revert<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Revert().into()
        }
        unsafe extern "system" fn Enum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctime: *const super::super::Foundation::FILETIME, patime: *const super::super::Foundation::FILETIME, pmtime: *const super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTimes(::core::mem::transmute_copy(&pctime), ::core::mem::transmute_copy(&patime), ::core::mem::transmute_copy(&pmtime)).into()
        }
        unsafe extern "system" fn SetClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClass(::core::mem::transmute_copy(&clsid)).into()
        }
        unsafe extern "system" fn Stat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatpsstg: *mut super::super::System::Com::StructuredStorage::STATPROPSETSTG) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Stat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatpsstg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgflags: *mut u32, rgpropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyAttributes(::core::mem::transmute_copy(&cpspec), ::core::mem::transmute_copy(&rgpspec), ::core::mem::transmute_copy(&rgflags), ::core::mem::transmute_copy(&rgpropvar)).into()
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulnumprops: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulnumprops, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcompatibilityid: *mut ::windows::core::GUID, ppistream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyStream(::core::mem::transmute_copy(&pcompatibilityid), ::core::mem::transmute_copy(&ppistream)).into()
        }
        unsafe extern "system" fn SetPropertyStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcompatibilityid: *mut ::windows::core::GUID, pistream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPropertyStream(::core::mem::transmute_copy(&pcompatibilityid), ::core::mem::transmute(&pistream)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ReadMultiple: ReadMultiple::<Identity, Impl, OFFSET>,
            WriteMultiple: WriteMultiple::<Identity, Impl, OFFSET>,
            DeleteMultiple: DeleteMultiple::<Identity, Impl, OFFSET>,
            ReadPropertyNames: ReadPropertyNames::<Identity, Impl, OFFSET>,
            WritePropertyNames: WritePropertyNames::<Identity, Impl, OFFSET>,
            DeletePropertyNames: DeletePropertyNames::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Revert: Revert::<Identity, Impl, OFFSET>,
            Enum: Enum::<Identity, Impl, OFFSET>,
            SetTimes: SetTimes::<Identity, Impl, OFFSET>,
            SetClass: SetClass::<Identity, Impl, OFFSET>,
            Stat: Stat::<Identity, Impl, OFFSET>,
            GetPropertyAttributes: GetPropertyAttributes::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetPropertyStream: GetPropertyStream::<Identity, Impl, OFFSET>,
            SetPropertyStream: SetPropertyStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaPropertyStorage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWiaSegmentationFilter_Impl: Sized {
    fn DetectRegions(&self, lflags: i32, pinputstream: &::core::option::Option<super::super::System::Com::IStream>, pwiaitem2: &::core::option::Option<IWiaItem2>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWiaSegmentationFilter {}
#[cfg(feature = "Win32_System_Com")]
impl IWiaSegmentationFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaSegmentationFilter_Impl, const OFFSET: isize>() -> IWiaSegmentationFilter_Vtbl {
        unsafe extern "system" fn DetectRegions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaSegmentationFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pinputstream: *mut ::core::ffi::c_void, pwiaitem2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DetectRegions(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pinputstream), ::core::mem::transmute(&pwiaitem2)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), DetectRegions: DetectRegions::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaSegmentationFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWiaTransfer_Impl: Sized {
    fn Download(&self, lflags: i32, piwiatransfercallback: &::core::option::Option<IWiaTransferCallback>) -> ::windows::core::Result<()>;
    fn Upload(&self, lflags: i32, psource: &::core::option::Option<super::super::System::Com::IStream>, piwiatransfercallback: &::core::option::Option<IWiaTransferCallback>) -> ::windows::core::Result<()>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn EnumWIA_FORMAT_INFO(&self) -> ::windows::core::Result<IEnumWIA_FORMAT_INFO>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWiaTransfer {}
#[cfg(feature = "Win32_System_Com")]
impl IWiaTransfer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaTransfer_Impl, const OFFSET: isize>() -> IWiaTransfer_Vtbl {
        unsafe extern "system" fn Download<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaTransfer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, piwiatransfercallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Download(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&piwiatransfercallback)).into()
        }
        unsafe extern "system" fn Upload<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaTransfer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, psource: *mut ::core::ffi::c_void, piwiatransfercallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Upload(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&psource), ::core::mem::transmute(&piwiatransfercallback)).into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaTransfer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        unsafe extern "system" fn EnumWIA_FORMAT_INFO<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaTransfer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumWIA_FORMAT_INFO() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Download: Download::<Identity, Impl, OFFSET>,
            Upload: Upload::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            EnumWIA_FORMAT_INFO: EnumWIA_FORMAT_INFO::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaTransfer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWiaTransferCallback_Impl: Sized {
    fn TransferCallback(&self, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> ::windows::core::Result<()>;
    fn GetNextStream(&self, lflags: i32, bstritemname: &super::super::Foundation::BSTR, bstrfullitemname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IWiaTransferCallback {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWiaTransferCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaTransferCallback_Impl, const OFFSET: isize>() -> IWiaTransferCallback_Vtbl {
        unsafe extern "system" fn TransferCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaTransferCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TransferCallback(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pwiatransferparams)).into()
        }
        unsafe extern "system" fn GetNextStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaTransferCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdestination: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNextStream(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstritemname), ::core::mem::transmute(&bstrfullitemname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdestination, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            TransferCallback: TransferCallback::<Identity, Impl, OFFSET>,
            GetNextStream: GetNextStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaTransferCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWiaUIExtension_Impl: Sized {
    fn DeviceDialog(&self, pdevicedialogdata: *const DEVICEDIALOGDATA) -> ::windows::core::Result<()>;
    fn GetDeviceIcon(&self, bstrdeviceid: &super::super::Foundation::BSTR, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> ::windows::core::Result<()>;
    fn GetDeviceBitmapLogo(&self, bstrdeviceid: &super::super::Foundation::BSTR, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, nmaxwidth: u32, nmaxheight: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows::core::RuntimeName for IWiaUIExtension {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWiaUIExtension_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaUIExtension_Impl, const OFFSET: isize>() -> IWiaUIExtension_Vtbl {
        unsafe extern "system" fn DeviceDialog<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaUIExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevicedialogdata: *const DEVICEDIALOGDATA) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceDialog(::core::mem::transmute_copy(&pdevicedialogdata)).into()
        }
        unsafe extern "system" fn GetDeviceIcon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaUIExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceIcon(::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute_copy(&phicon), ::core::mem::transmute_copy(&nsize)).into()
        }
        unsafe extern "system" fn GetDeviceBitmapLogo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaUIExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, nmaxwidth: u32, nmaxheight: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceBitmapLogo(::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute_copy(&phbitmap), ::core::mem::transmute_copy(&nmaxwidth), ::core::mem::transmute_copy(&nmaxheight)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            DeviceDialog: DeviceDialog::<Identity, Impl, OFFSET>,
            GetDeviceIcon: GetDeviceIcon::<Identity, Impl, OFFSET>,
            GetDeviceBitmapLogo: GetDeviceBitmapLogo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaUIExtension as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWiaUIExtension2_Impl: Sized {
    fn DeviceDialog(&self, pdevicedialogdata: *const DEVICEDIALOGDATA2) -> ::windows::core::Result<()>;
    fn GetDeviceIcon(&self, bstrdeviceid: &super::super::Foundation::BSTR, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows::core::RuntimeName for IWiaUIExtension2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWiaUIExtension2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaUIExtension2_Impl, const OFFSET: isize>() -> IWiaUIExtension2_Vtbl {
        unsafe extern "system" fn DeviceDialog<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaUIExtension2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevicedialogdata: *const DEVICEDIALOGDATA2) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceDialog(::core::mem::transmute_copy(&pdevicedialogdata)).into()
        }
        unsafe extern "system" fn GetDeviceIcon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaUIExtension2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceIcon(::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute_copy(&phicon), ::core::mem::transmute_copy(&nsize)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            DeviceDialog: DeviceDialog::<Identity, Impl, OFFSET>,
            GetDeviceIcon: GetDeviceIcon::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaUIExtension2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaVideo_Impl: Sized {
    fn PreviewVisible(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetPreviewVisible(&self, bpreviewvisible: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ImagesDirectory(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetImagesDirectory(&self, bstrimagedirectory: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CreateVideoByWiaDevID(&self, bstrwiadeviceid: &super::super::Foundation::BSTR, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CreateVideoByDevNum(&self, uidevicenumber: u32, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CreateVideoByName(&self, bstrfriendlyname: &super::super::Foundation::BSTR, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn DestroyVideo(&self) -> ::windows::core::Result<()>;
    fn Play(&self) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn TakePicture(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ResizeVideo(&self, bstretchtofitparent: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetCurrentState(&self) -> ::windows::core::Result<WIAVIDEO_STATE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWiaVideo {}
#[cfg(feature = "Win32_Foundation")]
impl IWiaVideo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: isize>() -> IWiaVideo_Vtbl {
        unsafe extern "system" fn PreviewVisible<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpreviewvisible: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PreviewVisible() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbpreviewvisible, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreviewVisible<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bpreviewvisible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPreviewVisible(::core::mem::transmute_copy(&bpreviewvisible)).into()
        }
        unsafe extern "system" fn ImagesDirectory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrimagedirectory: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ImagesDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrimagedirectory, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImagesDirectory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrimagedirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetImagesDirectory(::core::mem::transmute(&bstrimagedirectory)).into()
        }
        unsafe extern "system" fn CreateVideoByWiaDevID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrwiadeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateVideoByWiaDevID(::core::mem::transmute(&bstrwiadeviceid), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&bstretchtofitparent), ::core::mem::transmute_copy(&bautobeginplayback)).into()
        }
        unsafe extern "system" fn CreateVideoByDevNum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uidevicenumber: u32, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateVideoByDevNum(::core::mem::transmute_copy(&uidevicenumber), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&bstretchtofitparent), ::core::mem::transmute_copy(&bautobeginplayback)).into()
        }
        unsafe extern "system" fn CreateVideoByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateVideoByName(::core::mem::transmute(&bstrfriendlyname), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&bstretchtofitparent), ::core::mem::transmute_copy(&bautobeginplayback)).into()
        }
        unsafe extern "system" fn DestroyVideo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DestroyVideo().into()
        }
        unsafe extern "system" fn Play<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Play().into()
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Pause().into()
        }
        unsafe extern "system" fn TakePicture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnewimagefilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TakePicture() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrnewimagefilename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResizeVideo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstretchtofitparent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResizeVideo(::core::mem::transmute_copy(&bstretchtofitparent)).into()
        }
        unsafe extern "system" fn GetCurrentState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut WIAVIDEO_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            PreviewVisible: PreviewVisible::<Identity, Impl, OFFSET>,
            SetPreviewVisible: SetPreviewVisible::<Identity, Impl, OFFSET>,
            ImagesDirectory: ImagesDirectory::<Identity, Impl, OFFSET>,
            SetImagesDirectory: SetImagesDirectory::<Identity, Impl, OFFSET>,
            CreateVideoByWiaDevID: CreateVideoByWiaDevID::<Identity, Impl, OFFSET>,
            CreateVideoByDevNum: CreateVideoByDevNum::<Identity, Impl, OFFSET>,
            CreateVideoByName: CreateVideoByName::<Identity, Impl, OFFSET>,
            DestroyVideo: DestroyVideo::<Identity, Impl, OFFSET>,
            Play: Play::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            TakePicture: TakePicture::<Identity, Impl, OFFSET>,
            ResizeVideo: ResizeVideo::<Identity, Impl, OFFSET>,
            GetCurrentState: GetCurrentState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaVideo as ::windows::core::Interface>::IID
    }
}
