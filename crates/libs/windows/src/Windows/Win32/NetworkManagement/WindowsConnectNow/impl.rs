#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectNow\"`, `\"implement\"`*"]
pub trait IWCNConnectNotify_Impl: Sized {
    fn ConnectSucceeded(&self) -> ::windows::core::Result<()>;
    fn ConnectFailed(&self, hrfailure: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWCNConnectNotify {}
impl IWCNConnectNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCNConnectNotify_Impl, const OFFSET: isize>() -> IWCNConnectNotify_Vtbl {
        unsafe extern "system" fn ConnectSucceeded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCNConnectNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConnectSucceeded().into()
        }
        unsafe extern "system" fn ConnectFailed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCNConnectNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrfailure: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConnectFailed(::core::mem::transmute_copy(&hrfailure)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConnectSucceeded: ConnectSucceeded::<Identity, Impl, OFFSET>,
            ConnectFailed: ConnectFailed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWCNConnectNotify as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectNow\"`, `\"implement\"`*"]
pub trait IWCNDevice_Impl: Sized {
    fn SetPassword(&self, r#type: WCN_PASSWORD_TYPE, dwpasswordlength: u32, pbpassword: *const u8) -> ::windows::core::Result<()>;
    fn Connect(&self, pnotify: ::core::option::Option<&IWCNConnectNotify>) -> ::windows::core::Result<()>;
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
impl ::windows::core::RuntimeName for IWCNDevice {}
impl IWCNDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>() -> IWCNDevice_Vtbl {
        unsafe extern "system" fn SetPassword<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WCN_PASSWORD_TYPE, dwpasswordlength: u32, pbpassword: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPassword(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&dwpasswordlength), ::core::mem::transmute_copy(&pbpassword)).into()
        }
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotify: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Connect(::windows::core::from_raw_borrowed(&pnotify)).into()
        }
        unsafe extern "system" fn GetAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributetype: WCN_ATTRIBUTE_TYPE, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAttribute(::core::mem::transmute_copy(&attributetype), ::core::mem::transmute_copy(&dwmaxbuffersize), ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pdwbufferused)).into()
        }
        unsafe extern "system" fn GetIntegerAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributetype: WCN_ATTRIBUTE_TYPE, puinteger: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIntegerAttribute(::core::mem::transmute_copy(&attributetype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puinteger, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributetype: WCN_ATTRIBUTE_TYPE, cchmaxstring: u32, wszstring: ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStringAttribute(::core::mem::transmute_copy(&attributetype), ::core::mem::transmute_copy(&cchmaxstring), ::core::mem::transmute_copy(&wszstring)).into()
        }
        unsafe extern "system" fn GetNetworkProfile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchmaxstringlength: u32, wszprofile: ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNetworkProfile(::core::mem::transmute_copy(&cchmaxstringlength), ::core::mem::transmute_copy(&wszprofile)).into()
        }
        unsafe extern "system" fn SetNetworkProfile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszprofilexml: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNetworkProfile(::core::mem::transmute(&pszprofilexml)).into()
        }
        unsafe extern "system" fn GetVendorExtension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVendorExtension(::core::mem::transmute_copy(&pvendorextspec), ::core::mem::transmute_copy(&dwmaxbuffersize), ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pdwbufferused)).into()
        }
        unsafe extern "system" fn SetVendorExtension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, cbbuffer: u32, pbbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVendorExtension(::core::mem::transmute_copy(&pvendorextspec), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&pbbuffer)).into()
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unadvise().into()
        }
        unsafe extern "system" fn SetNFCPasswordParams<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WCN_PASSWORD_TYPE, dwoobpasswordid: u32, dwpasswordlength: u32, pbpassword: *const u8, dwremotepublickeyhashlength: u32, pbremotepublickeyhash: *const u8, dwdhkeybloblength: u32, pbdhkeyblob: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNFCPasswordParams(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&dwoobpasswordid), ::core::mem::transmute_copy(&dwpasswordlength), ::core::mem::transmute_copy(&pbpassword), ::core::mem::transmute_copy(&dwremotepublickeyhashlength), ::core::mem::transmute_copy(&pbremotepublickeyhash), ::core::mem::transmute_copy(&dwdhkeybloblength), ::core::mem::transmute_copy(&pbdhkeyblob)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IWCNDevice as ::windows::core::ComInterface>::IID
    }
}
