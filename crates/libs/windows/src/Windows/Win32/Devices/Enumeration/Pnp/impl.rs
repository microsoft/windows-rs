pub trait IUPnPAddressFamilyControl_Impl: Sized {
    fn SetAddressFamily(&self, dwflags: i32) -> ::windows::core::Result<()>;
    fn GetAddressFamily(&self) -> ::windows::core::Result<i32>;
}
impl ::windows::core::RuntimeName for IUPnPAddressFamilyControl {}
impl IUPnPAddressFamilyControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPAddressFamilyControl_Impl, const OFFSET: isize>() -> IUPnPAddressFamilyControl_Vtbl {
        unsafe extern "system" fn SetAddressFamily<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPAddressFamilyControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAddressFamily(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetAddressFamily<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPAddressFamilyControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAddressFamily() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetAddressFamily: SetAddressFamily::<Identity, Impl, OFFSET>,
            GetAddressFamily: GetAddressFamily::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPAddressFamilyControl as ::windows::core::Interface>::IID
    }
}
pub trait IUPnPAsyncResult_Impl: Sized {
    fn AsyncOperationComplete(&self, ullrequestid: u64) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IUPnPAsyncResult {}
impl IUPnPAsyncResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPAsyncResult_Impl, const OFFSET: isize>() -> IUPnPAsyncResult_Vtbl {
        unsafe extern "system" fn AsyncOperationComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsyncOperationComplete(::core::mem::transmute_copy(&ullrequestid)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), AsyncOperationComplete: AsyncOperationComplete::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPAsyncResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUPnPDescriptionDocument_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn ReadyState(&self) -> ::windows::core::Result<i32>;
    fn Load(&self, bstrurl: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LoadAsync(&self, bstrurl: &super::super::super::Foundation::BSTR, punkcallback: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn LoadResult(&self) -> ::windows::core::Result<i32>;
    fn Abort(&self) -> ::windows::core::Result<()>;
    fn RootDevice(&self) -> ::windows::core::Result<IUPnPDevice>;
    fn DeviceByUDN(&self, bstrudn: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IUPnPDevice>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IUPnPDescriptionDocument {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUPnPDescriptionDocument_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: isize>() -> IUPnPDescriptionDocument_Vtbl {
        unsafe extern "system" fn ReadyState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plreadystate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadyState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plreadystate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Load<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Load(::core::mem::transmute(&bstrurl)).into()
        }
        unsafe extern "system" fn LoadAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, punkcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadAsync(::core::mem::transmute(&bstrurl), ::core::mem::transmute(&punkcallback)).into()
        }
        unsafe extern "system" fn LoadResult<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerror: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoadResult() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrerror, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort().into()
        }
        unsafe extern "system" fn RootDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppudrootdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RootDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppudrootdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceByUDN<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppuddevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceByUDN(::core::mem::transmute(&bstrudn)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppuddevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ReadyState: ReadyState::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            LoadAsync: LoadAsync::<Identity, Impl, OFFSET>,
            LoadResult: LoadResult::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            RootDevice: RootDevice::<Identity, Impl, OFFSET>,
            DeviceByUDN: DeviceByUDN::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDescriptionDocument as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IUPnPDescriptionDocumentCallback_Impl: Sized {
    fn LoadComplete(&self, hrloadresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IUPnPDescriptionDocumentCallback {}
impl IUPnPDescriptionDocumentCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDescriptionDocumentCallback_Impl, const OFFSET: isize>() -> IUPnPDescriptionDocumentCallback_Vtbl {
        unsafe extern "system" fn LoadComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDescriptionDocumentCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrloadresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadComplete(::core::mem::transmute_copy(&hrloadresult)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), LoadComplete: LoadComplete::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDescriptionDocumentCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUPnPDevice_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn IsRootDevice(&self) -> ::windows::core::Result<i16>;
    fn RootDevice(&self) -> ::windows::core::Result<IUPnPDevice>;
    fn ParentDevice(&self) -> ::windows::core::Result<IUPnPDevice>;
    fn HasChildren(&self) -> ::windows::core::Result<i16>;
    fn Children(&self) -> ::windows::core::Result<IUPnPDevices>;
    fn UniqueDeviceName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn FriendlyName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Type(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn PresentationURL(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn ManufacturerName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn ManufacturerURL(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn ModelName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn ModelNumber(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Description(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn ModelURL(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn UPC(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SerialNumber(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn IconURL(&self, bstrencodingformat: &super::super::super::Foundation::BSTR, lsizex: i32, lsizey: i32, lbitdepth: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Services(&self) -> ::windows::core::Result<IUPnPServices>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IUPnPDevice {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUPnPDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>() -> IUPnPDevice_Vtbl {
        unsafe extern "system" fn IsRootDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarb: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsRootDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarb, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RootDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppudrootdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RootDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppudrootdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParentDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuddeviceparent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ParentDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppuddeviceparent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasChildren<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarb: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasChildren() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarb, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppudchildren: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Children() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppudchildren, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UniqueDeviceName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UniqueDeviceName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FriendlyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresentationURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PresentationURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManufacturerName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ManufacturerName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManufacturerURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ManufacturerURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ModelName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ModelNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ModelURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UPC<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UPC() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerialNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SerialNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IconURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrencodingformat: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, lsizex: i32, lsizey: i32, lbitdepth: i32, pbstriconurl: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IconURL(::core::mem::transmute(&bstrencodingformat), ::core::mem::transmute_copy(&lsizex), ::core::mem::transmute_copy(&lsizey), ::core::mem::transmute_copy(&lbitdepth)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstriconurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Services<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppusservices: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Services() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppusservices, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsRootDevice: IsRootDevice::<Identity, Impl, OFFSET>,
            RootDevice: RootDevice::<Identity, Impl, OFFSET>,
            ParentDevice: ParentDevice::<Identity, Impl, OFFSET>,
            HasChildren: HasChildren::<Identity, Impl, OFFSET>,
            Children: Children::<Identity, Impl, OFFSET>,
            UniqueDeviceName: UniqueDeviceName::<Identity, Impl, OFFSET>,
            FriendlyName: FriendlyName::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            PresentationURL: PresentationURL::<Identity, Impl, OFFSET>,
            ManufacturerName: ManufacturerName::<Identity, Impl, OFFSET>,
            ManufacturerURL: ManufacturerURL::<Identity, Impl, OFFSET>,
            ModelName: ModelName::<Identity, Impl, OFFSET>,
            ModelNumber: ModelNumber::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            ModelURL: ModelURL::<Identity, Impl, OFFSET>,
            UPC: UPC::<Identity, Impl, OFFSET>,
            SerialNumber: SerialNumber::<Identity, Impl, OFFSET>,
            IconURL: IconURL::<Identity, Impl, OFFSET>,
            Services: Services::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDevice as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IUPnPDeviceControl_Impl: Sized {
    fn Initialize(&self, bstrxmldesc: &super::super::super::Foundation::BSTR, bstrdeviceidentifier: &super::super::super::Foundation::BSTR, bstrinitstring: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetServiceObject(&self, bstrudn: &super::super::super::Foundation::BSTR, bstrserviceid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IUPnPDeviceControl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IUPnPDeviceControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceControl_Impl, const OFFSET: isize>() -> IUPnPDeviceControl_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute(&bstrxmldesc), ::core::mem::transmute(&bstrdeviceidentifier), ::core::mem::transmute(&bstrinitstring)).into()
        }
        unsafe extern "system" fn GetServiceObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrserviceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppdispservice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetServiceObject(::core::mem::transmute(&bstrudn), ::core::mem::transmute(&bstrserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdispservice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetServiceObject: GetServiceObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDeviceControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUPnPDeviceControlHttpHeaders_Impl: Sized {
    fn GetAdditionalResponseHeaders(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IUPnPDeviceControlHttpHeaders {}
#[cfg(feature = "Win32_Foundation")]
impl IUPnPDeviceControlHttpHeaders_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceControlHttpHeaders_Impl, const OFFSET: isize>() -> IUPnPDeviceControlHttpHeaders_Vtbl {
        unsafe extern "system" fn GetAdditionalResponseHeaders<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceControlHttpHeaders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhttpresponseheaders: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAdditionalResponseHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrhttpresponseheaders, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetAdditionalResponseHeaders: GetAdditionalResponseHeaders::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDeviceControlHttpHeaders as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUPnPDeviceDocumentAccess_Impl: Sized {
    fn GetDocumentURL(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IUPnPDeviceDocumentAccess {}
#[cfg(feature = "Win32_Foundation")]
impl IUPnPDeviceDocumentAccess_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceDocumentAccess_Impl, const OFFSET: isize>() -> IUPnPDeviceDocumentAccess_Vtbl {
        unsafe extern "system" fn GetDocumentURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceDocumentAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdocument: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocumentURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdocument, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetDocumentURL: GetDocumentURL::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDeviceDocumentAccess as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUPnPDeviceDocumentAccessEx_Impl: Sized {
    fn GetDocument(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IUPnPDeviceDocumentAccessEx {}
#[cfg(feature = "Win32_Foundation")]
impl IUPnPDeviceDocumentAccessEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceDocumentAccessEx_Impl, const OFFSET: isize>() -> IUPnPDeviceDocumentAccessEx_Vtbl {
        unsafe extern "system" fn GetDocument<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceDocumentAccessEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdocument: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocument() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdocument, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetDocument: GetDocument::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDeviceDocumentAccessEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUPnPDeviceFinder_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn FindByType(&self, bstrtypeuri: &super::super::super::Foundation::BSTR, dwflags: u32) -> ::windows::core::Result<IUPnPDevices>;
    fn CreateAsyncFind(&self, bstrtypeuri: &super::super::super::Foundation::BSTR, dwflags: u32, punkdevicefindercallback: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<i32>;
    fn StartAsyncFind(&self, lfinddata: i32) -> ::windows::core::Result<()>;
    fn CancelAsyncFind(&self, lfinddata: i32) -> ::windows::core::Result<()>;
    fn FindByUDN(&self, bstrudn: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IUPnPDevice>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IUPnPDeviceFinder {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUPnPDeviceFinder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinder_Impl, const OFFSET: isize>() -> IUPnPDeviceFinder_Vtbl {
        unsafe extern "system" fn FindByType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtypeuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dwflags: u32, pdevices: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindByType(::core::mem::transmute(&bstrtypeuri), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdevices, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAsyncFind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtypeuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dwflags: u32, punkdevicefindercallback: *mut ::core::ffi::c_void, plfinddata: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateAsyncFind(::core::mem::transmute(&bstrtypeuri), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&punkdevicefindercallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plfinddata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAsyncFind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartAsyncFind(::core::mem::transmute_copy(&lfinddata)).into()
        }
        unsafe extern "system" fn CancelAsyncFind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelAsyncFind(::core::mem::transmute_copy(&lfinddata)).into()
        }
        unsafe extern "system" fn FindByUDN<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindByUDN(::core::mem::transmute(&bstrudn)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            FindByType: FindByType::<Identity, Impl, OFFSET>,
            CreateAsyncFind: CreateAsyncFind::<Identity, Impl, OFFSET>,
            StartAsyncFind: StartAsyncFind::<Identity, Impl, OFFSET>,
            CancelAsyncFind: CancelAsyncFind::<Identity, Impl, OFFSET>,
            FindByUDN: FindByUDN::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDeviceFinder as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDeviceFinderAddCallbackWithInterface_Impl: Sized {
    fn DeviceAddedWithInterface(&self, lfinddata: i32, pdevice: &::core::option::Option<IUPnPDevice>, pguidinterface: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUPnPDeviceFinderAddCallbackWithInterface {}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDeviceFinderAddCallbackWithInterface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinderAddCallbackWithInterface_Impl, const OFFSET: isize>() -> IUPnPDeviceFinderAddCallbackWithInterface_Vtbl {
        unsafe extern "system" fn DeviceAddedWithInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinderAddCallbackWithInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32, pdevice: *mut ::core::ffi::c_void, pguidinterface: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceAddedWithInterface(::core::mem::transmute_copy(&lfinddata), ::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&pguidinterface)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), DeviceAddedWithInterface: DeviceAddedWithInterface::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDeviceFinderAddCallbackWithInterface as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IUPnPDeviceFinderCallback_Impl: Sized {
    fn DeviceAdded(&self, lfinddata: i32, pdevice: &::core::option::Option<IUPnPDevice>) -> ::windows::core::Result<()>;
    fn DeviceRemoved(&self, lfinddata: i32, bstrudn: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SearchComplete(&self, lfinddata: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IUPnPDeviceFinderCallback {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IUPnPDeviceFinderCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinderCallback_Impl, const OFFSET: isize>() -> IUPnPDeviceFinderCallback_Vtbl {
        unsafe extern "system" fn DeviceAdded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinderCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32, pdevice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceAdded(::core::mem::transmute_copy(&lfinddata), ::core::mem::transmute(&pdevice)).into()
        }
        unsafe extern "system" fn DeviceRemoved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinderCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceRemoved(::core::mem::transmute_copy(&lfinddata), ::core::mem::transmute(&bstrudn)).into()
        }
        unsafe extern "system" fn SearchComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceFinderCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SearchComplete(::core::mem::transmute_copy(&lfinddata)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            DeviceAdded: DeviceAdded::<Identity, Impl, OFFSET>,
            DeviceRemoved: DeviceRemoved::<Identity, Impl, OFFSET>,
            SearchComplete: SearchComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDeviceFinderCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUPnPDeviceProvider_Impl: Sized {
    fn Start(&self, bstrinitstring: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IUPnPDeviceProvider {}
#[cfg(feature = "Win32_Foundation")]
impl IUPnPDeviceProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceProvider_Impl, const OFFSET: isize>() -> IUPnPDeviceProvider_Vtbl {
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start(::core::mem::transmute(&bstrinitstring)).into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Start: Start::<Identity, Impl, OFFSET>, Stop: Stop::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDeviceProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUPnPDevices_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn get_Item(&self, bstrudn: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IUPnPDevice>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IUPnPDevices {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUPnPDevices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevices_Impl, const OFFSET: isize>() -> IUPnPDevices_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&bstrudn)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDevices as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUPnPEventSink_Impl: Sized {
    fn OnStateChanged(&self, cchanges: u32, rgdispidchanges: *const i32) -> ::windows::core::Result<()>;
    fn OnStateChangedSafe(&self, varsadispidchanges: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IUPnPEventSink {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUPnPEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPEventSink_Impl, const OFFSET: isize>() -> IUPnPEventSink_Vtbl {
        unsafe extern "system" fn OnStateChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchanges: u32, rgdispidchanges: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStateChanged(::core::mem::transmute_copy(&cchanges), ::core::mem::transmute_copy(&rgdispidchanges)).into()
        }
        unsafe extern "system" fn OnStateChangedSafe<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsadispidchanges: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStateChangedSafe(::core::mem::transmute(&varsadispidchanges)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnStateChanged: OnStateChanged::<Identity, Impl, OFFSET>,
            OnStateChangedSafe: OnStateChangedSafe::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPEventSink as ::windows::core::Interface>::IID
    }
}
pub trait IUPnPEventSource_Impl: Sized {
    fn Advise(&self, pessubscriber: &::core::option::Option<IUPnPEventSink>) -> ::windows::core::Result<()>;
    fn Unadvise(&self, pessubscriber: &::core::option::Option<IUPnPEventSink>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IUPnPEventSource {}
impl IUPnPEventSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPEventSource_Impl, const OFFSET: isize>() -> IUPnPEventSource_Vtbl {
        unsafe extern "system" fn Advise<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPEventSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pessubscriber: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Advise(::core::mem::transmute(&pessubscriber)).into()
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPEventSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pessubscriber: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unadvise(::core::mem::transmute(&pessubscriber)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPEventSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUPnPHttpHeaderControl_Impl: Sized {
    fn AddRequestHeaders(&self, bstrhttpheaders: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IUPnPHttpHeaderControl {}
#[cfg(feature = "Win32_Foundation")]
impl IUPnPHttpHeaderControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPHttpHeaderControl_Impl, const OFFSET: isize>() -> IUPnPHttpHeaderControl_Vtbl {
        unsafe extern "system" fn AddRequestHeaders<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPHttpHeaderControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhttpheaders: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRequestHeaders(::core::mem::transmute(&bstrhttpheaders)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), AddRequestHeaders: AddRequestHeaders::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPHttpHeaderControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUPnPRegistrar_Impl: Sized {
    fn RegisterDevice(&self, bstrxmldesc: &super::super::super::Foundation::BSTR, bstrprogiddevicecontrolclass: &super::super::super::Foundation::BSTR, bstrinitstring: &super::super::super::Foundation::BSTR, bstrcontainerid: &super::super::super::Foundation::BSTR, bstrresourcepath: &super::super::super::Foundation::BSTR, nlifetime: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn RegisterRunningDevice(&self, bstrxmldesc: &super::super::super::Foundation::BSTR, punkdevicecontrol: &::core::option::Option<::windows::core::IUnknown>, bstrinitstring: &super::super::super::Foundation::BSTR, bstrresourcepath: &super::super::super::Foundation::BSTR, nlifetime: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn RegisterDeviceProvider(&self, bstrprovidername: &super::super::super::Foundation::BSTR, bstrprogidproviderclass: &super::super::super::Foundation::BSTR, bstrinitstring: &super::super::super::Foundation::BSTR, bstrcontainerid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetUniqueDeviceName(&self, bstrdeviceidentifier: &super::super::super::Foundation::BSTR, bstrtemplateudn: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn UnregisterDevice(&self, bstrdeviceidentifier: &super::super::super::Foundation::BSTR, fpermanent: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn UnregisterDeviceProvider(&self, bstrprovidername: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IUPnPRegistrar {}
#[cfg(feature = "Win32_Foundation")]
impl IUPnPRegistrar_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPRegistrar_Impl, const OFFSET: isize>() -> IUPnPRegistrar_Vtbl {
        unsafe extern "system" fn RegisterDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrprogiddevicecontrolclass: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcontainerid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32, pbstrdeviceidentifier: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterDevice(::core::mem::transmute(&bstrxmldesc), ::core::mem::transmute(&bstrprogiddevicecontrolclass), ::core::mem::transmute(&bstrinitstring), ::core::mem::transmute(&bstrcontainerid), ::core::mem::transmute(&bstrresourcepath), ::core::mem::transmute_copy(&nlifetime)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdeviceidentifier, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterRunningDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, punkdevicecontrol: *mut ::core::ffi::c_void, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32, pbstrdeviceidentifier: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterRunningDevice(::core::mem::transmute(&bstrxmldesc), ::core::mem::transmute(&punkdevicecontrol), ::core::mem::transmute(&bstrinitstring), ::core::mem::transmute(&bstrresourcepath), ::core::mem::transmute_copy(&nlifetime)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdeviceidentifier, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterDeviceProvider<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrprogidproviderclass: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcontainerid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterDeviceProvider(::core::mem::transmute(&bstrprovidername), ::core::mem::transmute(&bstrprogidproviderclass), ::core::mem::transmute(&bstrinitstring), ::core::mem::transmute(&bstrcontainerid)).into()
        }
        unsafe extern "system" fn GetUniqueDeviceName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrtemplateudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrudn: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUniqueDeviceName(::core::mem::transmute(&bstrdeviceidentifier), ::core::mem::transmute(&bstrtemplateudn)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrudn, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fpermanent: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterDevice(::core::mem::transmute(&bstrdeviceidentifier), ::core::mem::transmute_copy(&fpermanent)).into()
        }
        unsafe extern "system" fn UnregisterDeviceProvider<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterDeviceProvider(::core::mem::transmute(&bstrprovidername)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterDevice: RegisterDevice::<Identity, Impl, OFFSET>,
            RegisterRunningDevice: RegisterRunningDevice::<Identity, Impl, OFFSET>,
            RegisterDeviceProvider: RegisterDeviceProvider::<Identity, Impl, OFFSET>,
            GetUniqueDeviceName: GetUniqueDeviceName::<Identity, Impl, OFFSET>,
            UnregisterDevice: UnregisterDevice::<Identity, Impl, OFFSET>,
            UnregisterDeviceProvider: UnregisterDeviceProvider::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPRegistrar as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUPnPRemoteEndpointInfo_Impl: Sized {
    fn GetDwordValue(&self, bstrvaluename: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<u32>;
    fn GetStringValue(&self, bstrvaluename: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetGuidValue(&self, bstrvaluename: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IUPnPRemoteEndpointInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IUPnPRemoteEndpointInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPRemoteEndpointInfo_Impl, const OFFSET: isize>() -> IUPnPRemoteEndpointInfo_Vtbl {
        unsafe extern "system" fn GetDwordValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPRemoteEndpointInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvaluename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdwvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDwordValue(::core::mem::transmute(&bstrvaluename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPRemoteEndpointInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvaluename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStringValue(::core::mem::transmute(&bstrvaluename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGuidValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPRemoteEndpointInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvaluename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pguidvalue: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGuidValue(::core::mem::transmute(&bstrvaluename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDwordValue: GetDwordValue::<Identity, Impl, OFFSET>,
            GetStringValue: GetStringValue::<Identity, Impl, OFFSET>,
            GetGuidValue: GetGuidValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPRemoteEndpointInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUPnPReregistrar_Impl: Sized {
    fn ReregisterDevice(&self, bstrdeviceidentifier: &super::super::super::Foundation::BSTR, bstrxmldesc: &super::super::super::Foundation::BSTR, bstrprogiddevicecontrolclass: &super::super::super::Foundation::BSTR, bstrinitstring: &super::super::super::Foundation::BSTR, bstrcontainerid: &super::super::super::Foundation::BSTR, bstrresourcepath: &super::super::super::Foundation::BSTR, nlifetime: i32) -> ::windows::core::Result<()>;
    fn ReregisterRunningDevice(&self, bstrdeviceidentifier: &super::super::super::Foundation::BSTR, bstrxmldesc: &super::super::super::Foundation::BSTR, punkdevicecontrol: &::core::option::Option<::windows::core::IUnknown>, bstrinitstring: &super::super::super::Foundation::BSTR, bstrresourcepath: &super::super::super::Foundation::BSTR, nlifetime: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IUPnPReregistrar {}
#[cfg(feature = "Win32_Foundation")]
impl IUPnPReregistrar_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPReregistrar_Impl, const OFFSET: isize>() -> IUPnPReregistrar_Vtbl {
        unsafe extern "system" fn ReregisterDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPReregistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrprogiddevicecontrolclass: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcontainerid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReregisterDevice(::core::mem::transmute(&bstrdeviceidentifier), ::core::mem::transmute(&bstrxmldesc), ::core::mem::transmute(&bstrprogiddevicecontrolclass), ::core::mem::transmute(&bstrinitstring), ::core::mem::transmute(&bstrcontainerid), ::core::mem::transmute(&bstrresourcepath), ::core::mem::transmute_copy(&nlifetime)).into()
        }
        unsafe extern "system" fn ReregisterRunningDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPReregistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, punkdevicecontrol: *mut ::core::ffi::c_void, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReregisterRunningDevice(::core::mem::transmute(&bstrdeviceidentifier), ::core::mem::transmute(&bstrxmldesc), ::core::mem::transmute(&punkdevicecontrol), ::core::mem::transmute(&bstrinitstring), ::core::mem::transmute(&bstrresourcepath), ::core::mem::transmute_copy(&nlifetime)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ReregisterDevice: ReregisterDevice::<Identity, Impl, OFFSET>,
            ReregisterRunningDevice: ReregisterRunningDevice::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPReregistrar as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUPnPService_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn QueryStateVariable(&self, bstrvariablename: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn InvokeAction(&self, bstractionname: &super::super::super::Foundation::BSTR, vinactionargs: &super::super::super::System::Com::VARIANT, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ServiceTypeIdentifier(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn AddCallback(&self, punkcallback: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn LastTransportStatus(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IUPnPService {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUPnPService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPService_Impl, const OFFSET: isize>() -> IUPnPService_Vtbl {
        unsafe extern "system" fn QueryStateVariable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvariablename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryStateVariable(::core::mem::transmute(&bstrvariablename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeAction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstractionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vinactionargs: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InvokeAction(::core::mem::transmute(&bstractionname), ::core::mem::transmute(&vinactionargs), ::core::mem::transmute_copy(&pvoutactionargs), ::core::mem::transmute_copy(&pvretval)).into()
        }
        unsafe extern "system" fn ServiceTypeIdentifier<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceTypeIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddCallback(::core::mem::transmute(&punkcallback)).into()
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastTransportStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastTransportStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            QueryStateVariable: QueryStateVariable::<Identity, Impl, OFFSET>,
            InvokeAction: InvokeAction::<Identity, Impl, OFFSET>,
            ServiceTypeIdentifier: ServiceTypeIdentifier::<Identity, Impl, OFFSET>,
            AddCallback: AddCallback::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            LastTransportStatus: LastTransportStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPService as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUPnPServiceAsync_Impl: Sized {
    fn BeginInvokeAction(&self, bstractionname: &super::super::super::Foundation::BSTR, vinactionargs: &super::super::super::System::Com::VARIANT, pasyncresult: &::core::option::Option<IUPnPAsyncResult>) -> ::windows::core::Result<u64>;
    fn EndInvokeAction(&self, ullrequestid: u64, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn BeginQueryStateVariable(&self, bstrvariablename: &super::super::super::Foundation::BSTR, pasyncresult: &::core::option::Option<IUPnPAsyncResult>) -> ::windows::core::Result<u64>;
    fn EndQueryStateVariable(&self, ullrequestid: u64, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn BeginSubscribeToEvents(&self, punkcallback: &::core::option::Option<::windows::core::IUnknown>, pasyncresult: &::core::option::Option<IUPnPAsyncResult>) -> ::windows::core::Result<u64>;
    fn EndSubscribeToEvents(&self, ullrequestid: u64) -> ::windows::core::Result<()>;
    fn BeginSCPDDownload(&self, pasyncresult: &::core::option::Option<IUPnPAsyncResult>) -> ::windows::core::Result<u64>;
    fn EndSCPDDownload(&self, ullrequestid: u64) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn CancelAsyncOperation(&self, ullrequestid: u64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IUPnPServiceAsync {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUPnPServiceAsync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>() -> IUPnPServiceAsync_Vtbl {
        unsafe extern "system" fn BeginInvokeAction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstractionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vinactionargs: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginInvokeAction(::core::mem::transmute(&bstractionname), ::core::mem::transmute(&vinactionargs), ::core::mem::transmute(&pasyncresult)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullrequestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndInvokeAction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndInvokeAction(::core::mem::transmute_copy(&ullrequestid), ::core::mem::transmute_copy(&pvoutactionargs), ::core::mem::transmute_copy(&pvretval)).into()
        }
        unsafe extern "system" fn BeginQueryStateVariable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvariablename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginQueryStateVariable(::core::mem::transmute(&bstrvariablename), ::core::mem::transmute(&pasyncresult)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullrequestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndQueryStateVariable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndQueryStateVariable(::core::mem::transmute_copy(&ullrequestid), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn BeginSubscribeToEvents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkcallback: *mut ::core::ffi::c_void, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginSubscribeToEvents(::core::mem::transmute(&punkcallback), ::core::mem::transmute(&pasyncresult)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullrequestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSubscribeToEvents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndSubscribeToEvents(::core::mem::transmute_copy(&ullrequestid)).into()
        }
        unsafe extern "system" fn BeginSCPDDownload<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginSCPDDownload(::core::mem::transmute(&pasyncresult)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullrequestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSCPDDownload<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64, pbstrscpddoc: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndSCPDDownload(::core::mem::transmute_copy(&ullrequestid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrscpddoc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelAsyncOperation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelAsyncOperation(::core::mem::transmute_copy(&ullrequestid)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            BeginInvokeAction: BeginInvokeAction::<Identity, Impl, OFFSET>,
            EndInvokeAction: EndInvokeAction::<Identity, Impl, OFFSET>,
            BeginQueryStateVariable: BeginQueryStateVariable::<Identity, Impl, OFFSET>,
            EndQueryStateVariable: EndQueryStateVariable::<Identity, Impl, OFFSET>,
            BeginSubscribeToEvents: BeginSubscribeToEvents::<Identity, Impl, OFFSET>,
            EndSubscribeToEvents: EndSubscribeToEvents::<Identity, Impl, OFFSET>,
            BeginSCPDDownload: BeginSCPDDownload::<Identity, Impl, OFFSET>,
            EndSCPDDownload: EndSCPDDownload::<Identity, Impl, OFFSET>,
            CancelAsyncOperation: CancelAsyncOperation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPServiceAsync as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUPnPServiceCallback_Impl: Sized {
    fn StateVariableChanged(&self, pus: &::core::option::Option<IUPnPService>, pcwszstatevarname: &::windows::core::PCWSTR, vavalue: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ServiceInstanceDied(&self, pus: &::core::option::Option<IUPnPService>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IUPnPServiceCallback {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUPnPServiceCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceCallback_Impl, const OFFSET: isize>() -> IUPnPServiceCallback_Vtbl {
        unsafe extern "system" fn StateVariableChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pus: *mut ::core::ffi::c_void, pcwszstatevarname: ::windows::core::PCWSTR, vavalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StateVariableChanged(::core::mem::transmute(&pus), ::core::mem::transmute(&pcwszstatevarname), ::core::mem::transmute(&vavalue)).into()
        }
        unsafe extern "system" fn ServiceInstanceDied<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pus: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ServiceInstanceDied(::core::mem::transmute(&pus)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            StateVariableChanged: StateVariableChanged::<Identity, Impl, OFFSET>,
            ServiceInstanceDied: ServiceInstanceDied::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPServiceCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUPnPServiceDocumentAccess_Impl: Sized {
    fn GetDocumentURL(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetDocument(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IUPnPServiceDocumentAccess {}
#[cfg(feature = "Win32_Foundation")]
impl IUPnPServiceDocumentAccess_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceDocumentAccess_Impl, const OFFSET: isize>() -> IUPnPServiceDocumentAccess_Vtbl {
        unsafe extern "system" fn GetDocumentURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceDocumentAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdocurl: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocumentURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdocurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocument<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceDocumentAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdoc: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocument() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdoc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDocumentURL: GetDocumentURL::<Identity, Impl, OFFSET>,
            GetDocument: GetDocument::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPServiceDocumentAccess as ::windows::core::Interface>::IID
    }
}
pub trait IUPnPServiceEnumProperty_Impl: Sized {
    fn SetServiceEnumProperty(&self, dwmask: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IUPnPServiceEnumProperty {}
impl IUPnPServiceEnumProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceEnumProperty_Impl, const OFFSET: isize>() -> IUPnPServiceEnumProperty_Vtbl {
        unsafe extern "system" fn SetServiceEnumProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServiceEnumProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServiceEnumProperty(::core::mem::transmute_copy(&dwmask)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetServiceEnumProperty: SetServiceEnumProperty::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPServiceEnumProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUPnPServices_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn get_Item(&self, bstrserviceid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IUPnPService>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IUPnPServices {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUPnPServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServices_Impl, const OFFSET: isize>() -> IUPnPServices_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUPnPServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrserviceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&bstrserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPServices as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
