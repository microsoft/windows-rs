pub trait IWCNConnectNotifyImpl: Sized {
    fn ConnectSucceeded();
    fn ConnectFailed();
}
impl IWCNConnectNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWCNConnectNotifyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWCNConnectNotifyVtbl {
        unsafe extern "system" fn ConnectSucceeded<Impl: IWCNConnectNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConnectFailed<Impl: IWCNConnectNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrfailure: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(feature = "Win32_Foundation")]
impl IWCNDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWCNDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWCNDeviceVtbl {
        unsafe extern "system" fn SetPassword<Impl: IWCNDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WCN_PASSWORD_TYPE, dwpasswordlength: u32, pbpassword: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Connect<Impl: IWCNDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotify: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttribute<Impl: IWCNDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributetype: WCN_ATTRIBUTE_TYPE, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIntegerAttribute<Impl: IWCNDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributetype: WCN_ATTRIBUTE_TYPE, puinteger: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStringAttribute<Impl: IWCNDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributetype: WCN_ATTRIBUTE_TYPE, cchmaxstring: u32, wszstring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNetworkProfile<Impl: IWCNDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchmaxstringlength: u32, wszprofile: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNetworkProfile<Impl: IWCNDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszprofilexml: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVendorExtension<Impl: IWCNDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVendorExtension<Impl: IWCNDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, cbbuffer: u32, pbbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unadvise<Impl: IWCNDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNFCPasswordParams<Impl: IWCNDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WCN_PASSWORD_TYPE, dwoobpasswordid: u32, dwpasswordlength: u32, pbpassword: *const u8, dwremotepublickeyhashlength: u32, pbremotepublickeyhash: *const u8, dwdhkeybloblength: u32, pbdhkeyblob: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
