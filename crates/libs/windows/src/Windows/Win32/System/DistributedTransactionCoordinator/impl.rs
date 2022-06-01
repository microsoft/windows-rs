pub trait IDtcLuConfigure_Impl: Sized {
    fn Add(&self, puclupair: *const u8, cblupair: u32) -> ::windows::core::Result<()>;
    fn Delete(&self, puclupair: *const u8, cblupair: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDtcLuConfigure {}
impl IDtcLuConfigure_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuConfigure_Impl, const OFFSET: isize>() -> IDtcLuConfigure_Vtbl {
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuConfigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puclupair: *const u8, cblupair: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::core::mem::transmute_copy(&puclupair), ::core::mem::transmute_copy(&cblupair)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuConfigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puclupair: *const u8, cblupair: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete(::core::mem::transmute_copy(&puclupair), ::core::mem::transmute_copy(&cblupair)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Add: Add::<Identity, Impl, OFFSET>, Delete: Delete::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuConfigure as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuRecovery_Impl: Sized {}
impl ::windows::core::RuntimeName for IDtcLuRecovery {}
impl IDtcLuRecovery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecovery_Impl, const OFFSET: isize>() -> IDtcLuRecovery_Vtbl {
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRecovery as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuRecoveryFactory_Impl: Sized {
    fn Create(&self, puclupair: *const u8, cblupair: u32) -> ::windows::core::Result<IDtcLuRecovery>;
}
impl ::windows::core::RuntimeName for IDtcLuRecoveryFactory {}
impl IDtcLuRecoveryFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryFactory_Impl, const OFFSET: isize>() -> IDtcLuRecoveryFactory_Vtbl {
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puclupair: *const u8, cblupair: u32, pprecovery: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create(::core::mem::transmute_copy(&puclupair), ::core::mem::transmute_copy(&cblupair)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprecovery, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRecoveryFactory as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuRecoveryInitiatedByDtc_Impl: Sized {
    fn GetWork(&self, pwork: *mut _DtcLu_LocalRecovery_Work, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDtcLuRecoveryInitiatedByDtc {}
impl IDtcLuRecoveryInitiatedByDtc_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtc_Impl, const OFFSET: isize>() -> IDtcLuRecoveryInitiatedByDtc_Vtbl {
        unsafe extern "system" fn GetWork<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwork: *mut _DtcLu_LocalRecovery_Work, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWork(::core::mem::transmute_copy(&pwork), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetWork: GetWork::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByDtc as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuRecoveryInitiatedByDtcStatusWork_Impl: Sized {
    fn HandleCheckLuStatus(&self, lrecoveryseqnum: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDtcLuRecoveryInitiatedByDtcStatusWork {}
impl IDtcLuRecoveryInitiatedByDtcStatusWork_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcStatusWork_Impl, const OFFSET: isize>() -> IDtcLuRecoveryInitiatedByDtcStatusWork_Vtbl {
        unsafe extern "system" fn HandleCheckLuStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcStatusWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrecoveryseqnum: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleCheckLuStatus(::core::mem::transmute_copy(&lrecoveryseqnum)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), HandleCheckLuStatus: HandleCheckLuStatus::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByDtcStatusWork as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcLuRecoveryInitiatedByDtcTransWork_Impl: Sized {
    fn GetLogNameSizes(&self, pcbourlogname: *mut u32, pcbremotelogname: *mut u32) -> ::windows::core::Result<()>;
    fn GetOurXln(&self, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::core::Result<()>;
    fn HandleConfirmationFromOurXln(&self, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::core::Result<()>;
    fn HandleTheirXlnResponse(&self, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, dwprotocol: u32, pconfirmation: *mut _DtcLu_Xln_Confirmation) -> ::windows::core::Result<()>;
    fn HandleErrorFromOurXln(&self, error: _DtcLu_Xln_Error) -> ::windows::core::Result<()>;
    fn CheckForCompareStates(&self, fcomparestates: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetOurTransIdSize(&self, pcbourtransid: *mut u32) -> ::windows::core::Result<()>;
    fn GetOurCompareStates(&self, pourtransid: *mut u8, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::core::Result<()>;
    fn HandleTheirCompareStatesResponse(&self, comparestate: _DtcLu_CompareState, pconfirmation: *mut _DtcLu_CompareStates_Confirmation) -> ::windows::core::Result<()>;
    fn HandleErrorFromOurCompareStates(&self, error: _DtcLu_CompareStates_Error) -> ::windows::core::Result<()>;
    fn ConversationLost(&self) -> ::windows::core::Result<()>;
    fn GetRecoverySeqNum(&self, plrecoveryseqnum: *mut i32) -> ::windows::core::Result<()>;
    fn ObsoleteRecoverySeqNum(&self, lnewrecoveryseqnum: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDtcLuRecoveryInitiatedByDtcTransWork {}
#[cfg(feature = "Win32_Foundation")]
impl IDtcLuRecoveryInitiatedByDtcTransWork_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>() -> IDtcLuRecoveryInitiatedByDtcTransWork_Vtbl {
        unsafe extern "system" fn GetLogNameSizes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbourlogname: *mut u32, pcbremotelogname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLogNameSizes(::core::mem::transmute_copy(&pcbourlogname), ::core::mem::transmute_copy(&pcbremotelogname)).into()
        }
        unsafe extern "system" fn GetOurXln<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOurXln(::core::mem::transmute_copy(&pxln), ::core::mem::transmute_copy(&pourlogname), ::core::mem::transmute_copy(&premotelogname), ::core::mem::transmute_copy(&pdwprotocol)).into()
        }
        unsafe extern "system" fn HandleConfirmationFromOurXln<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleConfirmationFromOurXln(::core::mem::transmute_copy(&confirmation)).into()
        }
        unsafe extern "system" fn HandleTheirXlnResponse<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, dwprotocol: u32, pconfirmation: *mut _DtcLu_Xln_Confirmation) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleTheirXlnResponse(::core::mem::transmute_copy(&xln), ::core::mem::transmute_copy(&premotelogname), ::core::mem::transmute_copy(&cbremotelogname), ::core::mem::transmute_copy(&dwprotocol), ::core::mem::transmute_copy(&pconfirmation)).into()
        }
        unsafe extern "system" fn HandleErrorFromOurXln<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: _DtcLu_Xln_Error) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleErrorFromOurXln(::core::mem::transmute_copy(&error)).into()
        }
        unsafe extern "system" fn CheckForCompareStates<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcomparestates: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CheckForCompareStates(::core::mem::transmute_copy(&fcomparestates)).into()
        }
        unsafe extern "system" fn GetOurTransIdSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbourtransid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOurTransIdSize(::core::mem::transmute_copy(&pcbourtransid)).into()
        }
        unsafe extern "system" fn GetOurCompareStates<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pourtransid: *mut u8, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOurCompareStates(::core::mem::transmute_copy(&pourtransid), ::core::mem::transmute_copy(&pcomparestate)).into()
        }
        unsafe extern "system" fn HandleTheirCompareStatesResponse<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparestate: _DtcLu_CompareState, pconfirmation: *mut _DtcLu_CompareStates_Confirmation) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleTheirCompareStatesResponse(::core::mem::transmute_copy(&comparestate), ::core::mem::transmute_copy(&pconfirmation)).into()
        }
        unsafe extern "system" fn HandleErrorFromOurCompareStates<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: _DtcLu_CompareStates_Error) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleErrorFromOurCompareStates(::core::mem::transmute_copy(&error)).into()
        }
        unsafe extern "system" fn ConversationLost<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConversationLost().into()
        }
        unsafe extern "system" fn GetRecoverySeqNum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plrecoveryseqnum: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRecoverySeqNum(::core::mem::transmute_copy(&plrecoveryseqnum)).into()
        }
        unsafe extern "system" fn ObsoleteRecoverySeqNum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnewrecoveryseqnum: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ObsoleteRecoverySeqNum(::core::mem::transmute_copy(&lnewrecoveryseqnum)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetLogNameSizes: GetLogNameSizes::<Identity, Impl, OFFSET>,
            GetOurXln: GetOurXln::<Identity, Impl, OFFSET>,
            HandleConfirmationFromOurXln: HandleConfirmationFromOurXln::<Identity, Impl, OFFSET>,
            HandleTheirXlnResponse: HandleTheirXlnResponse::<Identity, Impl, OFFSET>,
            HandleErrorFromOurXln: HandleErrorFromOurXln::<Identity, Impl, OFFSET>,
            CheckForCompareStates: CheckForCompareStates::<Identity, Impl, OFFSET>,
            GetOurTransIdSize: GetOurTransIdSize::<Identity, Impl, OFFSET>,
            GetOurCompareStates: GetOurCompareStates::<Identity, Impl, OFFSET>,
            HandleTheirCompareStatesResponse: HandleTheirCompareStatesResponse::<Identity, Impl, OFFSET>,
            HandleErrorFromOurCompareStates: HandleErrorFromOurCompareStates::<Identity, Impl, OFFSET>,
            ConversationLost: ConversationLost::<Identity, Impl, OFFSET>,
            GetRecoverySeqNum: GetRecoverySeqNum::<Identity, Impl, OFFSET>,
            ObsoleteRecoverySeqNum: ObsoleteRecoverySeqNum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByDtcTransWork as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuRecoveryInitiatedByLu_Impl: Sized {
    fn GetObjectToHandleWorkFromLu(&self) -> ::windows::core::Result<IDtcLuRecoveryInitiatedByLuWork>;
}
impl ::windows::core::RuntimeName for IDtcLuRecoveryInitiatedByLu {}
impl IDtcLuRecoveryInitiatedByLu_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByLu_Impl, const OFFSET: isize>() -> IDtcLuRecoveryInitiatedByLu_Vtbl {
        unsafe extern "system" fn GetObjectToHandleWorkFromLu<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByLu_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwork: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetObjectToHandleWorkFromLu() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwork, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetObjectToHandleWorkFromLu: GetObjectToHandleWorkFromLu::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByLu as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuRecoveryInitiatedByLuWork_Impl: Sized {
    fn HandleTheirXln(&self, lrecoveryseqnum: i32, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, pourlogname: *mut u8, cbourlogname: u32, dwprotocol: u32, presponse: *mut _DtcLu_Xln_Response) -> ::windows::core::Result<()>;
    fn GetOurLogNameSize(&self, pcbourlogname: *mut u32) -> ::windows::core::Result<()>;
    fn GetOurXln(&self, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::core::Result<()>;
    fn HandleConfirmationOfOurXln(&self, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::core::Result<()>;
    fn HandleTheirCompareStates(&self, premotetransid: *mut u8, cbremotetransid: u32, comparestate: _DtcLu_CompareState, presponse: *mut _DtcLu_CompareStates_Response, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::core::Result<()>;
    fn HandleConfirmationOfOurCompareStates(&self, confirmation: _DtcLu_CompareStates_Confirmation) -> ::windows::core::Result<()>;
    fn HandleErrorFromOurCompareStates(&self, error: _DtcLu_CompareStates_Error) -> ::windows::core::Result<()>;
    fn ConversationLost(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDtcLuRecoveryInitiatedByLuWork {}
impl IDtcLuRecoveryInitiatedByLuWork_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>() -> IDtcLuRecoveryInitiatedByLuWork_Vtbl {
        unsafe extern "system" fn HandleTheirXln<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrecoveryseqnum: i32, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, pourlogname: *mut u8, cbourlogname: u32, dwprotocol: u32, presponse: *mut _DtcLu_Xln_Response) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleTheirXln(::core::mem::transmute_copy(&lrecoveryseqnum), ::core::mem::transmute_copy(&xln), ::core::mem::transmute_copy(&premotelogname), ::core::mem::transmute_copy(&cbremotelogname), ::core::mem::transmute_copy(&pourlogname), ::core::mem::transmute_copy(&cbourlogname), ::core::mem::transmute_copy(&dwprotocol), ::core::mem::transmute_copy(&presponse)).into()
        }
        unsafe extern "system" fn GetOurLogNameSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbourlogname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOurLogNameSize(::core::mem::transmute_copy(&pcbourlogname)).into()
        }
        unsafe extern "system" fn GetOurXln<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOurXln(::core::mem::transmute_copy(&pxln), ::core::mem::transmute_copy(&pourlogname), ::core::mem::transmute_copy(&pdwprotocol)).into()
        }
        unsafe extern "system" fn HandleConfirmationOfOurXln<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleConfirmationOfOurXln(::core::mem::transmute_copy(&confirmation)).into()
        }
        unsafe extern "system" fn HandleTheirCompareStates<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, premotetransid: *mut u8, cbremotetransid: u32, comparestate: _DtcLu_CompareState, presponse: *mut _DtcLu_CompareStates_Response, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleTheirCompareStates(::core::mem::transmute_copy(&premotetransid), ::core::mem::transmute_copy(&cbremotetransid), ::core::mem::transmute_copy(&comparestate), ::core::mem::transmute_copy(&presponse), ::core::mem::transmute_copy(&pcomparestate)).into()
        }
        unsafe extern "system" fn HandleConfirmationOfOurCompareStates<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confirmation: _DtcLu_CompareStates_Confirmation) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleConfirmationOfOurCompareStates(::core::mem::transmute_copy(&confirmation)).into()
        }
        unsafe extern "system" fn HandleErrorFromOurCompareStates<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: _DtcLu_CompareStates_Error) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleErrorFromOurCompareStates(::core::mem::transmute_copy(&error)).into()
        }
        unsafe extern "system" fn ConversationLost<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConversationLost().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            HandleTheirXln: HandleTheirXln::<Identity, Impl, OFFSET>,
            GetOurLogNameSize: GetOurLogNameSize::<Identity, Impl, OFFSET>,
            GetOurXln: GetOurXln::<Identity, Impl, OFFSET>,
            HandleConfirmationOfOurXln: HandleConfirmationOfOurXln::<Identity, Impl, OFFSET>,
            HandleTheirCompareStates: HandleTheirCompareStates::<Identity, Impl, OFFSET>,
            HandleConfirmationOfOurCompareStates: HandleConfirmationOfOurCompareStates::<Identity, Impl, OFFSET>,
            HandleErrorFromOurCompareStates: HandleErrorFromOurCompareStates::<Identity, Impl, OFFSET>,
            ConversationLost: ConversationLost::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByLuWork as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcLuRmEnlistment_Impl: Sized {
    fn Unplug(&self, fconversationlost: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn BackedOut(&self) -> ::windows::core::Result<()>;
    fn BackOut(&self) -> ::windows::core::Result<()>;
    fn Committed(&self) -> ::windows::core::Result<()>;
    fn Forget(&self) -> ::windows::core::Result<()>;
    fn RequestCommit(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDtcLuRmEnlistment {}
#[cfg(feature = "Win32_Foundation")]
impl IDtcLuRmEnlistment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRmEnlistment_Impl, const OFFSET: isize>() -> IDtcLuRmEnlistment_Vtbl {
        unsafe extern "system" fn Unplug<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fconversationlost: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unplug(::core::mem::transmute_copy(&fconversationlost)).into()
        }
        unsafe extern "system" fn BackedOut<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BackedOut().into()
        }
        unsafe extern "system" fn BackOut<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BackOut().into()
        }
        unsafe extern "system" fn Committed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Committed().into()
        }
        unsafe extern "system" fn Forget<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Forget().into()
        }
        unsafe extern "system" fn RequestCommit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestCommit().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Unplug: Unplug::<Identity, Impl, OFFSET>,
            BackedOut: BackedOut::<Identity, Impl, OFFSET>,
            BackOut: BackOut::<Identity, Impl, OFFSET>,
            Committed: Committed::<Identity, Impl, OFFSET>,
            Forget: Forget::<Identity, Impl, OFFSET>,
            RequestCommit: RequestCommit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRmEnlistment as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuRmEnlistmentFactory_Impl: Sized {
    fn Create(&self, puclupair: *mut u8, cblupair: u32, pitransaction: &::core::option::Option<ITransaction>, ptransid: *mut u8, cbtransid: u32, prmenlistmentsink: &::core::option::Option<IDtcLuRmEnlistmentSink>, pprmenlistment: *mut ::core::option::Option<IDtcLuRmEnlistment>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDtcLuRmEnlistmentFactory {}
impl IDtcLuRmEnlistmentFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRmEnlistmentFactory_Impl, const OFFSET: isize>() -> IDtcLuRmEnlistmentFactory_Vtbl {
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRmEnlistmentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puclupair: *mut u8, cblupair: u32, pitransaction: *mut ::core::ffi::c_void, ptransid: *mut u8, cbtransid: u32, prmenlistmentsink: *mut ::core::ffi::c_void, pprmenlistment: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Create(::core::mem::transmute_copy(&puclupair), ::core::mem::transmute_copy(&cblupair), ::core::mem::transmute(&pitransaction), ::core::mem::transmute_copy(&ptransid), ::core::mem::transmute_copy(&cbtransid), ::core::mem::transmute(&prmenlistmentsink), ::core::mem::transmute_copy(&pprmenlistment)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRmEnlistmentFactory as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuRmEnlistmentSink_Impl: Sized {
    fn AckUnplug(&self) -> ::windows::core::Result<()>;
    fn TmDown(&self) -> ::windows::core::Result<()>;
    fn SessionLost(&self) -> ::windows::core::Result<()>;
    fn BackedOut(&self) -> ::windows::core::Result<()>;
    fn BackOut(&self) -> ::windows::core::Result<()>;
    fn Committed(&self) -> ::windows::core::Result<()>;
    fn Forget(&self) -> ::windows::core::Result<()>;
    fn Prepare(&self) -> ::windows::core::Result<()>;
    fn RequestCommit(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDtcLuRmEnlistmentSink {}
impl IDtcLuRmEnlistmentSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>() -> IDtcLuRmEnlistmentSink_Vtbl {
        unsafe extern "system" fn AckUnplug<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AckUnplug().into()
        }
        unsafe extern "system" fn TmDown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TmDown().into()
        }
        unsafe extern "system" fn SessionLost<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SessionLost().into()
        }
        unsafe extern "system" fn BackedOut<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BackedOut().into()
        }
        unsafe extern "system" fn BackOut<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BackOut().into()
        }
        unsafe extern "system" fn Committed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Committed().into()
        }
        unsafe extern "system" fn Forget<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Forget().into()
        }
        unsafe extern "system" fn Prepare<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Prepare().into()
        }
        unsafe extern "system" fn RequestCommit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestCommit().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AckUnplug: AckUnplug::<Identity, Impl, OFFSET>,
            TmDown: TmDown::<Identity, Impl, OFFSET>,
            SessionLost: SessionLost::<Identity, Impl, OFFSET>,
            BackedOut: BackedOut::<Identity, Impl, OFFSET>,
            BackOut: BackOut::<Identity, Impl, OFFSET>,
            Committed: Committed::<Identity, Impl, OFFSET>,
            Forget: Forget::<Identity, Impl, OFFSET>,
            Prepare: Prepare::<Identity, Impl, OFFSET>,
            RequestCommit: RequestCommit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRmEnlistmentSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcLuSubordinateDtc_Impl: Sized {
    fn Unplug(&self, fconversationlost: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn BackedOut(&self) -> ::windows::core::Result<()>;
    fn BackOut(&self) -> ::windows::core::Result<()>;
    fn Committed(&self) -> ::windows::core::Result<()>;
    fn Forget(&self) -> ::windows::core::Result<()>;
    fn Prepare(&self) -> ::windows::core::Result<()>;
    fn RequestCommit(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDtcLuSubordinateDtc {}
#[cfg(feature = "Win32_Foundation")]
impl IDtcLuSubordinateDtc_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>() -> IDtcLuSubordinateDtc_Vtbl {
        unsafe extern "system" fn Unplug<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fconversationlost: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unplug(::core::mem::transmute_copy(&fconversationlost)).into()
        }
        unsafe extern "system" fn BackedOut<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BackedOut().into()
        }
        unsafe extern "system" fn BackOut<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BackOut().into()
        }
        unsafe extern "system" fn Committed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Committed().into()
        }
        unsafe extern "system" fn Forget<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Forget().into()
        }
        unsafe extern "system" fn Prepare<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Prepare().into()
        }
        unsafe extern "system" fn RequestCommit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestCommit().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Unplug: Unplug::<Identity, Impl, OFFSET>,
            BackedOut: BackedOut::<Identity, Impl, OFFSET>,
            BackOut: BackOut::<Identity, Impl, OFFSET>,
            Committed: Committed::<Identity, Impl, OFFSET>,
            Forget: Forget::<Identity, Impl, OFFSET>,
            Prepare: Prepare::<Identity, Impl, OFFSET>,
            RequestCommit: RequestCommit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuSubordinateDtc as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuSubordinateDtcFactory_Impl: Sized {
    fn Create(&self, puclupair: *mut u8, cblupair: u32, punktransactionouter: &::core::option::Option<::windows::core::IUnknown>, isolevel: i32, isoflags: u32, poptions: &::core::option::Option<ITransactionOptions>, pptransaction: *mut ::core::option::Option<ITransaction>, ptransid: *mut u8, cbtransid: u32, psubordinatedtcsink: &::core::option::Option<IDtcLuSubordinateDtcSink>, ppsubordinatedtc: *mut ::core::option::Option<IDtcLuSubordinateDtc>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDtcLuSubordinateDtcFactory {}
impl IDtcLuSubordinateDtcFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuSubordinateDtcFactory_Impl, const OFFSET: isize>() -> IDtcLuSubordinateDtcFactory_Vtbl {
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuSubordinateDtcFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puclupair: *mut u8, cblupair: u32, punktransactionouter: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, poptions: *mut ::core::ffi::c_void, pptransaction: *mut *mut ::core::ffi::c_void, ptransid: *mut u8, cbtransid: u32, psubordinatedtcsink: *mut ::core::ffi::c_void, ppsubordinatedtc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Create(::core::mem::transmute_copy(&puclupair), ::core::mem::transmute_copy(&cblupair), ::core::mem::transmute(&punktransactionouter), ::core::mem::transmute_copy(&isolevel), ::core::mem::transmute_copy(&isoflags), ::core::mem::transmute(&poptions), ::core::mem::transmute_copy(&pptransaction), ::core::mem::transmute_copy(&ptransid), ::core::mem::transmute_copy(&cbtransid), ::core::mem::transmute(&psubordinatedtcsink), ::core::mem::transmute_copy(&ppsubordinatedtc))
                .into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuSubordinateDtcFactory as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuSubordinateDtcSink_Impl: Sized {
    fn AckUnplug(&self) -> ::windows::core::Result<()>;
    fn TmDown(&self) -> ::windows::core::Result<()>;
    fn SessionLost(&self) -> ::windows::core::Result<()>;
    fn BackedOut(&self) -> ::windows::core::Result<()>;
    fn BackOut(&self) -> ::windows::core::Result<()>;
    fn Committed(&self) -> ::windows::core::Result<()>;
    fn Forget(&self) -> ::windows::core::Result<()>;
    fn RequestCommit(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDtcLuSubordinateDtcSink {}
impl IDtcLuSubordinateDtcSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>() -> IDtcLuSubordinateDtcSink_Vtbl {
        unsafe extern "system" fn AckUnplug<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AckUnplug().into()
        }
        unsafe extern "system" fn TmDown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TmDown().into()
        }
        unsafe extern "system" fn SessionLost<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SessionLost().into()
        }
        unsafe extern "system" fn BackedOut<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BackedOut().into()
        }
        unsafe extern "system" fn BackOut<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BackOut().into()
        }
        unsafe extern "system" fn Committed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Committed().into()
        }
        unsafe extern "system" fn Forget<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Forget().into()
        }
        unsafe extern "system" fn RequestCommit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestCommit().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AckUnplug: AckUnplug::<Identity, Impl, OFFSET>,
            TmDown: TmDown::<Identity, Impl, OFFSET>,
            SessionLost: SessionLost::<Identity, Impl, OFFSET>,
            BackedOut: BackedOut::<Identity, Impl, OFFSET>,
            BackOut: BackOut::<Identity, Impl, OFFSET>,
            Committed: Committed::<Identity, Impl, OFFSET>,
            Forget: Forget::<Identity, Impl, OFFSET>,
            RequestCommit: RequestCommit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuSubordinateDtcSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcNetworkAccessConfig_Impl: Sized {
    fn GetAnyNetworkAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetAnyNetworkAccess(&self, banynetworkaccess: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetNetworkAdministrationAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetNetworkAdministrationAccess(&self, bnetworkadministrationaccess: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetNetworkTransactionAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetNetworkTransactionAccess(&self, bnetworktransactionaccess: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetNetworkClientAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetNetworkClientAccess(&self, bnetworkclientaccess: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetNetworkTIPAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetNetworkTIPAccess(&self, bnetworktipaccess: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetXAAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetXAAccess(&self, bxaaccess: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn RestartDtcService(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDtcNetworkAccessConfig {}
#[cfg(feature = "Win32_Foundation")]
impl IDtcNetworkAccessConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>() -> IDtcNetworkAccessConfig_Vtbl {
        unsafe extern "system" fn GetAnyNetworkAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbanynetworkaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAnyNetworkAccess() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbanynetworkaccess, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnyNetworkAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, banynetworkaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAnyNetworkAccess(::core::mem::transmute_copy(&banynetworkaccess)).into()
        }
        unsafe extern "system" fn GetNetworkAdministrationAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbnetworkadministrationaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNetworkAdministrationAccess() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbnetworkadministrationaccess, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkAdministrationAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bnetworkadministrationaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNetworkAdministrationAccess(::core::mem::transmute_copy(&bnetworkadministrationaccess)).into()
        }
        unsafe extern "system" fn GetNetworkTransactionAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbnetworktransactionaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNetworkTransactionAccess() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbnetworktransactionaccess, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkTransactionAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bnetworktransactionaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNetworkTransactionAccess(::core::mem::transmute_copy(&bnetworktransactionaccess)).into()
        }
        unsafe extern "system" fn GetNetworkClientAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbnetworkclientaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNetworkClientAccess() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbnetworkclientaccess, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkClientAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bnetworkclientaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNetworkClientAccess(::core::mem::transmute_copy(&bnetworkclientaccess)).into()
        }
        unsafe extern "system" fn GetNetworkTIPAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbnetworktipaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNetworkTIPAccess() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbnetworktipaccess, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkTIPAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bnetworktipaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNetworkTIPAccess(::core::mem::transmute_copy(&bnetworktipaccess)).into()
        }
        unsafe extern "system" fn GetXAAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbxaaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetXAAccess() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbxaaccess, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXAAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bxaaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetXAAccess(::core::mem::transmute_copy(&bxaaccess)).into()
        }
        unsafe extern "system" fn RestartDtcService<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestartDtcService().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetAnyNetworkAccess: GetAnyNetworkAccess::<Identity, Impl, OFFSET>,
            SetAnyNetworkAccess: SetAnyNetworkAccess::<Identity, Impl, OFFSET>,
            GetNetworkAdministrationAccess: GetNetworkAdministrationAccess::<Identity, Impl, OFFSET>,
            SetNetworkAdministrationAccess: SetNetworkAdministrationAccess::<Identity, Impl, OFFSET>,
            GetNetworkTransactionAccess: GetNetworkTransactionAccess::<Identity, Impl, OFFSET>,
            SetNetworkTransactionAccess: SetNetworkTransactionAccess::<Identity, Impl, OFFSET>,
            GetNetworkClientAccess: GetNetworkClientAccess::<Identity, Impl, OFFSET>,
            SetNetworkClientAccess: SetNetworkClientAccess::<Identity, Impl, OFFSET>,
            GetNetworkTIPAccess: GetNetworkTIPAccess::<Identity, Impl, OFFSET>,
            SetNetworkTIPAccess: SetNetworkTIPAccess::<Identity, Impl, OFFSET>,
            GetXAAccess: GetXAAccess::<Identity, Impl, OFFSET>,
            SetXAAccess: SetXAAccess::<Identity, Impl, OFFSET>,
            RestartDtcService: RestartDtcService::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcNetworkAccessConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcNetworkAccessConfig2_Impl: Sized + IDtcNetworkAccessConfig_Impl {
    fn GetNetworkInboundAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetNetworkOutboundAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetNetworkInboundAccess(&self, binbound: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetNetworkOutboundAccess(&self, boutbound: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetAuthenticationLevel(&self) -> ::windows::core::Result<AUTHENTICATION_LEVEL>;
    fn SetAuthenticationLevel(&self, authlevel: AUTHENTICATION_LEVEL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDtcNetworkAccessConfig2 {}
#[cfg(feature = "Win32_Foundation")]
impl IDtcNetworkAccessConfig2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>() -> IDtcNetworkAccessConfig2_Vtbl {
        unsafe extern "system" fn GetNetworkInboundAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbinbound: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNetworkInboundAccess() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbinbound, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkOutboundAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboutbound: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNetworkOutboundAccess() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pboutbound, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkInboundAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binbound: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNetworkInboundAccess(::core::mem::transmute_copy(&binbound)).into()
        }
        unsafe extern "system" fn SetNetworkOutboundAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boutbound: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNetworkOutboundAccess(::core::mem::transmute_copy(&boutbound)).into()
        }
        unsafe extern "system" fn GetAuthenticationLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauthlevel: *mut AUTHENTICATION_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAuthenticationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pauthlevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, authlevel: AUTHENTICATION_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuthenticationLevel(::core::mem::transmute_copy(&authlevel)).into()
        }
        Self {
            base__: IDtcNetworkAccessConfig_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetNetworkInboundAccess: GetNetworkInboundAccess::<Identity, Impl, OFFSET>,
            GetNetworkOutboundAccess: GetNetworkOutboundAccess::<Identity, Impl, OFFSET>,
            SetNetworkInboundAccess: SetNetworkInboundAccess::<Identity, Impl, OFFSET>,
            SetNetworkOutboundAccess: SetNetworkOutboundAccess::<Identity, Impl, OFFSET>,
            GetAuthenticationLevel: GetAuthenticationLevel::<Identity, Impl, OFFSET>,
            SetAuthenticationLevel: SetAuthenticationLevel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcNetworkAccessConfig2 as ::windows::core::Interface>::IID || iid == &<IDtcNetworkAccessConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcNetworkAccessConfig3_Impl: Sized + IDtcNetworkAccessConfig_Impl + IDtcNetworkAccessConfig2_Impl {
    fn GetLUAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetLUAccess(&self, bluaccess: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDtcNetworkAccessConfig3 {}
#[cfg(feature = "Win32_Foundation")]
impl IDtcNetworkAccessConfig3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig3_Impl, const OFFSET: isize>() -> IDtcNetworkAccessConfig3_Vtbl {
        unsafe extern "system" fn GetLUAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbluaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLUAccess() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbluaccess, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLUAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcNetworkAccessConfig3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLUAccess(::core::mem::transmute_copy(&bluaccess)).into()
        }
        Self {
            base__: IDtcNetworkAccessConfig2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetLUAccess: GetLUAccess::<Identity, Impl, OFFSET>,
            SetLUAccess: SetLUAccess::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcNetworkAccessConfig3 as ::windows::core::Interface>::IID || iid == &<IDtcNetworkAccessConfig as ::windows::core::Interface>::IID || iid == &<IDtcNetworkAccessConfig2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcToXaHelper_Impl: Sized {
    fn Close(&self, i_fdorecovery: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn TranslateTridToXid(&self, pitransaction: &::core::option::Option<ITransaction>, pguidbqual: *const ::windows::core::GUID) -> ::windows::core::Result<xid_t>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDtcToXaHelper {}
#[cfg(feature = "Win32_Foundation")]
impl IDtcToXaHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcToXaHelper_Impl, const OFFSET: isize>() -> IDtcToXaHelper_Vtbl {
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcToXaHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i_fdorecovery: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close(::core::mem::transmute_copy(&i_fdorecovery)).into()
        }
        unsafe extern "system" fn TranslateTridToXid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcToXaHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitransaction: *mut ::core::ffi::c_void, pguidbqual: *const ::windows::core::GUID, pxid: *mut xid_t) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TranslateTridToXid(::core::mem::transmute(&pitransaction), ::core::mem::transmute_copy(&pguidbqual)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pxid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Close: Close::<Identity, Impl, OFFSET>,
            TranslateTridToXid: TranslateTridToXid::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcToXaHelper as ::windows::core::Interface>::IID
    }
}
pub trait IDtcToXaHelperFactory_Impl: Sized {
    fn Create(&self, pszdsn: &::windows::core::PCSTR, pszclientdllname: &::windows::core::PCSTR, pguidrm: *mut ::windows::core::GUID, ppxahelper: *mut ::core::option::Option<IDtcToXaHelper>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDtcToXaHelperFactory {}
impl IDtcToXaHelperFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcToXaHelperFactory_Impl, const OFFSET: isize>() -> IDtcToXaHelperFactory_Vtbl {
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcToXaHelperFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdsn: ::windows::core::PCSTR, pszclientdllname: ::windows::core::PCSTR, pguidrm: *mut ::windows::core::GUID, ppxahelper: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Create(::core::mem::transmute(&pszdsn), ::core::mem::transmute(&pszclientdllname), ::core::mem::transmute_copy(&pguidrm), ::core::mem::transmute_copy(&ppxahelper)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcToXaHelperFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcToXaHelperSinglePipe_Impl: Sized {
    fn XARMCreate(&self, pszdsn: &::windows::core::PCSTR, pszclientdll: &::windows::core::PCSTR, pdwrmcookie: *mut u32) -> ::windows::core::Result<()>;
    fn ConvertTridToXID(&self, pdwitrans: *mut u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::core::Result<()>;
    fn EnlistWithRM(&self, dwrmcookie: u32, i_pitransaction: &::core::option::Option<ITransaction>, i_pitransres: &::core::option::Option<ITransactionResourceAsync>) -> ::windows::core::Result<ITransactionEnlistmentAsync>;
    fn ReleaseRMCookie(&self, i_dwrmcookie: u32, i_fnormal: super::super::Foundation::BOOL);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDtcToXaHelperSinglePipe {}
#[cfg(feature = "Win32_Foundation")]
impl IDtcToXaHelperSinglePipe_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcToXaHelperSinglePipe_Impl, const OFFSET: isize>() -> IDtcToXaHelperSinglePipe_Vtbl {
        unsafe extern "system" fn XARMCreate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcToXaHelperSinglePipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdsn: ::windows::core::PCSTR, pszclientdll: ::windows::core::PCSTR, pdwrmcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.XARMCreate(::core::mem::transmute(&pszdsn), ::core::mem::transmute(&pszclientdll), ::core::mem::transmute_copy(&pdwrmcookie)).into()
        }
        unsafe extern "system" fn ConvertTridToXID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcToXaHelperSinglePipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwitrans: *mut u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConvertTridToXID(::core::mem::transmute_copy(&pdwitrans), ::core::mem::transmute_copy(&dwrmcookie), ::core::mem::transmute_copy(&pxid)).into()
        }
        unsafe extern "system" fn EnlistWithRM<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcToXaHelperSinglePipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwrmcookie: u32, i_pitransaction: *mut ::core::ffi::c_void, i_pitransres: *mut ::core::ffi::c_void, o_ppitransenslitment: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnlistWithRM(::core::mem::transmute_copy(&dwrmcookie), ::core::mem::transmute(&i_pitransaction), ::core::mem::transmute(&i_pitransres)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(o_ppitransenslitment, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseRMCookie<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcToXaHelperSinglePipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i_dwrmcookie: u32, i_fnormal: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseRMCookie(::core::mem::transmute_copy(&i_dwrmcookie), ::core::mem::transmute_copy(&i_fnormal))
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            XARMCreate: XARMCreate::<Identity, Impl, OFFSET>,
            ConvertTridToXID: ConvertTridToXID::<Identity, Impl, OFFSET>,
            EnlistWithRM: EnlistWithRM::<Identity, Impl, OFFSET>,
            ReleaseRMCookie: ReleaseRMCookie::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcToXaHelperSinglePipe as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcToXaMapper_Impl: Sized {
    fn RequestNewResourceManager(&self, pszdsn: &::windows::core::PCSTR, pszclientdllname: &::windows::core::PCSTR, pdwrmcookie: *mut u32) -> ::windows::core::Result<()>;
    fn TranslateTridToXid(&self, pdwitransaction: *const u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::core::Result<()>;
    fn EnlistResourceManager(&self, dwrmcookie: u32, pdwitransaction: *const u32) -> ::windows::core::Result<()>;
    fn ReleaseResourceManager(&self, dwrmcookie: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDtcToXaMapper {}
#[cfg(feature = "Win32_Foundation")]
impl IDtcToXaMapper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcToXaMapper_Impl, const OFFSET: isize>() -> IDtcToXaMapper_Vtbl {
        unsafe extern "system" fn RequestNewResourceManager<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcToXaMapper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdsn: ::windows::core::PCSTR, pszclientdllname: ::windows::core::PCSTR, pdwrmcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestNewResourceManager(::core::mem::transmute(&pszdsn), ::core::mem::transmute(&pszclientdllname), ::core::mem::transmute_copy(&pdwrmcookie)).into()
        }
        unsafe extern "system" fn TranslateTridToXid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcToXaMapper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwitransaction: *const u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TranslateTridToXid(::core::mem::transmute_copy(&pdwitransaction), ::core::mem::transmute_copy(&dwrmcookie), ::core::mem::transmute_copy(&pxid)).into()
        }
        unsafe extern "system" fn EnlistResourceManager<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcToXaMapper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwrmcookie: u32, pdwitransaction: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnlistResourceManager(::core::mem::transmute_copy(&dwrmcookie), ::core::mem::transmute_copy(&pdwitransaction)).into()
        }
        unsafe extern "system" fn ReleaseResourceManager<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDtcToXaMapper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwrmcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseResourceManager(::core::mem::transmute_copy(&dwrmcookie)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RequestNewResourceManager: RequestNewResourceManager::<Identity, Impl, OFFSET>,
            TranslateTridToXid: TranslateTridToXid::<Identity, Impl, OFFSET>,
            EnlistResourceManager: EnlistResourceManager::<Identity, Impl, OFFSET>,
            ReleaseResourceManager: ReleaseResourceManager::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcToXaMapper as ::windows::core::Interface>::IID
    }
}
pub trait IGetDispenser_Impl: Sized {
    fn GetDispenser(&self, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IGetDispenser {}
impl IGetDispenser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetDispenser_Impl, const OFFSET: isize>() -> IGetDispenser_Vtbl {
        unsafe extern "system" fn GetDispenser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetDispenser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDispenser(::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvobject)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetDispenser: GetDispenser::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetDispenser as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IKernelTransaction_Impl: Sized {
    fn GetHandle(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IKernelTransaction {}
#[cfg(feature = "Win32_Foundation")]
impl IKernelTransaction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKernelTransaction_Impl, const OFFSET: isize>() -> IKernelTransaction_Vtbl {
        unsafe extern "system" fn GetHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKernelTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHandle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetHandle: GetHandle::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKernelTransaction as ::windows::core::Interface>::IID
    }
}
pub trait ILastResourceManager_Impl: Sized {
    fn TransactionCommitted(&self, pprepinfo: *const u8, cbprepinfo: u32) -> ::windows::core::Result<()>;
    fn RecoveryDone(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ILastResourceManager {}
impl ILastResourceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILastResourceManager_Impl, const OFFSET: isize>() -> ILastResourceManager_Vtbl {
        unsafe extern "system" fn TransactionCommitted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILastResourceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TransactionCommitted(::core::mem::transmute_copy(&pprepinfo), ::core::mem::transmute_copy(&cbprepinfo)).into()
        }
        unsafe extern "system" fn RecoveryDone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILastResourceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RecoveryDone().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            TransactionCommitted: TransactionCommitted::<Identity, Impl, OFFSET>,
            RecoveryDone: RecoveryDone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILastResourceManager as ::windows::core::Interface>::IID
    }
}
pub trait IPrepareInfo_Impl: Sized {
    fn GetPrepareInfoSize(&self, pcbprepinfo: *mut u32) -> ::windows::core::Result<()>;
    fn GetPrepareInfo(&self, pprepinfo: *mut u8) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPrepareInfo {}
impl IPrepareInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrepareInfo_Impl, const OFFSET: isize>() -> IPrepareInfo_Vtbl {
        unsafe extern "system" fn GetPrepareInfoSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrepareInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbprepinfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPrepareInfoSize(::core::mem::transmute_copy(&pcbprepinfo)).into()
        }
        unsafe extern "system" fn GetPrepareInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrepareInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprepinfo: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPrepareInfo(::core::mem::transmute_copy(&pprepinfo)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPrepareInfoSize: GetPrepareInfoSize::<Identity, Impl, OFFSET>,
            GetPrepareInfo: GetPrepareInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrepareInfo as ::windows::core::Interface>::IID
    }
}
pub trait IPrepareInfo2_Impl: Sized {
    fn GetPrepareInfoSize(&self) -> ::windows::core::Result<u32>;
    fn GetPrepareInfo(&self, cbprepareinfo: u32, pprepinfo: *mut u8) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPrepareInfo2 {}
impl IPrepareInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrepareInfo2_Impl, const OFFSET: isize>() -> IPrepareInfo2_Vtbl {
        unsafe extern "system" fn GetPrepareInfoSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrepareInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbprepinfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPrepareInfoSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbprepinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrepareInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrepareInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbprepareinfo: u32, pprepinfo: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPrepareInfo(::core::mem::transmute_copy(&cbprepareinfo), ::core::mem::transmute_copy(&pprepinfo)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPrepareInfoSize: GetPrepareInfoSize::<Identity, Impl, OFFSET>,
            GetPrepareInfo: GetPrepareInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrepareInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRMHelper_Impl: Sized {
    fn RMCount(&self, dwctotalnumberofrms: u32) -> ::windows::core::Result<()>;
    fn RMInfo(&self, pxa_switch: *mut xa_switch_t, fcdeclcallingconv: super::super::Foundation::BOOL, pszopenstring: &::windows::core::PCSTR, pszclosestring: &::windows::core::PCSTR, guidrmrecovery: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRMHelper {}
#[cfg(feature = "Win32_Foundation")]
impl IRMHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRMHelper_Impl, const OFFSET: isize>() -> IRMHelper_Vtbl {
        unsafe extern "system" fn RMCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRMHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwctotalnumberofrms: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RMCount(::core::mem::transmute_copy(&dwctotalnumberofrms)).into()
        }
        unsafe extern "system" fn RMInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRMHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxa_switch: *mut xa_switch_t, fcdeclcallingconv: super::super::Foundation::BOOL, pszopenstring: ::windows::core::PCSTR, pszclosestring: ::windows::core::PCSTR, guidrmrecovery: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RMInfo(::core::mem::transmute_copy(&pxa_switch), ::core::mem::transmute_copy(&fcdeclcallingconv), ::core::mem::transmute(&pszopenstring), ::core::mem::transmute(&pszclosestring), ::core::mem::transmute(&guidrmrecovery)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RMCount: RMCount::<Identity, Impl, OFFSET>,
            RMInfo: RMInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRMHelper as ::windows::core::Interface>::IID
    }
}
pub trait IResourceManager_Impl: Sized {
    fn Enlist(&self, ptransaction: &::core::option::Option<ITransaction>, pres: &::core::option::Option<ITransactionResourceAsync>, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows::core::Result<()>;
    fn Reenlist(&self, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32) -> ::windows::core::Result<XACTSTAT>;
    fn ReenlistmentComplete(&self) -> ::windows::core::Result<()>;
    fn GetDistributedTransactionManager(&self, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IResourceManager {}
impl IResourceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IResourceManager_Impl, const OFFSET: isize>() -> IResourceManager_Vtbl {
        unsafe extern "system" fn Enlist<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IResourceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void, pres: *mut ::core::ffi::c_void, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Enlist(::core::mem::transmute(&ptransaction), ::core::mem::transmute(&pres), ::core::mem::transmute_copy(&puow), ::core::mem::transmute_copy(&pisolevel), ::core::mem::transmute_copy(&ppenlist)).into()
        }
        unsafe extern "system" fn Reenlist<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IResourceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Reenlist(::core::mem::transmute_copy(&pprepinfo), ::core::mem::transmute_copy(&cbprepinfo), ::core::mem::transmute_copy(&ltimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pxactstat, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReenlistmentComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IResourceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReenlistmentComplete().into()
        }
        unsafe extern "system" fn GetDistributedTransactionManager<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IResourceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDistributedTransactionManager(::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvobject)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Enlist: Enlist::<Identity, Impl, OFFSET>,
            Reenlist: Reenlist::<Identity, Impl, OFFSET>,
            ReenlistmentComplete: ReenlistmentComplete::<Identity, Impl, OFFSET>,
            GetDistributedTransactionManager: GetDistributedTransactionManager::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IResourceManager2_Impl: Sized + IResourceManager_Impl {
    fn Enlist2(&self, ptransaction: &::core::option::Option<ITransaction>, presasync: &::core::option::Option<ITransactionResourceAsync>, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut xid_t, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows::core::Result<()>;
    fn Reenlist2(&self, pxid: *const xid_t, dwtimeout: u32) -> ::windows::core::Result<XACTSTAT>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IResourceManager2 {}
#[cfg(feature = "Win32_Foundation")]
impl IResourceManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IResourceManager2_Impl, const OFFSET: isize>() -> IResourceManager2_Vtbl {
        unsafe extern "system" fn Enlist2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IResourceManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void, presasync: *mut ::core::ffi::c_void, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut xid_t, ppenlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Enlist2(::core::mem::transmute(&ptransaction), ::core::mem::transmute(&presasync), ::core::mem::transmute_copy(&puow), ::core::mem::transmute_copy(&pisolevel), ::core::mem::transmute_copy(&pxid), ::core::mem::transmute_copy(&ppenlist)).into()
        }
        unsafe extern "system" fn Reenlist2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IResourceManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxid: *const xid_t, dwtimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Reenlist2(::core::mem::transmute_copy(&pxid), ::core::mem::transmute_copy(&dwtimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pxactstat, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IResourceManager_Vtbl::new::<Identity, Impl, OFFSET>(),
            Enlist2: Enlist2::<Identity, Impl, OFFSET>,
            Reenlist2: Reenlist2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceManager2 as ::windows::core::Interface>::IID || iid == &<IResourceManager as ::windows::core::Interface>::IID
    }
}
pub trait IResourceManagerFactory_Impl: Sized {
    fn Create(&self, pguidrm: *const ::windows::core::GUID, pszrmname: &::windows::core::PCSTR, piresmgrsink: &::core::option::Option<IResourceManagerSink>) -> ::windows::core::Result<IResourceManager>;
}
impl ::windows::core::RuntimeName for IResourceManagerFactory {}
impl IResourceManagerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IResourceManagerFactory_Impl, const OFFSET: isize>() -> IResourceManagerFactory_Vtbl {
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IResourceManagerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidrm: *const ::windows::core::GUID, pszrmname: ::windows::core::PCSTR, piresmgrsink: *mut ::core::ffi::c_void, ppresmgr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create(::core::mem::transmute_copy(&pguidrm), ::core::mem::transmute(&pszrmname), ::core::mem::transmute(&piresmgrsink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresmgr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceManagerFactory as ::windows::core::Interface>::IID
    }
}
pub trait IResourceManagerFactory2_Impl: Sized + IResourceManagerFactory_Impl {
    fn CreateEx(&self, pguidrm: *const ::windows::core::GUID, pszrmname: &::windows::core::PCSTR, piresmgrsink: &::core::option::Option<IResourceManagerSink>, riidrequested: *const ::windows::core::GUID, ppvresmgr: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IResourceManagerFactory2 {}
impl IResourceManagerFactory2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IResourceManagerFactory2_Impl, const OFFSET: isize>() -> IResourceManagerFactory2_Vtbl {
        unsafe extern "system" fn CreateEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IResourceManagerFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidrm: *const ::windows::core::GUID, pszrmname: ::windows::core::PCSTR, piresmgrsink: *mut ::core::ffi::c_void, riidrequested: *const ::windows::core::GUID, ppvresmgr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateEx(::core::mem::transmute_copy(&pguidrm), ::core::mem::transmute(&pszrmname), ::core::mem::transmute(&piresmgrsink), ::core::mem::transmute_copy(&riidrequested), ::core::mem::transmute_copy(&ppvresmgr)).into()
        }
        Self { base__: IResourceManagerFactory_Vtbl::new::<Identity, Impl, OFFSET>(), CreateEx: CreateEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceManagerFactory2 as ::windows::core::Interface>::IID || iid == &<IResourceManagerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IResourceManagerRejoinable_Impl: Sized + IResourceManager_Impl + IResourceManager2_Impl {
    fn Rejoin(&self, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32) -> ::windows::core::Result<XACTSTAT>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IResourceManagerRejoinable {}
#[cfg(feature = "Win32_Foundation")]
impl IResourceManagerRejoinable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IResourceManagerRejoinable_Impl, const OFFSET: isize>() -> IResourceManagerRejoinable_Vtbl {
        unsafe extern "system" fn Rejoin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IResourceManagerRejoinable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Rejoin(::core::mem::transmute_copy(&pprepinfo), ::core::mem::transmute_copy(&cbprepinfo), ::core::mem::transmute_copy(&ltimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pxactstat, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IResourceManager2_Vtbl::new::<Identity, Impl, OFFSET>(), Rejoin: Rejoin::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceManagerRejoinable as ::windows::core::Interface>::IID || iid == &<IResourceManager as ::windows::core::Interface>::IID || iid == &<IResourceManager2 as ::windows::core::Interface>::IID
    }
}
pub trait IResourceManagerSink_Impl: Sized {
    fn TMDown(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IResourceManagerSink {}
impl IResourceManagerSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IResourceManagerSink_Impl, const OFFSET: isize>() -> IResourceManagerSink_Vtbl {
        unsafe extern "system" fn TMDown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IResourceManagerSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TMDown().into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), TMDown: TMDown::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceManagerSink as ::windows::core::Interface>::IID
    }
}
pub trait ITipHelper_Impl: Sized {
    fn Pull(&self, i_psztxurl: *const u8) -> ::windows::core::Result<ITransaction>;
    fn PullAsync(&self, i_psztxurl: *const u8, i_ptippullsink: &::core::option::Option<ITipPullSink>) -> ::windows::core::Result<ITransaction>;
    fn GetLocalTmUrl(&self) -> ::windows::core::Result<*mut u8>;
}
impl ::windows::core::RuntimeName for ITipHelper {}
impl ITipHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITipHelper_Impl, const OFFSET: isize>() -> ITipHelper_Vtbl {
        unsafe extern "system" fn Pull<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITipHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i_psztxurl: *const u8, o_ppitransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Pull(::core::mem::transmute_copy(&i_psztxurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(o_ppitransaction, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PullAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITipHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i_psztxurl: *const u8, i_ptippullsink: *mut ::core::ffi::c_void, o_ppitransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PullAsync(::core::mem::transmute_copy(&i_psztxurl), ::core::mem::transmute(&i_ptippullsink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(o_ppitransaction, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalTmUrl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITipHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, o_ppszlocaltmurl: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLocalTmUrl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(o_ppszlocaltmurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Pull: Pull::<Identity, Impl, OFFSET>,
            PullAsync: PullAsync::<Identity, Impl, OFFSET>,
            GetLocalTmUrl: GetLocalTmUrl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITipHelper as ::windows::core::Interface>::IID
    }
}
pub trait ITipPullSink_Impl: Sized {
    fn PullComplete(&self, i_hrpull: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITipPullSink {}
impl ITipPullSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITipPullSink_Impl, const OFFSET: isize>() -> ITipPullSink_Vtbl {
        unsafe extern "system" fn PullComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITipPullSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i_hrpull: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PullComplete(::core::mem::transmute_copy(&i_hrpull)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), PullComplete: PullComplete::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITipPullSink as ::windows::core::Interface>::IID
    }
}
pub trait ITipTransaction_Impl: Sized {
    fn Push(&self, i_pszremotetmurl: *const u8) -> ::windows::core::Result<::windows::core::PSTR>;
    fn GetTransactionUrl(&self) -> ::windows::core::Result<::windows::core::PSTR>;
}
impl ::windows::core::RuntimeName for ITipTransaction {}
impl ITipTransaction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITipTransaction_Impl, const OFFSET: isize>() -> ITipTransaction_Vtbl {
        unsafe extern "system" fn Push<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITipTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i_pszremotetmurl: *const u8, o_ppszremotetxurl: *mut ::windows::core::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Push(::core::mem::transmute_copy(&i_pszremotetmurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(o_ppszremotetxurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransactionUrl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITipTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, o_ppszlocaltxurl: *mut ::windows::core::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransactionUrl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(o_ppszlocaltxurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Push: Push::<Identity, Impl, OFFSET>,
            GetTransactionUrl: GetTransactionUrl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITipTransaction as ::windows::core::Interface>::IID
    }
}
pub trait ITmNodeName_Impl: Sized {
    fn GetNodeNameSize(&self) -> ::windows::core::Result<u32>;
    fn GetNodeName(&self, cbnodenamebuffersize: u32, pnodenamebuffer: &::windows::core::PWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITmNodeName {}
impl ITmNodeName_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITmNodeName_Impl, const OFFSET: isize>() -> ITmNodeName_Vtbl {
        unsafe extern "system" fn GetNodeNameSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITmNodeName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbnodenamesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNodeNameSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbnodenamesize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNodeName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITmNodeName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbnodenamebuffersize: u32, pnodenamebuffer: ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNodeName(::core::mem::transmute_copy(&cbnodenamebuffersize), ::core::mem::transmute(&pnodenamebuffer)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetNodeNameSize: GetNodeNameSize::<Identity, Impl, OFFSET>,
            GetNodeName: GetNodeName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITmNodeName as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITransaction_Impl: Sized {
    fn Commit(&self, fretaining: super::super::Foundation::BOOL, grftc: u32, grfrm: u32) -> ::windows::core::Result<()>;
    fn Abort(&self, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, fasync: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetTransactionInfo(&self) -> ::windows::core::Result<XACTTRANSINFO>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ITransaction {}
#[cfg(feature = "Win32_Foundation")]
impl ITransaction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransaction_Impl, const OFFSET: isize>() -> ITransaction_Vtbl {
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grftc: u32, grfrm: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit(::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&grftc), ::core::mem::transmute_copy(&grfrm)).into()
        }
        unsafe extern "system" fn Abort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, fasync: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort(::core::mem::transmute_copy(&pboidreason), ::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&fasync)).into()
        }
        unsafe extern "system" fn GetTransactionInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut XACTTRANSINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransactionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Commit: Commit::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            GetTransactionInfo: GetTransactionInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransaction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITransaction2_Impl: Sized + ITransaction_Impl + ITransactionCloner_Impl {
    fn GetTransactionInfo2(&self) -> ::windows::core::Result<XACTTRANSINFO>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ITransaction2 {}
#[cfg(feature = "Win32_Foundation")]
impl ITransaction2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransaction2_Impl, const OFFSET: isize>() -> ITransaction2_Vtbl {
        unsafe extern "system" fn GetTransactionInfo2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransaction2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut XACTTRANSINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransactionInfo2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ITransactionCloner_Vtbl::new::<Identity, Impl, OFFSET>(), GetTransactionInfo2: GetTransactionInfo2::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransaction2 as ::windows::core::Interface>::IID || iid == &<ITransaction as ::windows::core::Interface>::IID || iid == &<ITransactionCloner as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITransactionCloner_Impl: Sized + ITransaction_Impl {
    fn CloneWithCommitDisabled(&self) -> ::windows::core::Result<ITransaction>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ITransactionCloner {}
#[cfg(feature = "Win32_Foundation")]
impl ITransactionCloner_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionCloner_Impl, const OFFSET: isize>() -> ITransactionCloner_Vtbl {
        unsafe extern "system" fn CloneWithCommitDisabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionCloner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CloneWithCommitDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitransaction, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ITransaction_Vtbl::new::<Identity, Impl, OFFSET>(), CloneWithCommitDisabled: CloneWithCommitDisabled::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionCloner as ::windows::core::Interface>::IID || iid == &<ITransaction as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionDispenser_Impl: Sized {
    fn GetOptionsObject(&self) -> ::windows::core::Result<ITransactionOptions>;
    fn BeginTransaction(&self, punkouter: &::core::option::Option<::windows::core::IUnknown>, isolevel: i32, isoflags: u32, poptions: &::core::option::Option<ITransactionOptions>) -> ::windows::core::Result<ITransaction>;
}
impl ::windows::core::RuntimeName for ITransactionDispenser {}
impl ITransactionDispenser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionDispenser_Impl, const OFFSET: isize>() -> ITransactionDispenser_Vtbl {
        unsafe extern "system" fn GetOptionsObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionDispenser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoptions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOptionsObject() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoptions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionDispenser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, poptions: *mut ::core::ffi::c_void, pptransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginTransaction(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&isolevel), ::core::mem::transmute_copy(&isoflags), ::core::mem::transmute(&poptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptransaction, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetOptionsObject: GetOptionsObject::<Identity, Impl, OFFSET>,
            BeginTransaction: BeginTransaction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionDispenser as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITransactionEnlistmentAsync_Impl: Sized {
    fn PrepareRequestDone(&self, hr: ::windows::core::HRESULT, pmk: &::core::option::Option<super::Com::IMoniker>, pboidreason: *const BOID) -> ::windows::core::Result<()>;
    fn CommitRequestDone(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn AbortRequestDone(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITransactionEnlistmentAsync {}
#[cfg(feature = "Win32_System_Com")]
impl ITransactionEnlistmentAsync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionEnlistmentAsync_Impl, const OFFSET: isize>() -> ITransactionEnlistmentAsync_Vtbl {
        unsafe extern "system" fn PrepareRequestDone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionEnlistmentAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, pmk: *mut ::core::ffi::c_void, pboidreason: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrepareRequestDone(::core::mem::transmute_copy(&hr), ::core::mem::transmute(&pmk), ::core::mem::transmute_copy(&pboidreason)).into()
        }
        unsafe extern "system" fn CommitRequestDone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionEnlistmentAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CommitRequestDone(::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn AbortRequestDone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionEnlistmentAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AbortRequestDone(::core::mem::transmute_copy(&hr)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            PrepareRequestDone: PrepareRequestDone::<Identity, Impl, OFFSET>,
            CommitRequestDone: CommitRequestDone::<Identity, Impl, OFFSET>,
            AbortRequestDone: AbortRequestDone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionEnlistmentAsync as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionExport_Impl: Sized {
    fn Export(&self, punktransaction: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<u32>;
    fn GetTransactionCookie(&self, punktransaction: &::core::option::Option<::windows::core::IUnknown>, cbtransactioncookie: u32, rgbtransactioncookie: *mut u8, pcbused: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITransactionExport {}
impl ITransactionExport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionExport_Impl, const OFFSET: isize>() -> ITransactionExport_Vtbl {
        unsafe extern "system" fn Export<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionExport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punktransaction: *mut ::core::ffi::c_void, pcbtransactioncookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Export(::core::mem::transmute(&punktransaction)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbtransactioncookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransactionCookie<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionExport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punktransaction: *mut ::core::ffi::c_void, cbtransactioncookie: u32, rgbtransactioncookie: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTransactionCookie(::core::mem::transmute(&punktransaction), ::core::mem::transmute_copy(&cbtransactioncookie), ::core::mem::transmute_copy(&rgbtransactioncookie), ::core::mem::transmute_copy(&pcbused)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Export: Export::<Identity, Impl, OFFSET>,
            GetTransactionCookie: GetTransactionCookie::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionExport as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionExportFactory_Impl: Sized {
    fn GetRemoteClassId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Create(&self, cbwhereabouts: u32, rgbwhereabouts: *const u8) -> ::windows::core::Result<ITransactionExport>;
}
impl ::windows::core::RuntimeName for ITransactionExportFactory {}
impl ITransactionExportFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionExportFactory_Impl, const OFFSET: isize>() -> ITransactionExportFactory_Vtbl {
        unsafe extern "system" fn GetRemoteClassId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionExportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRemoteClassId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionExportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbwhereabouts: u32, rgbwhereabouts: *const u8, ppexport: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create(::core::mem::transmute_copy(&cbwhereabouts), ::core::mem::transmute_copy(&rgbwhereabouts)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppexport, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetRemoteClassId: GetRemoteClassId::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionExportFactory as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionImport_Impl: Sized {
    fn Import(&self, cbtransactioncookie: u32, rgbtransactioncookie: *const u8, piid: *const ::windows::core::GUID, ppvtransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITransactionImport {}
impl ITransactionImport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionImport_Impl, const OFFSET: isize>() -> ITransactionImport_Vtbl {
        unsafe extern "system" fn Import<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionImport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbtransactioncookie: u32, rgbtransactioncookie: *const u8, piid: *const ::windows::core::GUID, ppvtransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Import(::core::mem::transmute_copy(&cbtransactioncookie), ::core::mem::transmute_copy(&rgbtransactioncookie), ::core::mem::transmute_copy(&piid), ::core::mem::transmute_copy(&ppvtransaction)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Import: Import::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionImport as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionImportWhereabouts_Impl: Sized {
    fn GetWhereaboutsSize(&self) -> ::windows::core::Result<u32>;
    fn GetWhereabouts(&self, cbwhereabouts: u32, rgbwhereabouts: *mut u8, pcbused: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITransactionImportWhereabouts {}
impl ITransactionImportWhereabouts_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionImportWhereabouts_Impl, const OFFSET: isize>() -> ITransactionImportWhereabouts_Vtbl {
        unsafe extern "system" fn GetWhereaboutsSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionImportWhereabouts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbwhereabouts: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWhereaboutsSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbwhereabouts, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWhereabouts<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionImportWhereabouts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbwhereabouts: u32, rgbwhereabouts: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWhereabouts(::core::mem::transmute_copy(&cbwhereabouts), ::core::mem::transmute_copy(&rgbwhereabouts), ::core::mem::transmute_copy(&pcbused)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetWhereaboutsSize: GetWhereaboutsSize::<Identity, Impl, OFFSET>,
            GetWhereabouts: GetWhereabouts::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionImportWhereabouts as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionLastEnlistmentAsync_Impl: Sized {
    fn TransactionOutcome(&self, xactstat: XACTSTAT, pboidreason: *const BOID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITransactionLastEnlistmentAsync {}
impl ITransactionLastEnlistmentAsync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionLastEnlistmentAsync_Impl, const OFFSET: isize>() -> ITransactionLastEnlistmentAsync_Vtbl {
        unsafe extern "system" fn TransactionOutcome<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionLastEnlistmentAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xactstat: XACTSTAT, pboidreason: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TransactionOutcome(::core::mem::transmute_copy(&xactstat), ::core::mem::transmute_copy(&pboidreason)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), TransactionOutcome: TransactionOutcome::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionLastEnlistmentAsync as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionLastResourceAsync_Impl: Sized {
    fn DelegateCommit(&self, grfrm: u32) -> ::windows::core::Result<()>;
    fn ForgetRequest(&self, pnewuow: *const BOID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITransactionLastResourceAsync {}
impl ITransactionLastResourceAsync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionLastResourceAsync_Impl, const OFFSET: isize>() -> ITransactionLastResourceAsync_Vtbl {
        unsafe extern "system" fn DelegateCommit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionLastResourceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfrm: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DelegateCommit(::core::mem::transmute_copy(&grfrm)).into()
        }
        unsafe extern "system" fn ForgetRequest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionLastResourceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewuow: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ForgetRequest(::core::mem::transmute_copy(&pnewuow)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            DelegateCommit: DelegateCommit::<Identity, Impl, OFFSET>,
            ForgetRequest: ForgetRequest::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionLastResourceAsync as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionOptions_Impl: Sized {
    fn SetOptions(&self, poptions: *const XACTOPT) -> ::windows::core::Result<()>;
    fn GetOptions(&self, poptions: *mut XACTOPT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITransactionOptions {}
impl ITransactionOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionOptions_Impl, const OFFSET: isize>() -> ITransactionOptions_Vtbl {
        unsafe extern "system" fn SetOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poptions: *const XACTOPT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOptions(::core::mem::transmute_copy(&poptions)).into()
        }
        unsafe extern "system" fn GetOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poptions: *mut XACTOPT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOptions(::core::mem::transmute_copy(&poptions)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetOptions: SetOptions::<Identity, Impl, OFFSET>,
            GetOptions: GetOptions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITransactionOutcomeEvents_Impl: Sized {
    fn Committed(&self, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn Aborted(&self, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn HeuristicDecision(&self, dwdecision: u32, pboidreason: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn Indoubt(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ITransactionOutcomeEvents {}
#[cfg(feature = "Win32_Foundation")]
impl ITransactionOutcomeEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionOutcomeEvents_Impl, const OFFSET: isize>() -> ITransactionOutcomeEvents_Vtbl {
        unsafe extern "system" fn Committed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionOutcomeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Committed(::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&pnewuow), ::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn Aborted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionOutcomeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Aborted(::core::mem::transmute_copy(&pboidreason), ::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&pnewuow), ::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn HeuristicDecision<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionOutcomeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdecision: u32, pboidreason: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HeuristicDecision(::core::mem::transmute_copy(&dwdecision), ::core::mem::transmute_copy(&pboidreason), ::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn Indoubt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionOutcomeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Indoubt().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Committed: Committed::<Identity, Impl, OFFSET>,
            Aborted: Aborted::<Identity, Impl, OFFSET>,
            HeuristicDecision: HeuristicDecision::<Identity, Impl, OFFSET>,
            Indoubt: Indoubt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionOutcomeEvents as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionPhase0EnlistmentAsync_Impl: Sized {
    fn Enable(&self) -> ::windows::core::Result<()>;
    fn WaitForEnlistment(&self) -> ::windows::core::Result<()>;
    fn Phase0Done(&self) -> ::windows::core::Result<()>;
    fn Unenlist(&self) -> ::windows::core::Result<()>;
    fn GetTransaction(&self) -> ::windows::core::Result<ITransaction>;
}
impl ::windows::core::RuntimeName for ITransactionPhase0EnlistmentAsync {}
impl ITransactionPhase0EnlistmentAsync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: isize>() -> ITransactionPhase0EnlistmentAsync_Vtbl {
        unsafe extern "system" fn Enable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Enable().into()
        }
        unsafe extern "system" fn WaitForEnlistment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WaitForEnlistment().into()
        }
        unsafe extern "system" fn Phase0Done<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Phase0Done().into()
        }
        unsafe extern "system" fn Unenlist<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unenlist().into()
        }
        unsafe extern "system" fn GetTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitransaction, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Enable: Enable::<Identity, Impl, OFFSET>,
            WaitForEnlistment: WaitForEnlistment::<Identity, Impl, OFFSET>,
            Phase0Done: Phase0Done::<Identity, Impl, OFFSET>,
            Unenlist: Unenlist::<Identity, Impl, OFFSET>,
            GetTransaction: GetTransaction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionPhase0EnlistmentAsync as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionPhase0Factory_Impl: Sized {
    fn Create(&self, pphase0notify: &::core::option::Option<ITransactionPhase0NotifyAsync>) -> ::windows::core::Result<ITransactionPhase0EnlistmentAsync>;
}
impl ::windows::core::RuntimeName for ITransactionPhase0Factory {}
impl ITransactionPhase0Factory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionPhase0Factory_Impl, const OFFSET: isize>() -> ITransactionPhase0Factory_Vtbl {
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionPhase0Factory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphase0notify: *mut ::core::ffi::c_void, ppphase0enlistment: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create(::core::mem::transmute(&pphase0notify)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppphase0enlistment, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionPhase0Factory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITransactionPhase0NotifyAsync_Impl: Sized {
    fn Phase0Request(&self, fabortinghint: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn EnlistCompleted(&self, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ITransactionPhase0NotifyAsync {}
#[cfg(feature = "Win32_Foundation")]
impl ITransactionPhase0NotifyAsync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionPhase0NotifyAsync_Impl, const OFFSET: isize>() -> ITransactionPhase0NotifyAsync_Vtbl {
        unsafe extern "system" fn Phase0Request<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionPhase0NotifyAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fabortinghint: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Phase0Request(::core::mem::transmute_copy(&fabortinghint)).into()
        }
        unsafe extern "system" fn EnlistCompleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionPhase0NotifyAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnlistCompleted(::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Phase0Request: Phase0Request::<Identity, Impl, OFFSET>,
            EnlistCompleted: EnlistCompleted::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionPhase0NotifyAsync as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionReceiver_Impl: Sized {
    fn UnmarshalPropagationToken(&self, cbtoken: u32, rgbtoken: *const u8) -> ::windows::core::Result<ITransaction>;
    fn GetReturnTokenSize(&self) -> ::windows::core::Result<u32>;
    fn MarshalReturnToken(&self, cbreturntoken: u32, rgbreturntoken: *mut u8, pcbused: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITransactionReceiver {}
impl ITransactionReceiver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionReceiver_Impl, const OFFSET: isize>() -> ITransactionReceiver_Vtbl {
        unsafe extern "system" fn UnmarshalPropagationToken<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbtoken: u32, rgbtoken: *const u8, pptransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UnmarshalPropagationToken(::core::mem::transmute_copy(&cbtoken), ::core::mem::transmute_copy(&rgbtoken)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptransaction, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReturnTokenSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbreturntoken: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReturnTokenSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbreturntoken, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarshalReturnToken<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbreturntoken: u32, rgbreturntoken: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MarshalReturnToken(::core::mem::transmute_copy(&cbreturntoken), ::core::mem::transmute_copy(&rgbreturntoken), ::core::mem::transmute_copy(&pcbused)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            UnmarshalPropagationToken: UnmarshalPropagationToken::<Identity, Impl, OFFSET>,
            GetReturnTokenSize: GetReturnTokenSize::<Identity, Impl, OFFSET>,
            MarshalReturnToken: MarshalReturnToken::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionReceiver as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionReceiverFactory_Impl: Sized {
    fn Create(&self) -> ::windows::core::Result<ITransactionReceiver>;
}
impl ::windows::core::RuntimeName for ITransactionReceiverFactory {}
impl ITransactionReceiverFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionReceiverFactory_Impl, const OFFSET: isize>() -> ITransactionReceiverFactory_Vtbl {
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionReceiverFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppreceiver: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreceiver, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionReceiverFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITransactionResource_Impl: Sized {
    fn PrepareRequest(&self, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CommitRequest(&self, grfrm: u32, pnewuow: *const BOID) -> ::windows::core::Result<()>;
    fn AbortRequest(&self, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows::core::Result<()>;
    fn TMDown(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ITransactionResource {}
#[cfg(feature = "Win32_Foundation")]
impl ITransactionResource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionResource_Impl, const OFFSET: isize>() -> ITransactionResource_Vtbl {
        unsafe extern "system" fn PrepareRequest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrepareRequest(::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&grfrm), ::core::mem::transmute_copy(&fwantmoniker), ::core::mem::transmute_copy(&fsinglephase)).into()
        }
        unsafe extern "system" fn CommitRequest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfrm: u32, pnewuow: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CommitRequest(::core::mem::transmute_copy(&grfrm), ::core::mem::transmute_copy(&pnewuow)).into()
        }
        unsafe extern "system" fn AbortRequest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AbortRequest(::core::mem::transmute_copy(&pboidreason), ::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&pnewuow)).into()
        }
        unsafe extern "system" fn TMDown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TMDown().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            PrepareRequest: PrepareRequest::<Identity, Impl, OFFSET>,
            CommitRequest: CommitRequest::<Identity, Impl, OFFSET>,
            AbortRequest: AbortRequest::<Identity, Impl, OFFSET>,
            TMDown: TMDown::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionResource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITransactionResourceAsync_Impl: Sized {
    fn PrepareRequest(&self, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CommitRequest(&self, grfrm: u32, pnewuow: *const BOID) -> ::windows::core::Result<()>;
    fn AbortRequest(&self, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows::core::Result<()>;
    fn TMDown(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ITransactionResourceAsync {}
#[cfg(feature = "Win32_Foundation")]
impl ITransactionResourceAsync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionResourceAsync_Impl, const OFFSET: isize>() -> ITransactionResourceAsync_Vtbl {
        unsafe extern "system" fn PrepareRequest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionResourceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrepareRequest(::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&grfrm), ::core::mem::transmute_copy(&fwantmoniker), ::core::mem::transmute_copy(&fsinglephase)).into()
        }
        unsafe extern "system" fn CommitRequest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionResourceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfrm: u32, pnewuow: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CommitRequest(::core::mem::transmute_copy(&grfrm), ::core::mem::transmute_copy(&pnewuow)).into()
        }
        unsafe extern "system" fn AbortRequest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionResourceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AbortRequest(::core::mem::transmute_copy(&pboidreason), ::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&pnewuow)).into()
        }
        unsafe extern "system" fn TMDown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionResourceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TMDown().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            PrepareRequest: PrepareRequest::<Identity, Impl, OFFSET>,
            CommitRequest: CommitRequest::<Identity, Impl, OFFSET>,
            AbortRequest: AbortRequest::<Identity, Impl, OFFSET>,
            TMDown: TMDown::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionResourceAsync as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionTransmitter_Impl: Sized {
    fn Set(&self, ptransaction: &::core::option::Option<ITransaction>) -> ::windows::core::Result<()>;
    fn GetPropagationTokenSize(&self) -> ::windows::core::Result<u32>;
    fn MarshalPropagationToken(&self, cbtoken: u32, rgbtoken: *mut u8, pcbused: *mut u32) -> ::windows::core::Result<()>;
    fn UnmarshalReturnToken(&self, cbreturntoken: u32, rgbreturntoken: *const u8) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITransactionTransmitter {}
impl ITransactionTransmitter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionTransmitter_Impl, const OFFSET: isize>() -> ITransactionTransmitter_Vtbl {
        unsafe extern "system" fn Set<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionTransmitter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Set(::core::mem::transmute(&ptransaction)).into()
        }
        unsafe extern "system" fn GetPropagationTokenSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionTransmitter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbtoken: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropagationTokenSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbtoken, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarshalPropagationToken<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionTransmitter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbtoken: u32, rgbtoken: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MarshalPropagationToken(::core::mem::transmute_copy(&cbtoken), ::core::mem::transmute_copy(&rgbtoken), ::core::mem::transmute_copy(&pcbused)).into()
        }
        unsafe extern "system" fn UnmarshalReturnToken<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionTransmitter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbreturntoken: u32, rgbreturntoken: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnmarshalReturnToken(::core::mem::transmute_copy(&cbreturntoken), ::core::mem::transmute_copy(&rgbreturntoken)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionTransmitter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Set: Set::<Identity, Impl, OFFSET>,
            GetPropagationTokenSize: GetPropagationTokenSize::<Identity, Impl, OFFSET>,
            MarshalPropagationToken: MarshalPropagationToken::<Identity, Impl, OFFSET>,
            UnmarshalReturnToken: UnmarshalReturnToken::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionTransmitter as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionTransmitterFactory_Impl: Sized {
    fn Create(&self) -> ::windows::core::Result<ITransactionTransmitter>;
}
impl ::windows::core::RuntimeName for ITransactionTransmitterFactory {}
impl ITransactionTransmitterFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionTransmitterFactory_Impl, const OFFSET: isize>() -> ITransactionTransmitterFactory_Vtbl {
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionTransmitterFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptransmitter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptransmitter, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionTransmitterFactory as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionVoterBallotAsync2_Impl: Sized {
    fn VoteRequestDone(&self, hr: ::windows::core::HRESULT, pboidreason: *const BOID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITransactionVoterBallotAsync2 {}
impl ITransactionVoterBallotAsync2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionVoterBallotAsync2_Impl, const OFFSET: isize>() -> ITransactionVoterBallotAsync2_Vtbl {
        unsafe extern "system" fn VoteRequestDone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionVoterBallotAsync2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, pboidreason: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.VoteRequestDone(::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&pboidreason)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), VoteRequestDone: VoteRequestDone::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionVoterBallotAsync2 as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionVoterFactory2_Impl: Sized {
    fn Create(&self, ptransaction: &::core::option::Option<ITransaction>, pvoternotify: &::core::option::Option<ITransactionVoterNotifyAsync2>) -> ::windows::core::Result<ITransactionVoterBallotAsync2>;
}
impl ::windows::core::RuntimeName for ITransactionVoterFactory2 {}
impl ITransactionVoterFactory2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionVoterFactory2_Impl, const OFFSET: isize>() -> ITransactionVoterFactory2_Vtbl {
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionVoterFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void, pvoternotify: *mut ::core::ffi::c_void, ppvoterballot: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create(::core::mem::transmute(&ptransaction), ::core::mem::transmute(&pvoternotify)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvoterballot, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionVoterFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITransactionVoterNotifyAsync2_Impl: Sized + ITransactionOutcomeEvents_Impl {
    fn VoteRequest(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ITransactionVoterNotifyAsync2 {}
#[cfg(feature = "Win32_Foundation")]
impl ITransactionVoterNotifyAsync2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionVoterNotifyAsync2_Impl, const OFFSET: isize>() -> ITransactionVoterNotifyAsync2_Vtbl {
        unsafe extern "system" fn VoteRequest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionVoterNotifyAsync2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.VoteRequest().into()
        }
        Self { base__: ITransactionOutcomeEvents_Vtbl::new::<Identity, Impl, OFFSET>(), VoteRequest: VoteRequest::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionVoterNotifyAsync2 as ::windows::core::Interface>::IID || iid == &<ITransactionOutcomeEvents as ::windows::core::Interface>::IID
    }
}
pub trait IXAConfig_Impl: Sized {
    fn Initialize(&self, clsidhelperdll: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Terminate(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IXAConfig {}
impl IXAConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAConfig_Impl, const OFFSET: isize>() -> IXAConfig_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidhelperdll: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute(&clsidhelperdll)).into()
        }
        unsafe extern "system" fn Terminate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Terminate().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAConfig as ::windows::core::Interface>::IID
    }
}
pub trait IXAObtainRMInfo_Impl: Sized {
    fn ObtainRMInfo(&self, pirmhelper: &::core::option::Option<IRMHelper>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IXAObtainRMInfo {}
impl IXAObtainRMInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAObtainRMInfo_Impl, const OFFSET: isize>() -> IXAObtainRMInfo_Vtbl {
        unsafe extern "system" fn ObtainRMInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAObtainRMInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirmhelper: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ObtainRMInfo(::core::mem::transmute(&pirmhelper)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ObtainRMInfo: ObtainRMInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAObtainRMInfo as ::windows::core::Interface>::IID
    }
}
pub trait IXATransLookup_Impl: Sized {
    fn Lookup(&self) -> ::windows::core::Result<ITransaction>;
}
impl ::windows::core::RuntimeName for IXATransLookup {}
impl IXATransLookup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXATransLookup_Impl, const OFFSET: isize>() -> IXATransLookup_Vtbl {
        unsafe extern "system" fn Lookup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXATransLookup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Lookup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptransaction, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Lookup: Lookup::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXATransLookup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXATransLookup2_Impl: Sized {
    fn Lookup(&self, pxid: *const xid_t) -> ::windows::core::Result<ITransaction>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IXATransLookup2 {}
#[cfg(feature = "Win32_Foundation")]
impl IXATransLookup2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXATransLookup2_Impl, const OFFSET: isize>() -> IXATransLookup2_Vtbl {
        unsafe extern "system" fn Lookup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXATransLookup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxid: *const xid_t, pptransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Lookup(::core::mem::transmute_copy(&pxid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptransaction, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Lookup: Lookup::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXATransLookup2 as ::windows::core::Interface>::IID
    }
}
