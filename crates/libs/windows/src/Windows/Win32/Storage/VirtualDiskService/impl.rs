pub trait IEnumVdsObject_Impl: Sized {
    fn Next(&mut self, celt: u32, ppobjectarray: *mut ::core::option::Option<::windows::core::IUnknown>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumVdsObject>;
}
impl IEnumVdsObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumVdsObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumVdsObject_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumVdsObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppobjectarray: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppobjectarray), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumVdsObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumVdsObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumVdsObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumVdsObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsAdmin_Impl: Sized {
    fn RegisterProvider(&mut self, providerid: &::windows::core::GUID, providerclsid: &::windows::core::GUID, pwszname: super::super::Foundation::PWSTR, r#type: VDS_PROVIDER_TYPE, pwszmachinename: super::super::Foundation::PWSTR, pwszversion: super::super::Foundation::PWSTR, guidversionid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn UnregisterProvider(&mut self, providerid: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsAdmin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsAdmin_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsAdmin_Vtbl {
        unsafe extern "system" fn RegisterProvider<Impl: IVdsAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, providerclsid: ::windows::core::GUID, pwszname: super::super::Foundation::PWSTR, r#type: VDS_PROVIDER_TYPE, pwszmachinename: super::super::Foundation::PWSTR, pwszversion: super::super::Foundation::PWSTR, guidversionid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterProvider(::core::mem::transmute_copy(&providerid), ::core::mem::transmute_copy(&providerclsid), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pwszmachinename), ::core::mem::transmute_copy(&pwszversion), ::core::mem::transmute_copy(&guidversionid)).into()
        }
        unsafe extern "system" fn UnregisterProvider<Impl: IVdsAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterProvider(::core::mem::transmute_copy(&providerid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterProvider: RegisterProvider::<Impl, IMPL_OFFSET>,
            UnregisterProvider: UnregisterProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsAdmin as ::windows::core::Interface>::IID
    }
}
pub trait IVdsAdviseSink_Impl: Sized {
    fn OnNotify(&mut self, lnumberofnotifications: i32, pnotificationarray: *const VDS_NOTIFICATION) -> ::windows::core::Result<()>;
}
impl IVdsAdviseSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsAdviseSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsAdviseSink_Vtbl {
        unsafe extern "system" fn OnNotify<Impl: IVdsAdviseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnumberofnotifications: i32, pnotificationarray: *const VDS_NOTIFICATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnNotify(::core::mem::transmute_copy(&lnumberofnotifications), ::core::mem::transmute_copy(&pnotificationarray)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnNotify: OnNotify::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsAdviseSink as ::windows::core::Interface>::IID
    }
}
pub trait IVdsAsync_Impl: Sized {
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
    fn Wait(&mut self, phrresult: *mut ::windows::core::HRESULT, pasyncout: *mut VDS_ASYNC_OUTPUT) -> ::windows::core::Result<()>;
    fn QueryStatus(&mut self, phrresult: *mut ::windows::core::HRESULT, pulpercentcompleted: *mut u32) -> ::windows::core::Result<()>;
}
impl IVdsAsync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsAsync_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsAsync_Vtbl {
        unsafe extern "system" fn Cancel<Impl: IVdsAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn Wait<Impl: IVdsAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT, pasyncout: *mut VDS_ASYNC_OUTPUT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Wait(::core::mem::transmute_copy(&phrresult), ::core::mem::transmute_copy(&pasyncout)).into()
        }
        unsafe extern "system" fn QueryStatus<Impl: IVdsAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT, pulpercentcompleted: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryStatus(::core::mem::transmute_copy(&phrresult), ::core::mem::transmute_copy(&pulpercentcompleted)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            Wait: Wait::<Impl, IMPL_OFFSET>,
            QueryStatus: QueryStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsAsync as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsController_Impl: Sized {
    fn GetProperties(&mut self) -> ::windows::core::Result<VDS_CONTROLLER_PROP>;
    fn GetSubSystem(&mut self) -> ::windows::core::Result<IVdsSubSystem>;
    fn GetPortProperties(&mut self, sportnumber: i16) -> ::windows::core::Result<VDS_PORT_PROP>;
    fn FlushCache(&mut self) -> ::windows::core::Result<()>;
    fn InvalidateCache(&mut self) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn QueryAssociatedLuns(&mut self) -> ::windows::core::Result<IEnumVdsObject>;
    fn SetStatus(&mut self, status: VDS_CONTROLLER_STATUS) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsController_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsController_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsController_Vtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontrollerprop: *mut VDS_CONTROLLER_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pcontrollerprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubSystem<Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsubsystem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPortProperties<Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sportnumber: i16, pportprop: *mut VDS_PORT_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPortProperties(::core::mem::transmute_copy(&sportnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *pportprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushCache<Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FlushCache().into()
        }
        unsafe extern "system" fn InvalidateCache<Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvalidateCache().into()
        }
        unsafe extern "system" fn Reset<Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn QueryAssociatedLuns<Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryAssociatedLuns() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_CONTROLLER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetProperties: GetProperties::<Impl, IMPL_OFFSET>,
            GetSubSystem: GetSubSystem::<Impl, IMPL_OFFSET>,
            GetPortProperties: GetPortProperties::<Impl, IMPL_OFFSET>,
            FlushCache: FlushCache::<Impl, IMPL_OFFSET>,
            InvalidateCache: InvalidateCache::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            QueryAssociatedLuns: QueryAssociatedLuns::<Impl, IMPL_OFFSET>,
            SetStatus: SetStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsController as ::windows::core::Interface>::IID
    }
}
pub trait IVdsControllerControllerPort_Impl: Sized {
    fn QueryControllerPorts(&mut self) -> ::windows::core::Result<IEnumVdsObject>;
}
impl IVdsControllerControllerPort_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsControllerControllerPort_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsControllerControllerPort_Vtbl {
        unsafe extern "system" fn QueryControllerPorts<Impl: IVdsControllerControllerPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryControllerPorts() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), QueryControllerPorts: QueryControllerPorts::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsControllerControllerPort as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsControllerPort_Impl: Sized {
    fn GetProperties(&mut self) -> ::windows::core::Result<VDS_PORT_PROP>;
    fn GetController(&mut self) -> ::windows::core::Result<IVdsController>;
    fn QueryAssociatedLuns(&mut self) -> ::windows::core::Result<IEnumVdsObject>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn SetStatus(&mut self, status: VDS_PORT_STATUS) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsControllerPort_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsControllerPort_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsControllerPort_Vtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsControllerPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportprop: *mut VDS_PORT_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pportprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetController<Impl: IVdsControllerPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontroller: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetController() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontroller = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAssociatedLuns<Impl: IVdsControllerPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryAssociatedLuns() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IVdsControllerPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn SetStatus<Impl: IVdsControllerPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_PORT_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetProperties: GetProperties::<Impl, IMPL_OFFSET>,
            GetController: GetController::<Impl, IMPL_OFFSET>,
            QueryAssociatedLuns: QueryAssociatedLuns::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            SetStatus: SetStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsControllerPort as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsDrive_Impl: Sized {
    fn GetProperties(&mut self) -> ::windows::core::Result<VDS_DRIVE_PROP>;
    fn GetSubSystem(&mut self) -> ::windows::core::Result<IVdsSubSystem>;
    fn QueryExtents(&mut self, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::Result<()>;
    fn SetFlags(&mut self, ulflags: u32) -> ::windows::core::Result<()>;
    fn ClearFlags(&mut self, ulflags: u32) -> ::windows::core::Result<()>;
    fn SetStatus(&mut self, status: VDS_DRIVE_STATUS) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsDrive_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsDrive_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsDrive_Vtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsDrive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdriveprop: *mut VDS_DRIVE_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pdriveprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubSystem<Impl: IVdsDrive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsubsystem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryExtents<Impl: IVdsDrive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryExtents(::core::mem::transmute_copy(&ppextentarray), ::core::mem::transmute_copy(&plnumberofextents)).into()
        }
        unsafe extern "system" fn SetFlags<Impl: IVdsDrive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFlags(::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn ClearFlags<Impl: IVdsDrive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearFlags(::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn SetStatus<Impl: IVdsDrive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_DRIVE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetProperties: GetProperties::<Impl, IMPL_OFFSET>,
            GetSubSystem: GetSubSystem::<Impl, IMPL_OFFSET>,
            QueryExtents: QueryExtents::<Impl, IMPL_OFFSET>,
            SetFlags: SetFlags::<Impl, IMPL_OFFSET>,
            ClearFlags: ClearFlags::<Impl, IMPL_OFFSET>,
            SetStatus: SetStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsDrive as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsDrive2_Impl: Sized {
    fn GetProperties2(&mut self) -> ::windows::core::Result<VDS_DRIVE_PROP2>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsDrive2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsDrive2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsDrive2_Vtbl {
        unsafe extern "system" fn GetProperties2<Impl: IVdsDrive2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdriveprop2: *mut VDS_DRIVE_PROP2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties2() {
                ::core::result::Result::Ok(ok__) => {
                    *pdriveprop2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetProperties2: GetProperties2::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsDrive2 as ::windows::core::Interface>::IID
    }
}
pub trait IVdsHwProvider_Impl: Sized {
    fn QuerySubSystems(&mut self) -> ::windows::core::Result<IEnumVdsObject>;
    fn Reenumerate(&mut self) -> ::windows::core::Result<()>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
}
impl IVdsHwProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsHwProvider_Vtbl {
        unsafe extern "system" fn QuerySubSystems<Impl: IVdsHwProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuerySubSystems() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reenumerate<Impl: IVdsHwProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reenumerate().into()
        }
        unsafe extern "system" fn Refresh<Impl: IVdsHwProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            QuerySubSystems: QuerySubSystems::<Impl, IMPL_OFFSET>,
            Reenumerate: Reenumerate::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsHwProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsHwProviderPrivate_Impl: Sized {
    fn QueryIfCreatedLun(&mut self, pwszdevicepath: super::super::Foundation::PWSTR, pvdsluninformation: *const VDS_LUN_INFORMATION) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsHwProviderPrivate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderPrivate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsHwProviderPrivate_Vtbl {
        unsafe extern "system" fn QueryIfCreatedLun<Impl: IVdsHwProviderPrivate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicepath: super::super::Foundation::PWSTR, pvdsluninformation: *const VDS_LUN_INFORMATION, plunid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryIfCreatedLun(::core::mem::transmute_copy(&pwszdevicepath), ::core::mem::transmute_copy(&pvdsluninformation)) {
                ::core::result::Result::Ok(ok__) => {
                    *plunid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), QueryIfCreatedLun: QueryIfCreatedLun::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsHwProviderPrivate as ::windows::core::Interface>::IID
    }
}
pub trait IVdsHwProviderPrivateMpio_Impl: Sized {
    fn SetAllPathStatusesFromHbaPort(&mut self, hbaportprop: &VDS_HBAPORT_PROP, status: VDS_PATH_STATUS) -> ::windows::core::Result<()>;
}
impl IVdsHwProviderPrivateMpio_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderPrivateMpio_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsHwProviderPrivateMpio_Vtbl {
        unsafe extern "system" fn SetAllPathStatusesFromHbaPort<Impl: IVdsHwProviderPrivateMpio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hbaportprop: VDS_HBAPORT_PROP, status: VDS_PATH_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllPathStatusesFromHbaPort(::core::mem::transmute_copy(&hbaportprop), ::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAllPathStatusesFromHbaPort: SetAllPathStatusesFromHbaPort::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsHwProviderPrivateMpio as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsHwProviderStoragePools_Impl: Sized {
    fn QueryStoragePools(&mut self, ulflags: u32, ullremainingfreespace: u64, ppoolattributes: *const VDS_POOL_ATTRIBUTES) -> ::windows::core::Result<IEnumVdsObject>;
    fn CreateLunInStoragePool(&mut self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, storagepoolid: &::windows::core::GUID, pwszunmaskinglist: super::super::Foundation::PWSTR, phints2: *const VDS_HINTS2) -> ::windows::core::Result<IVdsAsync>;
    fn QueryMaxLunCreateSizeInStoragePool(&mut self, r#type: VDS_LUN_TYPE, storagepoolid: &::windows::core::GUID, phints2: *const VDS_HINTS2) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsHwProviderStoragePools_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderStoragePools_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsHwProviderStoragePools_Vtbl {
        unsafe extern "system" fn QueryStoragePools<Impl: IVdsHwProviderStoragePools_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, ullremainingfreespace: u64, ppoolattributes: *const VDS_POOL_ATTRIBUTES, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryStoragePools(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&ullremainingfreespace), ::core::mem::transmute_copy(&ppoolattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLunInStoragePool<Impl: IVdsHwProviderStoragePools_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, storagepoolid: ::windows::core::GUID, pwszunmaskinglist: super::super::Foundation::PWSTR, phints2: *const VDS_HINTS2, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLunInStoragePool(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&ullsizeinbytes), ::core::mem::transmute_copy(&storagepoolid), ::core::mem::transmute_copy(&pwszunmaskinglist), ::core::mem::transmute_copy(&phints2)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryMaxLunCreateSizeInStoragePool<Impl: IVdsHwProviderStoragePools_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, storagepoolid: ::windows::core::GUID, phints2: *const VDS_HINTS2, pullmaxlunsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryMaxLunCreateSizeInStoragePool(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&storagepoolid), ::core::mem::transmute_copy(&phints2)) {
                ::core::result::Result::Ok(ok__) => {
                    *pullmaxlunsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            QueryStoragePools: QueryStoragePools::<Impl, IMPL_OFFSET>,
            CreateLunInStoragePool: CreateLunInStoragePool::<Impl, IMPL_OFFSET>,
            QueryMaxLunCreateSizeInStoragePool: QueryMaxLunCreateSizeInStoragePool::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsHwProviderStoragePools as ::windows::core::Interface>::IID
    }
}
pub trait IVdsHwProviderType_Impl: Sized {
    fn GetProviderType(&mut self) -> ::windows::core::Result<VDS_HWPROVIDER_TYPE>;
}
impl IVdsHwProviderType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderType_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsHwProviderType_Vtbl {
        unsafe extern "system" fn GetProviderType<Impl: IVdsHwProviderType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut VDS_HWPROVIDER_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderType() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetProviderType: GetProviderType::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsHwProviderType as ::windows::core::Interface>::IID
    }
}
pub trait IVdsHwProviderType2_Impl: Sized {
    fn GetProviderType2(&mut self) -> ::windows::core::Result<VDS_HWPROVIDER_TYPE>;
}
impl IVdsHwProviderType2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderType2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsHwProviderType2_Vtbl {
        unsafe extern "system" fn GetProviderType2<Impl: IVdsHwProviderType2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut VDS_HWPROVIDER_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderType2() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetProviderType2: GetProviderType2::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsHwProviderType2 as ::windows::core::Interface>::IID
    }
}
pub trait IVdsIscsiPortal_Impl: Sized {
    fn GetProperties(&mut self) -> ::windows::core::Result<VDS_ISCSI_PORTAL_PROP>;
    fn GetSubSystem(&mut self) -> ::windows::core::Result<IVdsSubSystem>;
    fn QueryAssociatedPortalGroups(&mut self) -> ::windows::core::Result<IEnumVdsObject>;
    fn SetStatus(&mut self, status: VDS_ISCSI_PORTAL_STATUS) -> ::windows::core::Result<()>;
    fn SetIpsecTunnelAddress(&mut self, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows::core::Result<()>;
    fn GetIpsecSecurity(&mut self, pinitiatorportaladdress: *const VDS_IPADDRESS) -> ::windows::core::Result<u64>;
    fn SetIpsecSecurity(&mut self, pinitiatorportaladdress: *const VDS_IPADDRESS, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::Result<()>;
}
impl IVdsIscsiPortal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiPortal_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsIscsiPortal_Vtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsIscsiPortal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportalprop: *mut VDS_ISCSI_PORTAL_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pportalprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubSystem<Impl: IVdsIscsiPortal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsubsystem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAssociatedPortalGroups<Impl: IVdsIscsiPortal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryAssociatedPortalGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Impl: IVdsIscsiPortal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_ISCSI_PORTAL_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn SetIpsecTunnelAddress<Impl: IVdsIscsiPortal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIpsecTunnelAddress(::core::mem::transmute_copy(&ptunneladdress), ::core::mem::transmute_copy(&pdestinationaddress)).into()
        }
        unsafe extern "system" fn GetIpsecSecurity<Impl: IVdsIscsiPortal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitiatorportaladdress: *const VDS_IPADDRESS, pullsecurityflags: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIpsecSecurity(::core::mem::transmute_copy(&pinitiatorportaladdress)) {
                ::core::result::Result::Ok(ok__) => {
                    *pullsecurityflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpsecSecurity<Impl: IVdsIscsiPortal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitiatorportaladdress: *const VDS_IPADDRESS, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIpsecSecurity(::core::mem::transmute_copy(&pinitiatorportaladdress), ::core::mem::transmute_copy(&ullsecurityflags), ::core::mem::transmute_copy(&pipseckey)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetProperties: GetProperties::<Impl, IMPL_OFFSET>,
            GetSubSystem: GetSubSystem::<Impl, IMPL_OFFSET>,
            QueryAssociatedPortalGroups: QueryAssociatedPortalGroups::<Impl, IMPL_OFFSET>,
            SetStatus: SetStatus::<Impl, IMPL_OFFSET>,
            SetIpsecTunnelAddress: SetIpsecTunnelAddress::<Impl, IMPL_OFFSET>,
            GetIpsecSecurity: GetIpsecSecurity::<Impl, IMPL_OFFSET>,
            SetIpsecSecurity: SetIpsecSecurity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsIscsiPortal as ::windows::core::Interface>::IID
    }
}
pub trait IVdsIscsiPortalGroup_Impl: Sized {
    fn GetProperties(&mut self) -> ::windows::core::Result<VDS_ISCSI_PORTALGROUP_PROP>;
    fn GetTarget(&mut self) -> ::windows::core::Result<IVdsIscsiTarget>;
    fn QueryAssociatedPortals(&mut self) -> ::windows::core::Result<IEnumVdsObject>;
    fn AddPortal(&mut self, portalid: &::windows::core::GUID) -> ::windows::core::Result<IVdsAsync>;
    fn RemovePortal(&mut self, portalid: &::windows::core::GUID) -> ::windows::core::Result<IVdsAsync>;
    fn Delete(&mut self) -> ::windows::core::Result<IVdsAsync>;
}
impl IVdsIscsiPortalGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiPortalGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsIscsiPortalGroup_Vtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportalgroupprop: *mut VDS_ISCSI_PORTALGROUP_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pportalgroupprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTarget<Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *pptarget = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAssociatedPortals<Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryAssociatedPortals() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPortal<Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portalid: ::windows::core::GUID, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPortal(::core::mem::transmute_copy(&portalid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePortal<Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portalid: ::windows::core::GUID, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemovePortal(::core::mem::transmute_copy(&portalid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete() {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetProperties: GetProperties::<Impl, IMPL_OFFSET>,
            GetTarget: GetTarget::<Impl, IMPL_OFFSET>,
            QueryAssociatedPortals: QueryAssociatedPortals::<Impl, IMPL_OFFSET>,
            AddPortal: AddPortal::<Impl, IMPL_OFFSET>,
            RemovePortal: RemovePortal::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsIscsiPortalGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsIscsiTarget_Impl: Sized {
    fn GetProperties(&mut self) -> ::windows::core::Result<VDS_ISCSI_TARGET_PROP>;
    fn GetSubSystem(&mut self) -> ::windows::core::Result<IVdsSubSystem>;
    fn QueryPortalGroups(&mut self) -> ::windows::core::Result<IEnumVdsObject>;
    fn QueryAssociatedLuns(&mut self) -> ::windows::core::Result<IEnumVdsObject>;
    fn CreatePortalGroup(&mut self) -> ::windows::core::Result<IVdsAsync>;
    fn Delete(&mut self) -> ::windows::core::Result<IVdsAsync>;
    fn SetFriendlyName(&mut self, pwszfriendlyname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetSharedSecret(&mut self, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET, pwszinitiatorname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RememberInitiatorSharedSecret(&mut self, pwszinitiatorname: super::super::Foundation::PWSTR, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> ::windows::core::Result<()>;
    fn GetConnectedInitiators(&mut self, pppwszinitiatorlist: *mut *mut super::super::Foundation::PWSTR, plnumberofinitiators: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsIscsiTarget_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiTarget_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsIscsiTarget_Vtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetprop: *mut VDS_ISCSI_TARGET_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ptargetprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubSystem<Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsubsystem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryPortalGroups<Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryPortalGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAssociatedLuns<Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryAssociatedLuns() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePortalGroup<Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePortalGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete() {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFriendlyName(::core::mem::transmute_copy(&pwszfriendlyname)).into()
        }
        unsafe extern "system" fn SetSharedSecret<Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET, pwszinitiatorname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSharedSecret(::core::mem::transmute_copy(&ptargetsharedsecret), ::core::mem::transmute_copy(&pwszinitiatorname)).into()
        }
        unsafe extern "system" fn RememberInitiatorSharedSecret<Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinitiatorname: super::super::Foundation::PWSTR, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RememberInitiatorSharedSecret(::core::mem::transmute_copy(&pwszinitiatorname), ::core::mem::transmute_copy(&pinitiatorsharedsecret)).into()
        }
        unsafe extern "system" fn GetConnectedInitiators<Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppwszinitiatorlist: *mut *mut super::super::Foundation::PWSTR, plnumberofinitiators: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConnectedInitiators(::core::mem::transmute_copy(&pppwszinitiatorlist), ::core::mem::transmute_copy(&plnumberofinitiators)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetProperties: GetProperties::<Impl, IMPL_OFFSET>,
            GetSubSystem: GetSubSystem::<Impl, IMPL_OFFSET>,
            QueryPortalGroups: QueryPortalGroups::<Impl, IMPL_OFFSET>,
            QueryAssociatedLuns: QueryAssociatedLuns::<Impl, IMPL_OFFSET>,
            CreatePortalGroup: CreatePortalGroup::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            SetFriendlyName: SetFriendlyName::<Impl, IMPL_OFFSET>,
            SetSharedSecret: SetSharedSecret::<Impl, IMPL_OFFSET>,
            RememberInitiatorSharedSecret: RememberInitiatorSharedSecret::<Impl, IMPL_OFFSET>,
            GetConnectedInitiators: GetConnectedInitiators::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsIscsiTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsLun_Impl: Sized {
    fn GetProperties(&mut self) -> ::windows::core::Result<VDS_LUN_PROP>;
    fn GetSubSystem(&mut self) -> ::windows::core::Result<IVdsSubSystem>;
    fn GetIdentificationData(&mut self) -> ::windows::core::Result<VDS_LUN_INFORMATION>;
    fn QueryActiveControllers(&mut self) -> ::windows::core::Result<IEnumVdsObject>;
    fn Extend(&mut self, ullnumberofbytestoadd: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32) -> ::windows::core::Result<IVdsAsync>;
    fn Shrink(&mut self, ullnumberofbytestoremove: u64) -> ::windows::core::Result<IVdsAsync>;
    fn QueryPlexes(&mut self) -> ::windows::core::Result<IEnumVdsObject>;
    fn AddPlex(&mut self, lunid: &::windows::core::GUID) -> ::windows::core::Result<IVdsAsync>;
    fn RemovePlex(&mut self, plexid: &::windows::core::GUID) -> ::windows::core::Result<IVdsAsync>;
    fn Recover(&mut self) -> ::windows::core::Result<IVdsAsync>;
    fn SetMask(&mut self, pwszunmaskinglist: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn AssociateControllers(&mut self, pactivecontrolleridarray: *const ::windows::core::GUID, lnumberofactivecontrollers: i32, pinactivecontrolleridarray: *const ::windows::core::GUID, lnumberofinactivecontrollers: i32) -> ::windows::core::Result<()>;
    fn QueryHints(&mut self) -> ::windows::core::Result<VDS_HINTS>;
    fn ApplyHints(&mut self, phints: *const VDS_HINTS) -> ::windows::core::Result<()>;
    fn SetStatus(&mut self, status: VDS_LUN_STATUS) -> ::windows::core::Result<()>;
    fn QueryMaxLunExtendSize(&mut self, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsLun_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsLun_Vtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plunprop: *mut VDS_LUN_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *plunprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubSystem<Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsubsystem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIdentificationData<Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pluninfo: *mut VDS_LUN_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIdentificationData() {
                ::core::result::Result::Ok(ok__) => {
                    *pluninfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryActiveControllers<Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryActiveControllers() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extend<Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullnumberofbytestoadd: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extend(::core::mem::transmute_copy(&ullnumberofbytestoadd), ::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shrink<Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullnumberofbytestoremove: u64, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shrink(::core::mem::transmute_copy(&ullnumberofbytestoremove)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryPlexes<Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryPlexes() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPlex<Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lunid: ::windows::core::GUID, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPlex(::core::mem::transmute_copy(&lunid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlex<Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plexid: ::windows::core::GUID, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemovePlex(::core::mem::transmute_copy(&plexid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recover<Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recover() {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMask<Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszunmaskinglist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMask(::core::mem::transmute_copy(&pwszunmaskinglist)).into()
        }
        unsafe extern "system" fn Delete<Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn AssociateControllers<Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactivecontrolleridarray: *const ::windows::core::GUID, lnumberofactivecontrollers: i32, pinactivecontrolleridarray: *const ::windows::core::GUID, lnumberofinactivecontrollers: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AssociateControllers(::core::mem::transmute_copy(&pactivecontrolleridarray), ::core::mem::transmute_copy(&lnumberofactivecontrollers), ::core::mem::transmute_copy(&pinactivecontrolleridarray), ::core::mem::transmute_copy(&lnumberofinactivecontrollers)).into()
        }
        unsafe extern "system" fn QueryHints<Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints: *mut VDS_HINTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryHints() {
                ::core::result::Result::Ok(ok__) => {
                    *phints = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyHints<Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints: *const VDS_HINTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplyHints(::core::mem::transmute_copy(&phints)).into()
        }
        unsafe extern "system" fn SetStatus<Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_LUN_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn QueryMaxLunExtendSize<Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pullmaxbytestobeadded: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryMaxLunExtendSize(::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives)) {
                ::core::result::Result::Ok(ok__) => {
                    *pullmaxbytestobeadded = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetProperties: GetProperties::<Impl, IMPL_OFFSET>,
            GetSubSystem: GetSubSystem::<Impl, IMPL_OFFSET>,
            GetIdentificationData: GetIdentificationData::<Impl, IMPL_OFFSET>,
            QueryActiveControllers: QueryActiveControllers::<Impl, IMPL_OFFSET>,
            Extend: Extend::<Impl, IMPL_OFFSET>,
            Shrink: Shrink::<Impl, IMPL_OFFSET>,
            QueryPlexes: QueryPlexes::<Impl, IMPL_OFFSET>,
            AddPlex: AddPlex::<Impl, IMPL_OFFSET>,
            RemovePlex: RemovePlex::<Impl, IMPL_OFFSET>,
            Recover: Recover::<Impl, IMPL_OFFSET>,
            SetMask: SetMask::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            AssociateControllers: AssociateControllers::<Impl, IMPL_OFFSET>,
            QueryHints: QueryHints::<Impl, IMPL_OFFSET>,
            ApplyHints: ApplyHints::<Impl, IMPL_OFFSET>,
            SetStatus: SetStatus::<Impl, IMPL_OFFSET>,
            QueryMaxLunExtendSize: QueryMaxLunExtendSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLun as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsLun2_Impl: Sized {
    fn QueryHints2(&mut self) -> ::windows::core::Result<VDS_HINTS2>;
    fn ApplyHints2(&mut self, phints2: *const VDS_HINTS2) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsLun2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsLun2_Vtbl {
        unsafe extern "system" fn QueryHints2<Impl: IVdsLun2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints2: *mut VDS_HINTS2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryHints2() {
                ::core::result::Result::Ok(ok__) => {
                    *phints2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyHints2<Impl: IVdsLun2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints2: *const VDS_HINTS2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplyHints2(::core::mem::transmute_copy(&phints2)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            QueryHints2: QueryHints2::<Impl, IMPL_OFFSET>,
            ApplyHints2: ApplyHints2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLun2 as ::windows::core::Interface>::IID
    }
}
pub trait IVdsLunControllerPorts_Impl: Sized {
    fn AssociateControllerPorts(&mut self, pactivecontrollerportidarray: *const ::windows::core::GUID, lnumberofactivecontrollerports: i32, pinactivecontrollerportidarray: *const ::windows::core::GUID, lnumberofinactivecontrollerports: i32) -> ::windows::core::Result<()>;
    fn QueryActiveControllerPorts(&mut self) -> ::windows::core::Result<IEnumVdsObject>;
}
impl IVdsLunControllerPorts_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunControllerPorts_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsLunControllerPorts_Vtbl {
        unsafe extern "system" fn AssociateControllerPorts<Impl: IVdsLunControllerPorts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactivecontrollerportidarray: *const ::windows::core::GUID, lnumberofactivecontrollerports: i32, pinactivecontrollerportidarray: *const ::windows::core::GUID, lnumberofinactivecontrollerports: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AssociateControllerPorts(::core::mem::transmute_copy(&pactivecontrollerportidarray), ::core::mem::transmute_copy(&lnumberofactivecontrollerports), ::core::mem::transmute_copy(&pinactivecontrollerportidarray), ::core::mem::transmute_copy(&lnumberofinactivecontrollerports)).into()
        }
        unsafe extern "system" fn QueryActiveControllerPorts<Impl: IVdsLunControllerPorts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryActiveControllerPorts() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AssociateControllerPorts: AssociateControllerPorts::<Impl, IMPL_OFFSET>,
            QueryActiveControllerPorts: QueryActiveControllerPorts::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLunControllerPorts as ::windows::core::Interface>::IID
    }
}
pub trait IVdsLunIscsi_Impl: Sized {
    fn AssociateTargets(&mut self, ptargetidarray: *const ::windows::core::GUID, lnumberoftargets: i32) -> ::windows::core::Result<()>;
    fn QueryAssociatedTargets(&mut self) -> ::windows::core::Result<IEnumVdsObject>;
}
impl IVdsLunIscsi_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunIscsi_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsLunIscsi_Vtbl {
        unsafe extern "system" fn AssociateTargets<Impl: IVdsLunIscsi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetidarray: *const ::windows::core::GUID, lnumberoftargets: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AssociateTargets(::core::mem::transmute_copy(&ptargetidarray), ::core::mem::transmute_copy(&lnumberoftargets)).into()
        }
        unsafe extern "system" fn QueryAssociatedTargets<Impl: IVdsLunIscsi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryAssociatedTargets() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AssociateTargets: AssociateTargets::<Impl, IMPL_OFFSET>,
            QueryAssociatedTargets: QueryAssociatedTargets::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLunIscsi as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsLunMpio_Impl: Sized {
    fn GetPathInfo(&mut self, pppaths: *mut *mut VDS_PATH_INFO, plnumberofpaths: *mut i32) -> ::windows::core::Result<()>;
    fn GetLoadBalancePolicy(&mut self, ppolicy: *mut VDS_LOADBALANCE_POLICY_ENUM, pppaths: *mut *mut VDS_PATH_POLICY, plnumberofpaths: *mut i32) -> ::windows::core::Result<()>;
    fn SetLoadBalancePolicy(&mut self, policy: VDS_LOADBALANCE_POLICY_ENUM, ppaths: *const VDS_PATH_POLICY, lnumberofpaths: i32) -> ::windows::core::Result<()>;
    fn GetSupportedLbPolicies(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsLunMpio_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunMpio_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsLunMpio_Vtbl {
        unsafe extern "system" fn GetPathInfo<Impl: IVdsLunMpio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppaths: *mut *mut VDS_PATH_INFO, plnumberofpaths: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPathInfo(::core::mem::transmute_copy(&pppaths), ::core::mem::transmute_copy(&plnumberofpaths)).into()
        }
        unsafe extern "system" fn GetLoadBalancePolicy<Impl: IVdsLunMpio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppolicy: *mut VDS_LOADBALANCE_POLICY_ENUM, pppaths: *mut *mut VDS_PATH_POLICY, plnumberofpaths: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLoadBalancePolicy(::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&pppaths), ::core::mem::transmute_copy(&plnumberofpaths)).into()
        }
        unsafe extern "system" fn SetLoadBalancePolicy<Impl: IVdsLunMpio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: VDS_LOADBALANCE_POLICY_ENUM, ppaths: *const VDS_PATH_POLICY, lnumberofpaths: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLoadBalancePolicy(::core::mem::transmute_copy(&policy), ::core::mem::transmute_copy(&ppaths), ::core::mem::transmute_copy(&lnumberofpaths)).into()
        }
        unsafe extern "system" fn GetSupportedLbPolicies<Impl: IVdsLunMpio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullbflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedLbPolicies() {
                ::core::result::Result::Ok(ok__) => {
                    *pullbflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPathInfo: GetPathInfo::<Impl, IMPL_OFFSET>,
            GetLoadBalancePolicy: GetLoadBalancePolicy::<Impl, IMPL_OFFSET>,
            SetLoadBalancePolicy: SetLoadBalancePolicy::<Impl, IMPL_OFFSET>,
            GetSupportedLbPolicies: GetSupportedLbPolicies::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLunMpio as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsLunNaming_Impl: Sized {
    fn SetFriendlyName(&mut self, pwszfriendlyname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsLunNaming_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunNaming_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsLunNaming_Vtbl {
        unsafe extern "system" fn SetFriendlyName<Impl: IVdsLunNaming_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFriendlyName(::core::mem::transmute_copy(&pwszfriendlyname)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetFriendlyName: SetFriendlyName::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLunNaming as ::windows::core::Interface>::IID
    }
}
pub trait IVdsLunNumber_Impl: Sized {
    fn GetLunNumber(&mut self) -> ::windows::core::Result<u32>;
}
impl IVdsLunNumber_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunNumber_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsLunNumber_Vtbl {
        unsafe extern "system" fn GetLunNumber<Impl: IVdsLunNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullunnumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLunNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pullunnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetLunNumber: GetLunNumber::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLunNumber as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsLunPlex_Impl: Sized {
    fn GetProperties(&mut self) -> ::windows::core::Result<VDS_LUN_PLEX_PROP>;
    fn GetLun(&mut self) -> ::windows::core::Result<IVdsLun>;
    fn QueryExtents(&mut self, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::Result<()>;
    fn QueryHints(&mut self) -> ::windows::core::Result<VDS_HINTS>;
    fn ApplyHints(&mut self, phints: *const VDS_HINTS) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsLunPlex_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunPlex_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsLunPlex_Vtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsLunPlex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplexprop: *mut VDS_LUN_PLEX_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pplexprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLun<Impl: IVdsLunPlex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplun: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLun() {
                ::core::result::Result::Ok(ok__) => {
                    *pplun = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryExtents<Impl: IVdsLunPlex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryExtents(::core::mem::transmute_copy(&ppextentarray), ::core::mem::transmute_copy(&plnumberofextents)).into()
        }
        unsafe extern "system" fn QueryHints<Impl: IVdsLunPlex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints: *mut VDS_HINTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryHints() {
                ::core::result::Result::Ok(ok__) => {
                    *phints = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyHints<Impl: IVdsLunPlex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints: *const VDS_HINTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplyHints(::core::mem::transmute_copy(&phints)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetProperties: GetProperties::<Impl, IMPL_OFFSET>,
            GetLun: GetLun::<Impl, IMPL_OFFSET>,
            QueryExtents: QueryExtents::<Impl, IMPL_OFFSET>,
            QueryHints: QueryHints::<Impl, IMPL_OFFSET>,
            ApplyHints: ApplyHints::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLunPlex as ::windows::core::Interface>::IID
    }
}
pub trait IVdsMaintenance_Impl: Sized {
    fn StartMaintenance(&mut self, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::core::Result<()>;
    fn StopMaintenance(&mut self, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::core::Result<()>;
    fn PulseMaintenance(&mut self, operation: VDS_MAINTENANCE_OPERATION, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IVdsMaintenance_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsMaintenance_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsMaintenance_Vtbl {
        unsafe extern "system" fn StartMaintenance<Impl: IVdsMaintenance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartMaintenance(::core::mem::transmute_copy(&operation)).into()
        }
        unsafe extern "system" fn StopMaintenance<Impl: IVdsMaintenance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopMaintenance(::core::mem::transmute_copy(&operation)).into()
        }
        unsafe extern "system" fn PulseMaintenance<Impl: IVdsMaintenance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PulseMaintenance(::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            StartMaintenance: StartMaintenance::<Impl, IMPL_OFFSET>,
            StopMaintenance: StopMaintenance::<Impl, IMPL_OFFSET>,
            PulseMaintenance: PulseMaintenance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsMaintenance as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsProvider_Impl: Sized {
    fn GetProperties(&mut self) -> ::windows::core::Result<VDS_PROVIDER_PROP>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsProvider_Vtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproviderprop: *mut VDS_PROVIDER_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pproviderprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetProperties: GetProperties::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsProviderPrivate_Impl: Sized {
    fn GetObject(&mut self, objectid: &::windows::core::GUID, r#type: VDS_OBJECT_TYPE) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn OnLoad(&mut self, pwszmachinename: super::super::Foundation::PWSTR, pcallbackobject: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn OnUnload(&mut self, bforceunload: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsProviderPrivate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsProviderPrivate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsProviderPrivate_Vtbl {
        unsafe extern "system" fn GetObject<Impl: IVdsProviderPrivate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: ::windows::core::GUID, r#type: VDS_OBJECT_TYPE, ppobjectunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObject(::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppobjectunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLoad<Impl: IVdsProviderPrivate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszmachinename: super::super::Foundation::PWSTR, pcallbackobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLoad(::core::mem::transmute_copy(&pwszmachinename), ::core::mem::transmute(&pcallbackobject)).into()
        }
        unsafe extern "system" fn OnUnload<Impl: IVdsProviderPrivate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bforceunload: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUnload(::core::mem::transmute_copy(&bforceunload)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetObject: GetObject::<Impl, IMPL_OFFSET>,
            OnLoad: OnLoad::<Impl, IMPL_OFFSET>,
            OnUnload: OnUnload::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsProviderPrivate as ::windows::core::Interface>::IID
    }
}
pub trait IVdsProviderSupport_Impl: Sized {
    fn GetVersionSupport(&mut self) -> ::windows::core::Result<u32>;
}
impl IVdsProviderSupport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsProviderSupport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsProviderSupport_Vtbl {
        unsafe extern "system" fn GetVersionSupport<Impl: IVdsProviderSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulversionsupport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVersionSupport() {
                ::core::result::Result::Ok(ok__) => {
                    *ulversionsupport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetVersionSupport: GetVersionSupport::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsProviderSupport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsStoragePool_Impl: Sized {
    fn GetProvider(&mut self) -> ::windows::core::Result<IVdsProvider>;
    fn GetProperties(&mut self) -> ::windows::core::Result<VDS_STORAGE_POOL_PROP>;
    fn GetAttributes(&mut self) -> ::windows::core::Result<VDS_POOL_ATTRIBUTES>;
    fn QueryDriveExtents(&mut self, ppextentarray: *mut *mut VDS_STORAGE_POOL_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::Result<()>;
    fn QueryAllocatedLuns(&mut self) -> ::windows::core::Result<IEnumVdsObject>;
    fn QueryAllocatedStoragePools(&mut self) -> ::windows::core::Result<IEnumVdsObject>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsStoragePool_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsStoragePool_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsStoragePool_Vtbl {
        unsafe extern "system" fn GetProvider<Impl: IVdsStoragePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Impl: IVdsStoragePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstoragepoolprop: *mut VDS_STORAGE_POOL_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pstoragepoolprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributes<Impl: IVdsStoragePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstoragepoolattributes: *mut VDS_POOL_ATTRIBUTES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *pstoragepoolattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDriveExtents<Impl: IVdsStoragePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_STORAGE_POOL_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryDriveExtents(::core::mem::transmute_copy(&ppextentarray), ::core::mem::transmute_copy(&plnumberofextents)).into()
        }
        unsafe extern "system" fn QueryAllocatedLuns<Impl: IVdsStoragePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryAllocatedLuns() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAllocatedStoragePools<Impl: IVdsStoragePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryAllocatedStoragePools() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetProvider: GetProvider::<Impl, IMPL_OFFSET>,
            GetProperties: GetProperties::<Impl, IMPL_OFFSET>,
            GetAttributes: GetAttributes::<Impl, IMPL_OFFSET>,
            QueryDriveExtents: QueryDriveExtents::<Impl, IMPL_OFFSET>,
            QueryAllocatedLuns: QueryAllocatedLuns::<Impl, IMPL_OFFSET>,
            QueryAllocatedStoragePools: QueryAllocatedStoragePools::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsStoragePool as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsSubSystem_Impl: Sized {
    fn GetProperties(&mut self) -> ::windows::core::Result<VDS_SUB_SYSTEM_PROP>;
    fn GetProvider(&mut self) -> ::windows::core::Result<IVdsProvider>;
    fn QueryControllers(&mut self) -> ::windows::core::Result<IEnumVdsObject>;
    fn QueryLuns(&mut self) -> ::windows::core::Result<IEnumVdsObject>;
    fn QueryDrives(&mut self) -> ::windows::core::Result<IEnumVdsObject>;
    fn GetDrive(&mut self, sbusnumber: i16, sslotnumber: i16) -> ::windows::core::Result<IVdsDrive>;
    fn Reenumerate(&mut self) -> ::windows::core::Result<()>;
    fn SetControllerStatus(&mut self, ponlinecontrolleridarray: *const ::windows::core::GUID, lnumberofonlinecontrollers: i32, pofflinecontrolleridarray: *const ::windows::core::GUID, lnumberofofflinecontrollers: i32) -> ::windows::core::Result<()>;
    fn CreateLun(&mut self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pwszunmaskinglist: super::super::Foundation::PWSTR, phints: *const VDS_HINTS) -> ::windows::core::Result<IVdsAsync>;
    fn ReplaceDrive(&mut self, drivetobereplaced: &::windows::core::GUID, replacementdrive: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetStatus(&mut self, status: VDS_SUB_SYSTEM_STATUS) -> ::windows::core::Result<()>;
    fn QueryMaxLunCreateSize(&mut self, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, phints: *const VDS_HINTS) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsSubSystem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsSubSystem_Vtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubsystemprop: *mut VDS_SUB_SYSTEM_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *psubsystemprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProvider<Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryControllers<Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryControllers() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryLuns<Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryLuns() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDrives<Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryDrives() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDrive<Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sbusnumber: i16, sslotnumber: i16, ppdrive: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDrive(::core::mem::transmute_copy(&sbusnumber), ::core::mem::transmute_copy(&sslotnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdrive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reenumerate<Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reenumerate().into()
        }
        unsafe extern "system" fn SetControllerStatus<Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ponlinecontrolleridarray: *const ::windows::core::GUID, lnumberofonlinecontrollers: i32, pofflinecontrolleridarray: *const ::windows::core::GUID, lnumberofofflinecontrollers: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetControllerStatus(::core::mem::transmute_copy(&ponlinecontrolleridarray), ::core::mem::transmute_copy(&lnumberofonlinecontrollers), ::core::mem::transmute_copy(&pofflinecontrolleridarray), ::core::mem::transmute_copy(&lnumberofofflinecontrollers)).into()
        }
        unsafe extern "system" fn CreateLun<Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pwszunmaskinglist: super::super::Foundation::PWSTR, phints: *const VDS_HINTS, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLun(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&ullsizeinbytes), ::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives), ::core::mem::transmute_copy(&pwszunmaskinglist), ::core::mem::transmute_copy(&phints)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceDrive<Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drivetobereplaced: ::windows::core::GUID, replacementdrive: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReplaceDrive(::core::mem::transmute_copy(&drivetobereplaced), ::core::mem::transmute_copy(&replacementdrive)).into()
        }
        unsafe extern "system" fn SetStatus<Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_SUB_SYSTEM_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn QueryMaxLunCreateSize<Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, phints: *const VDS_HINTS, pullmaxlunsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryMaxLunCreateSize(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives), ::core::mem::transmute_copy(&phints)) {
                ::core::result::Result::Ok(ok__) => {
                    *pullmaxlunsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetProperties: GetProperties::<Impl, IMPL_OFFSET>,
            GetProvider: GetProvider::<Impl, IMPL_OFFSET>,
            QueryControllers: QueryControllers::<Impl, IMPL_OFFSET>,
            QueryLuns: QueryLuns::<Impl, IMPL_OFFSET>,
            QueryDrives: QueryDrives::<Impl, IMPL_OFFSET>,
            GetDrive: GetDrive::<Impl, IMPL_OFFSET>,
            Reenumerate: Reenumerate::<Impl, IMPL_OFFSET>,
            SetControllerStatus: SetControllerStatus::<Impl, IMPL_OFFSET>,
            CreateLun: CreateLun::<Impl, IMPL_OFFSET>,
            ReplaceDrive: ReplaceDrive::<Impl, IMPL_OFFSET>,
            SetStatus: SetStatus::<Impl, IMPL_OFFSET>,
            QueryMaxLunCreateSize: QueryMaxLunCreateSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsSubSystem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsSubSystem2_Impl: Sized {
    fn GetProperties2(&mut self) -> ::windows::core::Result<VDS_SUB_SYSTEM_PROP2>;
    fn GetDrive2(&mut self, sbusnumber: i16, sslotnumber: i16, ulenclosurenumber: u32) -> ::windows::core::Result<IVdsDrive>;
    fn CreateLun2(&mut self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pwszunmaskinglist: super::super::Foundation::PWSTR, phints2: *const VDS_HINTS2) -> ::windows::core::Result<IVdsAsync>;
    fn QueryMaxLunCreateSize2(&mut self, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, phints2: *const VDS_HINTS2) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsSubSystem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystem2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsSubSystem2_Vtbl {
        unsafe extern "system" fn GetProperties2<Impl: IVdsSubSystem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubsystemprop2: *mut VDS_SUB_SYSTEM_PROP2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties2() {
                ::core::result::Result::Ok(ok__) => {
                    *psubsystemprop2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDrive2<Impl: IVdsSubSystem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sbusnumber: i16, sslotnumber: i16, ulenclosurenumber: u32, ppdrive: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDrive2(::core::mem::transmute_copy(&sbusnumber), ::core::mem::transmute_copy(&sslotnumber), ::core::mem::transmute_copy(&ulenclosurenumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdrive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLun2<Impl: IVdsSubSystem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pwszunmaskinglist: super::super::Foundation::PWSTR, phints2: *const VDS_HINTS2, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLun2(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&ullsizeinbytes), ::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives), ::core::mem::transmute_copy(&pwszunmaskinglist), ::core::mem::transmute_copy(&phints2)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryMaxLunCreateSize2<Impl: IVdsSubSystem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, phints2: *const VDS_HINTS2, pullmaxlunsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryMaxLunCreateSize2(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives), ::core::mem::transmute_copy(&phints2)) {
                ::core::result::Result::Ok(ok__) => {
                    *pullmaxlunsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetProperties2: GetProperties2::<Impl, IMPL_OFFSET>,
            GetDrive2: GetDrive2::<Impl, IMPL_OFFSET>,
            CreateLun2: CreateLun2::<Impl, IMPL_OFFSET>,
            QueryMaxLunCreateSize2: QueryMaxLunCreateSize2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsSubSystem2 as ::windows::core::Interface>::IID
    }
}
pub trait IVdsSubSystemInterconnect_Impl: Sized {
    fn GetSupportedInterconnects(&mut self) -> ::windows::core::Result<u32>;
}
impl IVdsSubSystemInterconnect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystemInterconnect_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsSubSystemInterconnect_Vtbl {
        unsafe extern "system" fn GetSupportedInterconnects<Impl: IVdsSubSystemInterconnect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulsupportedinterconnectsflag: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedInterconnects() {
                ::core::result::Result::Ok(ok__) => {
                    *pulsupportedinterconnectsflag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetSupportedInterconnects: GetSupportedInterconnects::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsSubSystemInterconnect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsSubSystemIscsi_Impl: Sized {
    fn QueryTargets(&mut self) -> ::windows::core::Result<IEnumVdsObject>;
    fn QueryPortals(&mut self) -> ::windows::core::Result<IEnumVdsObject>;
    fn CreateTarget(&mut self, pwsziscsiname: super::super::Foundation::PWSTR, pwszfriendlyname: super::super::Foundation::PWSTR) -> ::windows::core::Result<IVdsAsync>;
    fn SetIpsecGroupPresharedKey(&mut self, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsSubSystemIscsi_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystemIscsi_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsSubSystemIscsi_Vtbl {
        unsafe extern "system" fn QueryTargets<Impl: IVdsSubSystemIscsi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryTargets() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryPortals<Impl: IVdsSubSystemIscsi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryPortals() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTarget<Impl: IVdsSubSystemIscsi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsziscsiname: super::super::Foundation::PWSTR, pwszfriendlyname: super::super::Foundation::PWSTR, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTarget(::core::mem::transmute_copy(&pwsziscsiname), ::core::mem::transmute_copy(&pwszfriendlyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpsecGroupPresharedKey<Impl: IVdsSubSystemIscsi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIpsecGroupPresharedKey(::core::mem::transmute_copy(&pipseckey)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            QueryTargets: QueryTargets::<Impl, IMPL_OFFSET>,
            QueryPortals: QueryPortals::<Impl, IMPL_OFFSET>,
            CreateTarget: CreateTarget::<Impl, IMPL_OFFSET>,
            SetIpsecGroupPresharedKey: SetIpsecGroupPresharedKey::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsSubSystemIscsi as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsSubSystemNaming_Impl: Sized {
    fn SetFriendlyName(&mut self, pwszfriendlyname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsSubSystemNaming_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystemNaming_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsSubSystemNaming_Vtbl {
        unsafe extern "system" fn SetFriendlyName<Impl: IVdsSubSystemNaming_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFriendlyName(::core::mem::transmute_copy(&pwszfriendlyname)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetFriendlyName: SetFriendlyName::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsSubSystemNaming as ::windows::core::Interface>::IID
    }
}
