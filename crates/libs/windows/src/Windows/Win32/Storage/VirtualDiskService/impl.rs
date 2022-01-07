pub trait IEnumVdsObjectImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumVdsObject {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IEnumVdsObject";
}
impl IEnumVdsObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumVdsObjectImpl, const OFFSET: isize>() -> IEnumVdsObjectVtbl {
        unsafe extern "system" fn Next<Impl: IEnumVdsObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppobjectarray: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppobjectarray), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumVdsObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumVdsObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumVdsObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumVdsObject>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IVdsAdminImpl: Sized {
    fn RegisterProvider();
    fn UnregisterProvider();
}
impl ::windows::core::RuntimeName for IVdsAdmin {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsAdmin";
}
impl IVdsAdminVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsAdminImpl, const OFFSET: isize>() -> IVdsAdminVtbl {
        unsafe extern "system" fn RegisterProvider<Impl: IVdsAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, providerclsid: ::windows::core::GUID, pwszname: super::super::Foundation::PWSTR, r#type: VDS_PROVIDER_TYPE, pwszmachinename: super::super::Foundation::PWSTR, pwszversion: super::super::Foundation::PWSTR, guidversionid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterProvider(
                &*(&providerid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&providerclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                r#type,
                &*(&pwszmachinename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszversion as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&guidversionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterProvider<Impl: IVdsAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterProvider(&*(&providerid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsAdmin>, ::windows::core::GetTrustLevel, RegisterProvider::<Impl, OFFSET>, UnregisterProvider::<Impl, OFFSET>)
    }
}
pub trait IVdsAdviseSinkImpl: Sized {
    fn OnNotify();
}
impl ::windows::core::RuntimeName for IVdsAdviseSink {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsAdviseSink";
}
impl IVdsAdviseSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsAdviseSinkImpl, const OFFSET: isize>() -> IVdsAdviseSinkVtbl {
        unsafe extern "system" fn OnNotify<Impl: IVdsAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnumberofnotifications: i32, pnotificationarray: *const VDS_NOTIFICATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnNotify(lnumberofnotifications, &*(&pnotificationarray as *const <VDS_NOTIFICATION as ::windows::core::Abi>::Abi as *const <VDS_NOTIFICATION as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsAdviseSink>, ::windows::core::GetTrustLevel, OnNotify::<Impl, OFFSET>)
    }
}
pub trait IVdsAsyncImpl: Sized {
    fn Cancel();
    fn Wait();
    fn QueryStatus();
}
impl ::windows::core::RuntimeName for IVdsAsync {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsAsync";
}
impl IVdsAsyncVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsAsyncImpl, const OFFSET: isize>() -> IVdsAsyncVtbl {
        unsafe extern "system" fn Cancel<Impl: IVdsAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Wait<Impl: IVdsAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT, pasyncout: *mut VDS_ASYNC_OUTPUT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Wait(::core::mem::transmute_copy(&phrresult), ::core::mem::transmute_copy(&pasyncout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryStatus<Impl: IVdsAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT, pulpercentcompleted: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryStatus(::core::mem::transmute_copy(&phrresult), ::core::mem::transmute_copy(&pulpercentcompleted)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsAsync>, ::windows::core::GetTrustLevel, Cancel::<Impl, OFFSET>, Wait::<Impl, OFFSET>, QueryStatus::<Impl, OFFSET>)
    }
}
pub trait IVdsControllerImpl: Sized {
    fn GetProperties();
    fn GetSubSystem();
    fn GetPortProperties();
    fn FlushCache();
    fn InvalidateCache();
    fn Reset();
    fn QueryAssociatedLuns();
    fn SetStatus();
}
impl ::windows::core::RuntimeName for IVdsController {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsController";
}
impl IVdsControllerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsControllerImpl, const OFFSET: isize>() -> IVdsControllerVtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontrollerprop: *mut VDS_CONTROLLER_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties(::core::mem::transmute_copy(&pcontrollerprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubSystem<Impl: IVdsControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubSystem(::core::mem::transmute_copy(&ppsubsystem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPortProperties<Impl: IVdsControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sportnumber: i16, pportprop: *mut VDS_PORT_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPortProperties(sportnumber, ::core::mem::transmute_copy(&pportprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushCache<Impl: IVdsControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlushCache() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidateCache<Impl: IVdsControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvalidateCache() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IVdsControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAssociatedLuns<Impl: IVdsControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryAssociatedLuns(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Impl: IVdsControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_CONTROLLER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStatus(status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IVdsController>,
            ::windows::core::GetTrustLevel,
            GetProperties::<Impl, OFFSET>,
            GetSubSystem::<Impl, OFFSET>,
            GetPortProperties::<Impl, OFFSET>,
            FlushCache::<Impl, OFFSET>,
            InvalidateCache::<Impl, OFFSET>,
            Reset::<Impl, OFFSET>,
            QueryAssociatedLuns::<Impl, OFFSET>,
            SetStatus::<Impl, OFFSET>,
        )
    }
}
pub trait IVdsControllerControllerPortImpl: Sized {
    fn QueryControllerPorts();
}
impl ::windows::core::RuntimeName for IVdsControllerControllerPort {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsControllerControllerPort";
}
impl IVdsControllerControllerPortVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsControllerControllerPortImpl, const OFFSET: isize>() -> IVdsControllerControllerPortVtbl {
        unsafe extern "system" fn QueryControllerPorts<Impl: IVdsControllerControllerPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryControllerPorts(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsControllerControllerPort>, ::windows::core::GetTrustLevel, QueryControllerPorts::<Impl, OFFSET>)
    }
}
pub trait IVdsControllerPortImpl: Sized {
    fn GetProperties();
    fn GetController();
    fn QueryAssociatedLuns();
    fn Reset();
    fn SetStatus();
}
impl ::windows::core::RuntimeName for IVdsControllerPort {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsControllerPort";
}
impl IVdsControllerPortVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsControllerPortImpl, const OFFSET: isize>() -> IVdsControllerPortVtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsControllerPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportprop: *mut VDS_PORT_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties(::core::mem::transmute_copy(&pportprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetController<Impl: IVdsControllerPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontroller: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetController(::core::mem::transmute_copy(&ppcontroller)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAssociatedLuns<Impl: IVdsControllerPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryAssociatedLuns(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IVdsControllerPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Impl: IVdsControllerPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_PORT_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStatus(status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsControllerPort>, ::windows::core::GetTrustLevel, GetProperties::<Impl, OFFSET>, GetController::<Impl, OFFSET>, QueryAssociatedLuns::<Impl, OFFSET>, Reset::<Impl, OFFSET>, SetStatus::<Impl, OFFSET>)
    }
}
pub trait IVdsDriveImpl: Sized {
    fn GetProperties();
    fn GetSubSystem();
    fn QueryExtents();
    fn SetFlags();
    fn ClearFlags();
    fn SetStatus();
}
impl ::windows::core::RuntimeName for IVdsDrive {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsDrive";
}
impl IVdsDriveVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsDriveImpl, const OFFSET: isize>() -> IVdsDriveVtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsDriveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdriveprop: *mut VDS_DRIVE_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties(::core::mem::transmute_copy(&pdriveprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubSystem<Impl: IVdsDriveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubSystem(::core::mem::transmute_copy(&ppsubsystem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryExtents<Impl: IVdsDriveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryExtents(::core::mem::transmute_copy(&ppextentarray), ::core::mem::transmute_copy(&plnumberofextents)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Impl: IVdsDriveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFlags(ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearFlags<Impl: IVdsDriveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearFlags(ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Impl: IVdsDriveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_DRIVE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStatus(status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsDrive>, ::windows::core::GetTrustLevel, GetProperties::<Impl, OFFSET>, GetSubSystem::<Impl, OFFSET>, QueryExtents::<Impl, OFFSET>, SetFlags::<Impl, OFFSET>, ClearFlags::<Impl, OFFSET>, SetStatus::<Impl, OFFSET>)
    }
}
pub trait IVdsDrive2Impl: Sized {
    fn GetProperties2();
}
impl ::windows::core::RuntimeName for IVdsDrive2 {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsDrive2";
}
impl IVdsDrive2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsDrive2Impl, const OFFSET: isize>() -> IVdsDrive2Vtbl {
        unsafe extern "system" fn GetProperties2<Impl: IVdsDrive2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdriveprop2: *mut VDS_DRIVE_PROP2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties2(::core::mem::transmute_copy(&pdriveprop2)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsDrive2>, ::windows::core::GetTrustLevel, GetProperties2::<Impl, OFFSET>)
    }
}
pub trait IVdsHwProviderImpl: Sized {
    fn QuerySubSystems();
    fn Reenumerate();
    fn Refresh();
}
impl ::windows::core::RuntimeName for IVdsHwProvider {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsHwProvider";
}
impl IVdsHwProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderImpl, const OFFSET: isize>() -> IVdsHwProviderVtbl {
        unsafe extern "system" fn QuerySubSystems<Impl: IVdsHwProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuerySubSystems(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reenumerate<Impl: IVdsHwProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reenumerate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IVdsHwProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsHwProvider>, ::windows::core::GetTrustLevel, QuerySubSystems::<Impl, OFFSET>, Reenumerate::<Impl, OFFSET>, Refresh::<Impl, OFFSET>)
    }
}
pub trait IVdsHwProviderPrivateImpl: Sized {
    fn QueryIfCreatedLun();
}
impl ::windows::core::RuntimeName for IVdsHwProviderPrivate {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsHwProviderPrivate";
}
impl IVdsHwProviderPrivateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderPrivateImpl, const OFFSET: isize>() -> IVdsHwProviderPrivateVtbl {
        unsafe extern "system" fn QueryIfCreatedLun<Impl: IVdsHwProviderPrivateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicepath: super::super::Foundation::PWSTR, pvdsluninformation: *const VDS_LUN_INFORMATION, plunid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryIfCreatedLun(&*(&pwszdevicepath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pvdsluninformation as *const <VDS_LUN_INFORMATION as ::windows::core::Abi>::Abi as *const <VDS_LUN_INFORMATION as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&plunid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsHwProviderPrivate>, ::windows::core::GetTrustLevel, QueryIfCreatedLun::<Impl, OFFSET>)
    }
}
pub trait IVdsHwProviderPrivateMpioImpl: Sized {
    fn SetAllPathStatusesFromHbaPort();
}
impl ::windows::core::RuntimeName for IVdsHwProviderPrivateMpio {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsHwProviderPrivateMpio";
}
impl IVdsHwProviderPrivateMpioVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderPrivateMpioImpl, const OFFSET: isize>() -> IVdsHwProviderPrivateMpioVtbl {
        unsafe extern "system" fn SetAllPathStatusesFromHbaPort<Impl: IVdsHwProviderPrivateMpioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hbaportprop: VDS_HBAPORT_PROP, status: VDS_PATH_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllPathStatusesFromHbaPort(&*(&hbaportprop as *const <VDS_HBAPORT_PROP as ::windows::core::Abi>::Abi as *const <VDS_HBAPORT_PROP as ::windows::core::DefaultType>::DefaultType), status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsHwProviderPrivateMpio>, ::windows::core::GetTrustLevel, SetAllPathStatusesFromHbaPort::<Impl, OFFSET>)
    }
}
pub trait IVdsHwProviderStoragePoolsImpl: Sized {
    fn QueryStoragePools();
    fn CreateLunInStoragePool();
    fn QueryMaxLunCreateSizeInStoragePool();
}
impl ::windows::core::RuntimeName for IVdsHwProviderStoragePools {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsHwProviderStoragePools";
}
impl IVdsHwProviderStoragePoolsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderStoragePoolsImpl, const OFFSET: isize>() -> IVdsHwProviderStoragePoolsVtbl {
        unsafe extern "system" fn QueryStoragePools<Impl: IVdsHwProviderStoragePoolsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, ullremainingfreespace: u64, ppoolattributes: *const VDS_POOL_ATTRIBUTES, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryStoragePools(ulflags, ullremainingfreespace, &*(&ppoolattributes as *const <VDS_POOL_ATTRIBUTES as ::windows::core::Abi>::Abi as *const <VDS_POOL_ATTRIBUTES as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLunInStoragePool<Impl: IVdsHwProviderStoragePoolsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, storagepoolid: ::windows::core::GUID, pwszunmaskinglist: super::super::Foundation::PWSTR, phints2: *const VDS_HINTS2, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLunInStoragePool(
                r#type,
                ullsizeinbytes,
                &*(&storagepoolid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszunmaskinglist as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&phints2 as *const <VDS_HINTS2 as ::windows::core::Abi>::Abi as *const <VDS_HINTS2 as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppasync),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryMaxLunCreateSizeInStoragePool<Impl: IVdsHwProviderStoragePoolsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, storagepoolid: ::windows::core::GUID, phints2: *const VDS_HINTS2, pullmaxlunsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryMaxLunCreateSizeInStoragePool(r#type, &*(&storagepoolid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&phints2 as *const <VDS_HINTS2 as ::windows::core::Abi>::Abi as *const <VDS_HINTS2 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pullmaxlunsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsHwProviderStoragePools>, ::windows::core::GetTrustLevel, QueryStoragePools::<Impl, OFFSET>, CreateLunInStoragePool::<Impl, OFFSET>, QueryMaxLunCreateSizeInStoragePool::<Impl, OFFSET>)
    }
}
pub trait IVdsHwProviderTypeImpl: Sized {
    fn GetProviderType();
}
impl ::windows::core::RuntimeName for IVdsHwProviderType {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsHwProviderType";
}
impl IVdsHwProviderTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderTypeImpl, const OFFSET: isize>() -> IVdsHwProviderTypeVtbl {
        unsafe extern "system" fn GetProviderType<Impl: IVdsHwProviderTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut VDS_HWPROVIDER_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderType(::core::mem::transmute_copy(&ptype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsHwProviderType>, ::windows::core::GetTrustLevel, GetProviderType::<Impl, OFFSET>)
    }
}
pub trait IVdsHwProviderType2Impl: Sized {
    fn GetProviderType2();
}
impl ::windows::core::RuntimeName for IVdsHwProviderType2 {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsHwProviderType2";
}
impl IVdsHwProviderType2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderType2Impl, const OFFSET: isize>() -> IVdsHwProviderType2Vtbl {
        unsafe extern "system" fn GetProviderType2<Impl: IVdsHwProviderType2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut VDS_HWPROVIDER_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderType2(::core::mem::transmute_copy(&ptype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsHwProviderType2>, ::windows::core::GetTrustLevel, GetProviderType2::<Impl, OFFSET>)
    }
}
pub trait IVdsIscsiPortalImpl: Sized {
    fn GetProperties();
    fn GetSubSystem();
    fn QueryAssociatedPortalGroups();
    fn SetStatus();
    fn SetIpsecTunnelAddress();
    fn GetIpsecSecurity();
    fn SetIpsecSecurity();
}
impl ::windows::core::RuntimeName for IVdsIscsiPortal {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsIscsiPortal";
}
impl IVdsIscsiPortalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiPortalImpl, const OFFSET: isize>() -> IVdsIscsiPortalVtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsIscsiPortalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportalprop: *mut VDS_ISCSI_PORTAL_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties(::core::mem::transmute_copy(&pportalprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubSystem<Impl: IVdsIscsiPortalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubSystem(::core::mem::transmute_copy(&ppsubsystem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAssociatedPortalGroups<Impl: IVdsIscsiPortalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryAssociatedPortalGroups(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Impl: IVdsIscsiPortalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_ISCSI_PORTAL_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStatus(status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpsecTunnelAddress<Impl: IVdsIscsiPortalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIpsecTunnelAddress(&*(&ptunneladdress as *const <VDS_IPADDRESS as ::windows::core::Abi>::Abi as *const <VDS_IPADDRESS as ::windows::core::DefaultType>::DefaultType), &*(&pdestinationaddress as *const <VDS_IPADDRESS as ::windows::core::Abi>::Abi as *const <VDS_IPADDRESS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIpsecSecurity<Impl: IVdsIscsiPortalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitiatorportaladdress: *const VDS_IPADDRESS, pullsecurityflags: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIpsecSecurity(&*(&pinitiatorportaladdress as *const <VDS_IPADDRESS as ::windows::core::Abi>::Abi as *const <VDS_IPADDRESS as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pullsecurityflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpsecSecurity<Impl: IVdsIscsiPortalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitiatorportaladdress: *const VDS_IPADDRESS, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIpsecSecurity(&*(&pinitiatorportaladdress as *const <VDS_IPADDRESS as ::windows::core::Abi>::Abi as *const <VDS_IPADDRESS as ::windows::core::DefaultType>::DefaultType), ullsecurityflags, &*(&pipseckey as *const <VDS_ISCSI_IPSEC_KEY as ::windows::core::Abi>::Abi as *const <VDS_ISCSI_IPSEC_KEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IVdsIscsiPortal>,
            ::windows::core::GetTrustLevel,
            GetProperties::<Impl, OFFSET>,
            GetSubSystem::<Impl, OFFSET>,
            QueryAssociatedPortalGroups::<Impl, OFFSET>,
            SetStatus::<Impl, OFFSET>,
            SetIpsecTunnelAddress::<Impl, OFFSET>,
            GetIpsecSecurity::<Impl, OFFSET>,
            SetIpsecSecurity::<Impl, OFFSET>,
        )
    }
}
pub trait IVdsIscsiPortalGroupImpl: Sized {
    fn GetProperties();
    fn GetTarget();
    fn QueryAssociatedPortals();
    fn AddPortal();
    fn RemovePortal();
    fn Delete();
}
impl ::windows::core::RuntimeName for IVdsIscsiPortalGroup {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsIscsiPortalGroup";
}
impl IVdsIscsiPortalGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiPortalGroupImpl, const OFFSET: isize>() -> IVdsIscsiPortalGroupVtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsIscsiPortalGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportalgroupprop: *mut VDS_ISCSI_PORTALGROUP_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties(::core::mem::transmute_copy(&pportalgroupprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTarget<Impl: IVdsIscsiPortalGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTarget(::core::mem::transmute_copy(&pptarget)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAssociatedPortals<Impl: IVdsIscsiPortalGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryAssociatedPortals(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPortal<Impl: IVdsIscsiPortalGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portalid: ::windows::core::GUID, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPortal(&*(&portalid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppasync)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePortal<Impl: IVdsIscsiPortalGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portalid: ::windows::core::GUID, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemovePortal(&*(&portalid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppasync)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IVdsIscsiPortalGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete(::core::mem::transmute_copy(&ppasync)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsIscsiPortalGroup>, ::windows::core::GetTrustLevel, GetProperties::<Impl, OFFSET>, GetTarget::<Impl, OFFSET>, QueryAssociatedPortals::<Impl, OFFSET>, AddPortal::<Impl, OFFSET>, RemovePortal::<Impl, OFFSET>, Delete::<Impl, OFFSET>)
    }
}
pub trait IVdsIscsiTargetImpl: Sized {
    fn GetProperties();
    fn GetSubSystem();
    fn QueryPortalGroups();
    fn QueryAssociatedLuns();
    fn CreatePortalGroup();
    fn Delete();
    fn SetFriendlyName();
    fn SetSharedSecret();
    fn RememberInitiatorSharedSecret();
    fn GetConnectedInitiators();
}
impl ::windows::core::RuntimeName for IVdsIscsiTarget {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsIscsiTarget";
}
impl IVdsIscsiTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiTargetImpl, const OFFSET: isize>() -> IVdsIscsiTargetVtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsIscsiTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetprop: *mut VDS_ISCSI_TARGET_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties(::core::mem::transmute_copy(&ptargetprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubSystem<Impl: IVdsIscsiTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubSystem(::core::mem::transmute_copy(&ppsubsystem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryPortalGroups<Impl: IVdsIscsiTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryPortalGroups(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAssociatedLuns<Impl: IVdsIscsiTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryAssociatedLuns(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePortalGroup<Impl: IVdsIscsiTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePortalGroup(::core::mem::transmute_copy(&ppasync)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IVdsIscsiTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete(::core::mem::transmute_copy(&ppasync)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Impl: IVdsIscsiTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFriendlyName(&*(&pwszfriendlyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSharedSecret<Impl: IVdsIscsiTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET, pwszinitiatorname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSharedSecret(&*(&ptargetsharedsecret as *const <VDS_ISCSI_SHARED_SECRET as ::windows::core::Abi>::Abi as *const <VDS_ISCSI_SHARED_SECRET as ::windows::core::DefaultType>::DefaultType), &*(&pwszinitiatorname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RememberInitiatorSharedSecret<Impl: IVdsIscsiTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinitiatorname: super::super::Foundation::PWSTR, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RememberInitiatorSharedSecret(&*(&pwszinitiatorname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pinitiatorsharedsecret as *const <VDS_ISCSI_SHARED_SECRET as ::windows::core::Abi>::Abi as *const <VDS_ISCSI_SHARED_SECRET as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectedInitiators<Impl: IVdsIscsiTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppwszinitiatorlist: *mut *mut super::super::Foundation::PWSTR, plnumberofinitiators: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectedInitiators(::core::mem::transmute_copy(&pppwszinitiatorlist), ::core::mem::transmute_copy(&plnumberofinitiators)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IVdsIscsiTarget>,
            ::windows::core::GetTrustLevel,
            GetProperties::<Impl, OFFSET>,
            GetSubSystem::<Impl, OFFSET>,
            QueryPortalGroups::<Impl, OFFSET>,
            QueryAssociatedLuns::<Impl, OFFSET>,
            CreatePortalGroup::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
            SetFriendlyName::<Impl, OFFSET>,
            SetSharedSecret::<Impl, OFFSET>,
            RememberInitiatorSharedSecret::<Impl, OFFSET>,
            GetConnectedInitiators::<Impl, OFFSET>,
        )
    }
}
pub trait IVdsLunImpl: Sized {
    fn GetProperties();
    fn GetSubSystem();
    fn GetIdentificationData();
    fn QueryActiveControllers();
    fn Extend();
    fn Shrink();
    fn QueryPlexes();
    fn AddPlex();
    fn RemovePlex();
    fn Recover();
    fn SetMask();
    fn Delete();
    fn AssociateControllers();
    fn QueryHints();
    fn ApplyHints();
    fn SetStatus();
    fn QueryMaxLunExtendSize();
}
impl ::windows::core::RuntimeName for IVdsLun {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsLun";
}
impl IVdsLunVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunImpl, const OFFSET: isize>() -> IVdsLunVtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plunprop: *mut VDS_LUN_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties(::core::mem::transmute_copy(&plunprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubSystem<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubSystem(::core::mem::transmute_copy(&ppsubsystem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIdentificationData<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pluninfo: *mut VDS_LUN_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIdentificationData(::core::mem::transmute_copy(&pluninfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryActiveControllers<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryActiveControllers(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extend<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullnumberofbytestoadd: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extend(ullnumberofbytestoadd, &*(&pdriveidarray as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), lnumberofdrives, ::core::mem::transmute_copy(&ppasync)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shrink<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullnumberofbytestoremove: u64, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shrink(ullnumberofbytestoremove, ::core::mem::transmute_copy(&ppasync)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryPlexes<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryPlexes(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPlex<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lunid: ::windows::core::GUID, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPlex(&*(&lunid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppasync)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlex<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plexid: ::windows::core::GUID, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemovePlex(&*(&plexid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppasync)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recover<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recover(::core::mem::transmute_copy(&ppasync)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMask<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszunmaskinglist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMask(&*(&pwszunmaskinglist as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssociateControllers<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactivecontrolleridarray: *const ::windows::core::GUID, lnumberofactivecontrollers: i32, pinactivecontrolleridarray: *const ::windows::core::GUID, lnumberofinactivecontrollers: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssociateControllers(&*(&pactivecontrolleridarray as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), lnumberofactivecontrollers, &*(&pinactivecontrolleridarray as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), lnumberofinactivecontrollers) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHints<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints: *mut VDS_HINTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryHints(::core::mem::transmute_copy(&phints)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyHints<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints: *const VDS_HINTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplyHints(&*(&phints as *const <VDS_HINTS as ::windows::core::Abi>::Abi as *const <VDS_HINTS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_LUN_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStatus(status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryMaxLunExtendSize<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pullmaxbytestobeadded: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryMaxLunExtendSize(&*(&pdriveidarray as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), lnumberofdrives, ::core::mem::transmute_copy(&pullmaxbytestobeadded)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IVdsLun>,
            ::windows::core::GetTrustLevel,
            GetProperties::<Impl, OFFSET>,
            GetSubSystem::<Impl, OFFSET>,
            GetIdentificationData::<Impl, OFFSET>,
            QueryActiveControllers::<Impl, OFFSET>,
            Extend::<Impl, OFFSET>,
            Shrink::<Impl, OFFSET>,
            QueryPlexes::<Impl, OFFSET>,
            AddPlex::<Impl, OFFSET>,
            RemovePlex::<Impl, OFFSET>,
            Recover::<Impl, OFFSET>,
            SetMask::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
            AssociateControllers::<Impl, OFFSET>,
            QueryHints::<Impl, OFFSET>,
            ApplyHints::<Impl, OFFSET>,
            SetStatus::<Impl, OFFSET>,
            QueryMaxLunExtendSize::<Impl, OFFSET>,
        )
    }
}
pub trait IVdsLun2Impl: Sized {
    fn QueryHints2();
    fn ApplyHints2();
}
impl ::windows::core::RuntimeName for IVdsLun2 {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsLun2";
}
impl IVdsLun2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun2Impl, const OFFSET: isize>() -> IVdsLun2Vtbl {
        unsafe extern "system" fn QueryHints2<Impl: IVdsLun2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints2: *mut VDS_HINTS2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryHints2(::core::mem::transmute_copy(&phints2)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyHints2<Impl: IVdsLun2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints2: *const VDS_HINTS2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplyHints2(&*(&phints2 as *const <VDS_HINTS2 as ::windows::core::Abi>::Abi as *const <VDS_HINTS2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsLun2>, ::windows::core::GetTrustLevel, QueryHints2::<Impl, OFFSET>, ApplyHints2::<Impl, OFFSET>)
    }
}
pub trait IVdsLunControllerPortsImpl: Sized {
    fn AssociateControllerPorts();
    fn QueryActiveControllerPorts();
}
impl ::windows::core::RuntimeName for IVdsLunControllerPorts {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsLunControllerPorts";
}
impl IVdsLunControllerPortsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunControllerPortsImpl, const OFFSET: isize>() -> IVdsLunControllerPortsVtbl {
        unsafe extern "system" fn AssociateControllerPorts<Impl: IVdsLunControllerPortsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactivecontrollerportidarray: *const ::windows::core::GUID, lnumberofactivecontrollerports: i32, pinactivecontrollerportidarray: *const ::windows::core::GUID, lnumberofinactivecontrollerports: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssociateControllerPorts(&*(&pactivecontrollerportidarray as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), lnumberofactivecontrollerports, &*(&pinactivecontrollerportidarray as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), lnumberofinactivecontrollerports) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryActiveControllerPorts<Impl: IVdsLunControllerPortsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryActiveControllerPorts(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsLunControllerPorts>, ::windows::core::GetTrustLevel, AssociateControllerPorts::<Impl, OFFSET>, QueryActiveControllerPorts::<Impl, OFFSET>)
    }
}
pub trait IVdsLunIscsiImpl: Sized {
    fn AssociateTargets();
    fn QueryAssociatedTargets();
}
impl ::windows::core::RuntimeName for IVdsLunIscsi {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsLunIscsi";
}
impl IVdsLunIscsiVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunIscsiImpl, const OFFSET: isize>() -> IVdsLunIscsiVtbl {
        unsafe extern "system" fn AssociateTargets<Impl: IVdsLunIscsiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetidarray: *const ::windows::core::GUID, lnumberoftargets: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssociateTargets(&*(&ptargetidarray as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), lnumberoftargets) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAssociatedTargets<Impl: IVdsLunIscsiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryAssociatedTargets(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsLunIscsi>, ::windows::core::GetTrustLevel, AssociateTargets::<Impl, OFFSET>, QueryAssociatedTargets::<Impl, OFFSET>)
    }
}
pub trait IVdsLunMpioImpl: Sized {
    fn GetPathInfo();
    fn GetLoadBalancePolicy();
    fn SetLoadBalancePolicy();
    fn GetSupportedLbPolicies();
}
impl ::windows::core::RuntimeName for IVdsLunMpio {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsLunMpio";
}
impl IVdsLunMpioVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunMpioImpl, const OFFSET: isize>() -> IVdsLunMpioVtbl {
        unsafe extern "system" fn GetPathInfo<Impl: IVdsLunMpioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppaths: *mut *mut VDS_PATH_INFO, plnumberofpaths: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPathInfo(::core::mem::transmute_copy(&pppaths), ::core::mem::transmute_copy(&plnumberofpaths)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLoadBalancePolicy<Impl: IVdsLunMpioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppolicy: *mut VDS_LOADBALANCE_POLICY_ENUM, pppaths: *mut *mut VDS_PATH_POLICY, plnumberofpaths: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLoadBalancePolicy(::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&pppaths), ::core::mem::transmute_copy(&plnumberofpaths)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoadBalancePolicy<Impl: IVdsLunMpioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: VDS_LOADBALANCE_POLICY_ENUM, ppaths: *const VDS_PATH_POLICY, lnumberofpaths: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLoadBalancePolicy(policy, &*(&ppaths as *const <VDS_PATH_POLICY as ::windows::core::Abi>::Abi as *const <VDS_PATH_POLICY as ::windows::core::DefaultType>::DefaultType), lnumberofpaths) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedLbPolicies<Impl: IVdsLunMpioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullbflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedLbPolicies(::core::mem::transmute_copy(&pullbflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsLunMpio>, ::windows::core::GetTrustLevel, GetPathInfo::<Impl, OFFSET>, GetLoadBalancePolicy::<Impl, OFFSET>, SetLoadBalancePolicy::<Impl, OFFSET>, GetSupportedLbPolicies::<Impl, OFFSET>)
    }
}
pub trait IVdsLunNamingImpl: Sized {
    fn SetFriendlyName();
}
impl ::windows::core::RuntimeName for IVdsLunNaming {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsLunNaming";
}
impl IVdsLunNamingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunNamingImpl, const OFFSET: isize>() -> IVdsLunNamingVtbl {
        unsafe extern "system" fn SetFriendlyName<Impl: IVdsLunNamingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFriendlyName(&*(&pwszfriendlyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsLunNaming>, ::windows::core::GetTrustLevel, SetFriendlyName::<Impl, OFFSET>)
    }
}
pub trait IVdsLunNumberImpl: Sized {
    fn GetLunNumber();
}
impl ::windows::core::RuntimeName for IVdsLunNumber {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsLunNumber";
}
impl IVdsLunNumberVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunNumberImpl, const OFFSET: isize>() -> IVdsLunNumberVtbl {
        unsafe extern "system" fn GetLunNumber<Impl: IVdsLunNumberImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullunnumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLunNumber(::core::mem::transmute_copy(&pullunnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsLunNumber>, ::windows::core::GetTrustLevel, GetLunNumber::<Impl, OFFSET>)
    }
}
pub trait IVdsLunPlexImpl: Sized {
    fn GetProperties();
    fn GetLun();
    fn QueryExtents();
    fn QueryHints();
    fn ApplyHints();
}
impl ::windows::core::RuntimeName for IVdsLunPlex {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsLunPlex";
}
impl IVdsLunPlexVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunPlexImpl, const OFFSET: isize>() -> IVdsLunPlexVtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsLunPlexImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplexprop: *mut VDS_LUN_PLEX_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties(::core::mem::transmute_copy(&pplexprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLun<Impl: IVdsLunPlexImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplun: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLun(::core::mem::transmute_copy(&pplun)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryExtents<Impl: IVdsLunPlexImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryExtents(::core::mem::transmute_copy(&ppextentarray), ::core::mem::transmute_copy(&plnumberofextents)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHints<Impl: IVdsLunPlexImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints: *mut VDS_HINTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryHints(::core::mem::transmute_copy(&phints)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyHints<Impl: IVdsLunPlexImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints: *const VDS_HINTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplyHints(&*(&phints as *const <VDS_HINTS as ::windows::core::Abi>::Abi as *const <VDS_HINTS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsLunPlex>, ::windows::core::GetTrustLevel, GetProperties::<Impl, OFFSET>, GetLun::<Impl, OFFSET>, QueryExtents::<Impl, OFFSET>, QueryHints::<Impl, OFFSET>, ApplyHints::<Impl, OFFSET>)
    }
}
pub trait IVdsMaintenanceImpl: Sized {
    fn StartMaintenance();
    fn StopMaintenance();
    fn PulseMaintenance();
}
impl ::windows::core::RuntimeName for IVdsMaintenance {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsMaintenance";
}
impl IVdsMaintenanceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsMaintenanceImpl, const OFFSET: isize>() -> IVdsMaintenanceVtbl {
        unsafe extern "system" fn StartMaintenance<Impl: IVdsMaintenanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartMaintenance(operation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopMaintenance<Impl: IVdsMaintenanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopMaintenance(operation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PulseMaintenance<Impl: IVdsMaintenanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PulseMaintenance(operation, ulcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsMaintenance>, ::windows::core::GetTrustLevel, StartMaintenance::<Impl, OFFSET>, StopMaintenance::<Impl, OFFSET>, PulseMaintenance::<Impl, OFFSET>)
    }
}
pub trait IVdsProviderImpl: Sized {
    fn GetProperties();
}
impl ::windows::core::RuntimeName for IVdsProvider {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsProvider";
}
impl IVdsProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsProviderImpl, const OFFSET: isize>() -> IVdsProviderVtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproviderprop: *mut VDS_PROVIDER_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties(::core::mem::transmute_copy(&pproviderprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsProvider>, ::windows::core::GetTrustLevel, GetProperties::<Impl, OFFSET>)
    }
}
pub trait IVdsProviderPrivateImpl: Sized {
    fn GetObject();
    fn OnLoad();
    fn OnUnload();
}
impl ::windows::core::RuntimeName for IVdsProviderPrivate {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsProviderPrivate";
}
impl IVdsProviderPrivateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsProviderPrivateImpl, const OFFSET: isize>() -> IVdsProviderPrivateVtbl {
        unsafe extern "system" fn GetObject<Impl: IVdsProviderPrivateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: ::windows::core::GUID, r#type: VDS_OBJECT_TYPE, ppobjectunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObject(&*(&objectid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), r#type, ::core::mem::transmute_copy(&ppobjectunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLoad<Impl: IVdsProviderPrivateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszmachinename: super::super::Foundation::PWSTR, pcallbackobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLoad(&*(&pwszmachinename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pcallbackobject as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnUnload<Impl: IVdsProviderPrivateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bforceunload: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnUnload(&*(&bforceunload as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsProviderPrivate>, ::windows::core::GetTrustLevel, GetObject::<Impl, OFFSET>, OnLoad::<Impl, OFFSET>, OnUnload::<Impl, OFFSET>)
    }
}
pub trait IVdsProviderSupportImpl: Sized {
    fn GetVersionSupport();
}
impl ::windows::core::RuntimeName for IVdsProviderSupport {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsProviderSupport";
}
impl IVdsProviderSupportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsProviderSupportImpl, const OFFSET: isize>() -> IVdsProviderSupportVtbl {
        unsafe extern "system" fn GetVersionSupport<Impl: IVdsProviderSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulversionsupport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVersionSupport(::core::mem::transmute_copy(&ulversionsupport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsProviderSupport>, ::windows::core::GetTrustLevel, GetVersionSupport::<Impl, OFFSET>)
    }
}
pub trait IVdsStoragePoolImpl: Sized {
    fn GetProvider();
    fn GetProperties();
    fn GetAttributes();
    fn QueryDriveExtents();
    fn QueryAllocatedLuns();
    fn QueryAllocatedStoragePools();
}
impl ::windows::core::RuntimeName for IVdsStoragePool {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsStoragePool";
}
impl IVdsStoragePoolVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsStoragePoolImpl, const OFFSET: isize>() -> IVdsStoragePoolVtbl {
        unsafe extern "system" fn GetProvider<Impl: IVdsStoragePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProvider(::core::mem::transmute_copy(&ppprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Impl: IVdsStoragePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstoragepoolprop: *mut VDS_STORAGE_POOL_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties(::core::mem::transmute_copy(&pstoragepoolprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributes<Impl: IVdsStoragePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstoragepoolattributes: *mut VDS_POOL_ATTRIBUTES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributes(::core::mem::transmute_copy(&pstoragepoolattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDriveExtents<Impl: IVdsStoragePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_STORAGE_POOL_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryDriveExtents(::core::mem::transmute_copy(&ppextentarray), ::core::mem::transmute_copy(&plnumberofextents)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAllocatedLuns<Impl: IVdsStoragePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryAllocatedLuns(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAllocatedStoragePools<Impl: IVdsStoragePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryAllocatedStoragePools(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsStoragePool>, ::windows::core::GetTrustLevel, GetProvider::<Impl, OFFSET>, GetProperties::<Impl, OFFSET>, GetAttributes::<Impl, OFFSET>, QueryDriveExtents::<Impl, OFFSET>, QueryAllocatedLuns::<Impl, OFFSET>, QueryAllocatedStoragePools::<Impl, OFFSET>)
    }
}
pub trait IVdsSubSystemImpl: Sized {
    fn GetProperties();
    fn GetProvider();
    fn QueryControllers();
    fn QueryLuns();
    fn QueryDrives();
    fn GetDrive();
    fn Reenumerate();
    fn SetControllerStatus();
    fn CreateLun();
    fn ReplaceDrive();
    fn SetStatus();
    fn QueryMaxLunCreateSize();
}
impl ::windows::core::RuntimeName for IVdsSubSystem {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsSubSystem";
}
impl IVdsSubSystemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystemImpl, const OFFSET: isize>() -> IVdsSubSystemVtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubsystemprop: *mut VDS_SUB_SYSTEM_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties(::core::mem::transmute_copy(&psubsystemprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProvider<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProvider(::core::mem::transmute_copy(&ppprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryControllers<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryControllers(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryLuns<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryLuns(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDrives<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryDrives(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDrive<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sbusnumber: i16, sslotnumber: i16, ppdrive: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDrive(sbusnumber, sslotnumber, ::core::mem::transmute_copy(&ppdrive)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reenumerate<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reenumerate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControllerStatus<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ponlinecontrolleridarray: *const ::windows::core::GUID, lnumberofonlinecontrollers: i32, pofflinecontrolleridarray: *const ::windows::core::GUID, lnumberofofflinecontrollers: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetControllerStatus(&*(&ponlinecontrolleridarray as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), lnumberofonlinecontrollers, &*(&pofflinecontrolleridarray as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), lnumberofofflinecontrollers) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLun<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pwszunmaskinglist: super::super::Foundation::PWSTR, phints: *const VDS_HINTS, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLun(
                r#type,
                ullsizeinbytes,
                &*(&pdriveidarray as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                lnumberofdrives,
                &*(&pwszunmaskinglist as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&phints as *const <VDS_HINTS as ::windows::core::Abi>::Abi as *const <VDS_HINTS as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppasync),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceDrive<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drivetobereplaced: ::windows::core::GUID, replacementdrive: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReplaceDrive(&*(&drivetobereplaced as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&replacementdrive as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_SUB_SYSTEM_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStatus(status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryMaxLunCreateSize<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, phints: *const VDS_HINTS, pullmaxlunsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryMaxLunCreateSize(r#type, &*(&pdriveidarray as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), lnumberofdrives, &*(&phints as *const <VDS_HINTS as ::windows::core::Abi>::Abi as *const <VDS_HINTS as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pullmaxlunsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IVdsSubSystem>,
            ::windows::core::GetTrustLevel,
            GetProperties::<Impl, OFFSET>,
            GetProvider::<Impl, OFFSET>,
            QueryControllers::<Impl, OFFSET>,
            QueryLuns::<Impl, OFFSET>,
            QueryDrives::<Impl, OFFSET>,
            GetDrive::<Impl, OFFSET>,
            Reenumerate::<Impl, OFFSET>,
            SetControllerStatus::<Impl, OFFSET>,
            CreateLun::<Impl, OFFSET>,
            ReplaceDrive::<Impl, OFFSET>,
            SetStatus::<Impl, OFFSET>,
            QueryMaxLunCreateSize::<Impl, OFFSET>,
        )
    }
}
pub trait IVdsSubSystem2Impl: Sized {
    fn GetProperties2();
    fn GetDrive2();
    fn CreateLun2();
    fn QueryMaxLunCreateSize2();
}
impl ::windows::core::RuntimeName for IVdsSubSystem2 {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsSubSystem2";
}
impl IVdsSubSystem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystem2Impl, const OFFSET: isize>() -> IVdsSubSystem2Vtbl {
        unsafe extern "system" fn GetProperties2<Impl: IVdsSubSystem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubsystemprop2: *mut VDS_SUB_SYSTEM_PROP2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties2(::core::mem::transmute_copy(&psubsystemprop2)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDrive2<Impl: IVdsSubSystem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sbusnumber: i16, sslotnumber: i16, ulenclosurenumber: u32, ppdrive: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDrive2(sbusnumber, sslotnumber, ulenclosurenumber, ::core::mem::transmute_copy(&ppdrive)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLun2<Impl: IVdsSubSystem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pwszunmaskinglist: super::super::Foundation::PWSTR, phints2: *const VDS_HINTS2, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLun2(
                r#type,
                ullsizeinbytes,
                &*(&pdriveidarray as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                lnumberofdrives,
                &*(&pwszunmaskinglist as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&phints2 as *const <VDS_HINTS2 as ::windows::core::Abi>::Abi as *const <VDS_HINTS2 as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppasync),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryMaxLunCreateSize2<Impl: IVdsSubSystem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, phints2: *const VDS_HINTS2, pullmaxlunsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryMaxLunCreateSize2(r#type, &*(&pdriveidarray as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), lnumberofdrives, &*(&phints2 as *const <VDS_HINTS2 as ::windows::core::Abi>::Abi as *const <VDS_HINTS2 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pullmaxlunsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsSubSystem2>, ::windows::core::GetTrustLevel, GetProperties2::<Impl, OFFSET>, GetDrive2::<Impl, OFFSET>, CreateLun2::<Impl, OFFSET>, QueryMaxLunCreateSize2::<Impl, OFFSET>)
    }
}
pub trait IVdsSubSystemInterconnectImpl: Sized {
    fn GetSupportedInterconnects();
}
impl ::windows::core::RuntimeName for IVdsSubSystemInterconnect {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsSubSystemInterconnect";
}
impl IVdsSubSystemInterconnectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystemInterconnectImpl, const OFFSET: isize>() -> IVdsSubSystemInterconnectVtbl {
        unsafe extern "system" fn GetSupportedInterconnects<Impl: IVdsSubSystemInterconnectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulsupportedinterconnectsflag: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedInterconnects(::core::mem::transmute_copy(&pulsupportedinterconnectsflag)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsSubSystemInterconnect>, ::windows::core::GetTrustLevel, GetSupportedInterconnects::<Impl, OFFSET>)
    }
}
pub trait IVdsSubSystemIscsiImpl: Sized {
    fn QueryTargets();
    fn QueryPortals();
    fn CreateTarget();
    fn SetIpsecGroupPresharedKey();
}
impl ::windows::core::RuntimeName for IVdsSubSystemIscsi {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsSubSystemIscsi";
}
impl IVdsSubSystemIscsiVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystemIscsiImpl, const OFFSET: isize>() -> IVdsSubSystemIscsiVtbl {
        unsafe extern "system" fn QueryTargets<Impl: IVdsSubSystemIscsiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryTargets(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryPortals<Impl: IVdsSubSystemIscsiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryPortals(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTarget<Impl: IVdsSubSystemIscsiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsziscsiname: super::super::Foundation::PWSTR, pwszfriendlyname: super::super::Foundation::PWSTR, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTarget(&*(&pwsziscsiname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pwszfriendlyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppasync)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpsecGroupPresharedKey<Impl: IVdsSubSystemIscsiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIpsecGroupPresharedKey(&*(&pipseckey as *const <VDS_ISCSI_IPSEC_KEY as ::windows::core::Abi>::Abi as *const <VDS_ISCSI_IPSEC_KEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsSubSystemIscsi>, ::windows::core::GetTrustLevel, QueryTargets::<Impl, OFFSET>, QueryPortals::<Impl, OFFSET>, CreateTarget::<Impl, OFFSET>, SetIpsecGroupPresharedKey::<Impl, OFFSET>)
    }
}
pub trait IVdsSubSystemNamingImpl: Sized {
    fn SetFriendlyName();
}
impl ::windows::core::RuntimeName for IVdsSubSystemNaming {
    const NAME: &'static str = "Windows.Win32.Storage.VirtualDiskService.IVdsSubSystemNaming";
}
impl IVdsSubSystemNamingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystemNamingImpl, const OFFSET: isize>() -> IVdsSubSystemNamingVtbl {
        unsafe extern "system" fn SetFriendlyName<Impl: IVdsSubSystemNamingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFriendlyName(&*(&pwszfriendlyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVdsSubSystemNaming>, ::windows::core::GetTrustLevel, SetFriendlyName::<Impl, OFFSET>)
    }
}
