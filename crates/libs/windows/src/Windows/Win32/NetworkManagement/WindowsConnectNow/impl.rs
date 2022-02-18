pub trait IWCNConnectNotify_Impl: Sized {
    fn ConnectSucceeded(&self) -> ::windows::core::Result<()>;
    fn ConnectFailed(&self, hrfailure: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IWCNConnectNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWCNConnectNotify_Impl, const OFFSET: isize>() -> IWCNConnectNotify_Vtbl {
        unsafe extern "system" fn ConnectSucceeded<Identity: ::windows::core::IUnknownImpl, Impl: IWCNConnectNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConnectSucceeded().into()
        }
        unsafe extern "system" fn ConnectFailed<Identity: ::windows::core::IUnknownImpl, Impl: IWCNConnectNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrfailure: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConnectFailed(::core::mem::transmute_copy(&hrfailure)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ConnectSucceeded: ConnectSucceeded::<Identity, Impl, OFFSET>,
            ConnectFailed: ConnectFailed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWCNConnectNotify as ::windows::core::Interface>::IID
    }
}
pub trait IWCNDevice_Impl: Sized {
    fn SetPassword(&self, r#type: WCN_PASSWORD_TYPE, dwpasswordlength: u32, pbpassword: *const u8) -> ::windows::core::Result<()>;
    fn Connect(&self, pnotify: &::core::option::Option<IWCNConnectNotify>) -> ::windows::core::Result<()>;
    fn GetAttribute(&self, attributetype: WCN_ATTRIBUTE_TYPE, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> ::windows::core::Result<()>;
    fn GetIntegerAttribute(&self, attributetype: WCN_ATTRIBUTE_TYPE) -> ::windows::core::Result<u32>;
    fn GetStringAttribute(&self, attributetype: WCN_ATTRIBUTE_TYPE, cchmaxstring: u32, wszstring: ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn GetNetworkProfile(&self, cchmaxstringlength: u32, wszprofile: ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn SetNetworkProfile(&self, pszprofilexml: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetVendorExtension(&self, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> ::windows::core::Result<()>;
    fn SetVendorExtension(&self, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, cbbuffer: u32, pbbuffer: *const u8) -> ::windows::core::Result<()>;
    fn Unadvise(&self) -> ::windows::core::Result<()>;
    fn SetNFCPasswordParams(&self, r#type: WCN_PASSWORD_TYPE, dwoobpasswordid: u32, dwpasswordlength: u32, pbpassword: *const u8, dwremotepublickeyhashlength: u32, pbremotepublickeyhash: *const u8, dwdhkeybloblength: u32, pbdhkeyblob: *const u8) -> ::windows::core::Result<()>;
}
impl IWCNDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWCNDevice_Impl, const OFFSET: isize>() -> IWCNDevice_Vtbl {
        unsafe extern "system" fn SetPassword<Identity: ::windows::core::IUnknownImpl, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WCN_PASSWORD_TYPE, dwpasswordlength: u32, pbpassword: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPassword(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&dwpasswordlength), ::core::mem::transmute_copy(&pbpassword)).into()
        }
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotify: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Connect(::core::mem::transmute(&pnotify)).into()
        }
        unsafe extern "system" fn GetAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributetype: WCN_ATTRIBUTE_TYPE, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAttribute(::core::mem::transmute_copy(&attributetype), ::core::mem::transmute_copy(&dwmaxbuffersize), ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pdwbufferused)).into()
        }
        unsafe extern "system" fn GetIntegerAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributetype: WCN_ATTRIBUTE_TYPE, puinteger: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIntegerAttribute(::core::mem::transmute_copy(&attributetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *puinteger = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributetype: WCN_ATTRIBUTE_TYPE, cchmaxstring: u32, wszstring: ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStringAttribute(::core::mem::transmute_copy(&attributetype), ::core::mem::transmute_copy(&cchmaxstring), ::core::mem::transmute_copy(&wszstring)).into()
        }
        unsafe extern "system" fn GetNetworkProfile<Identity: ::windows::core::IUnknownImpl, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchmaxstringlength: u32, wszprofile: ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNetworkProfile(::core::mem::transmute_copy(&cchmaxstringlength), ::core::mem::transmute_copy(&wszprofile)).into()
        }
        unsafe extern "system" fn SetNetworkProfile<Identity: ::windows::core::IUnknownImpl, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszprofilexml: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNetworkProfile(::core::mem::transmute(&pszprofilexml)).into()
        }
        unsafe extern "system" fn GetVendorExtension<Identity: ::windows::core::IUnknownImpl, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVendorExtension(::core::mem::transmute_copy(&pvendorextspec), ::core::mem::transmute_copy(&dwmaxbuffersize), ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pdwbufferused)).into()
        }
        unsafe extern "system" fn SetVendorExtension<Identity: ::windows::core::IUnknownImpl, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, cbbuffer: u32, pbbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVendorExtension(::core::mem::transmute_copy(&pvendorextspec), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&pbbuffer)).into()
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows::core::IUnknownImpl, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unadvise().into()
        }
        unsafe extern "system" fn SetNFCPasswordParams<Identity: ::windows::core::IUnknownImpl, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WCN_PASSWORD_TYPE, dwoobpasswordid: u32, dwpasswordlength: u32, pbpassword: *const u8, dwremotepublickeyhashlength: u32, pbremotepublickeyhash: *const u8, dwdhkeybloblength: u32, pbdhkeyblob: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNFCPasswordParams(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&dwoobpasswordid), ::core::mem::transmute_copy(&dwpasswordlength), ::core::mem::transmute_copy(&pbpassword), ::core::mem::transmute_copy(&dwremotepublickeyhashlength), ::core::mem::transmute_copy(&pbremotepublickeyhash), ::core::mem::transmute_copy(&dwdhkeybloblength), ::core::mem::transmute_copy(&pbdhkeyblob)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetPassword: SetPassword::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            GetAttribute: GetAttribute::<Identity, Impl, OFFSET>,
            GetIntegerAttribute: GetIntegerAttribute::<Identity, Impl, OFFSET>,
            GetStringAttribute: GetStringAttribute::<Identity, Impl, OFFSET>,
            GetNetworkProfile: GetNetworkProfile::<Identity, Impl, OFFSET>,
            SetNetworkProfile: SetNetworkProfile::<Identity, Impl, OFFSET>,
            GetVendorExtension: GetVendorExtension::<Identity, Impl, OFFSET>,
            SetVendorExtension: SetVendorExtension::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            SetNFCPasswordParams: SetNFCPasswordParams::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWCNDevice as ::windows::core::Interface>::IID
    }
}
