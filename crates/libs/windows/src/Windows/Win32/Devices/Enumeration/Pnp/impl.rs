pub trait IUPnPAddressFamilyControlImpl: Sized {
    fn SetAddressFamily();
    fn GetAddressFamily();
}
impl ::windows::core::RuntimeName for IUPnPAddressFamilyControl {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPAddressFamilyControl";
}
impl IUPnPAddressFamilyControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPAddressFamilyControlImpl, const OFFSET: isize>() -> IUPnPAddressFamilyControlVtbl {
        unsafe extern "system" fn SetAddressFamily<Impl: IUPnPAddressFamilyControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAddressFamily(dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAddressFamily<Impl: IUPnPAddressFamilyControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAddressFamily(::core::mem::transmute_copy(&pdwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPAddressFamilyControl>, ::windows::core::GetTrustLevel, SetAddressFamily::<Impl, OFFSET>, GetAddressFamily::<Impl, OFFSET>)
    }
}
pub trait IUPnPAsyncResultImpl: Sized {
    fn AsyncOperationComplete();
}
impl ::windows::core::RuntimeName for IUPnPAsyncResult {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPAsyncResult";
}
impl IUPnPAsyncResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPAsyncResultImpl, const OFFSET: isize>() -> IUPnPAsyncResultVtbl {
        unsafe extern "system" fn AsyncOperationComplete<Impl: IUPnPAsyncResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsyncOperationComplete(ullrequestid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPAsyncResult>, ::windows::core::GetTrustLevel, AsyncOperationComplete::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDescriptionDocumentImpl, const OFFSET: isize>() -> IUPnPDescriptionDocumentVtbl {
        unsafe extern "system" fn ReadyState<Impl: IUPnPDescriptionDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plreadystate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadyState(::core::mem::transmute_copy(&plreadystate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Load<Impl: IUPnPDescriptionDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Load(&*(&bstrurl as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadAsync<Impl: IUPnPDescriptionDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, punkcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadAsync(&*(&bstrurl as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&punkcallback as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadResult<Impl: IUPnPDescriptionDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerror: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadResult(::core::mem::transmute_copy(&phrerror)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Impl: IUPnPDescriptionDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Abort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RootDevice<Impl: IUPnPDescriptionDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppudrootdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RootDevice(::core::mem::transmute_copy(&ppudrootdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceByUDN<Impl: IUPnPDescriptionDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppuddevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceByUDN(&*(&bstrudn as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppuddevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPDescriptionDocument>, ::windows::core::GetTrustLevel, ReadyState::<Impl, OFFSET>, Load::<Impl, OFFSET>, LoadAsync::<Impl, OFFSET>, LoadResult::<Impl, OFFSET>, Abort::<Impl, OFFSET>, RootDevice::<Impl, OFFSET>, DeviceByUDN::<Impl, OFFSET>)
    }
}
pub trait IUPnPDescriptionDocumentCallbackImpl: Sized {
    fn LoadComplete();
}
impl ::windows::core::RuntimeName for IUPnPDescriptionDocumentCallback {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPDescriptionDocumentCallback";
}
impl IUPnPDescriptionDocumentCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDescriptionDocumentCallbackImpl, const OFFSET: isize>() -> IUPnPDescriptionDocumentCallbackVtbl {
        unsafe extern "system" fn LoadComplete<Impl: IUPnPDescriptionDocumentCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrloadresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadComplete(hrloadresult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPDescriptionDocumentCallback>, ::windows::core::GetTrustLevel, LoadComplete::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDeviceImpl, const OFFSET: isize>() -> IUPnPDeviceVtbl {
        unsafe extern "system" fn IsRootDevice<Impl: IUPnPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarb: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRootDevice(::core::mem::transmute_copy(&pvarb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RootDevice<Impl: IUPnPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppudrootdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RootDevice(::core::mem::transmute_copy(&ppudrootdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParentDevice<Impl: IUPnPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuddeviceparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParentDevice(::core::mem::transmute_copy(&ppuddeviceparent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasChildren<Impl: IUPnPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarb: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasChildren(::core::mem::transmute_copy(&pvarb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Impl: IUPnPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppudchildren: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Children(::core::mem::transmute_copy(&ppudchildren)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UniqueDeviceName<Impl: IUPnPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UniqueDeviceName(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FriendlyName<Impl: IUPnPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FriendlyName(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IUPnPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresentationURL<Impl: IUPnPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresentationURL(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManufacturerName<Impl: IUPnPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManufacturerName(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManufacturerURL<Impl: IUPnPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManufacturerURL(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelName<Impl: IUPnPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModelName(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelNumber<Impl: IUPnPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModelNumber(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IUPnPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelURL<Impl: IUPnPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModelURL(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UPC<Impl: IUPnPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UPC(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerialNumber<Impl: IUPnPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SerialNumber(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IconURL<Impl: IUPnPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrencodingformat: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, lsizex: i32, lsizey: i32, lbitdepth: i32, pbstriconurl: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IconURL(&*(&bstrencodingformat as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lsizex, lsizey, lbitdepth, ::core::mem::transmute_copy(&pbstriconurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Services<Impl: IUPnPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppusservices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUPnPDevice>,
            ::windows::core::GetTrustLevel,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDeviceControlImpl, const OFFSET: isize>() -> IUPnPDeviceControlVtbl {
        unsafe extern "system" fn Initialize<Impl: IUPnPDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn GetServiceObject<Impl: IUPnPDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrserviceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppdispservice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetServiceObject(&*(&bstrudn as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrserviceid as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdispservice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPDeviceControl>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>, GetServiceObject::<Impl, OFFSET>)
    }
}
pub trait IUPnPDeviceControlHttpHeadersImpl: Sized {
    fn GetAdditionalResponseHeaders();
}
impl ::windows::core::RuntimeName for IUPnPDeviceControlHttpHeaders {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPDeviceControlHttpHeaders";
}
impl IUPnPDeviceControlHttpHeadersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDeviceControlHttpHeadersImpl, const OFFSET: isize>() -> IUPnPDeviceControlHttpHeadersVtbl {
        unsafe extern "system" fn GetAdditionalResponseHeaders<Impl: IUPnPDeviceControlHttpHeadersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhttpresponseheaders: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAdditionalResponseHeaders(::core::mem::transmute_copy(&bstrhttpresponseheaders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPDeviceControlHttpHeaders>, ::windows::core::GetTrustLevel, GetAdditionalResponseHeaders::<Impl, OFFSET>)
    }
}
pub trait IUPnPDeviceDocumentAccessImpl: Sized {
    fn GetDocumentURL();
}
impl ::windows::core::RuntimeName for IUPnPDeviceDocumentAccess {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPDeviceDocumentAccess";
}
impl IUPnPDeviceDocumentAccessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDeviceDocumentAccessImpl, const OFFSET: isize>() -> IUPnPDeviceDocumentAccessVtbl {
        unsafe extern "system" fn GetDocumentURL<Impl: IUPnPDeviceDocumentAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdocument: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentURL(::core::mem::transmute_copy(&pbstrdocument)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPDeviceDocumentAccess>, ::windows::core::GetTrustLevel, GetDocumentURL::<Impl, OFFSET>)
    }
}
pub trait IUPnPDeviceDocumentAccessExImpl: Sized {
    fn GetDocument();
}
impl ::windows::core::RuntimeName for IUPnPDeviceDocumentAccessEx {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPDeviceDocumentAccessEx";
}
impl IUPnPDeviceDocumentAccessExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDeviceDocumentAccessExImpl, const OFFSET: isize>() -> IUPnPDeviceDocumentAccessExVtbl {
        unsafe extern "system" fn GetDocument<Impl: IUPnPDeviceDocumentAccessExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdocument: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocument(::core::mem::transmute_copy(&pbstrdocument)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPDeviceDocumentAccessEx>, ::windows::core::GetTrustLevel, GetDocument::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDeviceFinderImpl, const OFFSET: isize>() -> IUPnPDeviceFinderVtbl {
        unsafe extern "system" fn FindByType<Impl: IUPnPDeviceFinderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtypeuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dwflags: u32, pdevices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindByType(&*(&bstrtypeuri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), dwflags, ::core::mem::transmute_copy(&pdevices)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAsyncFind<Impl: IUPnPDeviceFinderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtypeuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dwflags: u32, punkdevicefindercallback: *mut ::core::ffi::c_void, plfinddata: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAsyncFind(&*(&bstrtypeuri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), dwflags, &*(&punkdevicefindercallback as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&plfinddata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAsyncFind<Impl: IUPnPDeviceFinderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartAsyncFind(lfinddata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelAsyncFind<Impl: IUPnPDeviceFinderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelAsyncFind(lfinddata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindByUDN<Impl: IUPnPDeviceFinderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindByUDN(&*(&bstrudn as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPDeviceFinder>, ::windows::core::GetTrustLevel, FindByType::<Impl, OFFSET>, CreateAsyncFind::<Impl, OFFSET>, StartAsyncFind::<Impl, OFFSET>, CancelAsyncFind::<Impl, OFFSET>, FindByUDN::<Impl, OFFSET>)
    }
}
pub trait IUPnPDeviceFinderAddCallbackWithInterfaceImpl: Sized {
    fn DeviceAddedWithInterface();
}
impl ::windows::core::RuntimeName for IUPnPDeviceFinderAddCallbackWithInterface {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPDeviceFinderAddCallbackWithInterface";
}
impl IUPnPDeviceFinderAddCallbackWithInterfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDeviceFinderAddCallbackWithInterfaceImpl, const OFFSET: isize>() -> IUPnPDeviceFinderAddCallbackWithInterfaceVtbl {
        unsafe extern "system" fn DeviceAddedWithInterface<Impl: IUPnPDeviceFinderAddCallbackWithInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32, pdevice: ::windows::core::RawPtr, pguidinterface: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceAddedWithInterface(lfinddata, &*(&pdevice as *const <IUPnPDevice as ::windows::core::Abi>::Abi as *const <IUPnPDevice as ::windows::core::DefaultType>::DefaultType), &*(&pguidinterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPDeviceFinderAddCallbackWithInterface>, ::windows::core::GetTrustLevel, DeviceAddedWithInterface::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDeviceFinderCallbackImpl, const OFFSET: isize>() -> IUPnPDeviceFinderCallbackVtbl {
        unsafe extern "system" fn DeviceAdded<Impl: IUPnPDeviceFinderCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32, pdevice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceAdded(lfinddata, &*(&pdevice as *const <IUPnPDevice as ::windows::core::Abi>::Abi as *const <IUPnPDevice as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceRemoved<Impl: IUPnPDeviceFinderCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceRemoved(lfinddata, &*(&bstrudn as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchComplete<Impl: IUPnPDeviceFinderCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchComplete(lfinddata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPDeviceFinderCallback>, ::windows::core::GetTrustLevel, DeviceAdded::<Impl, OFFSET>, DeviceRemoved::<Impl, OFFSET>, SearchComplete::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDeviceProviderImpl, const OFFSET: isize>() -> IUPnPDeviceProviderVtbl {
        unsafe extern "system" fn Start<Impl: IUPnPDeviceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Start(&*(&bstrinitstring as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Impl: IUPnPDeviceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPDeviceProvider>, ::windows::core::GetTrustLevel, Start::<Impl, OFFSET>, Stop::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPDevicesImpl, const OFFSET: isize>() -> IUPnPDevicesVtbl {
        unsafe extern "system" fn Count<Impl: IUPnPDevicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IUPnPDevicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IUPnPDevicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&bstrudn as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPDevices>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPEventSinkImpl, const OFFSET: isize>() -> IUPnPEventSinkVtbl {
        unsafe extern "system" fn OnStateChanged<Impl: IUPnPEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchanges: u32, rgdispidchanges: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnStateChanged(cchanges, rgdispidchanges) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnStateChangedSafe<Impl: IUPnPEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsadispidchanges: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnStateChangedSafe(&*(&varsadispidchanges as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPEventSink>, ::windows::core::GetTrustLevel, OnStateChanged::<Impl, OFFSET>, OnStateChangedSafe::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPEventSourceImpl, const OFFSET: isize>() -> IUPnPEventSourceVtbl {
        unsafe extern "system" fn Advise<Impl: IUPnPEventSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pessubscriber: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advise(&*(&pessubscriber as *const <IUPnPEventSink as ::windows::core::Abi>::Abi as *const <IUPnPEventSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Impl: IUPnPEventSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pessubscriber: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unadvise(&*(&pessubscriber as *const <IUPnPEventSink as ::windows::core::Abi>::Abi as *const <IUPnPEventSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPEventSource>, ::windows::core::GetTrustLevel, Advise::<Impl, OFFSET>, Unadvise::<Impl, OFFSET>)
    }
}
pub trait IUPnPHttpHeaderControlImpl: Sized {
    fn AddRequestHeaders();
}
impl ::windows::core::RuntimeName for IUPnPHttpHeaderControl {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPHttpHeaderControl";
}
impl IUPnPHttpHeaderControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPHttpHeaderControlImpl, const OFFSET: isize>() -> IUPnPHttpHeaderControlVtbl {
        unsafe extern "system" fn AddRequestHeaders<Impl: IUPnPHttpHeaderControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhttpheaders: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddRequestHeaders(&*(&bstrhttpheaders as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPHttpHeaderControl>, ::windows::core::GetTrustLevel, AddRequestHeaders::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPRegistrarImpl, const OFFSET: isize>() -> IUPnPRegistrarVtbl {
        unsafe extern "system" fn RegisterDevice<Impl: IUPnPRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrprogiddevicecontrolclass: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcontainerid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32, pbstrdeviceidentifier: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn RegisterRunningDevice<Impl: IUPnPRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, punkdevicecontrol: *mut ::core::ffi::c_void, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32, pbstrdeviceidentifier: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn RegisterDeviceProvider<Impl: IUPnPRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrprogidproviderclass: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcontainerid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn GetUniqueDeviceName<Impl: IUPnPRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrtemplateudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrudn: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUniqueDeviceName(&*(&bstrdeviceidentifier as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrtemplateudn as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrudn)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDevice<Impl: IUPnPRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fpermanent: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterDevice(&*(&bstrdeviceidentifier as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&fpermanent as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDeviceProvider<Impl: IUPnPRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterDeviceProvider(&*(&bstrprovidername as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPRegistrar>, ::windows::core::GetTrustLevel, RegisterDevice::<Impl, OFFSET>, RegisterRunningDevice::<Impl, OFFSET>, RegisterDeviceProvider::<Impl, OFFSET>, GetUniqueDeviceName::<Impl, OFFSET>, UnregisterDevice::<Impl, OFFSET>, UnregisterDeviceProvider::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPRemoteEndpointInfoImpl, const OFFSET: isize>() -> IUPnPRemoteEndpointInfoVtbl {
        unsafe extern "system" fn GetDwordValue<Impl: IUPnPRemoteEndpointInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvaluename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdwvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDwordValue(&*(&bstrvaluename as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringValue<Impl: IUPnPRemoteEndpointInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvaluename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringValue(&*(&bstrvaluename as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGuidValue<Impl: IUPnPRemoteEndpointInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvaluename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pguidvalue: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGuidValue(&*(&bstrvaluename as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pguidvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPRemoteEndpointInfo>, ::windows::core::GetTrustLevel, GetDwordValue::<Impl, OFFSET>, GetStringValue::<Impl, OFFSET>, GetGuidValue::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPReregistrarImpl, const OFFSET: isize>() -> IUPnPReregistrarVtbl {
        unsafe extern "system" fn ReregisterDevice<Impl: IUPnPReregistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrprogiddevicecontrolclass: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcontainerid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn ReregisterRunningDevice<Impl: IUPnPReregistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, punkdevicecontrol: *mut ::core::ffi::c_void, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPReregistrar>, ::windows::core::GetTrustLevel, ReregisterDevice::<Impl, OFFSET>, ReregisterRunningDevice::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPServiceImpl, const OFFSET: isize>() -> IUPnPServiceVtbl {
        unsafe extern "system" fn QueryStateVariable<Impl: IUPnPServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvariablename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryStateVariable(&*(&bstrvariablename as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeAction<Impl: IUPnPServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstractionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vinactionargs: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn ServiceTypeIdentifier<Impl: IUPnPServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceTypeIdentifier(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddCallback<Impl: IUPnPServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddCallback(&*(&punkcallback as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IUPnPServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&pbstrid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastTransportStatus<Impl: IUPnPServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastTransportStatus(::core::mem::transmute_copy(&plvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPService>, ::windows::core::GetTrustLevel, QueryStateVariable::<Impl, OFFSET>, InvokeAction::<Impl, OFFSET>, ServiceTypeIdentifier::<Impl, OFFSET>, AddCallback::<Impl, OFFSET>, Id::<Impl, OFFSET>, LastTransportStatus::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPServiceAsyncImpl, const OFFSET: isize>() -> IUPnPServiceAsyncVtbl {
        unsafe extern "system" fn BeginInvokeAction<Impl: IUPnPServiceAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstractionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vinactionargs: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pasyncresult: ::windows::core::RawPtr, pullrequestid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn EndInvokeAction<Impl: IUPnPServiceAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndInvokeAction(ullrequestid, &*(&pvoutactionargs as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&pvretval as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginQueryStateVariable<Impl: IUPnPServiceAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvariablename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pasyncresult: ::windows::core::RawPtr, pullrequestid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginQueryStateVariable(&*(&bstrvariablename as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pasyncresult as *const <IUPnPAsyncResult as ::windows::core::Abi>::Abi as *const <IUPnPAsyncResult as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pullrequestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndQueryStateVariable<Impl: IUPnPServiceAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndQueryStateVariable(ullrequestid, &*(&pvalue as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginSubscribeToEvents<Impl: IUPnPServiceAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkcallback: *mut ::core::ffi::c_void, pasyncresult: ::windows::core::RawPtr, pullrequestid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginSubscribeToEvents(&*(&punkcallback as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&pasyncresult as *const <IUPnPAsyncResult as ::windows::core::Abi>::Abi as *const <IUPnPAsyncResult as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pullrequestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSubscribeToEvents<Impl: IUPnPServiceAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndSubscribeToEvents(ullrequestid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginSCPDDownload<Impl: IUPnPServiceAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasyncresult: ::windows::core::RawPtr, pullrequestid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginSCPDDownload(&*(&pasyncresult as *const <IUPnPAsyncResult as ::windows::core::Abi>::Abi as *const <IUPnPAsyncResult as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pullrequestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSCPDDownload<Impl: IUPnPServiceAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64, pbstrscpddoc: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndSCPDDownload(ullrequestid, ::core::mem::transmute_copy(&pbstrscpddoc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelAsyncOperation<Impl: IUPnPServiceAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelAsyncOperation(ullrequestid) {
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
            ::windows::core::GetRuntimeClassName::<IUPnPServiceAsync>,
            ::windows::core::GetTrustLevel,
            BeginInvokeAction::<Impl, OFFSET>,
            EndInvokeAction::<Impl, OFFSET>,
            BeginQueryStateVariable::<Impl, OFFSET>,
            EndQueryStateVariable::<Impl, OFFSET>,
            BeginSubscribeToEvents::<Impl, OFFSET>,
            EndSubscribeToEvents::<Impl, OFFSET>,
            BeginSCPDDownload::<Impl, OFFSET>,
            EndSCPDDownload::<Impl, OFFSET>,
            CancelAsyncOperation::<Impl, OFFSET>,
        )
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPServiceCallbackImpl, const OFFSET: isize>() -> IUPnPServiceCallbackVtbl {
        unsafe extern "system" fn StateVariableChanged<Impl: IUPnPServiceCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pus: ::windows::core::RawPtr, pcwszstatevarname: super::super::super::Foundation::PWSTR, vavalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn ServiceInstanceDied<Impl: IUPnPServiceCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pus: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceInstanceDied(&*(&pus as *const <IUPnPService as ::windows::core::Abi>::Abi as *const <IUPnPService as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPServiceCallback>, ::windows::core::GetTrustLevel, StateVariableChanged::<Impl, OFFSET>, ServiceInstanceDied::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPServiceDocumentAccessImpl, const OFFSET: isize>() -> IUPnPServiceDocumentAccessVtbl {
        unsafe extern "system" fn GetDocumentURL<Impl: IUPnPServiceDocumentAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdocurl: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentURL(::core::mem::transmute_copy(&pbstrdocurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocument<Impl: IUPnPServiceDocumentAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdoc: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocument(::core::mem::transmute_copy(&pbstrdoc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPServiceDocumentAccess>, ::windows::core::GetTrustLevel, GetDocumentURL::<Impl, OFFSET>, GetDocument::<Impl, OFFSET>)
    }
}
pub trait IUPnPServiceEnumPropertyImpl: Sized {
    fn SetServiceEnumProperty();
}
impl ::windows::core::RuntimeName for IUPnPServiceEnumProperty {
    const NAME: &'static str = "Windows.Win32.Devices.Enumeration.Pnp.IUPnPServiceEnumProperty";
}
impl IUPnPServiceEnumPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPServiceEnumPropertyImpl, const OFFSET: isize>() -> IUPnPServiceEnumPropertyVtbl {
        unsafe extern "system" fn SetServiceEnumProperty<Impl: IUPnPServiceEnumPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetServiceEnumProperty(dwmask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPServiceEnumProperty>, ::windows::core::GetTrustLevel, SetServiceEnumProperty::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPServicesImpl, const OFFSET: isize>() -> IUPnPServicesVtbl {
        unsafe extern "system" fn Count<Impl: IUPnPServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IUPnPServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IUPnPServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrserviceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppservice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&bstrserviceid as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppservice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPServices>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
