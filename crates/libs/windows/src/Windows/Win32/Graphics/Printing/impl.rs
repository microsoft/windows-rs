#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAsyncGetSendNotificationCookie_Impl: Sized + IPrintAsyncCookie_Impl {
    fn FinishAsyncCallWithData(&self, param0: ::core::option::Option<&IPrintAsyncNotifyDataObject>, param1: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IAsyncGetSendNotificationCookie {}
#[cfg(feature = "Win32_Foundation")]
impl IAsyncGetSendNotificationCookie_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAsyncGetSendNotificationCookie_Impl, const OFFSET: isize>() -> IAsyncGetSendNotificationCookie_Vtbl {
        unsafe extern "system" fn FinishAsyncCallWithData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAsyncGetSendNotificationCookie_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FinishAsyncCallWithData(::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self { base__: IPrintAsyncCookie_Vtbl::new::<Identity, Impl, OFFSET>(), FinishAsyncCallWithData: FinishAsyncCallWithData::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IAsyncGetSendNotificationCookie as ::windows_core::ComInterface>::IID || *iid == <IPrintAsyncCookie as ::windows_core::ComInterface>::IID
    }
}
pub trait IAsyncGetSrvReferralCookie_Impl: Sized {
    fn FinishAsyncCall(&self, param0: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn CancelAsyncCall(&self, param0: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn FinishAsyncCallWithData(&self, param0: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IAsyncGetSrvReferralCookie {}
impl IAsyncGetSrvReferralCookie_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAsyncGetSrvReferralCookie_Impl, const OFFSET: isize>() -> IAsyncGetSrvReferralCookie_Vtbl {
        unsafe extern "system" fn FinishAsyncCall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAsyncGetSrvReferralCookie_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FinishAsyncCall(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn CancelAsyncCall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAsyncGetSrvReferralCookie_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelAsyncCall(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn FinishAsyncCallWithData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAsyncGetSrvReferralCookie_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FinishAsyncCallWithData(::core::mem::transmute(&param0)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FinishAsyncCall: FinishAsyncCall::<Identity, Impl, OFFSET>,
            CancelAsyncCall: CancelAsyncCall::<Identity, Impl, OFFSET>,
            FinishAsyncCallWithData: FinishAsyncCallWithData::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IAsyncGetSrvReferralCookie as ::windows_core::ComInterface>::IID
    }
}
pub trait IBidiAsyncNotifyChannel_Impl: Sized + IPrintAsyncNotifyChannel_Impl {
    fn CreateNotificationChannel(&self) -> ::windows_core::Result<()>;
    fn GetPrintName(&self, param0: *const ::core::option::Option<IPrintAsyncNotifyDataObject>) -> ::windows_core::Result<()>;
    fn GetChannelNotificationType(&self, param0: *const ::core::option::Option<IPrintAsyncNotifyDataObject>) -> ::windows_core::Result<()>;
    fn AsyncGetNotificationSendResponse(&self, param0: ::core::option::Option<&IPrintAsyncNotifyDataObject>, param1: ::core::option::Option<&IAsyncGetSendNotificationCookie>) -> ::windows_core::Result<()>;
    fn AsyncCloseChannel(&self, param0: ::core::option::Option<&IPrintAsyncNotifyDataObject>, param1: ::core::option::Option<&IPrintAsyncCookie>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IBidiAsyncNotifyChannel {}
impl IBidiAsyncNotifyChannel_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiAsyncNotifyChannel_Impl, const OFFSET: isize>() -> IBidiAsyncNotifyChannel_Vtbl {
        unsafe extern "system" fn CreateNotificationChannel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiAsyncNotifyChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateNotificationChannel().into()
        }
        unsafe extern "system" fn GetPrintName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiAsyncNotifyChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPrintName(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetChannelNotificationType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiAsyncNotifyChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetChannelNotificationType(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn AsyncGetNotificationSendResponse<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiAsyncNotifyChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsyncGetNotificationSendResponse(::windows_core::from_raw_borrowed(&param0), ::windows_core::from_raw_borrowed(&param1)).into()
        }
        unsafe extern "system" fn AsyncCloseChannel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiAsyncNotifyChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsyncCloseChannel(::windows_core::from_raw_borrowed(&param0), ::windows_core::from_raw_borrowed(&param1)).into()
        }
        Self {
            base__: IPrintAsyncNotifyChannel_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateNotificationChannel: CreateNotificationChannel::<Identity, Impl, OFFSET>,
            GetPrintName: GetPrintName::<Identity, Impl, OFFSET>,
            GetChannelNotificationType: GetChannelNotificationType::<Identity, Impl, OFFSET>,
            AsyncGetNotificationSendResponse: AsyncGetNotificationSendResponse::<Identity, Impl, OFFSET>,
            AsyncCloseChannel: AsyncCloseChannel::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IBidiAsyncNotifyChannel as ::windows_core::ComInterface>::IID || *iid == <IPrintAsyncNotifyChannel as ::windows_core::ComInterface>::IID
    }
}
pub trait IBidiRequest_Impl: Sized {
    fn SetSchema(&self, pszschema: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetInputData(&self, dwtype: u32, pdata: *const u8, usize: u32) -> ::windows_core::Result<()>;
    fn GetResult(&self) -> ::windows_core::Result<::windows_core::HRESULT>;
    fn GetOutputData(&self, dwindex: u32, ppszschema: *mut ::windows_core::PWSTR, pdwtype: *mut u32, ppdata: *mut *mut u8, usize: *mut u32) -> ::windows_core::Result<()>;
    fn GetEnumCount(&self) -> ::windows_core::Result<u32>;
}
impl ::windows_core::RuntimeName for IBidiRequest {}
impl IBidiRequest_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiRequest_Impl, const OFFSET: isize>() -> IBidiRequest_Vtbl {
        unsafe extern "system" fn SetSchema<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszschema: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSchema(::core::mem::transmute(&pszschema)).into()
        }
        unsafe extern "system" fn SetInputData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtype: u32, pdata: *const u8, usize: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInputData(::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&usize)).into()
        }
        unsafe extern "system" fn GetResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phr: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetResult() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppszschema: *mut ::windows_core::PWSTR, pdwtype: *mut u32, ppdata: *mut *mut u8, usize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOutputData(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&ppszschema), ::core::mem::transmute_copy(&pdwtype), ::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&usize)).into()
        }
        unsafe extern "system" fn GetEnumCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtotal: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnumCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwtotal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSchema: SetSchema::<Identity, Impl, OFFSET>,
            SetInputData: SetInputData::<Identity, Impl, OFFSET>,
            GetResult: GetResult::<Identity, Impl, OFFSET>,
            GetOutputData: GetOutputData::<Identity, Impl, OFFSET>,
            GetEnumCount: GetEnumCount::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IBidiRequest as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IBidiRequestContainer_Impl: Sized {
    fn AddRequest(&self, prequest: ::core::option::Option<&IBidiRequest>) -> ::windows_core::Result<()>;
    fn GetEnumObject(&self) -> ::windows_core::Result<super::super::System::Com::IEnumUnknown>;
    fn GetRequestCount(&self) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IBidiRequestContainer {}
#[cfg(feature = "Win32_System_Com")]
impl IBidiRequestContainer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiRequestContainer_Impl, const OFFSET: isize>() -> IBidiRequestContainer_Vtbl {
        unsafe extern "system" fn AddRequest<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiRequestContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequest: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRequest(::windows_core::from_raw_borrowed(&prequest)).into()
        }
        unsafe extern "system" fn GetEnumObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiRequestContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnumObject() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRequestCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiRequestContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRequestCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pucount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddRequest: AddRequest::<Identity, Impl, OFFSET>,
            GetEnumObject: GetEnumObject::<Identity, Impl, OFFSET>,
            GetRequestCount: GetRequestCount::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IBidiRequestContainer as ::windows_core::ComInterface>::IID
    }
}
pub trait IBidiSpl_Impl: Sized {
    fn BindDevice(&self, pszdevicename: &::windows_core::PCWSTR, dwaccess: u32) -> ::windows_core::Result<()>;
    fn UnbindDevice(&self) -> ::windows_core::Result<()>;
    fn SendRecv(&self, pszaction: &::windows_core::PCWSTR, prequest: ::core::option::Option<&IBidiRequest>) -> ::windows_core::Result<()>;
    fn MultiSendRecv(&self, pszaction: &::windows_core::PCWSTR, prequestcontainer: ::core::option::Option<&IBidiRequestContainer>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IBidiSpl {}
impl IBidiSpl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiSpl_Impl, const OFFSET: isize>() -> IBidiSpl_Vtbl {
        unsafe extern "system" fn BindDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiSpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdevicename: ::windows_core::PCWSTR, dwaccess: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindDevice(::core::mem::transmute(&pszdevicename), ::core::mem::transmute_copy(&dwaccess)).into()
        }
        unsafe extern "system" fn UnbindDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiSpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnbindDevice().into()
        }
        unsafe extern "system" fn SendRecv<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiSpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszaction: ::windows_core::PCWSTR, prequest: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendRecv(::core::mem::transmute(&pszaction), ::windows_core::from_raw_borrowed(&prequest)).into()
        }
        unsafe extern "system" fn MultiSendRecv<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiSpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszaction: ::windows_core::PCWSTR, prequestcontainer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MultiSendRecv(::core::mem::transmute(&pszaction), ::windows_core::from_raw_borrowed(&prequestcontainer)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BindDevice: BindDevice::<Identity, Impl, OFFSET>,
            UnbindDevice: UnbindDevice::<Identity, Impl, OFFSET>,
            SendRecv: SendRecv::<Identity, Impl, OFFSET>,
            MultiSendRecv: MultiSendRecv::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IBidiSpl as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IBidiSpl2_Impl: Sized {
    fn BindDevice(&self, pszdevicename: &::windows_core::PCWSTR, dwaccess: u32) -> ::windows_core::Result<()>;
    fn UnbindDevice(&self) -> ::windows_core::Result<()>;
    fn SendRecvXMLString(&self, bstrrequest: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SendRecvXMLStream(&self, psrequest: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IBidiSpl2 {}
#[cfg(feature = "Win32_System_Com")]
impl IBidiSpl2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiSpl2_Impl, const OFFSET: isize>() -> IBidiSpl2_Vtbl {
        unsafe extern "system" fn BindDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiSpl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdevicename: ::windows_core::PCWSTR, dwaccess: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindDevice(::core::mem::transmute(&pszdevicename), ::core::mem::transmute_copy(&dwaccess)).into()
        }
        unsafe extern "system" fn UnbindDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiSpl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnbindDevice().into()
        }
        unsafe extern "system" fn SendRecvXMLString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiSpl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrequest: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrresponse: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SendRecvXMLString(::core::mem::transmute(&bstrrequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrresponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendRecvXMLStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBidiSpl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrequest: *mut ::core::ffi::c_void, ppsresponse: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SendRecvXMLStream(::windows_core::from_raw_borrowed(&psrequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsresponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BindDevice: BindDevice::<Identity, Impl, OFFSET>,
            UnbindDevice: UnbindDevice::<Identity, Impl, OFFSET>,
            SendRecvXMLString: SendRecvXMLString::<Identity, Impl, OFFSET>,
            SendRecvXMLStream: SendRecvXMLStream::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IBidiSpl2 as ::windows_core::ComInterface>::IID
    }
}
pub trait IFixedDocument_Impl: Sized {
    fn GetUri(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetPrintTicket(&self) -> ::windows_core::Result<IPartPrintTicket>;
    fn SetPrintTicket(&self, pprintticket: ::core::option::Option<&IPartPrintTicket>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFixedDocument {}
impl IFixedDocument_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFixedDocument_Impl, const OFFSET: isize>() -> IFixedDocument_Vtbl {
        unsafe extern "system" fn GetUri<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFixedDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUri() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintTicket<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFixedDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprintticket: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPrintTicket() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprintticket, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicket<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFixedDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintticket: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrintTicket(::windows_core::from_raw_borrowed(&pprintticket)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUri: GetUri::<Identity, Impl, OFFSET>,
            GetPrintTicket: GetPrintTicket::<Identity, Impl, OFFSET>,
            SetPrintTicket: SetPrintTicket::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IFixedDocument as ::windows_core::ComInterface>::IID
    }
}
pub trait IFixedDocumentSequence_Impl: Sized {
    fn GetUri(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetPrintTicket(&self) -> ::windows_core::Result<IPartPrintTicket>;
    fn SetPrintTicket(&self, pprintticket: ::core::option::Option<&IPartPrintTicket>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFixedDocumentSequence {}
impl IFixedDocumentSequence_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFixedDocumentSequence_Impl, const OFFSET: isize>() -> IFixedDocumentSequence_Vtbl {
        unsafe extern "system" fn GetUri<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFixedDocumentSequence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUri() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintTicket<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFixedDocumentSequence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprintticket: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPrintTicket() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprintticket, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicket<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFixedDocumentSequence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintticket: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrintTicket(::windows_core::from_raw_borrowed(&pprintticket)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUri: GetUri::<Identity, Impl, OFFSET>,
            GetPrintTicket: GetPrintTicket::<Identity, Impl, OFFSET>,
            SetPrintTicket: SetPrintTicket::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IFixedDocumentSequence as ::windows_core::ComInterface>::IID
    }
}
pub trait IFixedPage_Impl: Sized + IPartBase_Impl {
    fn GetPrintTicket(&self) -> ::windows_core::Result<IPartPrintTicket>;
    fn GetPagePart(&self, uri: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetWriteStream(&self) -> ::windows_core::Result<IPrintWriteStream>;
    fn SetPrintTicket(&self, ppprintticket: ::core::option::Option<&IPartPrintTicket>) -> ::windows_core::Result<()>;
    fn SetPagePart(&self, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn DeleteResource(&self, uri: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetXpsPartIterator(&self) -> ::windows_core::Result<IXpsPartIterator>;
}
impl ::windows_core::RuntimeName for IFixedPage {}
impl IFixedPage_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFixedPage_Impl, const OFFSET: isize>() -> IFixedPage_Vtbl {
        unsafe extern "system" fn GetPrintTicket<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFixedPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprintticket: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPrintTicket() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprintticket, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPagePart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFixedPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows_core::PCWSTR, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPagePart(::core::mem::transmute(&uri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWriteStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFixedPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwritestream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWriteStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwritestream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicket<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFixedPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprintticket: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrintTicket(::windows_core::from_raw_borrowed(&ppprintticket)).into()
        }
        unsafe extern "system" fn SetPagePart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFixedPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPagePart(::windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn DeleteResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFixedPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteResource(::core::mem::transmute(&uri)).into()
        }
        unsafe extern "system" fn GetXpsPartIterator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFixedPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxpspartit: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetXpsPartIterator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pxpspartit, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IPartBase_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPrintTicket: GetPrintTicket::<Identity, Impl, OFFSET>,
            GetPagePart: GetPagePart::<Identity, Impl, OFFSET>,
            GetWriteStream: GetWriteStream::<Identity, Impl, OFFSET>,
            SetPrintTicket: SetPrintTicket::<Identity, Impl, OFFSET>,
            SetPagePart: SetPagePart::<Identity, Impl, OFFSET>,
            DeleteResource: DeleteResource::<Identity, Impl, OFFSET>,
            GetXpsPartIterator: GetXpsPartIterator::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IFixedPage as ::windows_core::ComInterface>::IID || *iid == <IPartBase as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Ole\"`"]
#[cfg(feature = "Win32_System_Ole")]
pub trait IImgCreateErrorInfo_Impl: Sized + super::super::System::Ole::ICreateErrorInfo_Impl {
    fn AttachToErrorInfo(&self, perrorinfo: *mut ImgErrorInfo) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::windows_core::RuntimeName for IImgCreateErrorInfo {}
#[cfg(feature = "Win32_System_Ole")]
impl IImgCreateErrorInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImgCreateErrorInfo_Impl, const OFFSET: isize>() -> IImgCreateErrorInfo_Vtbl {
        unsafe extern "system" fn AttachToErrorInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImgCreateErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perrorinfo: *mut ImgErrorInfo) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AttachToErrorInfo(::core::mem::transmute_copy(&perrorinfo)).into()
        }
        Self {
            base__: super::super::System::Ole::ICreateErrorInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            AttachToErrorInfo: AttachToErrorInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IImgCreateErrorInfo as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Ole::ICreateErrorInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IImgErrorInfo_Impl: Sized + super::super::System::Com::IErrorInfo_Impl {
    fn GetDeveloperDescription(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetUserErrorId(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetUserParameterCount(&self) -> ::windows_core::Result<u32>;
    fn GetUserParameter(&self, cparam: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetUserFallback(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetExceptionId(&self) -> ::windows_core::Result<u32>;
    fn DetachErrorInfo(&self, perrorinfo: *mut ImgErrorInfo) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IImgErrorInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IImgErrorInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImgErrorInfo_Impl, const OFFSET: isize>() -> IImgErrorInfo_Vtbl {
        unsafe extern "system" fn GetDeveloperDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImgErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdevdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeveloperDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdevdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserErrorId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImgErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perrorid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUserErrorId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(perrorid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserParameterCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImgErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcuserparams: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUserParameterCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcuserparams, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserParameter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImgErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cparam: u32, pbstrparam: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUserParameter(::core::mem::transmute_copy(&cparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrparam, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserFallback<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImgErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfallback: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUserFallback() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfallback, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExceptionId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImgErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexceptionid: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetExceptionId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pexceptionid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachErrorInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImgErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perrorinfo: *mut ImgErrorInfo) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DetachErrorInfo(::core::mem::transmute_copy(&perrorinfo)).into()
        }
        Self {
            base__: super::super::System::Com::IErrorInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDeveloperDescription: GetDeveloperDescription::<Identity, Impl, OFFSET>,
            GetUserErrorId: GetUserErrorId::<Identity, Impl, OFFSET>,
            GetUserParameterCount: GetUserParameterCount::<Identity, Impl, OFFSET>,
            GetUserParameter: GetUserParameter::<Identity, Impl, OFFSET>,
            GetUserFallback: GetUserFallback::<Identity, Impl, OFFSET>,
            GetExceptionId: GetExceptionId::<Identity, Impl, OFFSET>,
            DetachErrorInfo: DetachErrorInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IImgErrorInfo as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IErrorInfo as ::windows_core::ComInterface>::IID
    }
}
pub trait IInterFilterCommunicator_Impl: Sized {
    fn RequestReader(&self, ppireader: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn RequestWriter(&self, ppiwriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IInterFilterCommunicator {}
impl IInterFilterCommunicator_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInterFilterCommunicator_Impl, const OFFSET: isize>() -> IInterFilterCommunicator_Vtbl {
        unsafe extern "system" fn RequestReader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInterFilterCommunicator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppireader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestReader(::core::mem::transmute_copy(&ppireader)).into()
        }
        unsafe extern "system" fn RequestWriter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInterFilterCommunicator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestWriter(::core::mem::transmute_copy(&ppiwriter)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RequestReader: RequestReader::<Identity, Impl, OFFSET>,
            RequestWriter: RequestWriter::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IInterFilterCommunicator as ::windows_core::ComInterface>::IID
    }
}
pub trait IPartBase_Impl: Sized {
    fn GetUri(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetStream(&self) -> ::windows_core::Result<IPrintReadStream>;
    fn GetPartCompression(&self) -> ::windows_core::Result<EXpsCompressionOptions>;
    fn SetPartCompression(&self, compression: EXpsCompressionOptions) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPartBase {}
impl IPartBase_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPartBase_Impl, const OFFSET: isize>() -> IPartBase_Vtbl {
        unsafe extern "system" fn GetUri<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPartBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUri() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPartBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartCompression<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPartBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcompression: *mut EXpsCompressionOptions) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPartCompression() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcompression, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPartCompression<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPartBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compression: EXpsCompressionOptions) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPartCompression(::core::mem::transmute_copy(&compression)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUri: GetUri::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            GetPartCompression: GetPartCompression::<Identity, Impl, OFFSET>,
            SetPartCompression: SetPartCompression::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPartBase as ::windows_core::ComInterface>::IID
    }
}
pub trait IPartColorProfile_Impl: Sized + IPartBase_Impl {}
impl ::windows_core::RuntimeName for IPartColorProfile {}
impl IPartColorProfile_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPartColorProfile_Impl, const OFFSET: isize>() -> IPartColorProfile_Vtbl {
        Self { base__: IPartBase_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPartColorProfile as ::windows_core::ComInterface>::IID || *iid == <IPartBase as ::windows_core::ComInterface>::IID
    }
}
pub trait IPartDiscardControl_Impl: Sized {
    fn GetDiscardProperties(&self, urisentinelpage: *mut ::windows_core::BSTR, uriparttodiscard: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPartDiscardControl {}
impl IPartDiscardControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPartDiscardControl_Impl, const OFFSET: isize>() -> IPartDiscardControl_Vtbl {
        unsafe extern "system" fn GetDiscardProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPartDiscardControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, urisentinelpage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, uriparttodiscard: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDiscardProperties(::core::mem::transmute_copy(&urisentinelpage), ::core::mem::transmute_copy(&uriparttodiscard)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDiscardProperties: GetDiscardProperties::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPartDiscardControl as ::windows_core::ComInterface>::IID
    }
}
pub trait IPartFont_Impl: Sized + IPartBase_Impl {
    fn GetFontProperties(&self, pcontenttype: *mut ::windows_core::BSTR, pfontoptions: *mut EXpsFontOptions) -> ::windows_core::Result<()>;
    fn SetFontContent(&self, pcontenttype: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetFontOptions(&self, options: EXpsFontOptions) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPartFont {}
impl IPartFont_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPartFont_Impl, const OFFSET: isize>() -> IPartFont_Vtbl {
        unsafe extern "system" fn GetFontProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPartFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontenttype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pfontoptions: *mut EXpsFontOptions) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontProperties(::core::mem::transmute_copy(&pcontenttype), ::core::mem::transmute_copy(&pfontoptions)).into()
        }
        unsafe extern "system" fn SetFontContent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPartFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontenttype: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFontContent(::core::mem::transmute(&pcontenttype)).into()
        }
        unsafe extern "system" fn SetFontOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPartFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: EXpsFontOptions) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFontOptions(::core::mem::transmute_copy(&options)).into()
        }
        Self {
            base__: IPartBase_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFontProperties: GetFontProperties::<Identity, Impl, OFFSET>,
            SetFontContent: SetFontContent::<Identity, Impl, OFFSET>,
            SetFontOptions: SetFontOptions::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPartFont as ::windows_core::ComInterface>::IID || *iid == <IPartBase as ::windows_core::ComInterface>::IID
    }
}
pub trait IPartFont2_Impl: Sized + IPartFont_Impl {
    fn GetFontRestriction(&self) -> ::windows_core::Result<EXpsFontRestriction>;
}
impl ::windows_core::RuntimeName for IPartFont2 {}
impl IPartFont2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPartFont2_Impl, const OFFSET: isize>() -> IPartFont2_Vtbl {
        unsafe extern "system" fn GetFontRestriction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPartFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prestriction: *mut EXpsFontRestriction) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontRestriction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prestriction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IPartFont_Vtbl::new::<Identity, Impl, OFFSET>(), GetFontRestriction: GetFontRestriction::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPartFont2 as ::windows_core::ComInterface>::IID || *iid == <IPartBase as ::windows_core::ComInterface>::IID || *iid == <IPartFont as ::windows_core::ComInterface>::IID
    }
}
pub trait IPartImage_Impl: Sized + IPartBase_Impl {
    fn GetImageProperties(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetImageContent(&self, pcontenttype: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPartImage {}
impl IPartImage_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPartImage_Impl, const OFFSET: isize>() -> IPartImage_Vtbl {
        unsafe extern "system" fn GetImageProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPartImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontenttype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetImageProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcontenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImageContent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPartImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontenttype: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetImageContent(::core::mem::transmute(&pcontenttype)).into()
        }
        Self {
            base__: IPartBase_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetImageProperties: GetImageProperties::<Identity, Impl, OFFSET>,
            SetImageContent: SetImageContent::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPartImage as ::windows_core::ComInterface>::IID || *iid == <IPartBase as ::windows_core::ComInterface>::IID
    }
}
pub trait IPartPrintTicket_Impl: Sized + IPartBase_Impl {}
impl ::windows_core::RuntimeName for IPartPrintTicket {}
impl IPartPrintTicket_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPartPrintTicket_Impl, const OFFSET: isize>() -> IPartPrintTicket_Vtbl {
        Self { base__: IPartBase_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPartPrintTicket as ::windows_core::ComInterface>::IID || *iid == <IPartBase as ::windows_core::ComInterface>::IID
    }
}
pub trait IPartResourceDictionary_Impl: Sized + IPartBase_Impl {}
impl ::windows_core::RuntimeName for IPartResourceDictionary {}
impl IPartResourceDictionary_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPartResourceDictionary_Impl, const OFFSET: isize>() -> IPartResourceDictionary_Vtbl {
        Self { base__: IPartBase_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPartResourceDictionary as ::windows_core::ComInterface>::IID || *iid == <IPartBase as ::windows_core::ComInterface>::IID
    }
}
pub trait IPartThumbnail_Impl: Sized + IPartBase_Impl {
    fn GetThumbnailProperties(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetThumbnailContent(&self, pcontenttype: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPartThumbnail {}
impl IPartThumbnail_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPartThumbnail_Impl, const OFFSET: isize>() -> IPartThumbnail_Vtbl {
        unsafe extern "system" fn GetThumbnailProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPartThumbnail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontenttype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetThumbnailProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcontenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnailContent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPartThumbnail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontenttype: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetThumbnailContent(::core::mem::transmute(&pcontenttype)).into()
        }
        Self {
            base__: IPartBase_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetThumbnailProperties: GetThumbnailProperties::<Identity, Impl, OFFSET>,
            SetThumbnailContent: SetThumbnailContent::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPartThumbnail as ::windows_core::ComInterface>::IID || *iid == <IPartBase as ::windows_core::ComInterface>::IID
    }
}
pub trait IPrintAsyncCookie_Impl: Sized {
    fn FinishAsyncCall(&self, param0: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn CancelAsyncCall(&self, param0: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPrintAsyncCookie {}
impl IPrintAsyncCookie_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncCookie_Impl, const OFFSET: isize>() -> IPrintAsyncCookie_Vtbl {
        unsafe extern "system" fn FinishAsyncCall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncCookie_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FinishAsyncCall(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn CancelAsyncCall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncCookie_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelAsyncCall(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FinishAsyncCall: FinishAsyncCall::<Identity, Impl, OFFSET>,
            CancelAsyncCall: CancelAsyncCall::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintAsyncCookie as ::windows_core::ComInterface>::IID
    }
}
pub trait IPrintAsyncNewChannelCookie_Impl: Sized + IPrintAsyncCookie_Impl {
    fn FinishAsyncCallWithData(&self, param0: *const ::core::option::Option<IPrintAsyncNotifyChannel>, param1: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPrintAsyncNewChannelCookie {}
impl IPrintAsyncNewChannelCookie_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncNewChannelCookie_Impl, const OFFSET: isize>() -> IPrintAsyncNewChannelCookie_Vtbl {
        unsafe extern "system" fn FinishAsyncCallWithData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncNewChannelCookie_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const *mut ::core::ffi::c_void, param1: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FinishAsyncCallWithData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self { base__: IPrintAsyncCookie_Vtbl::new::<Identity, Impl, OFFSET>(), FinishAsyncCallWithData: FinishAsyncCallWithData::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintAsyncNewChannelCookie as ::windows_core::ComInterface>::IID || *iid == <IPrintAsyncCookie as ::windows_core::ComInterface>::IID
    }
}
pub trait IPrintAsyncNotify_Impl: Sized {
    fn CreatePrintAsyncNotifyChannel(&self, param0: u32, param1: *const ::windows_core::GUID, param2: PrintAsyncNotifyUserFilter, param3: PrintAsyncNotifyConversationStyle, param4: ::core::option::Option<&IPrintAsyncNotifyCallback>) -> ::windows_core::Result<IPrintAsyncNotifyChannel>;
    fn CreatePrintAsyncNotifyRegistration(&self, param0: *const ::windows_core::GUID, param1: PrintAsyncNotifyUserFilter, param2: PrintAsyncNotifyConversationStyle, param3: ::core::option::Option<&IPrintAsyncNotifyCallback>) -> ::windows_core::Result<IPrintAsyncNotifyRegistration>;
}
impl ::windows_core::RuntimeName for IPrintAsyncNotify {}
impl IPrintAsyncNotify_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncNotify_Impl, const OFFSET: isize>() -> IPrintAsyncNotify_Vtbl {
        unsafe extern "system" fn CreatePrintAsyncNotifyChannel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *const ::windows_core::GUID, param2: PrintAsyncNotifyUserFilter, param3: PrintAsyncNotifyConversationStyle, param4: *mut ::core::ffi::c_void, param5: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePrintAsyncNotifyChannel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::windows_core::from_raw_borrowed(&param4)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param5, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePrintAsyncNotifyRegistration<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: PrintAsyncNotifyUserFilter, param2: PrintAsyncNotifyConversationStyle, param3: *mut ::core::ffi::c_void, param4: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePrintAsyncNotifyRegistration(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows_core::from_raw_borrowed(&param3)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param4, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePrintAsyncNotifyChannel: CreatePrintAsyncNotifyChannel::<Identity, Impl, OFFSET>,
            CreatePrintAsyncNotifyRegistration: CreatePrintAsyncNotifyRegistration::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintAsyncNotify as ::windows_core::ComInterface>::IID
    }
}
pub trait IPrintAsyncNotifyCallback_Impl: Sized {
    fn OnEventNotify(&self, pchannel: ::core::option::Option<&IPrintAsyncNotifyChannel>, pdata: ::core::option::Option<&IPrintAsyncNotifyDataObject>) -> ::windows_core::Result<()>;
    fn ChannelClosed(&self, pchannel: ::core::option::Option<&IPrintAsyncNotifyChannel>, pdata: ::core::option::Option<&IPrintAsyncNotifyDataObject>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPrintAsyncNotifyCallback {}
impl IPrintAsyncNotifyCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncNotifyCallback_Impl, const OFFSET: isize>() -> IPrintAsyncNotifyCallback_Vtbl {
        unsafe extern "system" fn OnEventNotify<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncNotifyCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannel: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEventNotify(::windows_core::from_raw_borrowed(&pchannel), ::windows_core::from_raw_borrowed(&pdata)).into()
        }
        unsafe extern "system" fn ChannelClosed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncNotifyCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannel: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ChannelClosed(::windows_core::from_raw_borrowed(&pchannel), ::windows_core::from_raw_borrowed(&pdata)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnEventNotify: OnEventNotify::<Identity, Impl, OFFSET>,
            ChannelClosed: ChannelClosed::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintAsyncNotifyCallback as ::windows_core::ComInterface>::IID
    }
}
pub trait IPrintAsyncNotifyChannel_Impl: Sized {
    fn SendNotification(&self, pdata: ::core::option::Option<&IPrintAsyncNotifyDataObject>) -> ::windows_core::Result<()>;
    fn CloseChannel(&self, pdata: ::core::option::Option<&IPrintAsyncNotifyDataObject>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPrintAsyncNotifyChannel {}
impl IPrintAsyncNotifyChannel_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncNotifyChannel_Impl, const OFFSET: isize>() -> IPrintAsyncNotifyChannel_Vtbl {
        unsafe extern "system" fn SendNotification<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncNotifyChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendNotification(::windows_core::from_raw_borrowed(&pdata)).into()
        }
        unsafe extern "system" fn CloseChannel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncNotifyChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseChannel(::windows_core::from_raw_borrowed(&pdata)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SendNotification: SendNotification::<Identity, Impl, OFFSET>,
            CloseChannel: CloseChannel::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintAsyncNotifyChannel as ::windows_core::ComInterface>::IID
    }
}
pub trait IPrintAsyncNotifyDataObject_Impl: Sized {
    fn AcquireData(&self, ppnotificationdata: *mut *mut u8, psize: *mut u32, ppschema: *mut *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn ReleaseData(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPrintAsyncNotifyDataObject {}
impl IPrintAsyncNotifyDataObject_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncNotifyDataObject_Impl, const OFFSET: isize>() -> IPrintAsyncNotifyDataObject_Vtbl {
        unsafe extern "system" fn AcquireData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncNotifyDataObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnotificationdata: *mut *mut u8, psize: *mut u32, ppschema: *mut *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AcquireData(::core::mem::transmute_copy(&ppnotificationdata), ::core::mem::transmute_copy(&psize), ::core::mem::transmute_copy(&ppschema)).into()
        }
        unsafe extern "system" fn ReleaseData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncNotifyDataObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseData().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AcquireData: AcquireData::<Identity, Impl, OFFSET>,
            ReleaseData: ReleaseData::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintAsyncNotifyDataObject as ::windows_core::ComInterface>::IID
    }
}
pub trait IPrintAsyncNotifyRegistration_Impl: Sized {
    fn RegisterForNotifications(&self) -> ::windows_core::Result<()>;
    fn UnregisterForNotifications(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPrintAsyncNotifyRegistration {}
impl IPrintAsyncNotifyRegistration_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncNotifyRegistration_Impl, const OFFSET: isize>() -> IPrintAsyncNotifyRegistration_Vtbl {
        unsafe extern "system" fn RegisterForNotifications<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncNotifyRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterForNotifications().into()
        }
        unsafe extern "system" fn UnregisterForNotifications<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncNotifyRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterForNotifications().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterForNotifications: RegisterForNotifications::<Identity, Impl, OFFSET>,
            UnregisterForNotifications: UnregisterForNotifications::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintAsyncNotifyRegistration as ::windows_core::ComInterface>::IID
    }
}
pub trait IPrintAsyncNotifyServerReferral_Impl: Sized {
    fn GetServerReferral(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn AsyncGetServerReferral(&self, param0: ::core::option::Option<&IAsyncGetSrvReferralCookie>) -> ::windows_core::Result<()>;
    fn SetServerReferral(&self, prmtserverreferral: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPrintAsyncNotifyServerReferral {}
impl IPrintAsyncNotifyServerReferral_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncNotifyServerReferral_Impl, const OFFSET: isize>() -> IPrintAsyncNotifyServerReferral_Vtbl {
        unsafe extern "system" fn GetServerReferral<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncNotifyServerReferral_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetServerReferral() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsyncGetServerReferral<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncNotifyServerReferral_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsyncGetServerReferral(::windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn SetServerReferral<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintAsyncNotifyServerReferral_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prmtserverreferral: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServerReferral(::core::mem::transmute(&prmtserverreferral)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetServerReferral: GetServerReferral::<Identity, Impl, OFFSET>,
            AsyncGetServerReferral: AsyncGetServerReferral::<Identity, Impl, OFFSET>,
            SetServerReferral: SetServerReferral::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintAsyncNotifyServerReferral as ::windows_core::ComInterface>::IID
    }
}
pub trait IPrintBidiAsyncNotifyRegistration_Impl: Sized + IPrintAsyncNotifyRegistration_Impl {
    fn AsyncGetNewChannel(&self, param0: ::core::option::Option<&IPrintAsyncNewChannelCookie>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPrintBidiAsyncNotifyRegistration {}
impl IPrintBidiAsyncNotifyRegistration_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintBidiAsyncNotifyRegistration_Impl, const OFFSET: isize>() -> IPrintBidiAsyncNotifyRegistration_Vtbl {
        unsafe extern "system" fn AsyncGetNewChannel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintBidiAsyncNotifyRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsyncGetNewChannel(::windows_core::from_raw_borrowed(&param0)).into()
        }
        Self { base__: IPrintAsyncNotifyRegistration_Vtbl::new::<Identity, Impl, OFFSET>(), AsyncGetNewChannel: AsyncGetNewChannel::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintBidiAsyncNotifyRegistration as ::windows_core::ComInterface>::IID || *iid == <IPrintAsyncNotifyRegistration as ::windows_core::ComInterface>::IID
    }
}
pub trait IPrintClassObjectFactory_Impl: Sized {
    fn GetPrintClassObject(&self, pszprintername: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPrintClassObjectFactory {}
impl IPrintClassObjectFactory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintClassObjectFactory_Impl, const OFFSET: isize>() -> IPrintClassObjectFactory_Vtbl {
        unsafe extern "system" fn GetPrintClassObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintClassObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszprintername: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPrintClassObject(::core::mem::transmute(&pszprintername), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppnewobject)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetPrintClassObject: GetPrintClassObject::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintClassObjectFactory as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintCoreHelper_Impl: Sized {
    fn GetOption(&self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturerequested: &::windows_core::PCSTR) -> ::windows_core::Result<::windows_core::PCSTR>;
    fn SetOptions(&self, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, bresolveconflicts: super::super::Foundation::BOOL, pfopairs: *const PRINT_FEATURE_OPTION, cpairs: u32, pcpairswritten: *mut u32, pdwresult: *mut u32) -> ::windows_core::Result<()>;
    fn EnumConstrainedOptions(&self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: &::windows_core::PCSTR, pconstrainedoptionlist: *const *const *const ::windows_core::PCSTR, pdwnumoptions: *mut u32) -> ::windows_core::Result<()>;
    fn WhyConstrained(&self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: &::windows_core::PCSTR, pszoptionkeyword: &::windows_core::PCSTR, ppfoconstraints: *mut *mut PRINT_FEATURE_OPTION, pdwnumoptions: *mut u32) -> ::windows_core::Result<()>;
    fn EnumFeatures(&self, pfeaturelist: *mut *mut *mut ::windows_core::PCSTR, pdwnumfeatures: *mut u32) -> ::windows_core::Result<()>;
    fn EnumOptions(&self, pszfeaturekeyword: &::windows_core::PCSTR, poptionlist: *mut *mut *mut ::windows_core::PCSTR, pdwnumoptions: *mut u32) -> ::windows_core::Result<()>;
    fn GetFontSubstitution(&self, psztruetypefontname: &::windows_core::PCWSTR, ppszdevfontname: *const ::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetFontSubstitution(&self, psztruetypefontname: &::windows_core::PCWSTR, pszdevfontname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn CreateInstanceOfMSXMLObject(&self, rclsid: *const ::windows_core::GUID, punkouter: ::core::option::Option<&::windows_core::IUnknown>, dwclscontext: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::RuntimeName for IPrintCoreHelper {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IPrintCoreHelper_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreHelper_Impl, const OFFSET: isize>() -> IPrintCoreHelper_Vtbl {
        unsafe extern "system" fn GetOption<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturerequested: ::windows_core::PCSTR, ppszoption: *mut ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOption(::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute(&pszfeaturerequested)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszoption, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, bresolveconflicts: super::super::Foundation::BOOL, pfopairs: *const PRINT_FEATURE_OPTION, cpairs: u32, pcpairswritten: *mut u32, pdwresult: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOptions(::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&bresolveconflicts), ::core::mem::transmute_copy(&pfopairs), ::core::mem::transmute_copy(&cpairs), ::core::mem::transmute_copy(&pcpairswritten), ::core::mem::transmute_copy(&pdwresult)).into()
        }
        unsafe extern "system" fn EnumConstrainedOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: ::windows_core::PCSTR, pconstrainedoptionlist: *const *const *const ::windows_core::PCSTR, pdwnumoptions: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumConstrainedOptions(::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute(&pszfeaturekeyword), ::core::mem::transmute_copy(&pconstrainedoptionlist), ::core::mem::transmute_copy(&pdwnumoptions)).into()
        }
        unsafe extern "system" fn WhyConstrained<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: ::windows_core::PCSTR, pszoptionkeyword: ::windows_core::PCSTR, ppfoconstraints: *mut *mut PRINT_FEATURE_OPTION, pdwnumoptions: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WhyConstrained(::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute(&pszfeaturekeyword), ::core::mem::transmute(&pszoptionkeyword), ::core::mem::transmute_copy(&ppfoconstraints), ::core::mem::transmute_copy(&pdwnumoptions)).into()
        }
        unsafe extern "system" fn EnumFeatures<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfeaturelist: *mut *mut *mut ::windows_core::PCSTR, pdwnumfeatures: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumFeatures(::core::mem::transmute_copy(&pfeaturelist), ::core::mem::transmute_copy(&pdwnumfeatures)).into()
        }
        unsafe extern "system" fn EnumOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfeaturekeyword: ::windows_core::PCSTR, poptionlist: *mut *mut *mut ::windows_core::PCSTR, pdwnumoptions: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumOptions(::core::mem::transmute(&pszfeaturekeyword), ::core::mem::transmute_copy(&poptionlist), ::core::mem::transmute_copy(&pdwnumoptions)).into()
        }
        unsafe extern "system" fn GetFontSubstitution<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztruetypefontname: ::windows_core::PCWSTR, ppszdevfontname: *const ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontSubstitution(::core::mem::transmute(&psztruetypefontname), ::core::mem::transmute_copy(&ppszdevfontname)).into()
        }
        unsafe extern "system" fn SetFontSubstitution<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztruetypefontname: ::windows_core::PCWSTR, pszdevfontname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFontSubstitution(::core::mem::transmute(&psztruetypefontname), ::core::mem::transmute(&pszdevfontname)).into()
        }
        unsafe extern "system" fn CreateInstanceOfMSXMLObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateInstanceOfMSXMLObject(::core::mem::transmute_copy(&rclsid), ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&dwclscontext), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOption: GetOption::<Identity, Impl, OFFSET>,
            SetOptions: SetOptions::<Identity, Impl, OFFSET>,
            EnumConstrainedOptions: EnumConstrainedOptions::<Identity, Impl, OFFSET>,
            WhyConstrained: WhyConstrained::<Identity, Impl, OFFSET>,
            EnumFeatures: EnumFeatures::<Identity, Impl, OFFSET>,
            EnumOptions: EnumOptions::<Identity, Impl, OFFSET>,
            GetFontSubstitution: GetFontSubstitution::<Identity, Impl, OFFSET>,
            SetFontSubstitution: SetFontSubstitution::<Identity, Impl, OFFSET>,
            CreateInstanceOfMSXMLObject: CreateInstanceOfMSXMLObject::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintCoreHelper as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintCoreHelperPS_Impl: Sized + IPrintCoreHelper_Impl {
    fn GetGlobalAttribute(&self, pszattribute: &::windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetFeatureAttribute(&self, pszfeaturekeyword: &::windows_core::PCSTR, pszattribute: &::windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetOptionAttribute(&self, pszfeaturekeyword: &::windows_core::PCSTR, pszoptionkeyword: &::windows_core::PCSTR, pszattribute: &::windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::RuntimeName for IPrintCoreHelperPS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IPrintCoreHelperPS_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreHelperPS_Impl, const OFFSET: isize>() -> IPrintCoreHelperPS_Vtbl {
        unsafe extern "system" fn GetGlobalAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreHelperPS_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattribute: ::windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGlobalAttribute(::core::mem::transmute(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&pcbsize)).into()
        }
        unsafe extern "system" fn GetFeatureAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreHelperPS_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfeaturekeyword: ::windows_core::PCSTR, pszattribute: ::windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFeatureAttribute(::core::mem::transmute(&pszfeaturekeyword), ::core::mem::transmute(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&pcbsize)).into()
        }
        unsafe extern "system" fn GetOptionAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreHelperPS_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfeaturekeyword: ::windows_core::PCSTR, pszoptionkeyword: ::windows_core::PCSTR, pszattribute: ::windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOptionAttribute(::core::mem::transmute(&pszfeaturekeyword), ::core::mem::transmute(&pszoptionkeyword), ::core::mem::transmute(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&pcbsize)).into()
        }
        Self {
            base__: IPrintCoreHelper_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetGlobalAttribute: GetGlobalAttribute::<Identity, Impl, OFFSET>,
            GetFeatureAttribute: GetFeatureAttribute::<Identity, Impl, OFFSET>,
            GetOptionAttribute: GetOptionAttribute::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintCoreHelperPS as ::windows_core::ComInterface>::IID || *iid == <IPrintCoreHelper as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintCoreHelperUni_Impl: Sized + IPrintCoreHelper_Impl {
    fn CreateGDLSnapshot(&self, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, dwflags: u32, ppsnapshotstream: *mut ::core::option::Option<super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn CreateDefaultGDLSnapshot(&self, dwflags: u32) -> ::windows_core::Result<super::super::System::Com::IStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IPrintCoreHelperUni {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPrintCoreHelperUni_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreHelperUni_Impl, const OFFSET: isize>() -> IPrintCoreHelperUni_Vtbl {
        unsafe extern "system" fn CreateGDLSnapshot<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreHelperUni_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, dwflags: u32, ppsnapshotstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateGDLSnapshot(::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppsnapshotstream)).into()
        }
        unsafe extern "system" fn CreateDefaultGDLSnapshot<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreHelperUni_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppsnapshotstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDefaultGDLSnapshot(::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsnapshotstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IPrintCoreHelper_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateGDLSnapshot: CreateGDLSnapshot::<Identity, Impl, OFFSET>,
            CreateDefaultGDLSnapshot: CreateDefaultGDLSnapshot::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintCoreHelperUni as ::windows_core::ComInterface>::IID || *iid == <IPrintCoreHelper as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintCoreHelperUni2_Impl: Sized + IPrintCoreHelperUni_Impl {
    fn GetNamedCommand(&self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszcommandname: &::windows_core::PCWSTR, ppcommandbytes: *mut *mut u8, pcbcommandsize: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IPrintCoreHelperUni2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPrintCoreHelperUni2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreHelperUni2_Impl, const OFFSET: isize>() -> IPrintCoreHelperUni2_Vtbl {
        unsafe extern "system" fn GetNamedCommand<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreHelperUni2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszcommandname: ::windows_core::PCWSTR, ppcommandbytes: *mut *mut u8, pcbcommandsize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNamedCommand(::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute(&pszcommandname), ::core::mem::transmute_copy(&ppcommandbytes), ::core::mem::transmute_copy(&pcbcommandsize)).into()
        }
        Self { base__: IPrintCoreHelperUni_Vtbl::new::<Identity, Impl, OFFSET>(), GetNamedCommand: GetNamedCommand::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintCoreHelperUni2 as ::windows_core::ComInterface>::IID || *iid == <IPrintCoreHelper as ::windows_core::ComInterface>::IID || *iid == <IPrintCoreHelperUni as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPrintCoreUI2_Impl: Sized + IPrintOemDriverUI_Impl {
    fn GetOptions(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturesrequested: *const i8, cbin: u32, pmszfeatureoptionbuf: ::windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::Result<()>;
    fn SetOptions(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeatureoptionbuf: *const i8, cbin: u32) -> ::windows_core::Result<u32>;
    fn EnumConstrainedOptions(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: &::windows_core::PCSTR, pmszconstrainedoptionlist: ::windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::Result<()>;
    fn WhyConstrained(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: &::windows_core::PCSTR, pszoptionkeyword: &::windows_core::PCSTR, pmszreasonlist: ::windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::Result<()>;
    fn GetGlobalAttribute(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszattribute: &::windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::Result<()>;
    fn GetFeatureAttribute(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: &::windows_core::PCSTR, pszattribute: &::windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::Result<()>;
    fn GetOptionAttribute(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: &::windows_core::PCSTR, pszoptionkeyword: &::windows_core::PCSTR, pszattribute: &::windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::Result<()>;
    fn EnumFeatures(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturelist: ::windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::Result<()>;
    fn EnumOptions(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: &::windows_core::PCSTR, pmszoptionlist: ::windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::Result<()>;
    fn QuerySimulationSupport(&self, hprinter: super::super::Foundation::HANDLE, dwlevel: u32, pcaps: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IPrintCoreUI2 {}
#[cfg(feature = "Win32_Foundation")]
impl IPrintCoreUI2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreUI2_Impl, const OFFSET: isize>() -> IPrintCoreUI2_Vtbl {
        unsafe extern "system" fn GetOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturesrequested: *const i8, cbin: u32, pmszfeatureoptionbuf: ::windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOptions(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pmszfeaturesrequested), ::core::mem::transmute_copy(&cbin), ::core::mem::transmute_copy(&pmszfeatureoptionbuf), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn SetOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeatureoptionbuf: *const i8, cbin: u32, pdwresult: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetOptions(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pmszfeatureoptionbuf), ::core::mem::transmute_copy(&cbin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumConstrainedOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: ::windows_core::PCSTR, pmszconstrainedoptionlist: ::windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumConstrainedOptions(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszfeaturekeyword), ::core::mem::transmute_copy(&pmszconstrainedoptionlist), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn WhyConstrained<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: ::windows_core::PCSTR, pszoptionkeyword: ::windows_core::PCSTR, pmszreasonlist: ::windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WhyConstrained(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszfeaturekeyword), ::core::mem::transmute(&pszoptionkeyword), ::core::mem::transmute_copy(&pmszreasonlist), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn GetGlobalAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszattribute: ::windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGlobalAttribute(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn GetFeatureAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: ::windows_core::PCSTR, pszattribute: ::windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFeatureAttribute(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszfeaturekeyword), ::core::mem::transmute(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn GetOptionAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: ::windows_core::PCSTR, pszoptionkeyword: ::windows_core::PCSTR, pszattribute: ::windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOptionAttribute(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszfeaturekeyword), ::core::mem::transmute(&pszoptionkeyword), ::core::mem::transmute(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn EnumFeatures<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturelist: ::windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumFeatures(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pmszfeaturelist), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn EnumOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: ::windows_core::PCSTR, pmszoptionlist: ::windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumOptions(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszfeaturekeyword), ::core::mem::transmute_copy(&pmszoptionlist), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn QuerySimulationSupport<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, dwlevel: u32, pcaps: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QuerySimulationSupport(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&dwlevel), ::core::mem::transmute_copy(&pcaps), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        Self {
            base__: IPrintOemDriverUI_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOptions: GetOptions::<Identity, Impl, OFFSET>,
            SetOptions: SetOptions::<Identity, Impl, OFFSET>,
            EnumConstrainedOptions: EnumConstrainedOptions::<Identity, Impl, OFFSET>,
            WhyConstrained: WhyConstrained::<Identity, Impl, OFFSET>,
            GetGlobalAttribute: GetGlobalAttribute::<Identity, Impl, OFFSET>,
            GetFeatureAttribute: GetFeatureAttribute::<Identity, Impl, OFFSET>,
            GetOptionAttribute: GetOptionAttribute::<Identity, Impl, OFFSET>,
            EnumFeatures: EnumFeatures::<Identity, Impl, OFFSET>,
            EnumOptions: EnumOptions::<Identity, Impl, OFFSET>,
            QuerySimulationSupport: QuerySimulationSupport::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintCoreUI2 as ::windows_core::ComInterface>::IID || *iid == <IPrintOemDriverUI as ::windows_core::ComInterface>::IID
    }
}
pub trait IPrintJob_Impl: Sized {
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Id(&self) -> ::windows_core::Result<u32>;
    fn PrintedPages(&self) -> ::windows_core::Result<u32>;
    fn TotalPages(&self) -> ::windows_core::Result<u32>;
    fn Status(&self) -> ::windows_core::Result<PrintJobStatus>;
    fn SubmissionTime(&self) -> ::windows_core::Result<f64>;
    fn RequestCancel(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPrintJob {}
impl IPrintJob_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintJob_Impl, const OFFSET: isize>() -> IPrintJob_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrintedPages<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulpages: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrintedPages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulpages, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalPages<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulpages: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TotalPages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulpages, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut PrintJobStatus) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmissionTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubmissiontime: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SubmissionTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psubmissiontime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCancel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestCancel().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            PrintedPages: PrintedPages::<Identity, Impl, OFFSET>,
            TotalPages: TotalPages::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            SubmissionTime: SubmissionTime::<Identity, Impl, OFFSET>,
            RequestCancel: RequestCancel::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintJob as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintJobCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, ulindex: u32) -> ::windows_core::Result<IPrintJob>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrintJobCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintJobCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintJobCollection_Impl, const OFFSET: isize>() -> IPrintJobCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintJobCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintJobCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppjob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppjob, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintJobCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintJobCollection as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintOemCommon_Impl: Sized {
    fn GetInfo(&self, dwmode: u32, pbuffer: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::Result<()>;
    fn DevMode(&self, dwmode: u32, poemdmparam: *mut OEMDMPARAM) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::RuntimeName for IPrintOemCommon {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IPrintOemCommon_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemCommon_Impl, const OFFSET: isize>() -> IPrintOemCommon_Vtbl {
        unsafe extern "system" fn GetInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemCommon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmode: u32, pbuffer: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInfo(::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn DevMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemCommon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmode: u32, poemdmparam: *mut OEMDMPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DevMode(::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&poemdmparam)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
            DevMode: DevMode::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintOemCommon as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPrintOemDriverUI_Impl: Sized {
    fn DrvGetDriverSetting(&self, pci: *mut ::core::ffi::c_void, feature: &::windows_core::PCSTR, poutput: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32, pdwoptionsreturned: *mut u32) -> ::windows_core::Result<()>;
    fn DrvUpgradeRegistrySetting(&self, hprinter: super::super::Foundation::HANDLE, pfeature: &::windows_core::PCSTR, poption: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn DrvUpdateUISetting(&self, pci: *mut ::core::ffi::c_void, poptitem: *mut ::core::ffi::c_void, dwpreviousselection: u32, dwmode: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IPrintOemDriverUI {}
#[cfg(feature = "Win32_Foundation")]
impl IPrintOemDriverUI_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemDriverUI_Impl, const OFFSET: isize>() -> IPrintOemDriverUI_Vtbl {
        unsafe extern "system" fn DrvGetDriverSetting<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemDriverUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pci: *mut ::core::ffi::c_void, feature: ::windows_core::PCSTR, poutput: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32, pdwoptionsreturned: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrvGetDriverSetting(::core::mem::transmute_copy(&pci), ::core::mem::transmute(&feature), ::core::mem::transmute_copy(&poutput), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded), ::core::mem::transmute_copy(&pdwoptionsreturned)).into()
        }
        unsafe extern "system" fn DrvUpgradeRegistrySetting<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemDriverUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, pfeature: ::windows_core::PCSTR, poption: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrvUpgradeRegistrySetting(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute(&pfeature), ::core::mem::transmute(&poption)).into()
        }
        unsafe extern "system" fn DrvUpdateUISetting<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemDriverUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pci: *mut ::core::ffi::c_void, poptitem: *mut ::core::ffi::c_void, dwpreviousselection: u32, dwmode: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrvUpdateUISetting(::core::mem::transmute_copy(&pci), ::core::mem::transmute_copy(&poptitem), ::core::mem::transmute_copy(&dwpreviousselection), ::core::mem::transmute_copy(&dwmode)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DrvGetDriverSetting: DrvGetDriverSetting::<Identity, Impl, OFFSET>,
            DrvUpgradeRegistrySetting: DrvUpgradeRegistrySetting::<Identity, Impl, OFFSET>,
            DrvUpdateUISetting: DrvUpdateUISetting::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintOemDriverUI as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPrintOemUI_Impl: Sized + IPrintOemCommon_Impl {
    fn PublishDriverInterface(&self, piunknown: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CommonUIProp(&self, dwmode: u32, poemcuipparam: *const OEMCUIPPARAM) -> ::windows_core::Result<()>;
    fn DocumentPropertySheets(&self, ppsuiinfo: *mut PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn DevicePropertySheets(&self, ppsuiinfo: *const PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn DevQueryPrintEx(&self, poemuiobj: *const OEMUIOBJ, pdqpinfo: *const DEVQUERYPRINT_INFO, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn DeviceCapabilitiesA(&self, poemuiobj: *mut OEMUIOBJ, hprinter: super::super::Foundation::HANDLE, pdevicename: &::windows_core::PCWSTR, wcapability: u16, poutput: *mut ::core::ffi::c_void, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, dwold: u32, dwresult: *mut u32) -> ::windows_core::Result<()>;
    fn UpgradePrinter(&self, dwlevel: u32, pdriverupgradeinfo: *const u8) -> ::windows_core::Result<()>;
    fn PrinterEvent(&self, pprintername: &::windows_core::PCWSTR, idriverevent: i32, dwflags: u32, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn DriverEvent(&self, dwdriverevent: u32, dwlevel: u32, pdriverinfo: *const u8, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn QueryColorProfile(&self, hprinter: super::super::Foundation::HANDLE, poemuiobj: *const OEMUIOBJ, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, ulquerymode: u32, pvprofiledata: *mut ::core::ffi::c_void, pcbprofiledata: *mut u32, pflprofiledata: *mut u32) -> ::windows_core::Result<()>;
    fn FontInstallerDlgProc(&self, hwnd: super::super::Foundation::HWND, usmsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn UpdateExternalFonts(&self, hprinter: super::super::Foundation::HANDLE, hheap: super::super::Foundation::HANDLE, pwstrcartridges: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::RuntimeName for IPrintOemUI {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPrintOemUI_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: isize>() -> IPrintOemUI_Vtbl {
        unsafe extern "system" fn PublishDriverInterface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piunknown: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PublishDriverInterface(::windows_core::from_raw_borrowed(&piunknown)).into()
        }
        unsafe extern "system" fn CommonUIProp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmode: u32, poemcuipparam: *const OEMCUIPPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CommonUIProp(::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&poemcuipparam)).into()
        }
        unsafe extern "system" fn DocumentPropertySheets<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsuiinfo: *mut PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DocumentPropertySheets(::core::mem::transmute_copy(&ppsuiinfo), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn DevicePropertySheets<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsuiinfo: *const PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DevicePropertySheets(::core::mem::transmute_copy(&ppsuiinfo), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn DevQueryPrintEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, pdqpinfo: *const DEVQUERYPRINT_INFO, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DevQueryPrintEx(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&pdqpinfo), ::core::mem::transmute_copy(&ppublicdm), ::core::mem::transmute_copy(&poemdm)).into()
        }
        unsafe extern "system" fn DeviceCapabilitiesA<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *mut OEMUIOBJ, hprinter: super::super::Foundation::HANDLE, pdevicename: ::windows_core::PCWSTR, wcapability: u16, poutput: *mut ::core::ffi::c_void, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, dwold: u32, dwresult: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceCapabilitiesA(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&hprinter), ::core::mem::transmute(&pdevicename), ::core::mem::transmute_copy(&wcapability), ::core::mem::transmute_copy(&poutput), ::core::mem::transmute_copy(&ppublicdm), ::core::mem::transmute_copy(&poemdm), ::core::mem::transmute_copy(&dwold), ::core::mem::transmute_copy(&dwresult)).into()
        }
        unsafe extern "system" fn UpgradePrinter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlevel: u32, pdriverupgradeinfo: *const u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpgradePrinter(::core::mem::transmute_copy(&dwlevel), ::core::mem::transmute_copy(&pdriverupgradeinfo)).into()
        }
        unsafe extern "system" fn PrinterEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintername: ::windows_core::PCWSTR, idriverevent: i32, dwflags: u32, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrinterEvent(::core::mem::transmute(&pprintername), ::core::mem::transmute_copy(&idriverevent), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn DriverEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdriverevent: u32, dwlevel: u32, pdriverinfo: *const u8, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DriverEvent(::core::mem::transmute_copy(&dwdriverevent), ::core::mem::transmute_copy(&dwlevel), ::core::mem::transmute_copy(&pdriverinfo), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn QueryColorProfile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, poemuiobj: *const OEMUIOBJ, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, ulquerymode: u32, pvprofiledata: *mut ::core::ffi::c_void, pcbprofiledata: *mut u32, pflprofiledata: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryColorProfile(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&ppublicdm), ::core::mem::transmute_copy(&poemdm), ::core::mem::transmute_copy(&ulquerymode), ::core::mem::transmute_copy(&pvprofiledata), ::core::mem::transmute_copy(&pcbprofiledata), ::core::mem::transmute_copy(&pflprofiledata)).into()
        }
        unsafe extern "system" fn FontInstallerDlgProc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, usmsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FontInstallerDlgProc(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&usmsg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn UpdateExternalFonts<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, hheap: super::super::Foundation::HANDLE, pwstrcartridges: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateExternalFonts(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&hheap), ::core::mem::transmute(&pwstrcartridges)).into()
        }
        Self {
            base__: IPrintOemCommon_Vtbl::new::<Identity, Impl, OFFSET>(),
            PublishDriverInterface: PublishDriverInterface::<Identity, Impl, OFFSET>,
            CommonUIProp: CommonUIProp::<Identity, Impl, OFFSET>,
            DocumentPropertySheets: DocumentPropertySheets::<Identity, Impl, OFFSET>,
            DevicePropertySheets: DevicePropertySheets::<Identity, Impl, OFFSET>,
            DevQueryPrintEx: DevQueryPrintEx::<Identity, Impl, OFFSET>,
            DeviceCapabilitiesA: DeviceCapabilitiesA::<Identity, Impl, OFFSET>,
            UpgradePrinter: UpgradePrinter::<Identity, Impl, OFFSET>,
            PrinterEvent: PrinterEvent::<Identity, Impl, OFFSET>,
            DriverEvent: DriverEvent::<Identity, Impl, OFFSET>,
            QueryColorProfile: QueryColorProfile::<Identity, Impl, OFFSET>,
            FontInstallerDlgProc: FontInstallerDlgProc::<Identity, Impl, OFFSET>,
            UpdateExternalFonts: UpdateExternalFonts::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintOemUI as ::windows_core::ComInterface>::IID || *iid == <IPrintOemCommon as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPrintOemUI2_Impl: Sized + IPrintOemUI_Impl {
    fn QueryJobAttributes(&self, hprinter: super::super::Foundation::HANDLE, pdevmode: *const super::Gdi::DEVMODEA, dwlevel: u32, lpattributeinfo: *const u8) -> ::windows_core::Result<()>;
    fn HideStandardUI(&self, dwmode: u32) -> ::windows_core::Result<()>;
    fn DocumentEvent(&self, hprinter: super::super::Foundation::HANDLE, hdc: super::Gdi::HDC, iesc: i32, cbin: u32, pvin: *mut ::core::ffi::c_void, cbout: u32, pvout: *mut ::core::ffi::c_void, piresult: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::RuntimeName for IPrintOemUI2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPrintOemUI2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemUI2_Impl, const OFFSET: isize>() -> IPrintOemUI2_Vtbl {
        unsafe extern "system" fn QueryJobAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, pdevmode: *const super::Gdi::DEVMODEA, dwlevel: u32, lpattributeinfo: *const u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryJobAttributes(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&dwlevel), ::core::mem::transmute_copy(&lpattributeinfo)).into()
        }
        unsafe extern "system" fn HideStandardUI<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmode: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HideStandardUI(::core::mem::transmute_copy(&dwmode)).into()
        }
        unsafe extern "system" fn DocumentEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, hdc: super::Gdi::HDC, iesc: i32, cbin: u32, pvin: *mut ::core::ffi::c_void, cbout: u32, pvout: *mut ::core::ffi::c_void, piresult: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DocumentEvent(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&iesc), ::core::mem::transmute_copy(&cbin), ::core::mem::transmute_copy(&pvin), ::core::mem::transmute_copy(&cbout), ::core::mem::transmute_copy(&pvout), ::core::mem::transmute_copy(&piresult)).into()
        }
        Self {
            base__: IPrintOemUI_Vtbl::new::<Identity, Impl, OFFSET>(),
            QueryJobAttributes: QueryJobAttributes::<Identity, Impl, OFFSET>,
            HideStandardUI: HideStandardUI::<Identity, Impl, OFFSET>,
            DocumentEvent: DocumentEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintOemUI2 as ::windows_core::ComInterface>::IID || *iid == <IPrintOemCommon as ::windows_core::ComInterface>::IID || *iid == <IPrintOemUI as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintOemUIMXDC_Impl: Sized {
    fn AdjustImageableArea(&self, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, prclimageablearea: *mut super::super::Foundation::RECTL) -> ::windows_core::Result<()>;
    fn AdjustImageCompression(&self, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pcompressionmode: *mut i32) -> ::windows_core::Result<()>;
    fn AdjustDPI(&self, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pdpi: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::RuntimeName for IPrintOemUIMXDC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IPrintOemUIMXDC_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemUIMXDC_Impl, const OFFSET: isize>() -> IPrintOemUIMXDC_Vtbl {
        unsafe extern "system" fn AdjustImageableArea<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemUIMXDC_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, prclimageablearea: *mut super::super::Foundation::RECTL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AdjustImageableArea(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&cbdevmode), ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cboemdm), ::core::mem::transmute_copy(&poemdm), ::core::mem::transmute_copy(&prclimageablearea)).into()
        }
        unsafe extern "system" fn AdjustImageCompression<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemUIMXDC_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pcompressionmode: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AdjustImageCompression(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&cbdevmode), ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cboemdm), ::core::mem::transmute_copy(&poemdm), ::core::mem::transmute_copy(&pcompressionmode)).into()
        }
        unsafe extern "system" fn AdjustDPI<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOemUIMXDC_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pdpi: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AdjustDPI(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&cbdevmode), ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cboemdm), ::core::mem::transmute_copy(&poemdm), ::core::mem::transmute_copy(&pdpi)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdjustImageableArea: AdjustImageableArea::<Identity, Impl, OFFSET>,
            AdjustImageCompression: AdjustImageCompression::<Identity, Impl, OFFSET>,
            AdjustDPI: AdjustDPI::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintOemUIMXDC as ::windows_core::ComInterface>::IID
    }
}
pub trait IPrintPipelineFilter_Impl: Sized {
    fn InitializeFilter(&self, pinegotiation: ::core::option::Option<&IInterFilterCommunicator>, pipropertybag: ::core::option::Option<&IPrintPipelinePropertyBag>, pipipelinecontrol: ::core::option::Option<&IPrintPipelineManagerControl>) -> ::windows_core::Result<()>;
    fn ShutdownOperation(&self) -> ::windows_core::Result<()>;
    fn StartOperation(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPrintPipelineFilter {}
impl IPrintPipelineFilter_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintPipelineFilter_Impl, const OFFSET: isize>() -> IPrintPipelineFilter_Vtbl {
        unsafe extern "system" fn InitializeFilter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintPipelineFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinegotiation: *mut ::core::ffi::c_void, pipropertybag: *mut ::core::ffi::c_void, pipipelinecontrol: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeFilter(::windows_core::from_raw_borrowed(&pinegotiation), ::windows_core::from_raw_borrowed(&pipropertybag), ::windows_core::from_raw_borrowed(&pipipelinecontrol)).into()
        }
        unsafe extern "system" fn ShutdownOperation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintPipelineFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShutdownOperation().into()
        }
        unsafe extern "system" fn StartOperation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintPipelineFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartOperation().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeFilter: InitializeFilter::<Identity, Impl, OFFSET>,
            ShutdownOperation: ShutdownOperation::<Identity, Impl, OFFSET>,
            StartOperation: StartOperation::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintPipelineFilter as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintPipelineManagerControl_Impl: Sized {
    fn RequestShutdown(&self, hrreason: ::windows_core::HRESULT, preason: ::core::option::Option<&IImgErrorInfo>) -> ::windows_core::Result<()>;
    fn FilterFinished(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IPrintPipelineManagerControl {}
#[cfg(feature = "Win32_System_Com")]
impl IPrintPipelineManagerControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintPipelineManagerControl_Impl, const OFFSET: isize>() -> IPrintPipelineManagerControl_Vtbl {
        unsafe extern "system" fn RequestShutdown<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintPipelineManagerControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrreason: ::windows_core::HRESULT, preason: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestShutdown(::core::mem::transmute_copy(&hrreason), ::windows_core::from_raw_borrowed(&preason)).into()
        }
        unsafe extern "system" fn FilterFinished<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintPipelineManagerControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FilterFinished().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RequestShutdown: RequestShutdown::<Identity, Impl, OFFSET>,
            FilterFinished: FilterFinished::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintPipelineManagerControl as ::windows_core::ComInterface>::IID
    }
}
pub trait IPrintPipelineProgressReport_Impl: Sized {
    fn ReportProgress(&self, update: EXpsJobConsumption) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPrintPipelineProgressReport {}
impl IPrintPipelineProgressReport_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintPipelineProgressReport_Impl, const OFFSET: isize>() -> IPrintPipelineProgressReport_Vtbl {
        unsafe extern "system" fn ReportProgress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintPipelineProgressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, update: EXpsJobConsumption) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportProgress(::core::mem::transmute_copy(&update)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ReportProgress: ReportProgress::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintPipelineProgressReport as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintPipelinePropertyBag_Impl: Sized {
    fn AddProperty(&self, pszname: &::windows_core::PCWSTR, pvar: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetProperty(&self, pszname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn DeleteProperty(&self, pszname: &::windows_core::PCWSTR) -> super::super::Foundation::BOOL;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrintPipelinePropertyBag {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintPipelinePropertyBag_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintPipelinePropertyBag_Impl, const OFFSET: isize>() -> IPrintPipelinePropertyBag_Vtbl {
        unsafe extern "system" fn AddProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintPipelinePropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, pvar: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddProperty(::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&pvar)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintPipelinePropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, pvar: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute(&pszname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintPipelinePropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteProperty(::core::mem::transmute(&pszname))
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddProperty: AddProperty::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            DeleteProperty: DeleteProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintPipelinePropertyBag as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait IPrintPreviewDxgiPackageTarget_Impl: Sized {
    fn SetJobPageCount(&self, counttype: PageCountType, count: u32) -> ::windows_core::Result<()>;
    fn DrawPage(&self, jobpagenumber: u32, pageimage: ::core::option::Option<&super::Dxgi::IDXGISurface>, dpix: f32, dpiy: f32) -> ::windows_core::Result<()>;
    fn InvalidatePreview(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ::windows_core::RuntimeName for IPrintPreviewDxgiPackageTarget {}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl IPrintPreviewDxgiPackageTarget_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintPreviewDxgiPackageTarget_Impl, const OFFSET: isize>() -> IPrintPreviewDxgiPackageTarget_Vtbl {
        unsafe extern "system" fn SetJobPageCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintPreviewDxgiPackageTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, counttype: PageCountType, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetJobPageCount(::core::mem::transmute_copy(&counttype), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn DrawPage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintPreviewDxgiPackageTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobpagenumber: u32, pageimage: *mut ::core::ffi::c_void, dpix: f32, dpiy: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawPage(::core::mem::transmute_copy(&jobpagenumber), ::windows_core::from_raw_borrowed(&pageimage), ::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy)).into()
        }
        unsafe extern "system" fn InvalidatePreview<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintPreviewDxgiPackageTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InvalidatePreview().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetJobPageCount: SetJobPageCount::<Identity, Impl, OFFSET>,
            DrawPage: DrawPage::<Identity, Impl, OFFSET>,
            InvalidatePreview: InvalidatePreview::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintPreviewDxgiPackageTarget as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPrintReadStream_Impl: Sized {
    fn Seek(&self, dlibmove: i64, dworigin: u32, plibnewposition: *mut u64) -> ::windows_core::Result<()>;
    fn ReadBytes(&self, pvbuffer: *mut ::core::ffi::c_void, cbrequested: u32, pcbread: *mut u32, pbendoffile: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IPrintReadStream {}
#[cfg(feature = "Win32_Foundation")]
impl IPrintReadStream_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintReadStream_Impl, const OFFSET: isize>() -> IPrintReadStream_Vtbl {
        unsafe extern "system" fn Seek<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintReadStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dlibmove: i64, dworigin: u32, plibnewposition: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Seek(::core::mem::transmute_copy(&dlibmove), ::core::mem::transmute_copy(&dworigin), ::core::mem::transmute_copy(&plibnewposition)).into()
        }
        unsafe extern "system" fn ReadBytes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintReadStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbuffer: *mut ::core::ffi::c_void, cbrequested: u32, pcbread: *mut u32, pbendoffile: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadBytes(::core::mem::transmute_copy(&pvbuffer), ::core::mem::transmute_copy(&cbrequested), ::core::mem::transmute_copy(&pcbread), ::core::mem::transmute_copy(&pbendoffile)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Seek: Seek::<Identity, Impl, OFFSET>,
            ReadBytes: ReadBytes::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintReadStream as ::windows_core::ComInterface>::IID
    }
}
pub trait IPrintReadStreamFactory_Impl: Sized {
    fn GetStream(&self) -> ::windows_core::Result<IPrintReadStream>;
}
impl ::windows_core::RuntimeName for IPrintReadStreamFactory {}
impl IPrintReadStreamFactory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintReadStreamFactory_Impl, const OFFSET: isize>() -> IPrintReadStreamFactory_Vtbl {
        unsafe extern "system" fn GetStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintReadStreamFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetStream: GetStream::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintReadStreamFactory as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaAsyncOperation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Start(&self) -> ::windows_core::Result<()>;
    fn Cancel(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrintSchemaAsyncOperation {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaAsyncOperation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaAsyncOperation_Impl, const OFFSET: isize>() -> IPrintSchemaAsyncOperation_Vtbl {
        unsafe extern "system" fn Start<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaAsyncOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start().into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaAsyncOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Start: Start::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintSchemaAsyncOperation as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaAsyncOperationEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Completed(&self, pticket: ::core::option::Option<&IPrintSchemaTicket>, hroperation: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrintSchemaAsyncOperationEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaAsyncOperationEvent_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaAsyncOperationEvent_Impl, const OFFSET: isize>() -> IPrintSchemaAsyncOperationEvent_Vtbl {
        unsafe extern "system" fn Completed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaAsyncOperationEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pticket: *mut ::core::ffi::c_void, hroperation: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Completed(::windows_core::from_raw_borrowed(&pticket), ::core::mem::transmute_copy(&hroperation)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Completed: Completed::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintSchemaAsyncOperationEvent as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaCapabilities_Impl: Sized + IPrintSchemaElement_Impl {
    fn GetFeatureByKeyName(&self, bstrkeyname: &::windows_core::BSTR) -> ::windows_core::Result<IPrintSchemaFeature>;
    fn GetFeature(&self, bstrname: &::windows_core::BSTR, bstrnamespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<IPrintSchemaFeature>;
    fn PageImageableSize(&self) -> ::windows_core::Result<IPrintSchemaPageImageableSize>;
    fn JobCopiesAllDocumentsMinValue(&self) -> ::windows_core::Result<u32>;
    fn JobCopiesAllDocumentsMaxValue(&self) -> ::windows_core::Result<u32>;
    fn GetSelectedOptionInPrintTicket(&self, pfeature: ::core::option::Option<&IPrintSchemaFeature>) -> ::windows_core::Result<IPrintSchemaOption>;
    fn GetOptions(&self, pfeature: ::core::option::Option<&IPrintSchemaFeature>) -> ::windows_core::Result<IPrintSchemaOptionCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrintSchemaCapabilities {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaCapabilities_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: isize>() -> IPrintSchemaCapabilities_Vtbl {
        unsafe extern "system" fn GetFeatureByKeyName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrkeyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppfeature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFeatureByKeyName(::core::mem::transmute(&bstrkeyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfeature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeature<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrnamespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppfeature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFeature(::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfeature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageImageableSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppageimageablesize: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PageImageableSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppageimageablesize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobCopiesAllDocumentsMinValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puljobcopiesalldocumentsminvalue: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.JobCopiesAllDocumentsMinValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puljobcopiesalldocumentsminvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobCopiesAllDocumentsMaxValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puljobcopiesalldocumentsmaxvalue: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.JobCopiesAllDocumentsMaxValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puljobcopiesalldocumentsmaxvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelectedOptionInPrintTicket<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfeature: *mut ::core::ffi::c_void, ppoption: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSelectedOptionInPrintTicket(::windows_core::from_raw_borrowed(&pfeature)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoption, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfeature: *mut ::core::ffi::c_void, ppoptioncollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOptions(::windows_core::from_raw_borrowed(&pfeature)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoptioncollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IPrintSchemaElement_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFeatureByKeyName: GetFeatureByKeyName::<Identity, Impl, OFFSET>,
            GetFeature: GetFeature::<Identity, Impl, OFFSET>,
            PageImageableSize: PageImageableSize::<Identity, Impl, OFFSET>,
            JobCopiesAllDocumentsMinValue: JobCopiesAllDocumentsMinValue::<Identity, Impl, OFFSET>,
            JobCopiesAllDocumentsMaxValue: JobCopiesAllDocumentsMaxValue::<Identity, Impl, OFFSET>,
            GetSelectedOptionInPrintTicket: GetSelectedOptionInPrintTicket::<Identity, Impl, OFFSET>,
            GetOptions: GetOptions::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintSchemaCapabilities as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || *iid == <IPrintSchemaElement as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaCapabilities2_Impl: Sized + IPrintSchemaCapabilities_Impl {
    fn GetParameterDefinition(&self, bstrname: &::windows_core::BSTR, bstrnamespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<IPrintSchemaParameterDefinition>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrintSchemaCapabilities2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaCapabilities2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaCapabilities2_Impl, const OFFSET: isize>() -> IPrintSchemaCapabilities2_Vtbl {
        unsafe extern "system" fn GetParameterDefinition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaCapabilities2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrnamespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppparameterdefinition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParameterDefinition(::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparameterdefinition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IPrintSchemaCapabilities_Vtbl::new::<Identity, Impl, OFFSET>(), GetParameterDefinition: GetParameterDefinition::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintSchemaCapabilities2 as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || *iid == <IPrintSchemaElement as ::windows_core::ComInterface>::IID || *iid == <IPrintSchemaCapabilities as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaDisplayableElement_Impl: Sized + IPrintSchemaElement_Impl {
    fn DisplayName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrintSchemaDisplayableElement {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaDisplayableElement_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaDisplayableElement_Impl, const OFFSET: isize>() -> IPrintSchemaDisplayableElement_Vtbl {
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaDisplayableElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdisplayname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdisplayname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IPrintSchemaElement_Vtbl::new::<Identity, Impl, OFFSET>(), DisplayName: DisplayName::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintSchemaDisplayableElement as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || *iid == <IPrintSchemaElement as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaElement_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn XmlNode(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrintSchemaElement {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaElement_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaElement_Impl, const OFFSET: isize>() -> IPrintSchemaElement_Vtbl {
        unsafe extern "system" fn XmlNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppxmlnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.XmlNode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppxmlnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NamespaceUri<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnamespaceuri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NamespaceUri() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrnamespaceuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            XmlNode: XmlNode::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            NamespaceUri: NamespaceUri::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintSchemaElement as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaFeature_Impl: Sized + IPrintSchemaDisplayableElement_Impl {
    fn SelectedOption(&self) -> ::windows_core::Result<IPrintSchemaOption>;
    fn SetSelectedOption(&self, poption: ::core::option::Option<&IPrintSchemaOption>) -> ::windows_core::Result<()>;
    fn SelectionType(&self) -> ::windows_core::Result<PrintSchemaSelectionType>;
    fn GetOption(&self, bstrname: &::windows_core::BSTR, bstrnamespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<IPrintSchemaOption>;
    fn DisplayUI(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrintSchemaFeature {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaFeature_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaFeature_Impl, const OFFSET: isize>() -> IPrintSchemaFeature_Vtbl {
        unsafe extern "system" fn SelectedOption<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaFeature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoption: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelectedOption() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoption, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedOption<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaFeature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poption: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelectedOption(::windows_core::from_raw_borrowed(&poption)).into()
        }
        unsafe extern "system" fn SelectionType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaFeature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pselectiontype: *mut PrintSchemaSelectionType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelectionType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pselectiontype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOption<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaFeature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrnamespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppoption: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOption(::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoption, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaFeature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayUI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbshow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IPrintSchemaDisplayableElement_Vtbl::new::<Identity, Impl, OFFSET>(),
            SelectedOption: SelectedOption::<Identity, Impl, OFFSET>,
            SetSelectedOption: SetSelectedOption::<Identity, Impl, OFFSET>,
            SelectionType: SelectionType::<Identity, Impl, OFFSET>,
            GetOption: GetOption::<Identity, Impl, OFFSET>,
            DisplayUI: DisplayUI::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintSchemaFeature as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || *iid == <IPrintSchemaElement as ::windows_core::ComInterface>::IID || *iid == <IPrintSchemaDisplayableElement as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaNUpOption_Impl: Sized + IPrintSchemaOption_Impl {
    fn PagesPerSheet(&self) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrintSchemaNUpOption {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaNUpOption_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaNUpOption_Impl, const OFFSET: isize>() -> IPrintSchemaNUpOption_Vtbl {
        unsafe extern "system" fn PagesPerSheet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaNUpOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulpagespersheet: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PagesPerSheet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulpagespersheet, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IPrintSchemaOption_Vtbl::new::<Identity, Impl, OFFSET>(), PagesPerSheet: PagesPerSheet::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintSchemaNUpOption as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || *iid == <IPrintSchemaElement as ::windows_core::ComInterface>::IID || *iid == <IPrintSchemaDisplayableElement as ::windows_core::ComInterface>::IID || *iid == <IPrintSchemaOption as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaOption_Impl: Sized + IPrintSchemaDisplayableElement_Impl {
    fn Selected(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Constrained(&self) -> ::windows_core::Result<PrintSchemaConstrainedSetting>;
    fn GetPropertyValue(&self, bstrname: &::windows_core::BSTR, bstrnamespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrintSchemaOption {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaOption_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaOption_Impl, const OFFSET: isize>() -> IPrintSchemaOption_Vtbl {
        unsafe extern "system" fn Selected<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisselected: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Selected() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisselected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Constrained<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psetting: *mut PrintSchemaConstrainedSetting) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Constrained() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psetting, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrnamespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppxmlvaluenode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertyValue(::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppxmlvaluenode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IPrintSchemaDisplayableElement_Vtbl::new::<Identity, Impl, OFFSET>(),
            Selected: Selected::<Identity, Impl, OFFSET>,
            Constrained: Constrained::<Identity, Impl, OFFSET>,
            GetPropertyValue: GetPropertyValue::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintSchemaOption as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || *iid == <IPrintSchemaElement as ::windows_core::ComInterface>::IID || *iid == <IPrintSchemaDisplayableElement as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaOptionCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, ulindex: u32) -> ::windows_core::Result<IPrintSchemaOption>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrintSchemaOptionCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaOptionCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaOptionCollection_Impl, const OFFSET: isize>() -> IPrintSchemaOptionCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaOptionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaOptionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppoption: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoption, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaOptionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintSchemaOptionCollection as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaPageImageableSize_Impl: Sized + IPrintSchemaElement_Impl {
    fn ImageableSizeWidthInMicrons(&self) -> ::windows_core::Result<u32>;
    fn ImageableSizeHeightInMicrons(&self) -> ::windows_core::Result<u32>;
    fn OriginWidthInMicrons(&self) -> ::windows_core::Result<u32>;
    fn OriginHeightInMicrons(&self) -> ::windows_core::Result<u32>;
    fn ExtentWidthInMicrons(&self) -> ::windows_core::Result<u32>;
    fn ExtentHeightInMicrons(&self) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrintSchemaPageImageableSize {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaPageImageableSize_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaPageImageableSize_Impl, const OFFSET: isize>() -> IPrintSchemaPageImageableSize_Vtbl {
        unsafe extern "system" fn ImageableSizeWidthInMicrons<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaPageImageableSize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulimageablesizewidth: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ImageableSizeWidthInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulimageablesizewidth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageableSizeHeightInMicrons<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaPageImageableSize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulimageablesizeheight: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ImageableSizeHeightInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulimageablesizeheight, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OriginWidthInMicrons<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaPageImageableSize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puloriginwidth: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OriginWidthInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puloriginwidth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OriginHeightInMicrons<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaPageImageableSize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puloriginheight: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OriginHeightInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puloriginheight, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtentWidthInMicrons<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaPageImageableSize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulextentwidth: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExtentWidthInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulextentwidth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtentHeightInMicrons<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaPageImageableSize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulextentheight: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExtentHeightInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulextentheight, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IPrintSchemaElement_Vtbl::new::<Identity, Impl, OFFSET>(),
            ImageableSizeWidthInMicrons: ImageableSizeWidthInMicrons::<Identity, Impl, OFFSET>,
            ImageableSizeHeightInMicrons: ImageableSizeHeightInMicrons::<Identity, Impl, OFFSET>,
            OriginWidthInMicrons: OriginWidthInMicrons::<Identity, Impl, OFFSET>,
            OriginHeightInMicrons: OriginHeightInMicrons::<Identity, Impl, OFFSET>,
            ExtentWidthInMicrons: ExtentWidthInMicrons::<Identity, Impl, OFFSET>,
            ExtentHeightInMicrons: ExtentHeightInMicrons::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintSchemaPageImageableSize as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || *iid == <IPrintSchemaElement as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaPageMediaSizeOption_Impl: Sized + IPrintSchemaOption_Impl {
    fn WidthInMicrons(&self) -> ::windows_core::Result<u32>;
    fn HeightInMicrons(&self) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrintSchemaPageMediaSizeOption {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaPageMediaSizeOption_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaPageMediaSizeOption_Impl, const OFFSET: isize>() -> IPrintSchemaPageMediaSizeOption_Vtbl {
        unsafe extern "system" fn WidthInMicrons<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaPageMediaSizeOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulwidth: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WidthInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulwidth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeightInMicrons<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaPageMediaSizeOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulheight: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HeightInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulheight, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IPrintSchemaOption_Vtbl::new::<Identity, Impl, OFFSET>(),
            WidthInMicrons: WidthInMicrons::<Identity, Impl, OFFSET>,
            HeightInMicrons: HeightInMicrons::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintSchemaPageMediaSizeOption as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || *iid == <IPrintSchemaElement as ::windows_core::ComInterface>::IID || *iid == <IPrintSchemaDisplayableElement as ::windows_core::ComInterface>::IID || *iid == <IPrintSchemaOption as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaParameterDefinition_Impl: Sized + IPrintSchemaDisplayableElement_Impl {
    fn UserInputRequired(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn UnitType(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DataType(&self) -> ::windows_core::Result<PrintSchemaParameterDataType>;
    fn RangeMin(&self) -> ::windows_core::Result<i32>;
    fn RangeMax(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrintSchemaParameterDefinition {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaParameterDefinition_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaParameterDefinition_Impl, const OFFSET: isize>() -> IPrintSchemaParameterDefinition_Vtbl {
        unsafe extern "system" fn UserInputRequired<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaParameterDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisrequired: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserInputRequired() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisrequired, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnitType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaParameterDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrunittype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UnitType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrunittype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaParameterDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatatype: *mut PrintSchemaParameterDataType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DataType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatatype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeMin<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaParameterDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangemin: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RangeMin() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prangemin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeMax<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaParameterDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangemax: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RangeMax() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prangemax, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IPrintSchemaDisplayableElement_Vtbl::new::<Identity, Impl, OFFSET>(),
            UserInputRequired: UserInputRequired::<Identity, Impl, OFFSET>,
            UnitType: UnitType::<Identity, Impl, OFFSET>,
            DataType: DataType::<Identity, Impl, OFFSET>,
            RangeMin: RangeMin::<Identity, Impl, OFFSET>,
            RangeMax: RangeMax::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintSchemaParameterDefinition as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || *iid == <IPrintSchemaElement as ::windows_core::ComInterface>::IID || *iid == <IPrintSchemaDisplayableElement as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaParameterInitializer_Impl: Sized + IPrintSchemaElement_Impl {
    fn Value(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetValue(&self, pvar: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrintSchemaParameterInitializer {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaParameterInitializer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaParameterInitializer_Impl, const OFFSET: isize>() -> IPrintSchemaParameterInitializer_Vtbl {
        unsafe extern "system" fn Value<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaParameterInitializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaParameterInitializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute_copy(&pvar)).into()
        }
        Self {
            base__: IPrintSchemaElement_Vtbl::new::<Identity, Impl, OFFSET>(),
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintSchemaParameterInitializer as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || *iid == <IPrintSchemaElement as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaTicket_Impl: Sized + IPrintSchemaElement_Impl {
    fn GetFeatureByKeyName(&self, bstrkeyname: &::windows_core::BSTR) -> ::windows_core::Result<IPrintSchemaFeature>;
    fn GetFeature(&self, bstrname: &::windows_core::BSTR, bstrnamespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<IPrintSchemaFeature>;
    fn ValidateAsync(&self) -> ::windows_core::Result<IPrintSchemaAsyncOperation>;
    fn CommitAsync(&self, pprintticketcommit: ::core::option::Option<&IPrintSchemaTicket>) -> ::windows_core::Result<IPrintSchemaAsyncOperation>;
    fn NotifyXmlChanged(&self) -> ::windows_core::Result<()>;
    fn GetCapabilities(&self) -> ::windows_core::Result<IPrintSchemaCapabilities>;
    fn JobCopiesAllDocuments(&self) -> ::windows_core::Result<u32>;
    fn SetJobCopiesAllDocuments(&self, uljobcopiesalldocuments: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrintSchemaTicket {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaTicket_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaTicket_Impl, const OFFSET: isize>() -> IPrintSchemaTicket_Vtbl {
        unsafe extern "system" fn GetFeatureByKeyName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrkeyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppfeature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFeatureByKeyName(::core::mem::transmute(&bstrkeyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfeature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeature<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrnamespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppfeature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFeature(::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfeature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ValidateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasyncoperation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintticketcommit: *mut ::core::ffi::c_void, ppasyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommitAsync(::windows_core::from_raw_borrowed(&pprintticketcommit)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasyncoperation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyXmlChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyXmlChanged().into()
        }
        unsafe extern "system" fn GetCapabilities<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcapabilities: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcapabilities, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobCopiesAllDocuments<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puljobcopiesalldocuments: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.JobCopiesAllDocuments() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puljobcopiesalldocuments, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJobCopiesAllDocuments<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uljobcopiesalldocuments: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetJobCopiesAllDocuments(::core::mem::transmute_copy(&uljobcopiesalldocuments)).into()
        }
        Self {
            base__: IPrintSchemaElement_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFeatureByKeyName: GetFeatureByKeyName::<Identity, Impl, OFFSET>,
            GetFeature: GetFeature::<Identity, Impl, OFFSET>,
            ValidateAsync: ValidateAsync::<Identity, Impl, OFFSET>,
            CommitAsync: CommitAsync::<Identity, Impl, OFFSET>,
            NotifyXmlChanged: NotifyXmlChanged::<Identity, Impl, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            JobCopiesAllDocuments: JobCopiesAllDocuments::<Identity, Impl, OFFSET>,
            SetJobCopiesAllDocuments: SetJobCopiesAllDocuments::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintSchemaTicket as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || *iid == <IPrintSchemaElement as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaTicket2_Impl: Sized + IPrintSchemaTicket_Impl {
    fn GetParameterInitializer(&self, bstrname: &::windows_core::BSTR, bstrnamespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<IPrintSchemaParameterInitializer>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrintSchemaTicket2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaTicket2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaTicket2_Impl, const OFFSET: isize>() -> IPrintSchemaTicket2_Vtbl {
        unsafe extern "system" fn GetParameterInitializer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintSchemaTicket2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrnamespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppparameterinitializer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParameterInitializer(::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparameterinitializer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IPrintSchemaTicket_Vtbl::new::<Identity, Impl, OFFSET>(), GetParameterInitializer: GetParameterInitializer::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintSchemaTicket2 as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || *iid == <IPrintSchemaElement as ::windows_core::ComInterface>::IID || *iid == <IPrintSchemaTicket as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintTicketProvider_Impl: Sized {
    fn GetSupportedVersions(&self, hprinter: super::super::Foundation::HANDLE, ppversions: *mut *mut i32, cversions: *mut i32) -> ::windows_core::Result<()>;
    fn BindPrinter(&self, hprinter: super::super::Foundation::HANDLE, version: i32, poptions: *mut SHIMOPTS, pdevmodeflags: *mut u32, cnamespaces: *mut i32, ppnamespaces: *mut *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn QueryDeviceNamespace(&self, pdefaultnamespace: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ConvertPrintTicketToDevMode(&self, pprintticket: ::core::option::Option<&super::super::Data::Xml::MsXml::IXMLDOMDocument2>, cbdevmodein: u32, pdevmodein: *mut super::Gdi::DEVMODEA, pcbdevmodeout: *mut u32, ppdevmodeout: *mut *mut super::Gdi::DEVMODEA) -> ::windows_core::Result<()>;
    fn ConvertDevModeToPrintTicket(&self, cbdevmode: u32, pdevmode: *mut super::Gdi::DEVMODEA, pprintticket: ::core::option::Option<&super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> ::windows_core::Result<()>;
    fn GetPrintCapabilities(&self, pprintticket: ::core::option::Option<&super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> ::windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>;
    fn ValidatePrintTicket(&self, pbaseticket: ::core::option::Option<&super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IPrintTicketProvider {}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPrintTicketProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTicketProvider_Impl, const OFFSET: isize>() -> IPrintTicketProvider_Vtbl {
        unsafe extern "system" fn GetSupportedVersions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTicketProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, ppversions: *mut *mut i32, cversions: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSupportedVersions(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&ppversions), ::core::mem::transmute_copy(&cversions)).into()
        }
        unsafe extern "system" fn BindPrinter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTicketProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, version: i32, poptions: *mut SHIMOPTS, pdevmodeflags: *mut u32, cnamespaces: *mut i32, ppnamespaces: *mut *mut ::windows_core::BSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindPrinter(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&version), ::core::mem::transmute_copy(&poptions), ::core::mem::transmute_copy(&pdevmodeflags), ::core::mem::transmute_copy(&cnamespaces), ::core::mem::transmute_copy(&ppnamespaces)).into()
        }
        unsafe extern "system" fn QueryDeviceNamespace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTicketProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdefaultnamespace: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryDeviceNamespace(::core::mem::transmute_copy(&pdefaultnamespace)).into()
        }
        unsafe extern "system" fn ConvertPrintTicketToDevMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTicketProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintticket: *mut ::core::ffi::c_void, cbdevmodein: u32, pdevmodein: *mut super::Gdi::DEVMODEA, pcbdevmodeout: *mut u32, ppdevmodeout: *mut *mut super::Gdi::DEVMODEA) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConvertPrintTicketToDevMode(::windows_core::from_raw_borrowed(&pprintticket), ::core::mem::transmute_copy(&cbdevmodein), ::core::mem::transmute_copy(&pdevmodein), ::core::mem::transmute_copy(&pcbdevmodeout), ::core::mem::transmute_copy(&ppdevmodeout)).into()
        }
        unsafe extern "system" fn ConvertDevModeToPrintTicket<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTicketProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbdevmode: u32, pdevmode: *mut super::Gdi::DEVMODEA, pprintticket: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConvertDevModeToPrintTicket(::core::mem::transmute_copy(&cbdevmode), ::core::mem::transmute_copy(&pdevmode), ::windows_core::from_raw_borrowed(&pprintticket)).into()
        }
        unsafe extern "system" fn GetPrintCapabilities<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTicketProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintticket: *mut ::core::ffi::c_void, ppcapabilities: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPrintCapabilities(::windows_core::from_raw_borrowed(&pprintticket)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcapabilities, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidatePrintTicket<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTicketProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbaseticket: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ValidatePrintTicket(::windows_core::from_raw_borrowed(&pbaseticket)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSupportedVersions: GetSupportedVersions::<Identity, Impl, OFFSET>,
            BindPrinter: BindPrinter::<Identity, Impl, OFFSET>,
            QueryDeviceNamespace: QueryDeviceNamespace::<Identity, Impl, OFFSET>,
            ConvertPrintTicketToDevMode: ConvertPrintTicketToDevMode::<Identity, Impl, OFFSET>,
            ConvertDevModeToPrintTicket: ConvertDevModeToPrintTicket::<Identity, Impl, OFFSET>,
            GetPrintCapabilities: GetPrintCapabilities::<Identity, Impl, OFFSET>,
            ValidatePrintTicket: ValidatePrintTicket::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintTicketProvider as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintTicketProvider2_Impl: Sized + IPrintTicketProvider_Impl {
    fn GetPrintDeviceCapabilities(&self, pprintticket: ::core::option::Option<&super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> ::windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>;
    fn GetPrintDeviceResources(&self, pszlocalename: &::windows_core::PCWSTR, pprintticket: ::core::option::Option<&super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> ::windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IPrintTicketProvider2 {}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPrintTicketProvider2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTicketProvider2_Impl, const OFFSET: isize>() -> IPrintTicketProvider2_Vtbl {
        unsafe extern "system" fn GetPrintDeviceCapabilities<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTicketProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintticket: *mut ::core::ffi::c_void, ppdevicecapabilities: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPrintDeviceCapabilities(::windows_core::from_raw_borrowed(&pprintticket)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevicecapabilities, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintDeviceResources<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTicketProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszlocalename: ::windows_core::PCWSTR, pprintticket: *mut ::core::ffi::c_void, ppdeviceresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPrintDeviceResources(::core::mem::transmute(&pszlocalename), ::windows_core::from_raw_borrowed(&pprintticket)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdeviceresources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IPrintTicketProvider_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPrintDeviceCapabilities: GetPrintDeviceCapabilities::<Identity, Impl, OFFSET>,
            GetPrintDeviceResources: GetPrintDeviceResources::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintTicketProvider2 as ::windows_core::ComInterface>::IID || *iid == <IPrintTicketProvider as ::windows_core::ComInterface>::IID
    }
}
pub trait IPrintUnidiAsyncNotifyRegistration_Impl: Sized + IPrintAsyncNotifyRegistration_Impl {
    fn AsyncGetNotification(&self, param0: ::core::option::Option<&IAsyncGetSendNotificationCookie>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPrintUnidiAsyncNotifyRegistration {}
impl IPrintUnidiAsyncNotifyRegistration_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintUnidiAsyncNotifyRegistration_Impl, const OFFSET: isize>() -> IPrintUnidiAsyncNotifyRegistration_Vtbl {
        unsafe extern "system" fn AsyncGetNotification<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintUnidiAsyncNotifyRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsyncGetNotification(::windows_core::from_raw_borrowed(&param0)).into()
        }
        Self {
            base__: IPrintAsyncNotifyRegistration_Vtbl::new::<Identity, Impl, OFFSET>(),
            AsyncGetNotification: AsyncGetNotification::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintUnidiAsyncNotifyRegistration as ::windows_core::ComInterface>::IID || *iid == <IPrintAsyncNotifyRegistration as ::windows_core::ComInterface>::IID
    }
}
pub trait IPrintWriteStream_Impl: Sized {
    fn WriteBytes(&self, pvbuffer: *const ::core::ffi::c_void, cbbuffer: u32) -> ::windows_core::Result<u32>;
    fn Close(&self);
}
impl ::windows_core::RuntimeName for IPrintWriteStream {}
impl IPrintWriteStream_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintWriteStream_Impl, const OFFSET: isize>() -> IPrintWriteStream_Vtbl {
        unsafe extern "system" fn WriteBytes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintWriteStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbuffer: *const ::core::ffi::c_void, cbbuffer: u32, pcbwritten: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WriteBytes(::core::mem::transmute_copy(&pvbuffer), ::core::mem::transmute_copy(&cbbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbwritten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintWriteStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WriteBytes: WriteBytes::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintWriteStream as ::windows_core::ComInterface>::IID
    }
}
pub trait IPrintWriteStreamFlush_Impl: Sized {
    fn FlushData(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPrintWriteStreamFlush {}
impl IPrintWriteStreamFlush_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintWriteStreamFlush_Impl, const OFFSET: isize>() -> IPrintWriteStreamFlush_Vtbl {
        unsafe extern "system" fn FlushData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintWriteStreamFlush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FlushData().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FlushData: FlushData::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrintWriteStreamFlush as ::windows_core::ComInterface>::IID
    }
}
pub trait IPrinterBidiSetRequestCallback_Impl: Sized {
    fn Completed(&self, bstrresponse: &::windows_core::BSTR, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPrinterBidiSetRequestCallback {}
impl IPrinterBidiSetRequestCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterBidiSetRequestCallback_Impl, const OFFSET: isize>() -> IPrinterBidiSetRequestCallback_Vtbl {
        unsafe extern "system" fn Completed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterBidiSetRequestCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresponse: ::std::mem::MaybeUninit<::windows_core::BSTR>, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Completed(::core::mem::transmute(&bstrresponse), ::core::mem::transmute_copy(&hrstatus)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Completed: Completed::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrinterBidiSetRequestCallback as ::windows_core::ComInterface>::IID
    }
}
pub trait IPrinterExtensionAsyncOperation_Impl: Sized {
    fn Cancel(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPrinterExtensionAsyncOperation {}
impl IPrinterExtensionAsyncOperation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionAsyncOperation_Impl, const OFFSET: isize>() -> IPrinterExtensionAsyncOperation_Vtbl {
        unsafe extern "system" fn Cancel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionAsyncOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Cancel: Cancel::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrinterExtensionAsyncOperation as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterExtensionContext_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn PrinterQueue(&self) -> ::windows_core::Result<IPrinterQueue>;
    fn PrintSchemaTicket(&self) -> ::windows_core::Result<IPrintSchemaTicket>;
    fn DriverProperties(&self) -> ::windows_core::Result<IPrinterPropertyBag>;
    fn UserProperties(&self) -> ::windows_core::Result<IPrinterPropertyBag>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrinterExtensionContext {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterExtensionContext_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionContext_Impl, const OFFSET: isize>() -> IPrinterExtensionContext_Vtbl {
        unsafe extern "system" fn PrinterQueue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqueue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrinterQueue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqueue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrintSchemaTicket<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppticket: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrintSchemaTicket() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppticket, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DriverProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertybag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertybag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            PrinterQueue: PrinterQueue::<Identity, Impl, OFFSET>,
            PrintSchemaTicket: PrintSchemaTicket::<Identity, Impl, OFFSET>,
            DriverProperties: DriverProperties::<Identity, Impl, OFFSET>,
            UserProperties: UserProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrinterExtensionContext as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterExtensionContextCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, ulindex: u32) -> ::windows_core::Result<IPrinterExtensionContext>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrinterExtensionContextCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterExtensionContextCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionContextCollection_Impl, const OFFSET: isize>() -> IPrinterExtensionContextCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionContextCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionContextCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionContextCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrinterExtensionContextCollection as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterExtensionEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OnDriverEvent(&self, peventargs: ::core::option::Option<&IPrinterExtensionEventArgs>) -> ::windows_core::Result<()>;
    fn OnPrinterQueuesEnumerated(&self, pcontextcollection: ::core::option::Option<&IPrinterExtensionContextCollection>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrinterExtensionEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterExtensionEvent_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionEvent_Impl, const OFFSET: isize>() -> IPrinterExtensionEvent_Vtbl {
        unsafe extern "system" fn OnDriverEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventargs: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDriverEvent(::windows_core::from_raw_borrowed(&peventargs)).into()
        }
        unsafe extern "system" fn OnPrinterQueuesEnumerated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontextcollection: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPrinterQueuesEnumerated(::windows_core::from_raw_borrowed(&pcontextcollection)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnDriverEvent: OnDriverEvent::<Identity, Impl, OFFSET>,
            OnPrinterQueuesEnumerated: OnPrinterQueuesEnumerated::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrinterExtensionEvent as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterExtensionEventArgs_Impl: Sized + IPrinterExtensionContext_Impl {
    fn BidiNotification(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ReasonId(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn Request(&self) -> ::windows_core::Result<IPrinterExtensionRequest>;
    fn SourceApplication(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DetailedReasonId(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn WindowModal(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn WindowParent(&self) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrinterExtensionEventArgs {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterExtensionEventArgs_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>() -> IPrinterExtensionEventArgs_Vtbl {
        unsafe extern "system" fn BidiNotification<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbidinotification: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BidiNotification() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrbidinotification, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReasonId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preasonid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReasonId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preasonid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Request<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprequest: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Request() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprequest, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplication: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SourceApplication() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrapplication, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetailedReasonId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdetailedreasonid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DetailedReasonId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdetailedreasonid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowModal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmodal: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WindowModal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbmodal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowParent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwndparent: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WindowParent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwndparent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IPrinterExtensionContext_Vtbl::new::<Identity, Impl, OFFSET>(),
            BidiNotification: BidiNotification::<Identity, Impl, OFFSET>,
            ReasonId: ReasonId::<Identity, Impl, OFFSET>,
            Request: Request::<Identity, Impl, OFFSET>,
            SourceApplication: SourceApplication::<Identity, Impl, OFFSET>,
            DetailedReasonId: DetailedReasonId::<Identity, Impl, OFFSET>,
            WindowModal: WindowModal::<Identity, Impl, OFFSET>,
            WindowParent: WindowParent::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrinterExtensionEventArgs as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || *iid == <IPrinterExtensionContext as ::windows_core::ComInterface>::IID
    }
}
pub trait IPrinterExtensionManager_Impl: Sized {
    fn EnableEvents(&self, printerdriverid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn DisableEvents(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPrinterExtensionManager {}
impl IPrinterExtensionManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionManager_Impl, const OFFSET: isize>() -> IPrinterExtensionManager_Vtbl {
        unsafe extern "system" fn EnableEvents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printerdriverid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableEvents(::core::mem::transmute(&printerdriverid)).into()
        }
        unsafe extern "system" fn DisableEvents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisableEvents().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnableEvents: EnableEvents::<Identity, Impl, OFFSET>,
            DisableEvents: DisableEvents::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrinterExtensionManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterExtensionRequest_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Cancel(&self, hrstatus: ::windows_core::HRESULT, bstrlogmessage: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Complete(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrinterExtensionRequest {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterExtensionRequest_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionRequest_Impl, const OFFSET: isize>() -> IPrinterExtensionRequest_Vtbl {
        unsafe extern "system" fn Cancel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows_core::HRESULT, bstrlogmessage: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel(::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute(&bstrlogmessage)).into()
        }
        unsafe extern "system" fn Complete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterExtensionRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Complete().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Complete: Complete::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrinterExtensionRequest as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterPropertyBag_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetBool(&self, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetBool(&self, bstrname: &::windows_core::BSTR, bvalue: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetInt32(&self, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<i32>;
    fn SetInt32(&self, bstrname: &::windows_core::BSTR, nvalue: i32) -> ::windows_core::Result<()>;
    fn GetString(&self, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetString(&self, bstrname: &::windows_core::BSTR, bstrvalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetBytes(&self, bstrname: &::windows_core::BSTR, pcbvalue: *mut u32, ppvalue: *mut *mut u8) -> ::windows_core::Result<()>;
    fn SetBytes(&self, bstrname: &::windows_core::BSTR, cbvalue: u32, pvalue: *const u8) -> ::windows_core::Result<()>;
    fn GetReadStream(&self, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn GetWriteStream(&self, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrinterPropertyBag {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterPropertyBag_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterPropertyBag_Impl, const OFFSET: isize>() -> IPrinterPropertyBag_Vtbl {
        unsafe extern "system" fn GetBool<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbvalue: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBool(::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBool<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bvalue: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBool(::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&bvalue)).into()
        }
        unsafe extern "system" fn GetInt32<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pnvalue: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInt32(::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInt32<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, nvalue: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInt32(::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&nvalue)).into()
        }
        unsafe extern "system" fn GetString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetString(::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetString(::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrvalue)).into()
        }
        unsafe extern "system" fn GetBytes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcbvalue: *mut u32, ppvalue: *mut *mut u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBytes(::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&pcbvalue), ::core::mem::transmute_copy(&ppvalue)).into()
        }
        unsafe extern "system" fn SetBytes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, cbvalue: u32, pvalue: *const u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBytes(::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&cbvalue), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetReadStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReadStream(::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWriteStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWriteStream(::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetBool: GetBool::<Identity, Impl, OFFSET>,
            SetBool: SetBool::<Identity, Impl, OFFSET>,
            GetInt32: GetInt32::<Identity, Impl, OFFSET>,
            SetInt32: SetInt32::<Identity, Impl, OFFSET>,
            GetString: GetString::<Identity, Impl, OFFSET>,
            SetString: SetString::<Identity, Impl, OFFSET>,
            GetBytes: GetBytes::<Identity, Impl, OFFSET>,
            SetBytes: SetBytes::<Identity, Impl, OFFSET>,
            GetReadStream: GetReadStream::<Identity, Impl, OFFSET>,
            GetWriteStream: GetWriteStream::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrinterPropertyBag as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterQueue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Handle(&self) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SendBidiQuery(&self, bstrbidiquery: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetProperties(&self) -> ::windows_core::Result<IPrinterPropertyBag>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrinterQueue {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterQueue_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterQueue_Impl, const OFFSET: isize>() -> IPrinterQueue_Vtbl {
        unsafe extern "system" fn Handle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phprinter: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Handle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phprinter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendBidiQuery<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbidiquery: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendBidiQuery(::core::mem::transmute(&bstrbidiquery)).into()
        }
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertybag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Handle: Handle::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SendBidiQuery: SendBidiQuery::<Identity, Impl, OFFSET>,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrinterQueue as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterQueue2_Impl: Sized + IPrinterQueue_Impl {
    fn SendBidiSetRequestAsync(&self, bstrbidirequest: &::windows_core::BSTR, pcallback: ::core::option::Option<&IPrinterBidiSetRequestCallback>) -> ::windows_core::Result<IPrinterExtensionAsyncOperation>;
    fn GetPrinterQueueView(&self, ulviewoffset: u32, ulviewsize: u32) -> ::windows_core::Result<IPrinterQueueView>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrinterQueue2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterQueue2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterQueue2_Impl, const OFFSET: isize>() -> IPrinterQueue2_Vtbl {
        unsafe extern "system" fn SendBidiSetRequestAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbidirequest: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcallback: *mut ::core::ffi::c_void, ppasyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SendBidiSetRequestAsync(::core::mem::transmute(&bstrbidirequest), ::windows_core::from_raw_borrowed(&pcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasyncoperation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrinterQueueView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulviewoffset: u32, ulviewsize: u32, ppjobview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPrinterQueueView(::core::mem::transmute_copy(&ulviewoffset), ::core::mem::transmute_copy(&ulviewsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppjobview, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IPrinterQueue_Vtbl::new::<Identity, Impl, OFFSET>(),
            SendBidiSetRequestAsync: SendBidiSetRequestAsync::<Identity, Impl, OFFSET>,
            GetPrinterQueueView: GetPrinterQueueView::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrinterQueue2 as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || *iid == <IPrinterQueue as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterQueueEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OnBidiResponseReceived(&self, bstrresponse: &::windows_core::BSTR, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrinterQueueEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterQueueEvent_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterQueueEvent_Impl, const OFFSET: isize>() -> IPrinterQueueEvent_Vtbl {
        unsafe extern "system" fn OnBidiResponseReceived<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterQueueEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresponse: ::std::mem::MaybeUninit<::windows_core::BSTR>, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnBidiResponseReceived(::core::mem::transmute(&bstrresponse), ::core::mem::transmute_copy(&hrstatus)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnBidiResponseReceived: OnBidiResponseReceived::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrinterQueueEvent as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterQueueView_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetViewRange(&self, ulviewoffset: u32, ulviewsize: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrinterQueueView {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterQueueView_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterQueueView_Impl, const OFFSET: isize>() -> IPrinterQueueView_Vtbl {
        unsafe extern "system" fn SetViewRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterQueueView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulviewoffset: u32, ulviewsize: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetViewRange(::core::mem::transmute_copy(&ulviewoffset), ::core::mem::transmute_copy(&ulviewsize)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), SetViewRange: SetViewRange::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrinterQueueView as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterQueueViewEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OnChanged(&self, pcollection: ::core::option::Option<&IPrintJobCollection>, ulviewoffset: u32, ulviewsize: u32, ulcountjobsinprintqueue: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrinterQueueViewEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterQueueViewEvent_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterQueueViewEvent_Impl, const OFFSET: isize>() -> IPrinterQueueViewEvent_Vtbl {
        unsafe extern "system" fn OnChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterQueueViewEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcollection: *mut ::core::ffi::c_void, ulviewoffset: u32, ulviewsize: u32, ulcountjobsinprintqueue: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnChanged(::windows_core::from_raw_borrowed(&pcollection), ::core::mem::transmute_copy(&ulviewoffset), ::core::mem::transmute_copy(&ulviewsize), ::core::mem::transmute_copy(&ulcountjobsinprintqueue)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), OnChanged: OnChanged::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrinterQueueViewEvent as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterScriptContext_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DriverProperties(&self) -> ::windows_core::Result<IPrinterScriptablePropertyBag>;
    fn QueueProperties(&self) -> ::windows_core::Result<IPrinterScriptablePropertyBag>;
    fn UserProperties(&self) -> ::windows_core::Result<IPrinterScriptablePropertyBag>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrinterScriptContext {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterScriptContext_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptContext_Impl, const OFFSET: isize>() -> IPrinterScriptContext_Vtbl {
        unsafe extern "system" fn DriverProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DriverProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertybag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueueProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertybag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertybag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DriverProperties: DriverProperties::<Identity, Impl, OFFSET>,
            QueueProperties: QueueProperties::<Identity, Impl, OFFSET>,
            UserProperties: UserProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrinterScriptContext as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterScriptablePropertyBag_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetBool(&self, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetBool(&self, bstrname: &::windows_core::BSTR, bvalue: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetInt32(&self, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<i32>;
    fn SetInt32(&self, bstrname: &::windows_core::BSTR, nvalue: i32) -> ::windows_core::Result<()>;
    fn GetString(&self, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetString(&self, bstrname: &::windows_core::BSTR, bstrvalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetBytes(&self, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn SetBytes(&self, bstrname: &::windows_core::BSTR, parray: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn GetReadStream(&self, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<IPrinterScriptableStream>;
    fn GetWriteStream(&self, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<IPrinterScriptableStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrinterScriptablePropertyBag {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterScriptablePropertyBag_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>() -> IPrinterScriptablePropertyBag_Vtbl {
        unsafe extern "system" fn GetBool<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbvalue: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBool(::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBool<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bvalue: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBool(::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&bvalue)).into()
        }
        unsafe extern "system" fn GetInt32<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pnvalue: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInt32(::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInt32<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, nvalue: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInt32(::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&nvalue)).into()
        }
        unsafe extern "system" fn GetString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetString(::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetString(::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrvalue)).into()
        }
        unsafe extern "system" fn GetBytes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pparray: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBytes(::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pparray, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBytes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, parray: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBytes(::core::mem::transmute(&bstrname), ::windows_core::from_raw_borrowed(&parray)).into()
        }
        unsafe extern "system" fn GetReadStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReadStream(::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWriteStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWriteStream(::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetBool: GetBool::<Identity, Impl, OFFSET>,
            SetBool: SetBool::<Identity, Impl, OFFSET>,
            GetInt32: GetInt32::<Identity, Impl, OFFSET>,
            SetInt32: SetInt32::<Identity, Impl, OFFSET>,
            GetString: GetString::<Identity, Impl, OFFSET>,
            SetString: SetString::<Identity, Impl, OFFSET>,
            GetBytes: GetBytes::<Identity, Impl, OFFSET>,
            SetBytes: SetBytes::<Identity, Impl, OFFSET>,
            GetReadStream: GetReadStream::<Identity, Impl, OFFSET>,
            GetWriteStream: GetWriteStream::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrinterScriptablePropertyBag as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterScriptablePropertyBag2_Impl: Sized + IPrinterScriptablePropertyBag_Impl {
    fn GetReadStreamAsXML(&self, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrinterScriptablePropertyBag2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterScriptablePropertyBag2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptablePropertyBag2_Impl, const OFFSET: isize>() -> IPrinterScriptablePropertyBag2_Vtbl {
        unsafe extern "system" fn GetReadStreamAsXML<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptablePropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppxmlnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReadStreamAsXML(::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppxmlnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IPrinterScriptablePropertyBag_Vtbl::new::<Identity, Impl, OFFSET>(), GetReadStreamAsXML: GetReadStreamAsXML::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrinterScriptablePropertyBag2 as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || *iid == <IPrinterScriptablePropertyBag as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterScriptableSequentialStream_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Read(&self, cbread: i32) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn Write(&self, parray: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrinterScriptableSequentialStream {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterScriptableSequentialStream_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptableSequentialStream_Impl, const OFFSET: isize>() -> IPrinterScriptableSequentialStream_Vtbl {
        unsafe extern "system" fn Read<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptableSequentialStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbread: i32, pparray: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Read(::core::mem::transmute_copy(&cbread)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pparray, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptableSequentialStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parray: *mut ::core::ffi::c_void, pcbwritten: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Write(::windows_core::from_raw_borrowed(&parray)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbwritten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Read: Read::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrinterScriptableSequentialStream as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterScriptableStream_Impl: Sized + IPrinterScriptableSequentialStream_Impl {
    fn Commit(&self) -> ::windows_core::Result<()>;
    fn Seek(&self, loffset: i32, streamseek: super::super::System::Com::STREAM_SEEK) -> ::windows_core::Result<i32>;
    fn SetSize(&self, lsize: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPrinterScriptableStream {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterScriptableStream_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptableStream_Impl, const OFFSET: isize>() -> IPrinterScriptableStream_Vtbl {
        unsafe extern "system" fn Commit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptableStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit().into()
        }
        unsafe extern "system" fn Seek<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptableStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loffset: i32, streamseek: super::super::System::Com::STREAM_SEEK, plposition: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Seek(::core::mem::transmute_copy(&loffset), ::core::mem::transmute_copy(&streamseek)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinterScriptableStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsize: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSize(::core::mem::transmute_copy(&lsize)).into()
        }
        Self {
            base__: IPrinterScriptableSequentialStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            Commit: Commit::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPrinterScriptableStream as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || *iid == <IPrinterScriptableSequentialStream as ::windows_core::ComInterface>::IID
    }
}
pub trait IXpsDocument_Impl: Sized {
    fn GetThumbnail(&self) -> ::windows_core::Result<IPartThumbnail>;
    fn SetThumbnail(&self, pthumbnail: ::core::option::Option<&IPartThumbnail>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IXpsDocument {}
impl IXpsDocument_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocument_Impl, const OFFSET: isize>() -> IXpsDocument_Vtbl {
        unsafe extern "system" fn GetThumbnail<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppthumbnail: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetThumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppthumbnail, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnail<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pthumbnail: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetThumbnail(::windows_core::from_raw_borrowed(&pthumbnail)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetThumbnail: GetThumbnail::<Identity, Impl, OFFSET>,
            SetThumbnail: SetThumbnail::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IXpsDocument as ::windows_core::ComInterface>::IID
    }
}
pub trait IXpsDocumentConsumer_Impl: Sized {
    fn SendXpsUnknown(&self, punknown: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn SendXpsDocument(&self, pixpsdocument: ::core::option::Option<&IXpsDocument>) -> ::windows_core::Result<()>;
    fn SendFixedDocumentSequence(&self, pifixeddocumentsequence: ::core::option::Option<&IFixedDocumentSequence>) -> ::windows_core::Result<()>;
    fn SendFixedDocument(&self, pifixeddocument: ::core::option::Option<&IFixedDocument>) -> ::windows_core::Result<()>;
    fn SendFixedPage(&self, pifixedpage: ::core::option::Option<&IFixedPage>) -> ::windows_core::Result<()>;
    fn CloseSender(&self) -> ::windows_core::Result<()>;
    fn GetNewEmptyPart(&self, uri: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppnewobject: *mut *mut ::core::ffi::c_void, ppwritestream: *mut ::core::option::Option<IPrintWriteStream>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IXpsDocumentConsumer {}
impl IXpsDocumentConsumer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentConsumer_Impl, const OFFSET: isize>() -> IXpsDocumentConsumer_Vtbl {
        unsafe extern "system" fn SendXpsUnknown<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentConsumer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punknown: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendXpsUnknown(::windows_core::from_raw_borrowed(&punknown)).into()
        }
        unsafe extern "system" fn SendXpsDocument<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentConsumer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixpsdocument: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendXpsDocument(::windows_core::from_raw_borrowed(&pixpsdocument)).into()
        }
        unsafe extern "system" fn SendFixedDocumentSequence<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentConsumer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifixeddocumentsequence: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendFixedDocumentSequence(::windows_core::from_raw_borrowed(&pifixeddocumentsequence)).into()
        }
        unsafe extern "system" fn SendFixedDocument<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentConsumer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifixeddocument: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendFixedDocument(::windows_core::from_raw_borrowed(&pifixeddocument)).into()
        }
        unsafe extern "system" fn SendFixedPage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentConsumer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifixedpage: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendFixedPage(::windows_core::from_raw_borrowed(&pifixedpage)).into()
        }
        unsafe extern "system" fn CloseSender<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentConsumer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseSender().into()
        }
        unsafe extern "system" fn GetNewEmptyPart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentConsumer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppnewobject: *mut *mut ::core::ffi::c_void, ppwritestream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNewEmptyPart(::core::mem::transmute(&uri), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppnewobject), ::core::mem::transmute_copy(&ppwritestream)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SendXpsUnknown: SendXpsUnknown::<Identity, Impl, OFFSET>,
            SendXpsDocument: SendXpsDocument::<Identity, Impl, OFFSET>,
            SendFixedDocumentSequence: SendFixedDocumentSequence::<Identity, Impl, OFFSET>,
            SendFixedDocument: SendFixedDocument::<Identity, Impl, OFFSET>,
            SendFixedPage: SendFixedPage::<Identity, Impl, OFFSET>,
            CloseSender: CloseSender::<Identity, Impl, OFFSET>,
            GetNewEmptyPart: GetNewEmptyPart::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IXpsDocumentConsumer as ::windows_core::ComInterface>::IID
    }
}
pub trait IXpsDocumentProvider_Impl: Sized {
    fn GetXpsPart(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::RuntimeName for IXpsDocumentProvider {}
impl IXpsDocumentProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentProvider_Impl, const OFFSET: isize>() -> IXpsDocumentProvider_Vtbl {
        unsafe extern "system" fn GetXpsPart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixpspart: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetXpsPart() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppixpspart, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetXpsPart: GetXpsPart::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IXpsDocumentProvider as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsPartIterator_Impl: Sized {
    fn Reset(&self);
    fn Current(&self, puri: *mut ::windows_core::BSTR, ppxpspart: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn IsDone(&self) -> super::super::Foundation::BOOL;
    fn Next(&self);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IXpsPartIterator {}
#[cfg(feature = "Win32_Foundation")]
impl IXpsPartIterator_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsPartIterator_Impl, const OFFSET: isize>() -> IXpsPartIterator_Vtbl {
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsPartIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset()
        }
        unsafe extern "system" fn Current<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsPartIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, ppxpspart: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Current(::core::mem::transmute_copy(&puri), ::core::mem::transmute_copy(&ppxpspart)).into()
        }
        unsafe extern "system" fn IsDone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsPartIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsDone()
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsPartIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, Impl, OFFSET>,
            Current: Current::<Identity, Impl, OFFSET>,
            IsDone: IsDone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IXpsPartIterator as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Storage_Xps\"`"]
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IXpsRasterizationFactory_Impl: Sized {
    fn CreateRasterizer(&self, xpspage: ::core::option::Option<&super::super::Storage::Xps::IXpsOMPage>, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE) -> ::windows_core::Result<IXpsRasterizer>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl ::windows_core::RuntimeName for IXpsRasterizationFactory {}
#[cfg(feature = "Win32_Storage_Xps")]
impl IXpsRasterizationFactory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsRasterizationFactory_Impl, const OFFSET: isize>() -> IXpsRasterizationFactory_Vtbl {
        unsafe extern "system" fn CreateRasterizer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsRasterizationFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpspage: *mut ::core::ffi::c_void, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, ppixpsrasterizer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRasterizer(::windows_core::from_raw_borrowed(&xpspage), ::core::mem::transmute_copy(&dpi), ::core::mem::transmute_copy(&nontextrenderingmode), ::core::mem::transmute_copy(&textrenderingmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppixpsrasterizer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateRasterizer: CreateRasterizer::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IXpsRasterizationFactory as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Storage_Xps\"`"]
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IXpsRasterizationFactory1_Impl: Sized {
    fn CreateRasterizer(&self, xpspage: ::core::option::Option<&super::super::Storage::Xps::IXpsOMPage>, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT) -> ::windows_core::Result<IXpsRasterizer>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl ::windows_core::RuntimeName for IXpsRasterizationFactory1 {}
#[cfg(feature = "Win32_Storage_Xps")]
impl IXpsRasterizationFactory1_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsRasterizationFactory1_Impl, const OFFSET: isize>() -> IXpsRasterizationFactory1_Vtbl {
        unsafe extern "system" fn CreateRasterizer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsRasterizationFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpspage: *mut ::core::ffi::c_void, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, ppixpsrasterizer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRasterizer(::windows_core::from_raw_borrowed(&xpspage), ::core::mem::transmute_copy(&dpi), ::core::mem::transmute_copy(&nontextrenderingmode), ::core::mem::transmute_copy(&textrenderingmode), ::core::mem::transmute_copy(&pixelformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppixpsrasterizer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateRasterizer: CreateRasterizer::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IXpsRasterizationFactory1 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Storage_Xps\"`"]
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IXpsRasterizationFactory2_Impl: Sized {
    fn CreateRasterizer(&self, xpspage: ::core::option::Option<&super::super::Storage::Xps::IXpsOMPage>, dpix: f32, dpiy: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, backgroundcolor: XPSRAS_BACKGROUND_COLOR) -> ::windows_core::Result<IXpsRasterizer>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl ::windows_core::RuntimeName for IXpsRasterizationFactory2 {}
#[cfg(feature = "Win32_Storage_Xps")]
impl IXpsRasterizationFactory2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsRasterizationFactory2_Impl, const OFFSET: isize>() -> IXpsRasterizationFactory2_Vtbl {
        unsafe extern "system" fn CreateRasterizer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsRasterizationFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpspage: *mut ::core::ffi::c_void, dpix: f32, dpiy: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, backgroundcolor: XPSRAS_BACKGROUND_COLOR, ppixpsrasterizer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRasterizer(::windows_core::from_raw_borrowed(&xpspage), ::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy), ::core::mem::transmute_copy(&nontextrenderingmode), ::core::mem::transmute_copy(&textrenderingmode), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&backgroundcolor)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppixpsrasterizer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateRasterizer: CreateRasterizer::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IXpsRasterizationFactory2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Graphics_Imaging\"`"]
#[cfg(feature = "Win32_Graphics_Imaging")]
pub trait IXpsRasterizer_Impl: Sized {
    fn RasterizeRect(&self, x: i32, y: i32, width: i32, height: i32, notificationcallback: ::core::option::Option<&IXpsRasterizerNotificationCallback>) -> ::windows_core::Result<super::Imaging::IWICBitmap>;
    fn SetMinimalLineWidth(&self, width: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl ::windows_core::RuntimeName for IXpsRasterizer {}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl IXpsRasterizer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsRasterizer_Impl, const OFFSET: isize>() -> IXpsRasterizer_Vtbl {
        unsafe extern "system" fn RasterizeRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsRasterizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, width: i32, height: i32, notificationcallback: *mut ::core::ffi::c_void, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RasterizeRect(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::windows_core::from_raw_borrowed(&notificationcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bitmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimalLineWidth<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsRasterizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMinimalLineWidth(::core::mem::transmute_copy(&width)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RasterizeRect: RasterizeRect::<Identity, Impl, OFFSET>,
            SetMinimalLineWidth: SetMinimalLineWidth::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IXpsRasterizer as ::windows_core::ComInterface>::IID
    }
}
pub trait IXpsRasterizerNotificationCallback_Impl: Sized {
    fn Continue(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IXpsRasterizerNotificationCallback {}
impl IXpsRasterizerNotificationCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsRasterizerNotificationCallback_Impl, const OFFSET: isize>() -> IXpsRasterizerNotificationCallback_Vtbl {
        unsafe extern "system" fn Continue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsRasterizerNotificationCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Continue().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Continue: Continue::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IXpsRasterizerNotificationCallback as ::windows_core::ComInterface>::IID
    }
}
