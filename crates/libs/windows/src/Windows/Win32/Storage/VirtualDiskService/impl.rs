pub trait IEnumVdsObject_Impl: Sized {
    fn Next(&self, celt: u32, ppobjectarray: *mut ::core::option::Option<::windows::core::IUnknown>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumVdsObject>;
}
impl IEnumVdsObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumVdsObject_Impl, const OFFSET: isize>() -> IEnumVdsObject_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumVdsObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppobjectarray: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppobjectarray), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumVdsObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumVdsObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumVdsObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumVdsObject as ::windows::core::Interface>::IID
    }
}
pub trait IVdsAdmin_Impl: Sized {
    fn RegisterProvider(&self, providerid: &::windows::core::GUID, providerclsid: &::windows::core::GUID, pwszname: &::windows::core::PCWSTR, r#type: VDS_PROVIDER_TYPE, pwszmachinename: &::windows::core::PCWSTR, pwszversion: &::windows::core::PCWSTR, guidversionid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn UnregisterProvider(&self, providerid: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl IVdsAdmin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsAdmin_Impl, const OFFSET: isize>() -> IVdsAdmin_Vtbl {
        unsafe extern "system" fn RegisterProvider<Identity: ::windows::core::IUnknownImpl, Impl: IVdsAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, providerclsid: ::windows::core::GUID, pwszname: ::windows::core::PCWSTR, r#type: VDS_PROVIDER_TYPE, pwszmachinename: ::windows::core::PCWSTR, pwszversion: ::windows::core::PCWSTR, guidversionid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterProvider(::core::mem::transmute(&providerid), ::core::mem::transmute(&providerclsid), ::core::mem::transmute(&pwszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute(&pwszmachinename), ::core::mem::transmute(&pwszversion), ::core::mem::transmute(&guidversionid)).into()
        }
        unsafe extern "system" fn UnregisterProvider<Identity: ::windows::core::IUnknownImpl, Impl: IVdsAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterProvider(::core::mem::transmute(&providerid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterProvider: RegisterProvider::<Identity, Impl, OFFSET>,
            UnregisterProvider: UnregisterProvider::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsAdmin as ::windows::core::Interface>::IID
    }
}
pub trait IVdsAdviseSink_Impl: Sized {
    fn OnNotify(&self, lnumberofnotifications: i32, pnotificationarray: *const VDS_NOTIFICATION) -> ::windows::core::Result<()>;
}
impl IVdsAdviseSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsAdviseSink_Impl, const OFFSET: isize>() -> IVdsAdviseSink_Vtbl {
        unsafe extern "system" fn OnNotify<Identity: ::windows::core::IUnknownImpl, Impl: IVdsAdviseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnumberofnotifications: i32, pnotificationarray: *const VDS_NOTIFICATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnNotify(::core::mem::transmute_copy(&lnumberofnotifications), ::core::mem::transmute_copy(&pnotificationarray)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnNotify: OnNotify::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsAdviseSink as ::windows::core::Interface>::IID
    }
}
pub trait IVdsAsync_Impl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn Wait(&self, phrresult: *mut ::windows::core::HRESULT, pasyncout: *mut VDS_ASYNC_OUTPUT) -> ::windows::core::Result<()>;
    fn QueryStatus(&self, phrresult: *mut ::windows::core::HRESULT, pulpercentcompleted: *mut u32) -> ::windows::core::Result<()>;
}
impl IVdsAsync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsAsync_Impl, const OFFSET: isize>() -> IVdsAsync_Vtbl {
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IVdsAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn Wait<Identity: ::windows::core::IUnknownImpl, Impl: IVdsAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT, pasyncout: *mut VDS_ASYNC_OUTPUT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Wait(::core::mem::transmute_copy(&phrresult), ::core::mem::transmute_copy(&pasyncout)).into()
        }
        unsafe extern "system" fn QueryStatus<Identity: ::windows::core::IUnknownImpl, Impl: IVdsAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT, pulpercentcompleted: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryStatus(::core::mem::transmute_copy(&phrresult), ::core::mem::transmute_copy(&pulpercentcompleted)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Wait: Wait::<Identity, Impl, OFFSET>,
            QueryStatus: QueryStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsAsync as ::windows::core::Interface>::IID
    }
}
pub trait IVdsController_Impl: Sized {
    fn GetProperties(&self) -> ::windows::core::Result<VDS_CONTROLLER_PROP>;
    fn GetSubSystem(&self) -> ::windows::core::Result<IVdsSubSystem>;
    fn GetPortProperties(&self, sportnumber: i16) -> ::windows::core::Result<VDS_PORT_PROP>;
    fn FlushCache(&self) -> ::windows::core::Result<()>;
    fn InvalidateCache(&self) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn QueryAssociatedLuns(&self) -> ::windows::core::Result<IEnumVdsObject>;
    fn SetStatus(&self, status: VDS_CONTROLLER_STATUS) -> ::windows::core::Result<()>;
}
impl IVdsController_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsController_Impl, const OFFSET: isize>() -> IVdsController_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl, Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontrollerprop: *mut VDS_CONTROLLER_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pcontrollerprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubSystem<Identity: ::windows::core::IUnknownImpl, Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSubSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsubsystem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPortProperties<Identity: ::windows::core::IUnknownImpl, Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sportnumber: i16, pportprop: *mut VDS_PORT_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPortProperties(::core::mem::transmute_copy(&sportnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *pportprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushCache<Identity: ::windows::core::IUnknownImpl, Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FlushCache().into()
        }
        unsafe extern "system" fn InvalidateCache<Identity: ::windows::core::IUnknownImpl, Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InvalidateCache().into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn QueryAssociatedLuns<Identity: ::windows::core::IUnknownImpl, Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryAssociatedLuns() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_CONTROLLER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStatus(::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetSubSystem: GetSubSystem::<Identity, Impl, OFFSET>,
            GetPortProperties: GetPortProperties::<Identity, Impl, OFFSET>,
            FlushCache: FlushCache::<Identity, Impl, OFFSET>,
            InvalidateCache: InvalidateCache::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            QueryAssociatedLuns: QueryAssociatedLuns::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsController as ::windows::core::Interface>::IID
    }
}
pub trait IVdsControllerControllerPort_Impl: Sized {
    fn QueryControllerPorts(&self) -> ::windows::core::Result<IEnumVdsObject>;
}
impl IVdsControllerControllerPort_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsControllerControllerPort_Impl, const OFFSET: isize>() -> IVdsControllerControllerPort_Vtbl {
        unsafe extern "system" fn QueryControllerPorts<Identity: ::windows::core::IUnknownImpl, Impl: IVdsControllerControllerPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryControllerPorts() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), QueryControllerPorts: QueryControllerPorts::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsControllerControllerPort as ::windows::core::Interface>::IID
    }
}
pub trait IVdsControllerPort_Impl: Sized {
    fn GetProperties(&self) -> ::windows::core::Result<VDS_PORT_PROP>;
    fn GetController(&self) -> ::windows::core::Result<IVdsController>;
    fn QueryAssociatedLuns(&self) -> ::windows::core::Result<IEnumVdsObject>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn SetStatus(&self, status: VDS_PORT_STATUS) -> ::windows::core::Result<()>;
}
impl IVdsControllerPort_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsControllerPort_Impl, const OFFSET: isize>() -> IVdsControllerPort_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl, Impl: IVdsControllerPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportprop: *mut VDS_PORT_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pportprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetController<Identity: ::windows::core::IUnknownImpl, Impl: IVdsControllerPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontroller: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetController() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontroller = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAssociatedLuns<Identity: ::windows::core::IUnknownImpl, Impl: IVdsControllerPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryAssociatedLuns() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IVdsControllerPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IVdsControllerPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_PORT_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStatus(::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetController: GetController::<Identity, Impl, OFFSET>,
            QueryAssociatedLuns: QueryAssociatedLuns::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsControllerPort as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsDrive_Impl: Sized {
    fn GetProperties(&self) -> ::windows::core::Result<VDS_DRIVE_PROP>;
    fn GetSubSystem(&self) -> ::windows::core::Result<IVdsSubSystem>;
    fn QueryExtents(&self, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::Result<()>;
    fn SetFlags(&self, ulflags: u32) -> ::windows::core::Result<()>;
    fn ClearFlags(&self, ulflags: u32) -> ::windows::core::Result<()>;
    fn SetStatus(&self, status: VDS_DRIVE_STATUS) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsDrive_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsDrive_Impl, const OFFSET: isize>() -> IVdsDrive_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl, Impl: IVdsDrive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdriveprop: *mut VDS_DRIVE_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pdriveprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubSystem<Identity: ::windows::core::IUnknownImpl, Impl: IVdsDrive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSubSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsubsystem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryExtents<Identity: ::windows::core::IUnknownImpl, Impl: IVdsDrive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryExtents(::core::mem::transmute_copy(&ppextentarray), ::core::mem::transmute_copy(&plnumberofextents)).into()
        }
        unsafe extern "system" fn SetFlags<Identity: ::windows::core::IUnknownImpl, Impl: IVdsDrive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFlags(::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn ClearFlags<Identity: ::windows::core::IUnknownImpl, Impl: IVdsDrive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearFlags(::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IVdsDrive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_DRIVE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStatus(::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetSubSystem: GetSubSystem::<Identity, Impl, OFFSET>,
            QueryExtents: QueryExtents::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
            ClearFlags: ClearFlags::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsDrive as ::windows::core::Interface>::IID
    }
}
pub trait IVdsDrive2_Impl: Sized {
    fn GetProperties2(&self) -> ::windows::core::Result<VDS_DRIVE_PROP2>;
}
impl IVdsDrive2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsDrive2_Impl, const OFFSET: isize>() -> IVdsDrive2_Vtbl {
        unsafe extern "system" fn GetProperties2<Identity: ::windows::core::IUnknownImpl, Impl: IVdsDrive2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdriveprop2: *mut VDS_DRIVE_PROP2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperties2() {
                ::core::result::Result::Ok(ok__) => {
                    *pdriveprop2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetProperties2: GetProperties2::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsDrive2 as ::windows::core::Interface>::IID
    }
}
pub trait IVdsHwProvider_Impl: Sized {
    fn QuerySubSystems(&self) -> ::windows::core::Result<IEnumVdsObject>;
    fn Reenumerate(&self) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
}
impl IVdsHwProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProvider_Impl, const OFFSET: isize>() -> IVdsHwProvider_Vtbl {
        unsafe extern "system" fn QuerySubSystems<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QuerySubSystems() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reenumerate<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reenumerate().into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Refresh().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            QuerySubSystems: QuerySubSystems::<Identity, Impl, OFFSET>,
            Reenumerate: Reenumerate::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsHwProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsHwProviderPrivate_Impl: Sized {
    fn QueryIfCreatedLun(&self, pwszdevicepath: &::windows::core::PCWSTR, pvdsluninformation: *const VDS_LUN_INFORMATION) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsHwProviderPrivate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderPrivate_Impl, const OFFSET: isize>() -> IVdsHwProviderPrivate_Vtbl {
        unsafe extern "system" fn QueryIfCreatedLun<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderPrivate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicepath: ::windows::core::PCWSTR, pvdsluninformation: *const VDS_LUN_INFORMATION, plunid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryIfCreatedLun(::core::mem::transmute(&pwszdevicepath), ::core::mem::transmute_copy(&pvdsluninformation)) {
                ::core::result::Result::Ok(ok__) => {
                    *plunid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), QueryIfCreatedLun: QueryIfCreatedLun::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsHwProviderPrivate as ::windows::core::Interface>::IID
    }
}
pub trait IVdsHwProviderPrivateMpio_Impl: Sized {
    fn SetAllPathStatusesFromHbaPort(&self, hbaportprop: &VDS_HBAPORT_PROP, status: VDS_PATH_STATUS) -> ::windows::core::Result<()>;
}
impl IVdsHwProviderPrivateMpio_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderPrivateMpio_Impl, const OFFSET: isize>() -> IVdsHwProviderPrivateMpio_Vtbl {
        unsafe extern "system" fn SetAllPathStatusesFromHbaPort<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderPrivateMpio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hbaportprop: VDS_HBAPORT_PROP, status: VDS_PATH_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllPathStatusesFromHbaPort(::core::mem::transmute(&hbaportprop), ::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetAllPathStatusesFromHbaPort: SetAllPathStatusesFromHbaPort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsHwProviderPrivateMpio as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsHwProviderStoragePools_Impl: Sized {
    fn QueryStoragePools(&self, ulflags: u32, ullremainingfreespace: u64, ppoolattributes: *const VDS_POOL_ATTRIBUTES) -> ::windows::core::Result<IEnumVdsObject>;
    fn CreateLunInStoragePool(&self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, storagepoolid: &::windows::core::GUID, pwszunmaskinglist: &::windows::core::PCWSTR, phints2: *const VDS_HINTS2) -> ::windows::core::Result<IVdsAsync>;
    fn QueryMaxLunCreateSizeInStoragePool(&self, r#type: VDS_LUN_TYPE, storagepoolid: &::windows::core::GUID, phints2: *const VDS_HINTS2) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsHwProviderStoragePools_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderStoragePools_Impl, const OFFSET: isize>() -> IVdsHwProviderStoragePools_Vtbl {
        unsafe extern "system" fn QueryStoragePools<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderStoragePools_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, ullremainingfreespace: u64, ppoolattributes: *const VDS_POOL_ATTRIBUTES, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryStoragePools(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&ullremainingfreespace), ::core::mem::transmute_copy(&ppoolattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLunInStoragePool<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderStoragePools_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, storagepoolid: ::windows::core::GUID, pwszunmaskinglist: ::windows::core::PCWSTR, phints2: *const VDS_HINTS2, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateLunInStoragePool(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&ullsizeinbytes), ::core::mem::transmute(&storagepoolid), ::core::mem::transmute(&pwszunmaskinglist), ::core::mem::transmute_copy(&phints2)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryMaxLunCreateSizeInStoragePool<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderStoragePools_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, storagepoolid: ::windows::core::GUID, phints2: *const VDS_HINTS2, pullmaxlunsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryMaxLunCreateSizeInStoragePool(::core::mem::transmute_copy(&r#type), ::core::mem::transmute(&storagepoolid), ::core::mem::transmute_copy(&phints2)) {
                ::core::result::Result::Ok(ok__) => {
                    *pullmaxlunsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            QueryStoragePools: QueryStoragePools::<Identity, Impl, OFFSET>,
            CreateLunInStoragePool: CreateLunInStoragePool::<Identity, Impl, OFFSET>,
            QueryMaxLunCreateSizeInStoragePool: QueryMaxLunCreateSizeInStoragePool::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsHwProviderStoragePools as ::windows::core::Interface>::IID
    }
}
pub trait IVdsHwProviderType_Impl: Sized {
    fn GetProviderType(&self) -> ::windows::core::Result<VDS_HWPROVIDER_TYPE>;
}
impl IVdsHwProviderType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderType_Impl, const OFFSET: isize>() -> IVdsHwProviderType_Vtbl {
        unsafe extern "system" fn GetProviderType<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut VDS_HWPROVIDER_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProviderType() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetProviderType: GetProviderType::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsHwProviderType as ::windows::core::Interface>::IID
    }
}
pub trait IVdsHwProviderType2_Impl: Sized {
    fn GetProviderType2(&self) -> ::windows::core::Result<VDS_HWPROVIDER_TYPE>;
}
impl IVdsHwProviderType2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderType2_Impl, const OFFSET: isize>() -> IVdsHwProviderType2_Vtbl {
        unsafe extern "system" fn GetProviderType2<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderType2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut VDS_HWPROVIDER_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProviderType2() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetProviderType2: GetProviderType2::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsHwProviderType2 as ::windows::core::Interface>::IID
    }
}
pub trait IVdsIscsiPortal_Impl: Sized {
    fn GetProperties(&self) -> ::windows::core::Result<VDS_ISCSI_PORTAL_PROP>;
    fn GetSubSystem(&self) -> ::windows::core::Result<IVdsSubSystem>;
    fn QueryAssociatedPortalGroups(&self) -> ::windows::core::Result<IEnumVdsObject>;
    fn SetStatus(&self, status: VDS_ISCSI_PORTAL_STATUS) -> ::windows::core::Result<()>;
    fn SetIpsecTunnelAddress(&self, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows::core::Result<()>;
    fn GetIpsecSecurity(&self, pinitiatorportaladdress: *const VDS_IPADDRESS) -> ::windows::core::Result<u64>;
    fn SetIpsecSecurity(&self, pinitiatorportaladdress: *const VDS_IPADDRESS, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::Result<()>;
}
impl IVdsIscsiPortal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiPortal_Impl, const OFFSET: isize>() -> IVdsIscsiPortal_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiPortal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportalprop: *mut VDS_ISCSI_PORTAL_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pportalprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubSystem<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiPortal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSubSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsubsystem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAssociatedPortalGroups<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiPortal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryAssociatedPortalGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiPortal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_ISCSI_PORTAL_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStatus(::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn SetIpsecTunnelAddress<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiPortal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIpsecTunnelAddress(::core::mem::transmute_copy(&ptunneladdress), ::core::mem::transmute_copy(&pdestinationaddress)).into()
        }
        unsafe extern "system" fn GetIpsecSecurity<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiPortal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitiatorportaladdress: *const VDS_IPADDRESS, pullsecurityflags: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIpsecSecurity(::core::mem::transmute_copy(&pinitiatorportaladdress)) {
                ::core::result::Result::Ok(ok__) => {
                    *pullsecurityflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpsecSecurity<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiPortal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitiatorportaladdress: *const VDS_IPADDRESS, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIpsecSecurity(::core::mem::transmute_copy(&pinitiatorportaladdress), ::core::mem::transmute_copy(&ullsecurityflags), ::core::mem::transmute_copy(&pipseckey)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetSubSystem: GetSubSystem::<Identity, Impl, OFFSET>,
            QueryAssociatedPortalGroups: QueryAssociatedPortalGroups::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
            SetIpsecTunnelAddress: SetIpsecTunnelAddress::<Identity, Impl, OFFSET>,
            GetIpsecSecurity: GetIpsecSecurity::<Identity, Impl, OFFSET>,
            SetIpsecSecurity: SetIpsecSecurity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsIscsiPortal as ::windows::core::Interface>::IID
    }
}
pub trait IVdsIscsiPortalGroup_Impl: Sized {
    fn GetProperties(&self) -> ::windows::core::Result<VDS_ISCSI_PORTALGROUP_PROP>;
    fn GetTarget(&self) -> ::windows::core::Result<IVdsIscsiTarget>;
    fn QueryAssociatedPortals(&self) -> ::windows::core::Result<IEnumVdsObject>;
    fn AddPortal(&self, portalid: &::windows::core::GUID) -> ::windows::core::Result<IVdsAsync>;
    fn RemovePortal(&self, portalid: &::windows::core::GUID) -> ::windows::core::Result<IVdsAsync>;
    fn Delete(&self) -> ::windows::core::Result<IVdsAsync>;
}
impl IVdsIscsiPortalGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: isize>() -> IVdsIscsiPortalGroup_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportalgroupprop: *mut VDS_ISCSI_PORTALGROUP_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pportalgroupprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTarget<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *pptarget = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAssociatedPortals<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryAssociatedPortals() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPortal<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portalid: ::windows::core::GUID, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddPortal(::core::mem::transmute(&portalid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePortal<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portalid: ::windows::core::GUID, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RemovePortal(::core::mem::transmute(&portalid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Delete() {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetTarget: GetTarget::<Identity, Impl, OFFSET>,
            QueryAssociatedPortals: QueryAssociatedPortals::<Identity, Impl, OFFSET>,
            AddPortal: AddPortal::<Identity, Impl, OFFSET>,
            RemovePortal: RemovePortal::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsIscsiPortalGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsIscsiTarget_Impl: Sized {
    fn GetProperties(&self) -> ::windows::core::Result<VDS_ISCSI_TARGET_PROP>;
    fn GetSubSystem(&self) -> ::windows::core::Result<IVdsSubSystem>;
    fn QueryPortalGroups(&self) -> ::windows::core::Result<IEnumVdsObject>;
    fn QueryAssociatedLuns(&self) -> ::windows::core::Result<IEnumVdsObject>;
    fn CreatePortalGroup(&self) -> ::windows::core::Result<IVdsAsync>;
    fn Delete(&self) -> ::windows::core::Result<IVdsAsync>;
    fn SetFriendlyName(&self, pwszfriendlyname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SetSharedSecret(&self, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET, pwszinitiatorname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn RememberInitiatorSharedSecret(&self, pwszinitiatorname: &::windows::core::PCWSTR, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> ::windows::core::Result<()>;
    fn GetConnectedInitiators(&self, pppwszinitiatorlist: *mut *mut ::windows::core::PWSTR, plnumberofinitiators: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsIscsiTarget_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>() -> IVdsIscsiTarget_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetprop: *mut VDS_ISCSI_TARGET_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ptargetprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubSystem<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSubSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsubsystem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryPortalGroups<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryPortalGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAssociatedLuns<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryAssociatedLuns() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePortalGroup<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreatePortalGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Delete() {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFriendlyName(::core::mem::transmute(&pwszfriendlyname)).into()
        }
        unsafe extern "system" fn SetSharedSecret<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET, pwszinitiatorname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSharedSecret(::core::mem::transmute_copy(&ptargetsharedsecret), ::core::mem::transmute(&pwszinitiatorname)).into()
        }
        unsafe extern "system" fn RememberInitiatorSharedSecret<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinitiatorname: ::windows::core::PCWSTR, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RememberInitiatorSharedSecret(::core::mem::transmute(&pwszinitiatorname), ::core::mem::transmute_copy(&pinitiatorsharedsecret)).into()
        }
        unsafe extern "system" fn GetConnectedInitiators<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppwszinitiatorlist: *mut *mut ::windows::core::PWSTR, plnumberofinitiators: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConnectedInitiators(::core::mem::transmute_copy(&pppwszinitiatorlist), ::core::mem::transmute_copy(&plnumberofinitiators)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetSubSystem: GetSubSystem::<Identity, Impl, OFFSET>,
            QueryPortalGroups: QueryPortalGroups::<Identity, Impl, OFFSET>,
            QueryAssociatedLuns: QueryAssociatedLuns::<Identity, Impl, OFFSET>,
            CreatePortalGroup: CreatePortalGroup::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            SetFriendlyName: SetFriendlyName::<Identity, Impl, OFFSET>,
            SetSharedSecret: SetSharedSecret::<Identity, Impl, OFFSET>,
            RememberInitiatorSharedSecret: RememberInitiatorSharedSecret::<Identity, Impl, OFFSET>,
            GetConnectedInitiators: GetConnectedInitiators::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsIscsiTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsLun_Impl: Sized {
    fn GetProperties(&self) -> ::windows::core::Result<VDS_LUN_PROP>;
    fn GetSubSystem(&self) -> ::windows::core::Result<IVdsSubSystem>;
    fn GetIdentificationData(&self) -> ::windows::core::Result<VDS_LUN_INFORMATION>;
    fn QueryActiveControllers(&self) -> ::windows::core::Result<IEnumVdsObject>;
    fn Extend(&self, ullnumberofbytestoadd: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32) -> ::windows::core::Result<IVdsAsync>;
    fn Shrink(&self, ullnumberofbytestoremove: u64) -> ::windows::core::Result<IVdsAsync>;
    fn QueryPlexes(&self) -> ::windows::core::Result<IEnumVdsObject>;
    fn AddPlex(&self, lunid: &::windows::core::GUID) -> ::windows::core::Result<IVdsAsync>;
    fn RemovePlex(&self, plexid: &::windows::core::GUID) -> ::windows::core::Result<IVdsAsync>;
    fn Recover(&self) -> ::windows::core::Result<IVdsAsync>;
    fn SetMask(&self, pwszunmaskinglist: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn AssociateControllers(&self, pactivecontrolleridarray: *const ::windows::core::GUID, lnumberofactivecontrollers: i32, pinactivecontrolleridarray: *const ::windows::core::GUID, lnumberofinactivecontrollers: i32) -> ::windows::core::Result<()>;
    fn QueryHints(&self) -> ::windows::core::Result<VDS_HINTS>;
    fn ApplyHints(&self, phints: *const VDS_HINTS) -> ::windows::core::Result<()>;
    fn SetStatus(&self, status: VDS_LUN_STATUS) -> ::windows::core::Result<()>;
    fn QueryMaxLunExtendSize(&self, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsLun_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun_Impl, const OFFSET: isize>() -> IVdsLun_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plunprop: *mut VDS_LUN_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *plunprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubSystem<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSubSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsubsystem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIdentificationData<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pluninfo: *mut VDS_LUN_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIdentificationData() {
                ::core::result::Result::Ok(ok__) => {
                    *pluninfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryActiveControllers<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryActiveControllers() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extend<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullnumberofbytestoadd: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Extend(::core::mem::transmute_copy(&ullnumberofbytestoadd), ::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shrink<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullnumberofbytestoremove: u64, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Shrink(::core::mem::transmute_copy(&ullnumberofbytestoremove)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryPlexes<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryPlexes() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPlex<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lunid: ::windows::core::GUID, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddPlex(::core::mem::transmute(&lunid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlex<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plexid: ::windows::core::GUID, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RemovePlex(::core::mem::transmute(&plexid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recover<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Recover() {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMask<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszunmaskinglist: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMask(::core::mem::transmute(&pwszunmaskinglist)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn AssociateControllers<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactivecontrolleridarray: *const ::windows::core::GUID, lnumberofactivecontrollers: i32, pinactivecontrolleridarray: *const ::windows::core::GUID, lnumberofinactivecontrollers: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AssociateControllers(::core::mem::transmute_copy(&pactivecontrolleridarray), ::core::mem::transmute_copy(&lnumberofactivecontrollers), ::core::mem::transmute_copy(&pinactivecontrolleridarray), ::core::mem::transmute_copy(&lnumberofinactivecontrollers)).into()
        }
        unsafe extern "system" fn QueryHints<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints: *mut VDS_HINTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryHints() {
                ::core::result::Result::Ok(ok__) => {
                    *phints = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyHints<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints: *const VDS_HINTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ApplyHints(::core::mem::transmute_copy(&phints)).into()
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_LUN_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStatus(::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn QueryMaxLunExtendSize<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pullmaxbytestobeadded: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryMaxLunExtendSize(::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives)) {
                ::core::result::Result::Ok(ok__) => {
                    *pullmaxbytestobeadded = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetSubSystem: GetSubSystem::<Identity, Impl, OFFSET>,
            GetIdentificationData: GetIdentificationData::<Identity, Impl, OFFSET>,
            QueryActiveControllers: QueryActiveControllers::<Identity, Impl, OFFSET>,
            Extend: Extend::<Identity, Impl, OFFSET>,
            Shrink: Shrink::<Identity, Impl, OFFSET>,
            QueryPlexes: QueryPlexes::<Identity, Impl, OFFSET>,
            AddPlex: AddPlex::<Identity, Impl, OFFSET>,
            RemovePlex: RemovePlex::<Identity, Impl, OFFSET>,
            Recover: Recover::<Identity, Impl, OFFSET>,
            SetMask: SetMask::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            AssociateControllers: AssociateControllers::<Identity, Impl, OFFSET>,
            QueryHints: QueryHints::<Identity, Impl, OFFSET>,
            ApplyHints: ApplyHints::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
            QueryMaxLunExtendSize: QueryMaxLunExtendSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLun as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsLun2_Impl: Sized {
    fn QueryHints2(&self) -> ::windows::core::Result<VDS_HINTS2>;
    fn ApplyHints2(&self, phints2: *const VDS_HINTS2) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsLun2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun2_Impl, const OFFSET: isize>() -> IVdsLun2_Vtbl {
        unsafe extern "system" fn QueryHints2<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints2: *mut VDS_HINTS2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryHints2() {
                ::core::result::Result::Ok(ok__) => {
                    *phints2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyHints2<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints2: *const VDS_HINTS2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ApplyHints2(::core::mem::transmute_copy(&phints2)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            QueryHints2: QueryHints2::<Identity, Impl, OFFSET>,
            ApplyHints2: ApplyHints2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLun2 as ::windows::core::Interface>::IID
    }
}
pub trait IVdsLunControllerPorts_Impl: Sized {
    fn AssociateControllerPorts(&self, pactivecontrollerportidarray: *const ::windows::core::GUID, lnumberofactivecontrollerports: i32, pinactivecontrollerportidarray: *const ::windows::core::GUID, lnumberofinactivecontrollerports: i32) -> ::windows::core::Result<()>;
    fn QueryActiveControllerPorts(&self) -> ::windows::core::Result<IEnumVdsObject>;
}
impl IVdsLunControllerPorts_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunControllerPorts_Impl, const OFFSET: isize>() -> IVdsLunControllerPorts_Vtbl {
        unsafe extern "system" fn AssociateControllerPorts<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunControllerPorts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactivecontrollerportidarray: *const ::windows::core::GUID, lnumberofactivecontrollerports: i32, pinactivecontrollerportidarray: *const ::windows::core::GUID, lnumberofinactivecontrollerports: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AssociateControllerPorts(::core::mem::transmute_copy(&pactivecontrollerportidarray), ::core::mem::transmute_copy(&lnumberofactivecontrollerports), ::core::mem::transmute_copy(&pinactivecontrollerportidarray), ::core::mem::transmute_copy(&lnumberofinactivecontrollerports)).into()
        }
        unsafe extern "system" fn QueryActiveControllerPorts<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunControllerPorts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryActiveControllerPorts() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AssociateControllerPorts: AssociateControllerPorts::<Identity, Impl, OFFSET>,
            QueryActiveControllerPorts: QueryActiveControllerPorts::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLunControllerPorts as ::windows::core::Interface>::IID
    }
}
pub trait IVdsLunIscsi_Impl: Sized {
    fn AssociateTargets(&self, ptargetidarray: *const ::windows::core::GUID, lnumberoftargets: i32) -> ::windows::core::Result<()>;
    fn QueryAssociatedTargets(&self) -> ::windows::core::Result<IEnumVdsObject>;
}
impl IVdsLunIscsi_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunIscsi_Impl, const OFFSET: isize>() -> IVdsLunIscsi_Vtbl {
        unsafe extern "system" fn AssociateTargets<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunIscsi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetidarray: *const ::windows::core::GUID, lnumberoftargets: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AssociateTargets(::core::mem::transmute_copy(&ptargetidarray), ::core::mem::transmute_copy(&lnumberoftargets)).into()
        }
        unsafe extern "system" fn QueryAssociatedTargets<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunIscsi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryAssociatedTargets() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AssociateTargets: AssociateTargets::<Identity, Impl, OFFSET>,
            QueryAssociatedTargets: QueryAssociatedTargets::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLunIscsi as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsLunMpio_Impl: Sized {
    fn GetPathInfo(&self, pppaths: *mut *mut VDS_PATH_INFO, plnumberofpaths: *mut i32) -> ::windows::core::Result<()>;
    fn GetLoadBalancePolicy(&self, ppolicy: *mut VDS_LOADBALANCE_POLICY_ENUM, pppaths: *mut *mut VDS_PATH_POLICY, plnumberofpaths: *mut i32) -> ::windows::core::Result<()>;
    fn SetLoadBalancePolicy(&self, policy: VDS_LOADBALANCE_POLICY_ENUM, ppaths: *const VDS_PATH_POLICY, lnumberofpaths: i32) -> ::windows::core::Result<()>;
    fn GetSupportedLbPolicies(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsLunMpio_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunMpio_Impl, const OFFSET: isize>() -> IVdsLunMpio_Vtbl {
        unsafe extern "system" fn GetPathInfo<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunMpio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppaths: *mut *mut VDS_PATH_INFO, plnumberofpaths: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPathInfo(::core::mem::transmute_copy(&pppaths), ::core::mem::transmute_copy(&plnumberofpaths)).into()
        }
        unsafe extern "system" fn GetLoadBalancePolicy<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunMpio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppolicy: *mut VDS_LOADBALANCE_POLICY_ENUM, pppaths: *mut *mut VDS_PATH_POLICY, plnumberofpaths: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLoadBalancePolicy(::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&pppaths), ::core::mem::transmute_copy(&plnumberofpaths)).into()
        }
        unsafe extern "system" fn SetLoadBalancePolicy<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunMpio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: VDS_LOADBALANCE_POLICY_ENUM, ppaths: *const VDS_PATH_POLICY, lnumberofpaths: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLoadBalancePolicy(::core::mem::transmute_copy(&policy), ::core::mem::transmute_copy(&ppaths), ::core::mem::transmute_copy(&lnumberofpaths)).into()
        }
        unsafe extern "system" fn GetSupportedLbPolicies<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunMpio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullbflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSupportedLbPolicies() {
                ::core::result::Result::Ok(ok__) => {
                    *pullbflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPathInfo: GetPathInfo::<Identity, Impl, OFFSET>,
            GetLoadBalancePolicy: GetLoadBalancePolicy::<Identity, Impl, OFFSET>,
            SetLoadBalancePolicy: SetLoadBalancePolicy::<Identity, Impl, OFFSET>,
            GetSupportedLbPolicies: GetSupportedLbPolicies::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLunMpio as ::windows::core::Interface>::IID
    }
}
pub trait IVdsLunNaming_Impl: Sized {
    fn SetFriendlyName(&self, pwszfriendlyname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl IVdsLunNaming_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunNaming_Impl, const OFFSET: isize>() -> IVdsLunNaming_Vtbl {
        unsafe extern "system" fn SetFriendlyName<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunNaming_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFriendlyName(::core::mem::transmute(&pwszfriendlyname)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetFriendlyName: SetFriendlyName::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLunNaming as ::windows::core::Interface>::IID
    }
}
pub trait IVdsLunNumber_Impl: Sized {
    fn GetLunNumber(&self) -> ::windows::core::Result<u32>;
}
impl IVdsLunNumber_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunNumber_Impl, const OFFSET: isize>() -> IVdsLunNumber_Vtbl {
        unsafe extern "system" fn GetLunNumber<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullunnumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLunNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pullunnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetLunNumber: GetLunNumber::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLunNumber as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsLunPlex_Impl: Sized {
    fn GetProperties(&self) -> ::windows::core::Result<VDS_LUN_PLEX_PROP>;
    fn GetLun(&self) -> ::windows::core::Result<IVdsLun>;
    fn QueryExtents(&self, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::Result<()>;
    fn QueryHints(&self) -> ::windows::core::Result<VDS_HINTS>;
    fn ApplyHints(&self, phints: *const VDS_HINTS) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsLunPlex_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunPlex_Impl, const OFFSET: isize>() -> IVdsLunPlex_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunPlex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplexprop: *mut VDS_LUN_PLEX_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pplexprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLun<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunPlex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplun: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLun() {
                ::core::result::Result::Ok(ok__) => {
                    *pplun = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryExtents<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunPlex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryExtents(::core::mem::transmute_copy(&ppextentarray), ::core::mem::transmute_copy(&plnumberofextents)).into()
        }
        unsafe extern "system" fn QueryHints<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunPlex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints: *mut VDS_HINTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryHints() {
                ::core::result::Result::Ok(ok__) => {
                    *phints = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyHints<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunPlex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints: *const VDS_HINTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ApplyHints(::core::mem::transmute_copy(&phints)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetLun: GetLun::<Identity, Impl, OFFSET>,
            QueryExtents: QueryExtents::<Identity, Impl, OFFSET>,
            QueryHints: QueryHints::<Identity, Impl, OFFSET>,
            ApplyHints: ApplyHints::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLunPlex as ::windows::core::Interface>::IID
    }
}
pub trait IVdsMaintenance_Impl: Sized {
    fn StartMaintenance(&self, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::core::Result<()>;
    fn StopMaintenance(&self, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::core::Result<()>;
    fn PulseMaintenance(&self, operation: VDS_MAINTENANCE_OPERATION, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IVdsMaintenance_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsMaintenance_Impl, const OFFSET: isize>() -> IVdsMaintenance_Vtbl {
        unsafe extern "system" fn StartMaintenance<Identity: ::windows::core::IUnknownImpl, Impl: IVdsMaintenance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartMaintenance(::core::mem::transmute_copy(&operation)).into()
        }
        unsafe extern "system" fn StopMaintenance<Identity: ::windows::core::IUnknownImpl, Impl: IVdsMaintenance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StopMaintenance(::core::mem::transmute_copy(&operation)).into()
        }
        unsafe extern "system" fn PulseMaintenance<Identity: ::windows::core::IUnknownImpl, Impl: IVdsMaintenance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PulseMaintenance(::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            StartMaintenance: StartMaintenance::<Identity, Impl, OFFSET>,
            StopMaintenance: StopMaintenance::<Identity, Impl, OFFSET>,
            PulseMaintenance: PulseMaintenance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsMaintenance as ::windows::core::Interface>::IID
    }
}
pub trait IVdsProvider_Impl: Sized {
    fn GetProperties(&self) -> ::windows::core::Result<VDS_PROVIDER_PROP>;
}
impl IVdsProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsProvider_Impl, const OFFSET: isize>() -> IVdsProvider_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl, Impl: IVdsProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproviderprop: *mut VDS_PROVIDER_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pproviderprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetProperties: GetProperties::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsProviderPrivate_Impl: Sized {
    fn GetObject(&self, objectid: &::windows::core::GUID, r#type: VDS_OBJECT_TYPE) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn OnLoad(&self, pwszmachinename: &::windows::core::PCWSTR, pcallbackobject: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn OnUnload(&self, bforceunload: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsProviderPrivate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsProviderPrivate_Impl, const OFFSET: isize>() -> IVdsProviderPrivate_Vtbl {
        unsafe extern "system" fn GetObject<Identity: ::windows::core::IUnknownImpl, Impl: IVdsProviderPrivate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: ::windows::core::GUID, r#type: VDS_OBJECT_TYPE, ppobjectunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetObject(::core::mem::transmute(&objectid), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppobjectunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLoad<Identity: ::windows::core::IUnknownImpl, Impl: IVdsProviderPrivate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszmachinename: ::windows::core::PCWSTR, pcallbackobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnLoad(::core::mem::transmute(&pwszmachinename), ::core::mem::transmute(&pcallbackobject)).into()
        }
        unsafe extern "system" fn OnUnload<Identity: ::windows::core::IUnknownImpl, Impl: IVdsProviderPrivate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bforceunload: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnUnload(::core::mem::transmute_copy(&bforceunload)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            OnLoad: OnLoad::<Identity, Impl, OFFSET>,
            OnUnload: OnUnload::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsProviderPrivate as ::windows::core::Interface>::IID
    }
}
pub trait IVdsProviderSupport_Impl: Sized {
    fn GetVersionSupport(&self) -> ::windows::core::Result<u32>;
}
impl IVdsProviderSupport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsProviderSupport_Impl, const OFFSET: isize>() -> IVdsProviderSupport_Vtbl {
        unsafe extern "system" fn GetVersionSupport<Identity: ::windows::core::IUnknownImpl, Impl: IVdsProviderSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulversionsupport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVersionSupport() {
                ::core::result::Result::Ok(ok__) => {
                    *ulversionsupport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetVersionSupport: GetVersionSupport::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsProviderSupport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsStoragePool_Impl: Sized {
    fn GetProvider(&self) -> ::windows::core::Result<IVdsProvider>;
    fn GetProperties(&self) -> ::windows::core::Result<VDS_STORAGE_POOL_PROP>;
    fn GetAttributes(&self) -> ::windows::core::Result<VDS_POOL_ATTRIBUTES>;
    fn QueryDriveExtents(&self, ppextentarray: *mut *mut VDS_STORAGE_POOL_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::Result<()>;
    fn QueryAllocatedLuns(&self) -> ::windows::core::Result<IEnumVdsObject>;
    fn QueryAllocatedStoragePools(&self) -> ::windows::core::Result<IEnumVdsObject>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsStoragePool_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsStoragePool_Impl, const OFFSET: isize>() -> IVdsStoragePool_Vtbl {
        unsafe extern "system" fn GetProvider<Identity: ::windows::core::IUnknownImpl, Impl: IVdsStoragePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl, Impl: IVdsStoragePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstoragepoolprop: *mut VDS_STORAGE_POOL_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pstoragepoolprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IVdsStoragePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstoragepoolattributes: *mut VDS_POOL_ATTRIBUTES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *pstoragepoolattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDriveExtents<Identity: ::windows::core::IUnknownImpl, Impl: IVdsStoragePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_STORAGE_POOL_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryDriveExtents(::core::mem::transmute_copy(&ppextentarray), ::core::mem::transmute_copy(&plnumberofextents)).into()
        }
        unsafe extern "system" fn QueryAllocatedLuns<Identity: ::windows::core::IUnknownImpl, Impl: IVdsStoragePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryAllocatedLuns() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAllocatedStoragePools<Identity: ::windows::core::IUnknownImpl, Impl: IVdsStoragePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryAllocatedStoragePools() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProvider: GetProvider::<Identity, Impl, OFFSET>,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetAttributes: GetAttributes::<Identity, Impl, OFFSET>,
            QueryDriveExtents: QueryDriveExtents::<Identity, Impl, OFFSET>,
            QueryAllocatedLuns: QueryAllocatedLuns::<Identity, Impl, OFFSET>,
            QueryAllocatedStoragePools: QueryAllocatedStoragePools::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsStoragePool as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsSubSystem_Impl: Sized {
    fn GetProperties(&self) -> ::windows::core::Result<VDS_SUB_SYSTEM_PROP>;
    fn GetProvider(&self) -> ::windows::core::Result<IVdsProvider>;
    fn QueryControllers(&self) -> ::windows::core::Result<IEnumVdsObject>;
    fn QueryLuns(&self) -> ::windows::core::Result<IEnumVdsObject>;
    fn QueryDrives(&self) -> ::windows::core::Result<IEnumVdsObject>;
    fn GetDrive(&self, sbusnumber: i16, sslotnumber: i16) -> ::windows::core::Result<IVdsDrive>;
    fn Reenumerate(&self) -> ::windows::core::Result<()>;
    fn SetControllerStatus(&self, ponlinecontrolleridarray: *const ::windows::core::GUID, lnumberofonlinecontrollers: i32, pofflinecontrolleridarray: *const ::windows::core::GUID, lnumberofofflinecontrollers: i32) -> ::windows::core::Result<()>;
    fn CreateLun(&self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pwszunmaskinglist: &::windows::core::PCWSTR, phints: *const VDS_HINTS) -> ::windows::core::Result<IVdsAsync>;
    fn ReplaceDrive(&self, drivetobereplaced: &::windows::core::GUID, replacementdrive: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetStatus(&self, status: VDS_SUB_SYSTEM_STATUS) -> ::windows::core::Result<()>;
    fn QueryMaxLunCreateSize(&self, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, phints: *const VDS_HINTS) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsSubSystem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystem_Impl, const OFFSET: isize>() -> IVdsSubSystem_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubsystemprop: *mut VDS_SUB_SYSTEM_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *psubsystemprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProvider<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryControllers<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryControllers() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryLuns<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryLuns() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDrives<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryDrives() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDrive<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sbusnumber: i16, sslotnumber: i16, ppdrive: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDrive(::core::mem::transmute_copy(&sbusnumber), ::core::mem::transmute_copy(&sslotnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdrive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reenumerate<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reenumerate().into()
        }
        unsafe extern "system" fn SetControllerStatus<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ponlinecontrolleridarray: *const ::windows::core::GUID, lnumberofonlinecontrollers: i32, pofflinecontrolleridarray: *const ::windows::core::GUID, lnumberofofflinecontrollers: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetControllerStatus(::core::mem::transmute_copy(&ponlinecontrolleridarray), ::core::mem::transmute_copy(&lnumberofonlinecontrollers), ::core::mem::transmute_copy(&pofflinecontrolleridarray), ::core::mem::transmute_copy(&lnumberofofflinecontrollers)).into()
        }
        unsafe extern "system" fn CreateLun<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pwszunmaskinglist: ::windows::core::PCWSTR, phints: *const VDS_HINTS, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateLun(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&ullsizeinbytes), ::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives), ::core::mem::transmute(&pwszunmaskinglist), ::core::mem::transmute_copy(&phints)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceDrive<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drivetobereplaced: ::windows::core::GUID, replacementdrive: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReplaceDrive(::core::mem::transmute(&drivetobereplaced), ::core::mem::transmute(&replacementdrive)).into()
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_SUB_SYSTEM_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStatus(::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn QueryMaxLunCreateSize<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, phints: *const VDS_HINTS, pullmaxlunsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryMaxLunCreateSize(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives), ::core::mem::transmute_copy(&phints)) {
                ::core::result::Result::Ok(ok__) => {
                    *pullmaxlunsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetProvider: GetProvider::<Identity, Impl, OFFSET>,
            QueryControllers: QueryControllers::<Identity, Impl, OFFSET>,
            QueryLuns: QueryLuns::<Identity, Impl, OFFSET>,
            QueryDrives: QueryDrives::<Identity, Impl, OFFSET>,
            GetDrive: GetDrive::<Identity, Impl, OFFSET>,
            Reenumerate: Reenumerate::<Identity, Impl, OFFSET>,
            SetControllerStatus: SetControllerStatus::<Identity, Impl, OFFSET>,
            CreateLun: CreateLun::<Identity, Impl, OFFSET>,
            ReplaceDrive: ReplaceDrive::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
            QueryMaxLunCreateSize: QueryMaxLunCreateSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsSubSystem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsSubSystem2_Impl: Sized {
    fn GetProperties2(&self) -> ::windows::core::Result<VDS_SUB_SYSTEM_PROP2>;
    fn GetDrive2(&self, sbusnumber: i16, sslotnumber: i16, ulenclosurenumber: u32) -> ::windows::core::Result<IVdsDrive>;
    fn CreateLun2(&self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pwszunmaskinglist: &::windows::core::PCWSTR, phints2: *const VDS_HINTS2) -> ::windows::core::Result<IVdsAsync>;
    fn QueryMaxLunCreateSize2(&self, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, phints2: *const VDS_HINTS2) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsSubSystem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystem2_Impl, const OFFSET: isize>() -> IVdsSubSystem2_Vtbl {
        unsafe extern "system" fn GetProperties2<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubsystemprop2: *mut VDS_SUB_SYSTEM_PROP2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperties2() {
                ::core::result::Result::Ok(ok__) => {
                    *psubsystemprop2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDrive2<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sbusnumber: i16, sslotnumber: i16, ulenclosurenumber: u32, ppdrive: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDrive2(::core::mem::transmute_copy(&sbusnumber), ::core::mem::transmute_copy(&sslotnumber), ::core::mem::transmute_copy(&ulenclosurenumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdrive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLun2<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pwszunmaskinglist: ::windows::core::PCWSTR, phints2: *const VDS_HINTS2, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateLun2(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&ullsizeinbytes), ::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives), ::core::mem::transmute(&pwszunmaskinglist), ::core::mem::transmute_copy(&phints2)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryMaxLunCreateSize2<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, phints2: *const VDS_HINTS2, pullmaxlunsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryMaxLunCreateSize2(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives), ::core::mem::transmute_copy(&phints2)) {
                ::core::result::Result::Ok(ok__) => {
                    *pullmaxlunsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties2: GetProperties2::<Identity, Impl, OFFSET>,
            GetDrive2: GetDrive2::<Identity, Impl, OFFSET>,
            CreateLun2: CreateLun2::<Identity, Impl, OFFSET>,
            QueryMaxLunCreateSize2: QueryMaxLunCreateSize2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsSubSystem2 as ::windows::core::Interface>::IID
    }
}
pub trait IVdsSubSystemInterconnect_Impl: Sized {
    fn GetSupportedInterconnects(&self) -> ::windows::core::Result<u32>;
}
impl IVdsSubSystemInterconnect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystemInterconnect_Impl, const OFFSET: isize>() -> IVdsSubSystemInterconnect_Vtbl {
        unsafe extern "system" fn GetSupportedInterconnects<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystemInterconnect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulsupportedinterconnectsflag: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSupportedInterconnects() {
                ::core::result::Result::Ok(ok__) => {
                    *pulsupportedinterconnectsflag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetSupportedInterconnects: GetSupportedInterconnects::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsSubSystemInterconnect as ::windows::core::Interface>::IID
    }
}
pub trait IVdsSubSystemIscsi_Impl: Sized {
    fn QueryTargets(&self) -> ::windows::core::Result<IEnumVdsObject>;
    fn QueryPortals(&self) -> ::windows::core::Result<IEnumVdsObject>;
    fn CreateTarget(&self, pwsziscsiname: &::windows::core::PCWSTR, pwszfriendlyname: &::windows::core::PCWSTR) -> ::windows::core::Result<IVdsAsync>;
    fn SetIpsecGroupPresharedKey(&self, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::Result<()>;
}
impl IVdsSubSystemIscsi_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystemIscsi_Impl, const OFFSET: isize>() -> IVdsSubSystemIscsi_Vtbl {
        unsafe extern "system" fn QueryTargets<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystemIscsi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryTargets() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryPortals<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystemIscsi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryPortals() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTarget<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystemIscsi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsziscsiname: ::windows::core::PCWSTR, pwszfriendlyname: ::windows::core::PCWSTR, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTarget(::core::mem::transmute(&pwsziscsiname), ::core::mem::transmute(&pwszfriendlyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpsecGroupPresharedKey<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystemIscsi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIpsecGroupPresharedKey(::core::mem::transmute_copy(&pipseckey)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            QueryTargets: QueryTargets::<Identity, Impl, OFFSET>,
            QueryPortals: QueryPortals::<Identity, Impl, OFFSET>,
            CreateTarget: CreateTarget::<Identity, Impl, OFFSET>,
            SetIpsecGroupPresharedKey: SetIpsecGroupPresharedKey::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsSubSystemIscsi as ::windows::core::Interface>::IID
    }
}
pub trait IVdsSubSystemNaming_Impl: Sized {
    fn SetFriendlyName(&self, pwszfriendlyname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl IVdsSubSystemNaming_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystemNaming_Impl, const OFFSET: isize>() -> IVdsSubSystemNaming_Vtbl {
        unsafe extern "system" fn SetFriendlyName<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystemNaming_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFriendlyName(::core::mem::transmute(&pwszfriendlyname)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetFriendlyName: SetFriendlyName::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsSubSystemNaming as ::windows::core::Interface>::IID
    }
}
