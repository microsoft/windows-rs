pub trait IUPnPAddressFamilyControlImpl: Sized {
    fn SetAddressFamily();
    fn GetAddressFamily();
}
impl ::windows::core::RuntimeName for IUPnPAddressFamilyControl {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPAddressFamilyControl";
}
impl IUPnPAddressFamilyControlVtbl {
    pub const fn new<Impl: IUPnPAddressFamilyControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPAddressFamilyControlVtbl {
        unsafe extern "system" fn SetAddressFamily<Impl: IUPnPAddressFamilyControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAddressFamily(dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAddressFamily<Impl: IUPnPAddressFamilyControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAddressFamily(::core::mem::transmute_copy(&pdwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPAddressFamilyControl>, base.5, SetAddressFamily::<Impl, OFFSET>, GetAddressFamily::<Impl, OFFSET>)
    }
}
pub trait IUPnPAsyncResultImpl: Sized {
    fn AsyncOperationComplete();
}
impl ::windows::core::RuntimeName for IUPnPAsyncResult {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPAsyncResult";
}
impl IUPnPAsyncResultVtbl {
    pub const fn new<Impl: IUPnPAsyncResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPAsyncResultVtbl {
        unsafe extern "system" fn AsyncOperationComplete<Impl: IUPnPAsyncResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AsyncOperationComplete(ullrequestid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPAsyncResult>, base.5, AsyncOperationComplete::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDescriptionDocumentImpl: Sized + IDispatchImpl {
    fn ReadyState();
    fn Load();
    fn LoadAsync();
    fn LoadResult();
    fn Abort();
    fn RootDevice();
    fn DeviceByUDN();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUPnPDescriptionDocument {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPDescriptionDocument";
}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDescriptionDocumentVtbl {
    pub const fn new<Impl: IUPnPDescriptionDocumentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPDescriptionDocumentVtbl {
        unsafe extern "system" fn ReadyState<Impl: IUPnPDescriptionDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plreadystate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadyState(::core::mem::transmute_copy(&plreadystate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Load<Impl: IUPnPDescriptionDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Load(&*(&bstrurl as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadAsync<Impl: IUPnPDescriptionDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, punkcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadAsync(&*(&bstrurl as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&punkcallback as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadResult<Impl: IUPnPDescriptionDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrerror: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadResult(::core::mem::transmute_copy(&phrerror)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Impl: IUPnPDescriptionDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Abort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RootDevice<Impl: IUPnPDescriptionDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppudrootdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RootDevice(::core::mem::transmute_copy(&ppudrootdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceByUDN<Impl: IUPnPDescriptionDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppuddevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceByUDN(&*(&bstrudn as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppuddevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPDescriptionDocument>, base.5, ReadyState::<Impl, OFFSET>, Load::<Impl, OFFSET>, LoadAsync::<Impl, OFFSET>, LoadResult::<Impl, OFFSET>, Abort::<Impl, OFFSET>, RootDevice::<Impl, OFFSET>, DeviceByUDN::<Impl, OFFSET>)
    }
}
pub trait IUPnPDescriptionDocumentCallbackImpl: Sized {
    fn LoadComplete();
}
impl ::windows::core::RuntimeName for IUPnPDescriptionDocumentCallback {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPDescriptionDocumentCallback";
}
impl IUPnPDescriptionDocumentCallbackVtbl {
    pub const fn new<Impl: IUPnPDescriptionDocumentCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPDescriptionDocumentCallbackVtbl {
        unsafe extern "system" fn LoadComplete<Impl: IUPnPDescriptionDocumentCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrloadresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadComplete(hrloadresult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPDescriptionDocumentCallback>, base.5, LoadComplete::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDeviceImpl: Sized + IDispatchImpl {
    fn IsRootDevice();
    fn RootDevice();
    fn ParentDevice();
    fn HasChildren();
    fn Children();
    fn UniqueDeviceName();
    fn FriendlyName();
    fn Type();
    fn PresentationURL();
    fn ManufacturerName();
    fn ManufacturerURL();
    fn ModelName();
    fn ModelNumber();
    fn Description();
    fn ModelURL();
    fn UPC();
    fn SerialNumber();
    fn IconURL();
    fn Services();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUPnPDevice {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPDevice";
}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDeviceVtbl {
    pub const fn new<Impl: IUPnPDeviceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPDeviceVtbl {
        unsafe extern "system" fn IsRootDevice<Impl: IUPnPDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarb: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRootDevice(::core::mem::transmute_copy(&pvarb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RootDevice<Impl: IUPnPDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppudrootdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RootDevice(::core::mem::transmute_copy(&ppudrootdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParentDevice<Impl: IUPnPDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppuddeviceparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ParentDevice(::core::mem::transmute_copy(&ppuddeviceparent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasChildren<Impl: IUPnPDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarb: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasChildren(::core::mem::transmute_copy(&pvarb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Impl: IUPnPDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppudchildren: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Children(::core::mem::transmute_copy(&ppudchildren)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UniqueDeviceName<Impl: IUPnPDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UniqueDeviceName(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FriendlyName<Impl: IUPnPDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FriendlyName(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IUPnPDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Type(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresentationURL<Impl: IUPnPDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PresentationURL(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManufacturerName<Impl: IUPnPDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManufacturerName(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManufacturerURL<Impl: IUPnPDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManufacturerURL(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelName<Impl: IUPnPDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ModelName(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelNumber<Impl: IUPnPDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ModelNumber(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IUPnPDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelURL<Impl: IUPnPDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ModelURL(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UPC<Impl: IUPnPDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UPC(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerialNumber<Impl: IUPnPDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SerialNumber(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IconURL<Impl: IUPnPDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrencodingformat: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, lsizex: i32, lsizey: i32, lbitdepth: i32, pbstriconurl: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IconURL(&*(&bstrencodingformat as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lsizex, lsizey, lbitdepth, ::core::mem::transmute_copy(&pbstriconurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Services<Impl: IUPnPDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppusservices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Services(::core::mem::transmute_copy(&ppusservices)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IUPnPDevice>,
            base.5,
            IsRootDevice::<Impl, OFFSET>,
            RootDevice::<Impl, OFFSET>,
            ParentDevice::<Impl, OFFSET>,
            HasChildren::<Impl, OFFSET>,
            Children::<Impl, OFFSET>,
            UniqueDeviceName::<Impl, OFFSET>,
            FriendlyName::<Impl, OFFSET>,
            Type::<Impl, OFFSET>,
            PresentationURL::<Impl, OFFSET>,
            ManufacturerName::<Impl, OFFSET>,
            ManufacturerURL::<Impl, OFFSET>,
            ModelName::<Impl, OFFSET>,
            ModelNumber::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            ModelURL::<Impl, OFFSET>,
            UPC::<Impl, OFFSET>,
            SerialNumber::<Impl, OFFSET>,
            IconURL::<Impl, OFFSET>,
            Services::<Impl, OFFSET>,
        )
    }
}
pub trait IUPnPDeviceControlImpl: Sized {
    fn Initialize();
    fn GetServiceObject();
}
impl ::windows::core::RuntimeName for IUPnPDeviceControl {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPDeviceControl";
}
impl IUPnPDeviceControlVtbl {
    pub const fn new<Impl: IUPnPDeviceControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPDeviceControlVtbl {
        unsafe extern "system" fn Initialize<Impl: IUPnPDeviceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(
                &*(&bstrxmldesc as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdeviceidentifier as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrinitstring as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServiceObject<Impl: IUPnPDeviceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrserviceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppdispservice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetServiceObject(&*(&bstrudn as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrserviceid as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdispservice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPDeviceControl>, base.5, Initialize::<Impl, OFFSET>, GetServiceObject::<Impl, OFFSET>)
    }
}
pub trait IUPnPDeviceControlHttpHeadersImpl: Sized {
    fn GetAdditionalResponseHeaders();
}
impl ::windows::core::RuntimeName for IUPnPDeviceControlHttpHeaders {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPDeviceControlHttpHeaders";
}
impl IUPnPDeviceControlHttpHeadersVtbl {
    pub const fn new<Impl: IUPnPDeviceControlHttpHeadersImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPDeviceControlHttpHeadersVtbl {
        unsafe extern "system" fn GetAdditionalResponseHeaders<Impl: IUPnPDeviceControlHttpHeadersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrhttpresponseheaders: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAdditionalResponseHeaders(::core::mem::transmute_copy(&bstrhttpresponseheaders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPDeviceControlHttpHeaders>, base.5, GetAdditionalResponseHeaders::<Impl, OFFSET>)
    }
}
pub trait IUPnPDeviceDocumentAccessImpl: Sized {
    fn GetDocumentURL();
}
impl ::windows::core::RuntimeName for IUPnPDeviceDocumentAccess {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPDeviceDocumentAccess";
}
impl IUPnPDeviceDocumentAccessVtbl {
    pub const fn new<Impl: IUPnPDeviceDocumentAccessImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPDeviceDocumentAccessVtbl {
        unsafe extern "system" fn GetDocumentURL<Impl: IUPnPDeviceDocumentAccessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdocument: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDocumentURL(::core::mem::transmute_copy(&pbstrdocument)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPDeviceDocumentAccess>, base.5, GetDocumentURL::<Impl, OFFSET>)
    }
}
pub trait IUPnPDeviceDocumentAccessExImpl: Sized {
    fn GetDocument();
}
impl ::windows::core::RuntimeName for IUPnPDeviceDocumentAccessEx {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPDeviceDocumentAccessEx";
}
impl IUPnPDeviceDocumentAccessExVtbl {
    pub const fn new<Impl: IUPnPDeviceDocumentAccessExImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPDeviceDocumentAccessExVtbl {
        unsafe extern "system" fn GetDocument<Impl: IUPnPDeviceDocumentAccessExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdocument: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDocument(::core::mem::transmute_copy(&pbstrdocument)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPDeviceDocumentAccessEx>, base.5, GetDocument::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDeviceFinderImpl: Sized + IDispatchImpl {
    fn FindByType();
    fn CreateAsyncFind();
    fn StartAsyncFind();
    fn CancelAsyncFind();
    fn FindByUDN();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUPnPDeviceFinder {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPDeviceFinder";
}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDeviceFinderVtbl {
    pub const fn new<Impl: IUPnPDeviceFinderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPDeviceFinderVtbl {
        unsafe extern "system" fn FindByType<Impl: IUPnPDeviceFinderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtypeuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dwflags: u32, pdevices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindByType(&*(&bstrtypeuri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), dwflags, ::core::mem::transmute_copy(&pdevices)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAsyncFind<Impl: IUPnPDeviceFinderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtypeuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dwflags: u32, punkdevicefindercallback: *mut ::core::ffi::c_void, plfinddata: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAsyncFind(&*(&bstrtypeuri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), dwflags, &*(&punkdevicefindercallback as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&plfinddata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAsyncFind<Impl: IUPnPDeviceFinderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartAsyncFind(lfinddata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelAsyncFind<Impl: IUPnPDeviceFinderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CancelAsyncFind(lfinddata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindByUDN<Impl: IUPnPDeviceFinderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindByUDN(&*(&bstrudn as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPDeviceFinder>, base.5, FindByType::<Impl, OFFSET>, CreateAsyncFind::<Impl, OFFSET>, StartAsyncFind::<Impl, OFFSET>, CancelAsyncFind::<Impl, OFFSET>, FindByUDN::<Impl, OFFSET>)
    }
}
pub trait IUPnPDeviceFinderAddCallbackWithInterfaceImpl: Sized {
    fn DeviceAddedWithInterface();
}
impl ::windows::core::RuntimeName for IUPnPDeviceFinderAddCallbackWithInterface {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPDeviceFinderAddCallbackWithInterface";
}
impl IUPnPDeviceFinderAddCallbackWithInterfaceVtbl {
    pub const fn new<Impl: IUPnPDeviceFinderAddCallbackWithInterfaceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPDeviceFinderAddCallbackWithInterfaceVtbl {
        unsafe extern "system" fn DeviceAddedWithInterface<Impl: IUPnPDeviceFinderAddCallbackWithInterfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lfinddata: i32, pdevice: ::windows::core::RawPtr, pguidinterface: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceAddedWithInterface(lfinddata, &*(&pdevice as *const <IUPnPDevice as ::windows::core::Abi>::Abi as *const <IUPnPDevice as ::windows::core::DefaultType>::DefaultType), &*(&pguidinterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPDeviceFinderAddCallbackWithInterface>, base.5, DeviceAddedWithInterface::<Impl, OFFSET>)
    }
}
pub trait IUPnPDeviceFinderCallbackImpl: Sized {
    fn DeviceAdded();
    fn DeviceRemoved();
    fn SearchComplete();
}
impl ::windows::core::RuntimeName for IUPnPDeviceFinderCallback {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPDeviceFinderCallback";
}
impl IUPnPDeviceFinderCallbackVtbl {
    pub const fn new<Impl: IUPnPDeviceFinderCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPDeviceFinderCallbackVtbl {
        unsafe extern "system" fn DeviceAdded<Impl: IUPnPDeviceFinderCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lfinddata: i32, pdevice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceAdded(lfinddata, &*(&pdevice as *const <IUPnPDevice as ::windows::core::Abi>::Abi as *const <IUPnPDevice as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceRemoved<Impl: IUPnPDeviceFinderCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lfinddata: i32, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceRemoved(lfinddata, &*(&bstrudn as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchComplete<Impl: IUPnPDeviceFinderCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SearchComplete(lfinddata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPDeviceFinderCallback>, base.5, DeviceAdded::<Impl, OFFSET>, DeviceRemoved::<Impl, OFFSET>, SearchComplete::<Impl, OFFSET>)
    }
}
pub trait IUPnPDeviceProviderImpl: Sized {
    fn Start();
    fn Stop();
}
impl ::windows::core::RuntimeName for IUPnPDeviceProvider {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPDeviceProvider";
}
impl IUPnPDeviceProviderVtbl {
    pub const fn new<Impl: IUPnPDeviceProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPDeviceProviderVtbl {
        unsafe extern "system" fn Start<Impl: IUPnPDeviceProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Start(&*(&bstrinitstring as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Impl: IUPnPDeviceProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Stop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPDeviceProvider>, base.5, Start::<Impl, OFFSET>, Stop::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDevicesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUPnPDevices {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPDevices";
}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDevicesVtbl {
    pub const fn new<Impl: IUPnPDevicesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPDevicesVtbl {
        unsafe extern "system" fn Count<Impl: IUPnPDevicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IUPnPDevicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IUPnPDevicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&bstrudn as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPDevices>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
pub trait IUPnPEventSinkImpl: Sized {
    fn OnStateChanged();
    fn OnStateChangedSafe();
}
impl ::windows::core::RuntimeName for IUPnPEventSink {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPEventSink";
}
impl IUPnPEventSinkVtbl {
    pub const fn new<Impl: IUPnPEventSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPEventSinkVtbl {
        unsafe extern "system" fn OnStateChanged<Impl: IUPnPEventSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchanges: u32, rgdispidchanges: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnStateChanged(cchanges, rgdispidchanges) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnStateChangedSafe<Impl: IUPnPEventSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varsadispidchanges: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnStateChangedSafe(&*(&varsadispidchanges as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPEventSink>, base.5, OnStateChanged::<Impl, OFFSET>, OnStateChangedSafe::<Impl, OFFSET>)
    }
}
pub trait IUPnPEventSourceImpl: Sized {
    fn Advise();
    fn Unadvise();
}
impl ::windows::core::RuntimeName for IUPnPEventSource {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPEventSource";
}
impl IUPnPEventSourceVtbl {
    pub const fn new<Impl: IUPnPEventSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPEventSourceVtbl {
        unsafe extern "system" fn Advise<Impl: IUPnPEventSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pessubscriber: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Advise(&*(&pessubscriber as *const <IUPnPEventSink as ::windows::core::Abi>::Abi as *const <IUPnPEventSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Impl: IUPnPEventSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pessubscriber: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unadvise(&*(&pessubscriber as *const <IUPnPEventSink as ::windows::core::Abi>::Abi as *const <IUPnPEventSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPEventSource>, base.5, Advise::<Impl, OFFSET>, Unadvise::<Impl, OFFSET>)
    }
}
pub trait IUPnPHttpHeaderControlImpl: Sized {
    fn AddRequestHeaders();
}
impl ::windows::core::RuntimeName for IUPnPHttpHeaderControl {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPHttpHeaderControl";
}
impl IUPnPHttpHeaderControlVtbl {
    pub const fn new<Impl: IUPnPHttpHeaderControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPHttpHeaderControlVtbl {
        unsafe extern "system" fn AddRequestHeaders<Impl: IUPnPHttpHeaderControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrhttpheaders: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddRequestHeaders(&*(&bstrhttpheaders as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPHttpHeaderControl>, base.5, AddRequestHeaders::<Impl, OFFSET>)
    }
}
pub trait IUPnPRegistrarImpl: Sized {
    fn RegisterDevice();
    fn RegisterRunningDevice();
    fn RegisterDeviceProvider();
    fn GetUniqueDeviceName();
    fn UnregisterDevice();
    fn UnregisterDeviceProvider();
}
impl ::windows::core::RuntimeName for IUPnPRegistrar {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPRegistrar";
}
impl IUPnPRegistrarVtbl {
    pub const fn new<Impl: IUPnPRegistrarImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPRegistrarVtbl {
        unsafe extern "system" fn RegisterDevice<Impl: IUPnPRegistrarImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrprogiddevicecontrolclass: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcontainerid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32, pbstrdeviceidentifier: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterDevice(
                &*(&bstrxmldesc as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrprogiddevicecontrolclass as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrinitstring as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrcontainerid as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrresourcepath as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                nlifetime,
                ::core::mem::transmute_copy(&pbstrdeviceidentifier),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterRunningDevice<Impl: IUPnPRegistrarImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, punkdevicecontrol: *mut ::core::ffi::c_void, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32, pbstrdeviceidentifier: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterRunningDevice(
                &*(&bstrxmldesc as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&punkdevicecontrol as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrinitstring as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrresourcepath as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                nlifetime,
                ::core::mem::transmute_copy(&pbstrdeviceidentifier),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterDeviceProvider<Impl: IUPnPRegistrarImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrprogidproviderclass: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcontainerid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterDeviceProvider(
                &*(&bstrprovidername as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrprogidproviderclass as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrinitstring as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrcontainerid as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUniqueDeviceName<Impl: IUPnPRegistrarImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrtemplateudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrudn: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUniqueDeviceName(&*(&bstrdeviceidentifier as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrtemplateudn as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrudn)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDevice<Impl: IUPnPRegistrarImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fpermanent: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnregisterDevice(&*(&bstrdeviceidentifier as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&fpermanent as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDeviceProvider<Impl: IUPnPRegistrarImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnregisterDeviceProvider(&*(&bstrprovidername as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPRegistrar>, base.5, RegisterDevice::<Impl, OFFSET>, RegisterRunningDevice::<Impl, OFFSET>, RegisterDeviceProvider::<Impl, OFFSET>, GetUniqueDeviceName::<Impl, OFFSET>, UnregisterDevice::<Impl, OFFSET>, UnregisterDeviceProvider::<Impl, OFFSET>)
    }
}
pub trait IUPnPRemoteEndpointInfoImpl: Sized {
    fn GetDwordValue();
    fn GetStringValue();
    fn GetGuidValue();
}
impl ::windows::core::RuntimeName for IUPnPRemoteEndpointInfo {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPRemoteEndpointInfo";
}
impl IUPnPRemoteEndpointInfoVtbl {
    pub const fn new<Impl: IUPnPRemoteEndpointInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPRemoteEndpointInfoVtbl {
        unsafe extern "system" fn GetDwordValue<Impl: IUPnPRemoteEndpointInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrvaluename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdwvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDwordValue(&*(&bstrvaluename as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringValue<Impl: IUPnPRemoteEndpointInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrvaluename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStringValue(&*(&bstrvaluename as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGuidValue<Impl: IUPnPRemoteEndpointInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrvaluename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pguidvalue: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGuidValue(&*(&bstrvaluename as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pguidvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPRemoteEndpointInfo>, base.5, GetDwordValue::<Impl, OFFSET>, GetStringValue::<Impl, OFFSET>, GetGuidValue::<Impl, OFFSET>)
    }
}
pub trait IUPnPReregistrarImpl: Sized {
    fn ReregisterDevice();
    fn ReregisterRunningDevice();
}
impl ::windows::core::RuntimeName for IUPnPReregistrar {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPReregistrar";
}
impl IUPnPReregistrarVtbl {
    pub const fn new<Impl: IUPnPReregistrarImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPReregistrarVtbl {
        unsafe extern "system" fn ReregisterDevice<Impl: IUPnPReregistrarImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrprogiddevicecontrolclass: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcontainerid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReregisterDevice(
                &*(&bstrdeviceidentifier as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrxmldesc as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrprogiddevicecontrolclass as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrinitstring as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrcontainerid as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrresourcepath as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                nlifetime,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReregisterRunningDevice<Impl: IUPnPReregistrarImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, punkdevicecontrol: *mut ::core::ffi::c_void, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReregisterRunningDevice(
                &*(&bstrdeviceidentifier as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrxmldesc as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&punkdevicecontrol as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrinitstring as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrresourcepath as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                nlifetime,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPReregistrar>, base.5, ReregisterDevice::<Impl, OFFSET>, ReregisterRunningDevice::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPServiceImpl: Sized + IDispatchImpl {
    fn QueryStateVariable();
    fn InvokeAction();
    fn ServiceTypeIdentifier();
    fn AddCallback();
    fn Id();
    fn LastTransportStatus();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUPnPService {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPService";
}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPServiceVtbl {
    pub const fn new<Impl: IUPnPServiceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPServiceVtbl {
        unsafe extern "system" fn QueryStateVariable<Impl: IUPnPServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrvariablename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryStateVariable(&*(&bstrvariablename as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeAction<Impl: IUPnPServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstractionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vinactionargs: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InvokeAction(
                &*(&bstractionname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&vinactionargs as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&pvoutactionargs as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pvretval),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeIdentifier<Impl: IUPnPServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServiceTypeIdentifier(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddCallback<Impl: IUPnPServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddCallback(&*(&punkcallback as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IUPnPServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&pbstrid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastTransportStatus<Impl: IUPnPServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LastTransportStatus(::core::mem::transmute_copy(&plvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPService>, base.5, QueryStateVariable::<Impl, OFFSET>, InvokeAction::<Impl, OFFSET>, ServiceTypeIdentifier::<Impl, OFFSET>, AddCallback::<Impl, OFFSET>, Id::<Impl, OFFSET>, LastTransportStatus::<Impl, OFFSET>)
    }
}
pub trait IUPnPServiceAsyncImpl: Sized {
    fn BeginInvokeAction();
    fn EndInvokeAction();
    fn BeginQueryStateVariable();
    fn EndQueryStateVariable();
    fn BeginSubscribeToEvents();
    fn EndSubscribeToEvents();
    fn BeginSCPDDownload();
    fn EndSCPDDownload();
    fn CancelAsyncOperation();
}
impl ::windows::core::RuntimeName for IUPnPServiceAsync {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPServiceAsync";
}
impl IUPnPServiceAsyncVtbl {
    pub const fn new<Impl: IUPnPServiceAsyncImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPServiceAsyncVtbl {
        unsafe extern "system" fn BeginInvokeAction<Impl: IUPnPServiceAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstractionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vinactionargs: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pasyncresult: ::windows::core::RawPtr, pullrequestid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginInvokeAction(
                &*(&bstractionname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&vinactionargs as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&pasyncresult as *const <IUPnPAsyncResult as ::windows::core::Abi>::Abi as *const <IUPnPAsyncResult as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pullrequestid),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndInvokeAction<Impl: IUPnPServiceAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ullrequestid: u64, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndInvokeAction(ullrequestid, &*(&pvoutactionargs as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&pvretval as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginQueryStateVariable<Impl: IUPnPServiceAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrvariablename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pasyncresult: ::windows::core::RawPtr, pullrequestid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginQueryStateVariable(&*(&bstrvariablename as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pasyncresult as *const <IUPnPAsyncResult as ::windows::core::Abi>::Abi as *const <IUPnPAsyncResult as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pullrequestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndQueryStateVariable<Impl: IUPnPServiceAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ullrequestid: u64, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndQueryStateVariable(ullrequestid, &*(&pvalue as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginSubscribeToEvents<Impl: IUPnPServiceAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkcallback: *mut ::core::ffi::c_void, pasyncresult: ::windows::core::RawPtr, pullrequestid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginSubscribeToEvents(&*(&punkcallback as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&pasyncresult as *const <IUPnPAsyncResult as ::windows::core::Abi>::Abi as *const <IUPnPAsyncResult as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pullrequestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSubscribeToEvents<Impl: IUPnPServiceAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndSubscribeToEvents(ullrequestid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginSCPDDownload<Impl: IUPnPServiceAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pasyncresult: ::windows::core::RawPtr, pullrequestid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginSCPDDownload(&*(&pasyncresult as *const <IUPnPAsyncResult as ::windows::core::Abi>::Abi as *const <IUPnPAsyncResult as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pullrequestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSCPDDownload<Impl: IUPnPServiceAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ullrequestid: u64, pbstrscpddoc: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndSCPDDownload(ullrequestid, ::core::mem::transmute_copy(&pbstrscpddoc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelAsyncOperation<Impl: IUPnPServiceAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CancelAsyncOperation(ullrequestid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPServiceAsync>, base.5, BeginInvokeAction::<Impl, OFFSET>, EndInvokeAction::<Impl, OFFSET>, BeginQueryStateVariable::<Impl, OFFSET>, EndQueryStateVariable::<Impl, OFFSET>, BeginSubscribeToEvents::<Impl, OFFSET>, EndSubscribeToEvents::<Impl, OFFSET>, BeginSCPDDownload::<Impl, OFFSET>, EndSCPDDownload::<Impl, OFFSET>, CancelAsyncOperation::<Impl, OFFSET>)
    }
}
pub trait IUPnPServiceCallbackImpl: Sized {
    fn StateVariableChanged();
    fn ServiceInstanceDied();
}
impl ::windows::core::RuntimeName for IUPnPServiceCallback {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPServiceCallback";
}
impl IUPnPServiceCallbackVtbl {
    pub const fn new<Impl: IUPnPServiceCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPServiceCallbackVtbl {
        unsafe extern "system" fn StateVariableChanged<Impl: IUPnPServiceCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pus: ::windows::core::RawPtr, pcwszstatevarname: super::super::super::Foundation::PWSTR, vavalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StateVariableChanged(
                &*(&pus as *const <IUPnPService as ::windows::core::Abi>::Abi as *const <IUPnPService as ::windows::core::DefaultType>::DefaultType),
                &*(&pcwszstatevarname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&vavalue as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceInstanceDied<Impl: IUPnPServiceCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pus: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServiceInstanceDied(&*(&pus as *const <IUPnPService as ::windows::core::Abi>::Abi as *const <IUPnPService as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPServiceCallback>, base.5, StateVariableChanged::<Impl, OFFSET>, ServiceInstanceDied::<Impl, OFFSET>)
    }
}
pub trait IUPnPServiceDocumentAccessImpl: Sized {
    fn GetDocumentURL();
    fn GetDocument();
}
impl ::windows::core::RuntimeName for IUPnPServiceDocumentAccess {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPServiceDocumentAccess";
}
impl IUPnPServiceDocumentAccessVtbl {
    pub const fn new<Impl: IUPnPServiceDocumentAccessImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPServiceDocumentAccessVtbl {
        unsafe extern "system" fn GetDocumentURL<Impl: IUPnPServiceDocumentAccessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdocurl: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDocumentURL(::core::mem::transmute_copy(&pbstrdocurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocument<Impl: IUPnPServiceDocumentAccessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdoc: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDocument(::core::mem::transmute_copy(&pbstrdoc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPServiceDocumentAccess>, base.5, GetDocumentURL::<Impl, OFFSET>, GetDocument::<Impl, OFFSET>)
    }
}
pub trait IUPnPServiceEnumPropertyImpl: Sized {
    fn SetServiceEnumProperty();
}
impl ::windows::core::RuntimeName for IUPnPServiceEnumProperty {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPServiceEnumProperty";
}
impl IUPnPServiceEnumPropertyVtbl {
    pub const fn new<Impl: IUPnPServiceEnumPropertyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPServiceEnumPropertyVtbl {
        unsafe extern "system" fn SetServiceEnumProperty<Impl: IUPnPServiceEnumPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetServiceEnumProperty(dwmask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPServiceEnumProperty>, base.5, SetServiceEnumProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPServicesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUPnPServices {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPServices";
}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPServicesVtbl {
    pub const fn new<Impl: IUPnPServicesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUPnPServicesVtbl {
        unsafe extern "system" fn Count<Impl: IUPnPServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IUPnPServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IUPnPServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrserviceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppservice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&bstrserviceid as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppservice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUPnPServices>, base.5, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
