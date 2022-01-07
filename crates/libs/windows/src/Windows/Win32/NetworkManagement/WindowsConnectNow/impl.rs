pub trait IWCNConnectNotifyImpl: Sized {
    fn ConnectSucceeded();
    fn ConnectFailed();
}
impl ::windows::core::RuntimeName for IWCNConnectNotify {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsConnectNow.IWCNConnectNotify";
}
impl IWCNConnectNotifyVtbl {
    pub const fn new<Impl: IWCNConnectNotifyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWCNConnectNotifyVtbl {
        unsafe extern "system" fn ConnectSucceeded<Impl: IWCNConnectNotifyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectSucceeded() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectFailed<Impl: IWCNConnectNotifyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrfailure: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectFailed(hrfailure) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWCNConnectNotify>, base.5, ConnectSucceeded::<Impl, OFFSET>, ConnectFailed::<Impl, OFFSET>)
    }
}
pub trait IWCNDeviceImpl: Sized {
    fn SetPassword();
    fn Connect();
    fn GetAttribute();
    fn GetIntegerAttribute();
    fn GetStringAttribute();
    fn GetNetworkProfile();
    fn SetNetworkProfile();
    fn GetVendorExtension();
    fn SetVendorExtension();
    fn Unadvise();
    fn SetNFCPasswordParams();
}
impl ::windows::core::RuntimeName for IWCNDevice {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsConnectNow.IWCNDevice";
}
impl IWCNDeviceVtbl {
    pub const fn new<Impl: IWCNDeviceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWCNDeviceVtbl {
        unsafe extern "system" fn SetPassword<Impl: IWCNDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: WCN_PASSWORD_TYPE, dwpasswordlength: u32, pbpassword: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPassword(r#type, dwpasswordlength, pbpassword) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Impl: IWCNDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnotify: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Connect(&*(&pnotify as *const <IWCNConnectNotify as ::windows::core::Abi>::Abi as *const <IWCNConnectNotify as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttribute<Impl: IWCNDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributetype: WCN_ATTRIBUTE_TYPE, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttribute(attributetype, dwmaxbuffersize, ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pdwbufferused)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntegerAttribute<Impl: IWCNDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributetype: WCN_ATTRIBUTE_TYPE, puinteger: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIntegerAttribute(attributetype, ::core::mem::transmute_copy(&puinteger)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringAttribute<Impl: IWCNDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributetype: WCN_ATTRIBUTE_TYPE, cchmaxstring: u32, wszstring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStringAttribute(attributetype, cchmaxstring, ::core::mem::transmute_copy(&wszstring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkProfile<Impl: IWCNDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchmaxstringlength: u32, wszprofile: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNetworkProfile(cchmaxstringlength, ::core::mem::transmute_copy(&wszprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkProfile<Impl: IWCNDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszprofilexml: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNetworkProfile(&*(&pszprofilexml as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVendorExtension<Impl: IWCNDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVendorExtension(&*(&pvendorextspec as *const <WCN_VENDOR_EXTENSION_SPEC as ::windows::core::Abi>::Abi as *const <WCN_VENDOR_EXTENSION_SPEC as ::windows::core::DefaultType>::DefaultType), dwmaxbuffersize, ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pdwbufferused)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVendorExtension<Impl: IWCNDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, cbbuffer: u32, pbbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVendorExtension(&*(&pvendorextspec as *const <WCN_VENDOR_EXTENSION_SPEC as ::windows::core::Abi>::Abi as *const <WCN_VENDOR_EXTENSION_SPEC as ::windows::core::DefaultType>::DefaultType), cbbuffer, pbbuffer) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Impl: IWCNDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unadvise() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNFCPasswordParams<Impl: IWCNDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: WCN_PASSWORD_TYPE, dwoobpasswordid: u32, dwpasswordlength: u32, pbpassword: *const u8, dwremotepublickeyhashlength: u32, pbremotepublickeyhash: *const u8, dwdhkeybloblength: u32, pbdhkeyblob: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNFCPasswordParams(r#type, dwoobpasswordid, dwpasswordlength, pbpassword, dwremotepublickeyhashlength, pbremotepublickeyhash, dwdhkeybloblength, pbdhkeyblob) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWCNDevice>, base.5, SetPassword::<Impl, OFFSET>, Connect::<Impl, OFFSET>, GetAttribute::<Impl, OFFSET>, GetIntegerAttribute::<Impl, OFFSET>, GetStringAttribute::<Impl, OFFSET>, GetNetworkProfile::<Impl, OFFSET>, SetNetworkProfile::<Impl, OFFSET>, GetVendorExtension::<Impl, OFFSET>, SetVendorExtension::<Impl, OFFSET>, Unadvise::<Impl, OFFSET>, SetNFCPasswordParams::<Impl, OFFSET>)
    }
}
