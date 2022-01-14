pub trait IUPnPAddressFamilyControl_Impl: Sized {
    fn SetAddressFamily(&mut self, dwflags: i32) -> ::windows::core::Result<()>;
    fn GetAddressFamily(&mut self) -> ::windows::core::Result<i32>;
}
impl IUPnPAddressFamilyControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPAddressFamilyControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPAddressFamilyControl_Vtbl {
        unsafe extern "system" fn SetAddressFamily<Impl: IUPnPAddressFamilyControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAddressFamily(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetAddressFamily<Impl: IUPnPAddressFamilyControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAddressFamily() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAddressFamily: SetAddressFamily::<Impl, IMPL_OFFSET>,
            GetAddressFamily: GetAddressFamily::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPAddressFamilyControl as ::windows::core::Interface>::IID
    }
}
pub trait IUPnPAsyncResult_Impl: Sized {
    fn AsyncOperationComplete(&mut self, ullrequestid: u64) -> ::windows::core::Result<()>;
}
impl IUPnPAsyncResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPAsyncResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPAsyncResult_Vtbl {
        unsafe extern "system" fn AsyncOperationComplete<Impl: IUPnPAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsyncOperationComplete(::core::mem::transmute_copy(&ullrequestid)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AsyncOperationComplete: AsyncOperationComplete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPAsyncResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUPnPDescriptionDocument_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn ReadyState(&mut self) -> ::windows::core::Result<i32>;
    fn Load(&mut self, bstrurl: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LoadAsync(&mut self, bstrurl: super::super::super::Foundation::BSTR, punkcallback: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn LoadResult(&mut self) -> ::windows::core::Result<i32>;
    fn Abort(&mut self) -> ::windows::core::Result<()>;
    fn RootDevice(&mut self) -> ::windows::core::Result<IUPnPDevice>;
    fn DeviceByUDN(&mut self, bstrudn: super::super::super::Foundation::BSTR) -> ::windows::core::Result<IUPnPDevice>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUPnPDescriptionDocument_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDescriptionDocument_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPDescriptionDocument_Vtbl {
        unsafe extern "system" fn ReadyState<Impl: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plreadystate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadyState() {
                ::core::result::Result::Ok(ok__) => {
                    *plreadystate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Load<Impl: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Load(::core::mem::transmute_copy(&bstrurl)).into()
        }
        unsafe extern "system" fn LoadAsync<Impl: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, punkcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadAsync(::core::mem::transmute_copy(&bstrurl), ::core::mem::transmute(&punkcallback)).into()
        }
        unsafe extern "system" fn LoadResult<Impl: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerror: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadResult() {
                ::core::result::Result::Ok(ok__) => {
                    *phrerror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Impl: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Abort().into()
        }
        unsafe extern "system" fn RootDevice<Impl: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppudrootdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RootDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *ppudrootdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceByUDN<Impl: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppuddevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceByUDN(::core::mem::transmute_copy(&bstrudn)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppuddevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ReadyState: ReadyState::<Impl, IMPL_OFFSET>,
            Load: Load::<Impl, IMPL_OFFSET>,
            LoadAsync: LoadAsync::<Impl, IMPL_OFFSET>,
            LoadResult: LoadResult::<Impl, IMPL_OFFSET>,
            Abort: Abort::<Impl, IMPL_OFFSET>,
            RootDevice: RootDevice::<Impl, IMPL_OFFSET>,
            DeviceByUDN: DeviceByUDN::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDescriptionDocument as ::windows::core::Interface>::IID
    }
}
pub trait IUPnPDescriptionDocumentCallback_Impl: Sized {
    fn LoadComplete(&mut self, hrloadresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IUPnPDescriptionDocumentCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDescriptionDocumentCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPDescriptionDocumentCallback_Vtbl {
        unsafe extern "system" fn LoadComplete<Impl: IUPnPDescriptionDocumentCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrloadresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadComplete(::core::mem::transmute_copy(&hrloadresult)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), LoadComplete: LoadComplete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDescriptionDocumentCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUPnPDevice_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn IsRootDevice(&mut self) -> ::windows::core::Result<i16>;
    fn RootDevice(&mut self) -> ::windows::core::Result<IUPnPDevice>;
    fn ParentDevice(&mut self) -> ::windows::core::Result<IUPnPDevice>;
    fn HasChildren(&mut self) -> ::windows::core::Result<i16>;
    fn Children(&mut self) -> ::windows::core::Result<IUPnPDevices>;
    fn UniqueDeviceName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn FriendlyName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Type(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn PresentationURL(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn ManufacturerName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn ManufacturerURL(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn ModelName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn ModelNumber(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn ModelURL(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn UPC(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SerialNumber(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn IconURL(&mut self, bstrencodingformat: super::super::super::Foundation::BSTR, lsizex: i32, lsizey: i32, lbitdepth: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Services(&mut self) -> ::windows::core::Result<IUPnPServices>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUPnPDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPDevice_Vtbl {
        unsafe extern "system" fn IsRootDevice<Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarb: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRootDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RootDevice<Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppudrootdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RootDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *ppudrootdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParentDevice<Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuddeviceparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParentDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *ppuddeviceparent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasChildren<Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarb: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasChildren() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppudchildren: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Children() {
                ::core::result::Result::Ok(ok__) => {
                    *ppudchildren = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UniqueDeviceName<Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UniqueDeviceName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FriendlyName<Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresentationURL<Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresentationURL() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManufacturerName<Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManufacturerName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManufacturerURL<Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManufacturerURL() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelName<Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModelName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelNumber<Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModelNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelURL<Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModelURL() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UPC<Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UPC() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerialNumber<Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SerialNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IconURL<Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrencodingformat: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, lsizex: i32, lsizey: i32, lbitdepth: i32, pbstriconurl: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IconURL(::core::mem::transmute_copy(&bstrencodingformat), ::core::mem::transmute_copy(&lsizex), ::core::mem::transmute_copy(&lsizey), ::core::mem::transmute_copy(&lbitdepth)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstriconurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Services<Impl: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppusservices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Services() {
                ::core::result::Result::Ok(ok__) => {
                    *ppusservices = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IsRootDevice: IsRootDevice::<Impl, IMPL_OFFSET>,
            RootDevice: RootDevice::<Impl, IMPL_OFFSET>,
            ParentDevice: ParentDevice::<Impl, IMPL_OFFSET>,
            HasChildren: HasChildren::<Impl, IMPL_OFFSET>,
            Children: Children::<Impl, IMPL_OFFSET>,
            UniqueDeviceName: UniqueDeviceName::<Impl, IMPL_OFFSET>,
            FriendlyName: FriendlyName::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            PresentationURL: PresentationURL::<Impl, IMPL_OFFSET>,
            ManufacturerName: ManufacturerName::<Impl, IMPL_OFFSET>,
            ManufacturerURL: ManufacturerURL::<Impl, IMPL_OFFSET>,
            ModelName: ModelName::<Impl, IMPL_OFFSET>,
            ModelNumber: ModelNumber::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            ModelURL: ModelURL::<Impl, IMPL_OFFSET>,
            UPC: UPC::<Impl, IMPL_OFFSET>,
            SerialNumber: SerialNumber::<Impl, IMPL_OFFSET>,
            IconURL: IconURL::<Impl, IMPL_OFFSET>,
            Services: Services::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IUPnPDeviceControl_Impl: Sized {
    fn Initialize(&mut self, bstrxmldesc: super::super::super::Foundation::BSTR, bstrdeviceidentifier: super::super::super::Foundation::BSTR, bstrinitstring: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetServiceObject(&mut self, bstrudn: super::super::super::Foundation::BSTR, bstrserviceid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IUPnPDeviceControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDeviceControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPDeviceControl_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IUPnPDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&bstrxmldesc), ::core::mem::transmute_copy(&bstrdeviceidentifier), ::core::mem::transmute_copy(&bstrinitstring)).into()
        }
        unsafe extern "system" fn GetServiceObject<Impl: IUPnPDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrserviceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppdispservice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetServiceObject(::core::mem::transmute_copy(&bstrudn), ::core::mem::transmute_copy(&bstrserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdispservice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            GetServiceObject: GetServiceObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDeviceControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUPnPDeviceControlHttpHeaders_Impl: Sized {
    fn GetAdditionalResponseHeaders(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUPnPDeviceControlHttpHeaders_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDeviceControlHttpHeaders_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPDeviceControlHttpHeaders_Vtbl {
        unsafe extern "system" fn GetAdditionalResponseHeaders<Impl: IUPnPDeviceControlHttpHeaders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhttpresponseheaders: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAdditionalResponseHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrhttpresponseheaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAdditionalResponseHeaders: GetAdditionalResponseHeaders::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDeviceControlHttpHeaders as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUPnPDeviceDocumentAccess_Impl: Sized {
    fn GetDocumentURL(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUPnPDeviceDocumentAccess_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDeviceDocumentAccess_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPDeviceDocumentAccess_Vtbl {
        unsafe extern "system" fn GetDocumentURL<Impl: IUPnPDeviceDocumentAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdocument: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentURL() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdocument = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetDocumentURL: GetDocumentURL::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDeviceDocumentAccess as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUPnPDeviceDocumentAccessEx_Impl: Sized {
    fn GetDocument(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUPnPDeviceDocumentAccessEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDeviceDocumentAccessEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPDeviceDocumentAccessEx_Vtbl {
        unsafe extern "system" fn GetDocument<Impl: IUPnPDeviceDocumentAccessEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdocument: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocument() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdocument = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetDocument: GetDocument::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDeviceDocumentAccessEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUPnPDeviceFinder_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn FindByType(&mut self, bstrtypeuri: super::super::super::Foundation::BSTR, dwflags: u32) -> ::windows::core::Result<IUPnPDevices>;
    fn CreateAsyncFind(&mut self, bstrtypeuri: super::super::super::Foundation::BSTR, dwflags: u32, punkdevicefindercallback: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<i32>;
    fn StartAsyncFind(&mut self, lfinddata: i32) -> ::windows::core::Result<()>;
    fn CancelAsyncFind(&mut self, lfinddata: i32) -> ::windows::core::Result<()>;
    fn FindByUDN(&mut self, bstrudn: super::super::super::Foundation::BSTR) -> ::windows::core::Result<IUPnPDevice>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUPnPDeviceFinder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDeviceFinder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPDeviceFinder_Vtbl {
        unsafe extern "system" fn FindByType<Impl: IUPnPDeviceFinder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtypeuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dwflags: u32, pdevices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindByType(::core::mem::transmute_copy(&bstrtypeuri), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdevices = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAsyncFind<Impl: IUPnPDeviceFinder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtypeuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dwflags: u32, punkdevicefindercallback: *mut ::core::ffi::c_void, plfinddata: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAsyncFind(::core::mem::transmute_copy(&bstrtypeuri), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&punkdevicefindercallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *plfinddata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAsyncFind<Impl: IUPnPDeviceFinder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartAsyncFind(::core::mem::transmute_copy(&lfinddata)).into()
        }
        unsafe extern "system" fn CancelAsyncFind<Impl: IUPnPDeviceFinder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelAsyncFind(::core::mem::transmute_copy(&lfinddata)).into()
        }
        unsafe extern "system" fn FindByUDN<Impl: IUPnPDeviceFinder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindByUDN(::core::mem::transmute_copy(&bstrudn)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            FindByType: FindByType::<Impl, IMPL_OFFSET>,
            CreateAsyncFind: CreateAsyncFind::<Impl, IMPL_OFFSET>,
            StartAsyncFind: StartAsyncFind::<Impl, IMPL_OFFSET>,
            CancelAsyncFind: CancelAsyncFind::<Impl, IMPL_OFFSET>,
            FindByUDN: FindByUDN::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDeviceFinder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDeviceFinderAddCallbackWithInterface_Impl: Sized {
    fn DeviceAddedWithInterface(&mut self, lfinddata: i32, pdevice: ::core::option::Option<IUPnPDevice>, pguidinterface: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDeviceFinderAddCallbackWithInterface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDeviceFinderAddCallbackWithInterface_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPDeviceFinderAddCallbackWithInterface_Vtbl {
        unsafe extern "system" fn DeviceAddedWithInterface<Impl: IUPnPDeviceFinderAddCallbackWithInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32, pdevice: ::windows::core::RawPtr, pguidinterface: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceAddedWithInterface(::core::mem::transmute_copy(&lfinddata), ::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&pguidinterface)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), DeviceAddedWithInterface: DeviceAddedWithInterface::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDeviceFinderAddCallbackWithInterface as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IUPnPDeviceFinderCallback_Impl: Sized {
    fn DeviceAdded(&mut self, lfinddata: i32, pdevice: ::core::option::Option<IUPnPDevice>) -> ::windows::core::Result<()>;
    fn DeviceRemoved(&mut self, lfinddata: i32, bstrudn: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SearchComplete(&mut self, lfinddata: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IUPnPDeviceFinderCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDeviceFinderCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPDeviceFinderCallback_Vtbl {
        unsafe extern "system" fn DeviceAdded<Impl: IUPnPDeviceFinderCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32, pdevice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceAdded(::core::mem::transmute_copy(&lfinddata), ::core::mem::transmute(&pdevice)).into()
        }
        unsafe extern "system" fn DeviceRemoved<Impl: IUPnPDeviceFinderCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceRemoved(::core::mem::transmute_copy(&lfinddata), ::core::mem::transmute_copy(&bstrudn)).into()
        }
        unsafe extern "system" fn SearchComplete<Impl: IUPnPDeviceFinderCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SearchComplete(::core::mem::transmute_copy(&lfinddata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            DeviceAdded: DeviceAdded::<Impl, IMPL_OFFSET>,
            DeviceRemoved: DeviceRemoved::<Impl, IMPL_OFFSET>,
            SearchComplete: SearchComplete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDeviceFinderCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUPnPDeviceProvider_Impl: Sized {
    fn Start(&mut self, bstrinitstring: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUPnPDeviceProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDeviceProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPDeviceProvider_Vtbl {
        unsafe extern "system" fn Start<Impl: IUPnPDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start(::core::mem::transmute_copy(&bstrinitstring)).into()
        }
        unsafe extern "system" fn Stop<Impl: IUPnPDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Start: Start::<Impl, IMPL_OFFSET>, Stop: Stop::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDeviceProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUPnPDevices_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, bstrudn: super::super::super::Foundation::BSTR) -> ::windows::core::Result<IUPnPDevice>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUPnPDevices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDevices_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPDevices_Vtbl {
        unsafe extern "system" fn Count<Impl: IUPnPDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IUPnPDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IUPnPDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&bstrudn)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPDevices as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUPnPEventSink_Impl: Sized {
    fn OnStateChanged(&mut self, cchanges: u32, rgdispidchanges: *const i32) -> ::windows::core::Result<()>;
    fn OnStateChangedSafe(&mut self, varsadispidchanges: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUPnPEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPEventSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPEventSink_Vtbl {
        unsafe extern "system" fn OnStateChanged<Impl: IUPnPEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchanges: u32, rgdispidchanges: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStateChanged(::core::mem::transmute_copy(&cchanges), ::core::mem::transmute_copy(&rgdispidchanges)).into()
        }
        unsafe extern "system" fn OnStateChangedSafe<Impl: IUPnPEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsadispidchanges: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStateChangedSafe(::core::mem::transmute_copy(&varsadispidchanges)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnStateChanged: OnStateChanged::<Impl, IMPL_OFFSET>,
            OnStateChangedSafe: OnStateChangedSafe::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPEventSink as ::windows::core::Interface>::IID
    }
}
pub trait IUPnPEventSource_Impl: Sized {
    fn Advise(&mut self, pessubscriber: ::core::option::Option<IUPnPEventSink>) -> ::windows::core::Result<()>;
    fn Unadvise(&mut self, pessubscriber: ::core::option::Option<IUPnPEventSink>) -> ::windows::core::Result<()>;
}
impl IUPnPEventSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPEventSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPEventSource_Vtbl {
        unsafe extern "system" fn Advise<Impl: IUPnPEventSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pessubscriber: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Advise(::core::mem::transmute(&pessubscriber)).into()
        }
        unsafe extern "system" fn Unadvise<Impl: IUPnPEventSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pessubscriber: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unadvise(::core::mem::transmute(&pessubscriber)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Advise: Advise::<Impl, IMPL_OFFSET>,
            Unadvise: Unadvise::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPEventSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUPnPHttpHeaderControl_Impl: Sized {
    fn AddRequestHeaders(&mut self, bstrhttpheaders: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUPnPHttpHeaderControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPHttpHeaderControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPHttpHeaderControl_Vtbl {
        unsafe extern "system" fn AddRequestHeaders<Impl: IUPnPHttpHeaderControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhttpheaders: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRequestHeaders(::core::mem::transmute_copy(&bstrhttpheaders)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AddRequestHeaders: AddRequestHeaders::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPHttpHeaderControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUPnPRegistrar_Impl: Sized {
    fn RegisterDevice(&mut self, bstrxmldesc: super::super::super::Foundation::BSTR, bstrprogiddevicecontrolclass: super::super::super::Foundation::BSTR, bstrinitstring: super::super::super::Foundation::BSTR, bstrcontainerid: super::super::super::Foundation::BSTR, bstrresourcepath: super::super::super::Foundation::BSTR, nlifetime: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn RegisterRunningDevice(&mut self, bstrxmldesc: super::super::super::Foundation::BSTR, punkdevicecontrol: ::core::option::Option<::windows::core::IUnknown>, bstrinitstring: super::super::super::Foundation::BSTR, bstrresourcepath: super::super::super::Foundation::BSTR, nlifetime: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn RegisterDeviceProvider(&mut self, bstrprovidername: super::super::super::Foundation::BSTR, bstrprogidproviderclass: super::super::super::Foundation::BSTR, bstrinitstring: super::super::super::Foundation::BSTR, bstrcontainerid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetUniqueDeviceName(&mut self, bstrdeviceidentifier: super::super::super::Foundation::BSTR, bstrtemplateudn: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn UnregisterDevice(&mut self, bstrdeviceidentifier: super::super::super::Foundation::BSTR, fpermanent: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn UnregisterDeviceProvider(&mut self, bstrprovidername: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUPnPRegistrar_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPRegistrar_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPRegistrar_Vtbl {
        unsafe extern "system" fn RegisterDevice<Impl: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrprogiddevicecontrolclass: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcontainerid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32, pbstrdeviceidentifier: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterDevice(::core::mem::transmute_copy(&bstrxmldesc), ::core::mem::transmute_copy(&bstrprogiddevicecontrolclass), ::core::mem::transmute_copy(&bstrinitstring), ::core::mem::transmute_copy(&bstrcontainerid), ::core::mem::transmute_copy(&bstrresourcepath), ::core::mem::transmute_copy(&nlifetime)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdeviceidentifier = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterRunningDevice<Impl: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, punkdevicecontrol: *mut ::core::ffi::c_void, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32, pbstrdeviceidentifier: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterRunningDevice(::core::mem::transmute_copy(&bstrxmldesc), ::core::mem::transmute(&punkdevicecontrol), ::core::mem::transmute_copy(&bstrinitstring), ::core::mem::transmute_copy(&bstrresourcepath), ::core::mem::transmute_copy(&nlifetime)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdeviceidentifier = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterDeviceProvider<Impl: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrprogidproviderclass: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcontainerid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterDeviceProvider(::core::mem::transmute_copy(&bstrprovidername), ::core::mem::transmute_copy(&bstrprogidproviderclass), ::core::mem::transmute_copy(&bstrinitstring), ::core::mem::transmute_copy(&bstrcontainerid)).into()
        }
        unsafe extern "system" fn GetUniqueDeviceName<Impl: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrtemplateudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrudn: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUniqueDeviceName(::core::mem::transmute_copy(&bstrdeviceidentifier), ::core::mem::transmute_copy(&bstrtemplateudn)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrudn = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDevice<Impl: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fpermanent: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterDevice(::core::mem::transmute_copy(&bstrdeviceidentifier), ::core::mem::transmute_copy(&fpermanent)).into()
        }
        unsafe extern "system" fn UnregisterDeviceProvider<Impl: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterDeviceProvider(::core::mem::transmute_copy(&bstrprovidername)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterDevice: RegisterDevice::<Impl, IMPL_OFFSET>,
            RegisterRunningDevice: RegisterRunningDevice::<Impl, IMPL_OFFSET>,
            RegisterDeviceProvider: RegisterDeviceProvider::<Impl, IMPL_OFFSET>,
            GetUniqueDeviceName: GetUniqueDeviceName::<Impl, IMPL_OFFSET>,
            UnregisterDevice: UnregisterDevice::<Impl, IMPL_OFFSET>,
            UnregisterDeviceProvider: UnregisterDeviceProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPRegistrar as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUPnPRemoteEndpointInfo_Impl: Sized {
    fn GetDwordValue(&mut self, bstrvaluename: super::super::super::Foundation::BSTR) -> ::windows::core::Result<u32>;
    fn GetStringValue(&mut self, bstrvaluename: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetGuidValue(&mut self, bstrvaluename: super::super::super::Foundation::BSTR) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUPnPRemoteEndpointInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPRemoteEndpointInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPRemoteEndpointInfo_Vtbl {
        unsafe extern "system" fn GetDwordValue<Impl: IUPnPRemoteEndpointInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvaluename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdwvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDwordValue(::core::mem::transmute_copy(&bstrvaluename)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringValue<Impl: IUPnPRemoteEndpointInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvaluename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringValue(::core::mem::transmute_copy(&bstrvaluename)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGuidValue<Impl: IUPnPRemoteEndpointInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvaluename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pguidvalue: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGuidValue(::core::mem::transmute_copy(&bstrvaluename)) {
                ::core::result::Result::Ok(ok__) => {
                    *pguidvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDwordValue: GetDwordValue::<Impl, IMPL_OFFSET>,
            GetStringValue: GetStringValue::<Impl, IMPL_OFFSET>,
            GetGuidValue: GetGuidValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPRemoteEndpointInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUPnPReregistrar_Impl: Sized {
    fn ReregisterDevice(&mut self, bstrdeviceidentifier: super::super::super::Foundation::BSTR, bstrxmldesc: super::super::super::Foundation::BSTR, bstrprogiddevicecontrolclass: super::super::super::Foundation::BSTR, bstrinitstring: super::super::super::Foundation::BSTR, bstrcontainerid: super::super::super::Foundation::BSTR, bstrresourcepath: super::super::super::Foundation::BSTR, nlifetime: i32) -> ::windows::core::Result<()>;
    fn ReregisterRunningDevice(&mut self, bstrdeviceidentifier: super::super::super::Foundation::BSTR, bstrxmldesc: super::super::super::Foundation::BSTR, punkdevicecontrol: ::core::option::Option<::windows::core::IUnknown>, bstrinitstring: super::super::super::Foundation::BSTR, bstrresourcepath: super::super::super::Foundation::BSTR, nlifetime: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUPnPReregistrar_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPReregistrar_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPReregistrar_Vtbl {
        unsafe extern "system" fn ReregisterDevice<Impl: IUPnPReregistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrprogiddevicecontrolclass: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcontainerid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReregisterDevice(::core::mem::transmute_copy(&bstrdeviceidentifier), ::core::mem::transmute_copy(&bstrxmldesc), ::core::mem::transmute_copy(&bstrprogiddevicecontrolclass), ::core::mem::transmute_copy(&bstrinitstring), ::core::mem::transmute_copy(&bstrcontainerid), ::core::mem::transmute_copy(&bstrresourcepath), ::core::mem::transmute_copy(&nlifetime)).into()
        }
        unsafe extern "system" fn ReregisterRunningDevice<Impl: IUPnPReregistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, punkdevicecontrol: *mut ::core::ffi::c_void, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReregisterRunningDevice(::core::mem::transmute_copy(&bstrdeviceidentifier), ::core::mem::transmute_copy(&bstrxmldesc), ::core::mem::transmute(&punkdevicecontrol), ::core::mem::transmute_copy(&bstrinitstring), ::core::mem::transmute_copy(&bstrresourcepath), ::core::mem::transmute_copy(&nlifetime)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ReregisterDevice: ReregisterDevice::<Impl, IMPL_OFFSET>,
            ReregisterRunningDevice: ReregisterRunningDevice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPReregistrar as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUPnPService_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn QueryStateVariable(&mut self, bstrvariablename: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn InvokeAction(&mut self, bstractionname: super::super::super::Foundation::BSTR, vinactionargs: super::super::super::System::Com::VARIANT, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ServiceTypeIdentifier(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn AddCallback(&mut self, punkcallback: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Id(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn LastTransportStatus(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUPnPService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPService_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPService_Vtbl {
        unsafe extern "system" fn QueryStateVariable<Impl: IUPnPService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvariablename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryStateVariable(::core::mem::transmute_copy(&bstrvariablename)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeAction<Impl: IUPnPService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstractionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vinactionargs: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvokeAction(::core::mem::transmute_copy(&bstractionname), ::core::mem::transmute_copy(&vinactionargs), ::core::mem::transmute_copy(&pvoutactionargs), ::core::mem::transmute_copy(&pvretval)).into()
        }
        unsafe extern "system" fn ServiceTypeIdentifier<Impl: IUPnPService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceTypeIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddCallback<Impl: IUPnPService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddCallback(::core::mem::transmute(&punkcallback)).into()
        }
        unsafe extern "system" fn Id<Impl: IUPnPService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastTransportStatus<Impl: IUPnPService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastTransportStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *plvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            QueryStateVariable: QueryStateVariable::<Impl, IMPL_OFFSET>,
            InvokeAction: InvokeAction::<Impl, IMPL_OFFSET>,
            ServiceTypeIdentifier: ServiceTypeIdentifier::<Impl, IMPL_OFFSET>,
            AddCallback: AddCallback::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            LastTransportStatus: LastTransportStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUPnPServiceAsync_Impl: Sized {
    fn BeginInvokeAction(&mut self, bstractionname: super::super::super::Foundation::BSTR, vinactionargs: super::super::super::System::Com::VARIANT, pasyncresult: ::core::option::Option<IUPnPAsyncResult>) -> ::windows::core::Result<u64>;
    fn EndInvokeAction(&mut self, ullrequestid: u64, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn BeginQueryStateVariable(&mut self, bstrvariablename: super::super::super::Foundation::BSTR, pasyncresult: ::core::option::Option<IUPnPAsyncResult>) -> ::windows::core::Result<u64>;
    fn EndQueryStateVariable(&mut self, ullrequestid: u64, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn BeginSubscribeToEvents(&mut self, punkcallback: ::core::option::Option<::windows::core::IUnknown>, pasyncresult: ::core::option::Option<IUPnPAsyncResult>) -> ::windows::core::Result<u64>;
    fn EndSubscribeToEvents(&mut self, ullrequestid: u64) -> ::windows::core::Result<()>;
    fn BeginSCPDDownload(&mut self, pasyncresult: ::core::option::Option<IUPnPAsyncResult>) -> ::windows::core::Result<u64>;
    fn EndSCPDDownload(&mut self, ullrequestid: u64) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn CancelAsyncOperation(&mut self, ullrequestid: u64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUPnPServiceAsync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPServiceAsync_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPServiceAsync_Vtbl {
        unsafe extern "system" fn BeginInvokeAction<Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstractionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vinactionargs: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pasyncresult: ::windows::core::RawPtr, pullrequestid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginInvokeAction(::core::mem::transmute_copy(&bstractionname), ::core::mem::transmute_copy(&vinactionargs), ::core::mem::transmute(&pasyncresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *pullrequestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndInvokeAction<Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndInvokeAction(::core::mem::transmute_copy(&ullrequestid), ::core::mem::transmute_copy(&pvoutactionargs), ::core::mem::transmute_copy(&pvretval)).into()
        }
        unsafe extern "system" fn BeginQueryStateVariable<Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvariablename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pasyncresult: ::windows::core::RawPtr, pullrequestid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginQueryStateVariable(::core::mem::transmute_copy(&bstrvariablename), ::core::mem::transmute(&pasyncresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *pullrequestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndQueryStateVariable<Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndQueryStateVariable(::core::mem::transmute_copy(&ullrequestid), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn BeginSubscribeToEvents<Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkcallback: *mut ::core::ffi::c_void, pasyncresult: ::windows::core::RawPtr, pullrequestid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginSubscribeToEvents(::core::mem::transmute(&punkcallback), ::core::mem::transmute(&pasyncresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *pullrequestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSubscribeToEvents<Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndSubscribeToEvents(::core::mem::transmute_copy(&ullrequestid)).into()
        }
        unsafe extern "system" fn BeginSCPDDownload<Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasyncresult: ::windows::core::RawPtr, pullrequestid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginSCPDDownload(::core::mem::transmute(&pasyncresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *pullrequestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSCPDDownload<Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64, pbstrscpddoc: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndSCPDDownload(::core::mem::transmute_copy(&ullrequestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrscpddoc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelAsyncOperation<Impl: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelAsyncOperation(::core::mem::transmute_copy(&ullrequestid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BeginInvokeAction: BeginInvokeAction::<Impl, IMPL_OFFSET>,
            EndInvokeAction: EndInvokeAction::<Impl, IMPL_OFFSET>,
            BeginQueryStateVariable: BeginQueryStateVariable::<Impl, IMPL_OFFSET>,
            EndQueryStateVariable: EndQueryStateVariable::<Impl, IMPL_OFFSET>,
            BeginSubscribeToEvents: BeginSubscribeToEvents::<Impl, IMPL_OFFSET>,
            EndSubscribeToEvents: EndSubscribeToEvents::<Impl, IMPL_OFFSET>,
            BeginSCPDDownload: BeginSCPDDownload::<Impl, IMPL_OFFSET>,
            EndSCPDDownload: EndSCPDDownload::<Impl, IMPL_OFFSET>,
            CancelAsyncOperation: CancelAsyncOperation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPServiceAsync as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUPnPServiceCallback_Impl: Sized {
    fn StateVariableChanged(&mut self, pus: ::core::option::Option<IUPnPService>, pcwszstatevarname: super::super::super::Foundation::PWSTR, vavalue: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ServiceInstanceDied(&mut self, pus: ::core::option::Option<IUPnPService>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUPnPServiceCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPServiceCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPServiceCallback_Vtbl {
        unsafe extern "system" fn StateVariableChanged<Impl: IUPnPServiceCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pus: ::windows::core::RawPtr, pcwszstatevarname: super::super::super::Foundation::PWSTR, vavalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StateVariableChanged(::core::mem::transmute(&pus), ::core::mem::transmute_copy(&pcwszstatevarname), ::core::mem::transmute_copy(&vavalue)).into()
        }
        unsafe extern "system" fn ServiceInstanceDied<Impl: IUPnPServiceCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pus: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ServiceInstanceDied(::core::mem::transmute(&pus)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            StateVariableChanged: StateVariableChanged::<Impl, IMPL_OFFSET>,
            ServiceInstanceDied: ServiceInstanceDied::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPServiceCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUPnPServiceDocumentAccess_Impl: Sized {
    fn GetDocumentURL(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetDocument(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUPnPServiceDocumentAccess_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPServiceDocumentAccess_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPServiceDocumentAccess_Vtbl {
        unsafe extern "system" fn GetDocumentURL<Impl: IUPnPServiceDocumentAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdocurl: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentURL() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdocurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocument<Impl: IUPnPServiceDocumentAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdoc: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocument() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdoc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDocumentURL: GetDocumentURL::<Impl, IMPL_OFFSET>,
            GetDocument: GetDocument::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPServiceDocumentAccess as ::windows::core::Interface>::IID
    }
}
pub trait IUPnPServiceEnumProperty_Impl: Sized {
    fn SetServiceEnumProperty(&mut self, dwmask: u32) -> ::windows::core::Result<()>;
}
impl IUPnPServiceEnumProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPServiceEnumProperty_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPServiceEnumProperty_Vtbl {
        unsafe extern "system" fn SetServiceEnumProperty<Impl: IUPnPServiceEnumProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServiceEnumProperty(::core::mem::transmute_copy(&dwmask)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetServiceEnumProperty: SetServiceEnumProperty::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPServiceEnumProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUPnPServices_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, bstrserviceid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<IUPnPService>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUPnPServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPServices_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPServices_Vtbl {
        unsafe extern "system" fn Count<Impl: IUPnPServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IUPnPServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IUPnPServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrserviceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppservice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&bstrserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppservice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPServices as ::windows::core::Interface>::IID
    }
}
