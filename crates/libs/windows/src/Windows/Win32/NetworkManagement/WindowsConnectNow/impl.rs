pub trait IWCNConnectNotify_Impl: Sized {
    fn ConnectSucceeded(&mut self) -> ::windows::core::Result<()>;
    fn ConnectFailed(&mut self, hrfailure: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IWCNConnectNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWCNConnectNotify_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWCNConnectNotify_Vtbl {
        unsafe extern "system" fn ConnectSucceeded<Impl: IWCNConnectNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConnectSucceeded().into()
        }
        unsafe extern "system" fn ConnectFailed<Impl: IWCNConnectNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrfailure: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConnectFailed(::core::mem::transmute_copy(&hrfailure)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ConnectSucceeded: ConnectSucceeded::<Impl, IMPL_OFFSET>,
            ConnectFailed: ConnectFailed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWCNConnectNotify as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWCNDevice_Impl: Sized {
    fn SetPassword(&mut self, r#type: WCN_PASSWORD_TYPE, dwpasswordlength: u32, pbpassword: *const u8) -> ::windows::core::Result<()>;
    fn Connect(&mut self, pnotify: &::core::option::Option<IWCNConnectNotify>) -> ::windows::core::Result<()>;
    fn GetAttribute(&mut self, attributetype: WCN_ATTRIBUTE_TYPE, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> ::windows::core::Result<()>;
    fn GetIntegerAttribute(&mut self, attributetype: WCN_ATTRIBUTE_TYPE) -> ::windows::core::Result<u32>;
    fn GetStringAttribute(&mut self, attributetype: WCN_ATTRIBUTE_TYPE, cchmaxstring: u32, wszstring: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetNetworkProfile(&mut self, cchmaxstringlength: u32, wszprofile: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetNetworkProfile(&mut self, pszprofilexml: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetVendorExtension(&mut self, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> ::windows::core::Result<()>;
    fn SetVendorExtension(&mut self, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, cbbuffer: u32, pbbuffer: *const u8) -> ::windows::core::Result<()>;
    fn Unadvise(&mut self) -> ::windows::core::Result<()>;
    fn SetNFCPasswordParams(&mut self, r#type: WCN_PASSWORD_TYPE, dwoobpasswordid: u32, dwpasswordlength: u32, pbpassword: *const u8, dwremotepublickeyhashlength: u32, pbremotepublickeyhash: *const u8, dwdhkeybloblength: u32, pbdhkeyblob: *const u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWCNDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWCNDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWCNDevice_Vtbl {
        unsafe extern "system" fn SetPassword<Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WCN_PASSWORD_TYPE, dwpasswordlength: u32, pbpassword: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPassword(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&dwpasswordlength), ::core::mem::transmute_copy(&pbpassword)).into()
        }
        unsafe extern "system" fn Connect<Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotify: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect(::core::mem::transmute(&pnotify)).into()
        }
        unsafe extern "system" fn GetAttribute<Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributetype: WCN_ATTRIBUTE_TYPE, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAttribute(::core::mem::transmute_copy(&attributetype), ::core::mem::transmute_copy(&dwmaxbuffersize), ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pdwbufferused)).into()
        }
        unsafe extern "system" fn GetIntegerAttribute<Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributetype: WCN_ATTRIBUTE_TYPE, puinteger: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIntegerAttribute(::core::mem::transmute_copy(&attributetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *puinteger = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringAttribute<Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributetype: WCN_ATTRIBUTE_TYPE, cchmaxstring: u32, wszstring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStringAttribute(::core::mem::transmute_copy(&attributetype), ::core::mem::transmute_copy(&cchmaxstring), ::core::mem::transmute_copy(&wszstring)).into()
        }
        unsafe extern "system" fn GetNetworkProfile<Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchmaxstringlength: u32, wszprofile: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNetworkProfile(::core::mem::transmute_copy(&cchmaxstringlength), ::core::mem::transmute_copy(&wszprofile)).into()
        }
        unsafe extern "system" fn SetNetworkProfile<Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszprofilexml: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNetworkProfile(::core::mem::transmute_copy(&pszprofilexml)).into()
        }
        unsafe extern "system" fn GetVendorExtension<Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVendorExtension(::core::mem::transmute_copy(&pvendorextspec), ::core::mem::transmute_copy(&dwmaxbuffersize), ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pdwbufferused)).into()
        }
        unsafe extern "system" fn SetVendorExtension<Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, cbbuffer: u32, pbbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVendorExtension(::core::mem::transmute_copy(&pvendorextspec), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&pbbuffer)).into()
        }
        unsafe extern "system" fn Unadvise<Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unadvise().into()
        }
        unsafe extern "system" fn SetNFCPasswordParams<Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WCN_PASSWORD_TYPE, dwoobpasswordid: u32, dwpasswordlength: u32, pbpassword: *const u8, dwremotepublickeyhashlength: u32, pbremotepublickeyhash: *const u8, dwdhkeybloblength: u32, pbdhkeyblob: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNFCPasswordParams(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&dwoobpasswordid), ::core::mem::transmute_copy(&dwpasswordlength), ::core::mem::transmute_copy(&pbpassword), ::core::mem::transmute_copy(&dwremotepublickeyhashlength), ::core::mem::transmute_copy(&pbremotepublickeyhash), ::core::mem::transmute_copy(&dwdhkeybloblength), ::core::mem::transmute_copy(&pbdhkeyblob)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetPassword: SetPassword::<Impl, IMPL_OFFSET>,
            Connect: Connect::<Impl, IMPL_OFFSET>,
            GetAttribute: GetAttribute::<Impl, IMPL_OFFSET>,
            GetIntegerAttribute: GetIntegerAttribute::<Impl, IMPL_OFFSET>,
            GetStringAttribute: GetStringAttribute::<Impl, IMPL_OFFSET>,
            GetNetworkProfile: GetNetworkProfile::<Impl, IMPL_OFFSET>,
            SetNetworkProfile: SetNetworkProfile::<Impl, IMPL_OFFSET>,
            GetVendorExtension: GetVendorExtension::<Impl, IMPL_OFFSET>,
            SetVendorExtension: SetVendorExtension::<Impl, IMPL_OFFSET>,
            Unadvise: Unadvise::<Impl, IMPL_OFFSET>,
            SetNFCPasswordParams: SetNFCPasswordParams::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWCNDevice as ::windows::core::Interface>::IID
    }
}
