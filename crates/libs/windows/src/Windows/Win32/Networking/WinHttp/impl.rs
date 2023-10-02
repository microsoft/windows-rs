#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWinHttpRequest_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetProxy(&self, proxysetting: i32, proxyserver: &super::super::System::Variant::VARIANT, bypasslist: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetCredentials(&self, username: &::windows_core::BSTR, password: &::windows_core::BSTR, flags: i32) -> ::windows_core::Result<()>;
    fn Open(&self, method: &::windows_core::BSTR, url: &::windows_core::BSTR, r#async: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetRequestHeader(&self, header: &::windows_core::BSTR, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetResponseHeader(&self, header: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetAllResponseHeaders(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Send(&self, body: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Status(&self) -> ::windows_core::Result<i32>;
    fn StatusText(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ResponseText(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ResponseBody(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn ResponseStream(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn get_Option(&self, option: WinHttpRequestOption) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn put_Option(&self, option: WinHttpRequestOption, value: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn WaitForResponse(&self, timeout: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Abort(&self) -> ::windows_core::Result<()>;
    fn SetTimeouts(&self, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> ::windows_core::Result<()>;
    fn SetClientCertificate(&self, clientcertificate: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetAutoLogonPolicy(&self, autologonpolicy: WinHttpRequestAutoLogonPolicy) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IWinHttpRequest {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWinHttpRequest_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: isize>() -> IWinHttpRequest_Vtbl {
        unsafe extern "system" fn SetProxy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, proxysetting: i32, proxyserver: super::super::System::Variant::VARIANT, bypasslist: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProxy(::core::mem::transmute_copy(&proxysetting), ::core::mem::transmute(&proxyserver), ::core::mem::transmute(&bypasslist)).into()
        }
        unsafe extern "system" fn SetCredentials<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, password: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCredentials(::core::mem::transmute(&username), ::core::mem::transmute(&password), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn Open<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: ::std::mem::MaybeUninit<::windows_core::BSTR>, url: ::std::mem::MaybeUninit<::windows_core::BSTR>, r#async: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open(::core::mem::transmute(&method), ::core::mem::transmute(&url), ::core::mem::transmute(&r#async)).into()
        }
        unsafe extern "system" fn SetRequestHeader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, header: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRequestHeader(::core::mem::transmute(&header), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GetResponseHeader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, header: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetResponseHeader(::core::mem::transmute(&header)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllResponseHeaders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headers: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAllResponseHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(headers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Send<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, body: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Send(::core::mem::transmute(&body)).into()
        }
        unsafe extern "system" fn Status<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, body: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResponseText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(body, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseBody<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, body: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResponseBody() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(body, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, body: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResponseStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(body, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Option<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: WinHttpRequestOption, value: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Option(::core::mem::transmute_copy(&option)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_Option<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: WinHttpRequestOption, value: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_Option(::core::mem::transmute_copy(&option), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn WaitForResponse<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: super::super::System::Variant::VARIANT, succeeded: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WaitForResponse(::core::mem::transmute(&timeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(succeeded, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort().into()
        }
        unsafe extern "system" fn SetTimeouts<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTimeouts(::core::mem::transmute_copy(&resolvetimeout), ::core::mem::transmute_copy(&connecttimeout), ::core::mem::transmute_copy(&sendtimeout), ::core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn SetClientCertificate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientcertificate: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClientCertificate(::core::mem::transmute(&clientcertificate)).into()
        }
        unsafe extern "system" fn SetAutoLogonPolicy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autologonpolicy: WinHttpRequestAutoLogonPolicy) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAutoLogonPolicy(::core::mem::transmute_copy(&autologonpolicy)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetProxy: SetProxy::<Identity, Impl, OFFSET>,
            SetCredentials: SetCredentials::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            SetRequestHeader: SetRequestHeader::<Identity, Impl, OFFSET>,
            GetResponseHeader: GetResponseHeader::<Identity, Impl, OFFSET>,
            GetAllResponseHeaders: GetAllResponseHeaders::<Identity, Impl, OFFSET>,
            Send: Send::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            StatusText: StatusText::<Identity, Impl, OFFSET>,
            ResponseText: ResponseText::<Identity, Impl, OFFSET>,
            ResponseBody: ResponseBody::<Identity, Impl, OFFSET>,
            ResponseStream: ResponseStream::<Identity, Impl, OFFSET>,
            get_Option: get_Option::<Identity, Impl, OFFSET>,
            put_Option: put_Option::<Identity, Impl, OFFSET>,
            WaitForResponse: WaitForResponse::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            SetTimeouts: SetTimeouts::<Identity, Impl, OFFSET>,
            SetClientCertificate: SetClientCertificate::<Identity, Impl, OFFSET>,
            SetAutoLogonPolicy: SetAutoLogonPolicy::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IWinHttpRequest as ::windows_core::ComInterface>::IID || *iid == <super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IWinHttpRequestEvents_Impl: Sized {
    fn OnResponseStart(&self, status: i32, contenttype: &::windows_core::BSTR);
    fn OnResponseDataAvailable(&self, data: *const *const super::super::System::Com::SAFEARRAY);
    fn OnResponseFinished(&self);
    fn OnError(&self, errornumber: i32, errordescription: &::windows_core::BSTR);
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IWinHttpRequestEvents {}
#[cfg(feature = "Win32_System_Com")]
impl IWinHttpRequestEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequestEvents_Impl, const OFFSET: isize>() -> IWinHttpRequestEvents_Vtbl {
        unsafe extern "system" fn OnResponseStart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequestEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: i32, contenttype: ::std::mem::MaybeUninit<::windows_core::BSTR>) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnResponseStart(::core::mem::transmute_copy(&status), ::core::mem::transmute(&contenttype))
        }
        unsafe extern "system" fn OnResponseDataAvailable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequestEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *const *const super::super::System::Com::SAFEARRAY) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnResponseDataAvailable(::core::mem::transmute_copy(&data))
        }
        unsafe extern "system" fn OnResponseFinished<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequestEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnResponseFinished()
        }
        unsafe extern "system" fn OnError<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinHttpRequestEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errornumber: i32, errordescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnError(::core::mem::transmute_copy(&errornumber), ::core::mem::transmute(&errordescription))
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnResponseStart: OnResponseStart::<Identity, Impl, OFFSET>,
            OnResponseDataAvailable: OnResponseDataAvailable::<Identity, Impl, OFFSET>,
            OnResponseFinished: OnResponseFinished::<Identity, Impl, OFFSET>,
            OnError: OnError::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IWinHttpRequestEvents as ::windows_core::ComInterface>::IID
    }
}
