pub trait IDtcLuConfigure_Impl: Sized {
    fn Add(&mut self, puclupair: *const u8, cblupair: u32) -> ::windows::core::Result<()>;
    fn Delete(&mut self, puclupair: *const u8, cblupair: u32) -> ::windows::core::Result<()>;
}
impl IDtcLuConfigure_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuConfigure_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuConfigure_Vtbl {
        unsafe extern "system" fn Add<Impl: IDtcLuConfigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puclupair: *const u8, cblupair: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&puclupair), ::core::mem::transmute_copy(&cblupair)).into()
        }
        unsafe extern "system" fn Delete<Impl: IDtcLuConfigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puclupair: *const u8, cblupair: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&puclupair), ::core::mem::transmute_copy(&cblupair)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Add: Add::<Impl, IMPL_OFFSET>, Delete: Delete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuConfigure as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuRecovery_Impl: Sized {}
impl IDtcLuRecovery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuRecovery_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuRecovery_Vtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRecovery as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuRecoveryFactory_Impl: Sized {
    fn Create(&mut self, puclupair: *const u8, cblupair: u32) -> ::windows::core::Result<IDtcLuRecovery>;
}
impl IDtcLuRecoveryFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuRecoveryFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuRecoveryFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IDtcLuRecoveryFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puclupair: *const u8, cblupair: u32, pprecovery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(::core::mem::transmute_copy(&puclupair), ::core::mem::transmute_copy(&cblupair)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprecovery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRecoveryFactory as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuRecoveryInitiatedByDtc_Impl: Sized {
    fn GetWork(&mut self, pwork: *mut _DtcLu_LocalRecovery_Work, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IDtcLuRecoveryInitiatedByDtc_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuRecoveryInitiatedByDtc_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuRecoveryInitiatedByDtc_Vtbl {
        unsafe extern "system" fn GetWork<Impl: IDtcLuRecoveryInitiatedByDtc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwork: *mut _DtcLu_LocalRecovery_Work, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWork(::core::mem::transmute_copy(&pwork), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetWork: GetWork::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByDtc as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuRecoveryInitiatedByDtcStatusWork_Impl: Sized {
    fn HandleCheckLuStatus(&mut self, lrecoveryseqnum: i32) -> ::windows::core::Result<()>;
}
impl IDtcLuRecoveryInitiatedByDtcStatusWork_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuRecoveryInitiatedByDtcStatusWork_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuRecoveryInitiatedByDtcStatusWork_Vtbl {
        unsafe extern "system" fn HandleCheckLuStatus<Impl: IDtcLuRecoveryInitiatedByDtcStatusWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrecoveryseqnum: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandleCheckLuStatus(::core::mem::transmute_copy(&lrecoveryseqnum)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), HandleCheckLuStatus: HandleCheckLuStatus::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByDtcStatusWork as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcLuRecoveryInitiatedByDtcTransWork_Impl: Sized {
    fn GetLogNameSizes(&mut self, pcbourlogname: *mut u32, pcbremotelogname: *mut u32) -> ::windows::core::Result<()>;
    fn GetOurXln(&mut self, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::core::Result<()>;
    fn HandleConfirmationFromOurXln(&mut self, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::core::Result<()>;
    fn HandleTheirXlnResponse(&mut self, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, dwprotocol: u32, pconfirmation: *mut _DtcLu_Xln_Confirmation) -> ::windows::core::Result<()>;
    fn HandleErrorFromOurXln(&mut self, error: _DtcLu_Xln_Error) -> ::windows::core::Result<()>;
    fn CheckForCompareStates(&mut self, fcomparestates: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetOurTransIdSize(&mut self, pcbourtransid: *mut u32) -> ::windows::core::Result<()>;
    fn GetOurCompareStates(&mut self, pourtransid: *mut u8, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::core::Result<()>;
    fn HandleTheirCompareStatesResponse(&mut self, comparestate: _DtcLu_CompareState, pconfirmation: *mut _DtcLu_CompareStates_Confirmation) -> ::windows::core::Result<()>;
    fn HandleErrorFromOurCompareStates(&mut self, error: _DtcLu_CompareStates_Error) -> ::windows::core::Result<()>;
    fn ConversationLost(&mut self) -> ::windows::core::Result<()>;
    fn GetRecoverySeqNum(&mut self, plrecoveryseqnum: *mut i32) -> ::windows::core::Result<()>;
    fn ObsoleteRecoverySeqNum(&mut self, lnewrecoveryseqnum: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDtcLuRecoveryInitiatedByDtcTransWork_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuRecoveryInitiatedByDtcTransWork_Vtbl {
        unsafe extern "system" fn GetLogNameSizes<Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbourlogname: *mut u32, pcbremotelogname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLogNameSizes(::core::mem::transmute_copy(&pcbourlogname), ::core::mem::transmute_copy(&pcbremotelogname)).into()
        }
        unsafe extern "system" fn GetOurXln<Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOurXln(::core::mem::transmute_copy(&pxln), ::core::mem::transmute_copy(&pourlogname), ::core::mem::transmute_copy(&premotelogname), ::core::mem::transmute_copy(&pdwprotocol)).into()
        }
        unsafe extern "system" fn HandleConfirmationFromOurXln<Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandleConfirmationFromOurXln(::core::mem::transmute_copy(&confirmation)).into()
        }
        unsafe extern "system" fn HandleTheirXlnResponse<Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, dwprotocol: u32, pconfirmation: *mut _DtcLu_Xln_Confirmation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandleTheirXlnResponse(::core::mem::transmute_copy(&xln), ::core::mem::transmute_copy(&premotelogname), ::core::mem::transmute_copy(&cbremotelogname), ::core::mem::transmute_copy(&dwprotocol), ::core::mem::transmute_copy(&pconfirmation)).into()
        }
        unsafe extern "system" fn HandleErrorFromOurXln<Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: _DtcLu_Xln_Error) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandleErrorFromOurXln(::core::mem::transmute_copy(&error)).into()
        }
        unsafe extern "system" fn CheckForCompareStates<Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcomparestates: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckForCompareStates(::core::mem::transmute_copy(&fcomparestates)).into()
        }
        unsafe extern "system" fn GetOurTransIdSize<Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbourtransid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOurTransIdSize(::core::mem::transmute_copy(&pcbourtransid)).into()
        }
        unsafe extern "system" fn GetOurCompareStates<Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pourtransid: *mut u8, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOurCompareStates(::core::mem::transmute_copy(&pourtransid), ::core::mem::transmute_copy(&pcomparestate)).into()
        }
        unsafe extern "system" fn HandleTheirCompareStatesResponse<Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparestate: _DtcLu_CompareState, pconfirmation: *mut _DtcLu_CompareStates_Confirmation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandleTheirCompareStatesResponse(::core::mem::transmute_copy(&comparestate), ::core::mem::transmute_copy(&pconfirmation)).into()
        }
        unsafe extern "system" fn HandleErrorFromOurCompareStates<Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: _DtcLu_CompareStates_Error) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandleErrorFromOurCompareStates(::core::mem::transmute_copy(&error)).into()
        }
        unsafe extern "system" fn ConversationLost<Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConversationLost().into()
        }
        unsafe extern "system" fn GetRecoverySeqNum<Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plrecoveryseqnum: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRecoverySeqNum(::core::mem::transmute_copy(&plrecoveryseqnum)).into()
        }
        unsafe extern "system" fn ObsoleteRecoverySeqNum<Impl: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnewrecoveryseqnum: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ObsoleteRecoverySeqNum(::core::mem::transmute_copy(&lnewrecoveryseqnum)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetLogNameSizes: GetLogNameSizes::<Impl, IMPL_OFFSET>,
            GetOurXln: GetOurXln::<Impl, IMPL_OFFSET>,
            HandleConfirmationFromOurXln: HandleConfirmationFromOurXln::<Impl, IMPL_OFFSET>,
            HandleTheirXlnResponse: HandleTheirXlnResponse::<Impl, IMPL_OFFSET>,
            HandleErrorFromOurXln: HandleErrorFromOurXln::<Impl, IMPL_OFFSET>,
            CheckForCompareStates: CheckForCompareStates::<Impl, IMPL_OFFSET>,
            GetOurTransIdSize: GetOurTransIdSize::<Impl, IMPL_OFFSET>,
            GetOurCompareStates: GetOurCompareStates::<Impl, IMPL_OFFSET>,
            HandleTheirCompareStatesResponse: HandleTheirCompareStatesResponse::<Impl, IMPL_OFFSET>,
            HandleErrorFromOurCompareStates: HandleErrorFromOurCompareStates::<Impl, IMPL_OFFSET>,
            ConversationLost: ConversationLost::<Impl, IMPL_OFFSET>,
            GetRecoverySeqNum: GetRecoverySeqNum::<Impl, IMPL_OFFSET>,
            ObsoleteRecoverySeqNum: ObsoleteRecoverySeqNum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByDtcTransWork as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuRecoveryInitiatedByLu_Impl: Sized {
    fn GetObjectToHandleWorkFromLu(&mut self) -> ::windows::core::Result<IDtcLuRecoveryInitiatedByLuWork>;
}
impl IDtcLuRecoveryInitiatedByLu_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuRecoveryInitiatedByLu_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuRecoveryInitiatedByLu_Vtbl {
        unsafe extern "system" fn GetObjectToHandleWorkFromLu<Impl: IDtcLuRecoveryInitiatedByLu_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectToHandleWorkFromLu() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwork = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetObjectToHandleWorkFromLu: GetObjectToHandleWorkFromLu::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByLu as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuRecoveryInitiatedByLuWork_Impl: Sized {
    fn HandleTheirXln(&mut self, lrecoveryseqnum: i32, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, pourlogname: *mut u8, cbourlogname: u32, dwprotocol: u32, presponse: *mut _DtcLu_Xln_Response) -> ::windows::core::Result<()>;
    fn GetOurLogNameSize(&mut self, pcbourlogname: *mut u32) -> ::windows::core::Result<()>;
    fn GetOurXln(&mut self, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::core::Result<()>;
    fn HandleConfirmationOfOurXln(&mut self, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::core::Result<()>;
    fn HandleTheirCompareStates(&mut self, premotetransid: *mut u8, cbremotetransid: u32, comparestate: _DtcLu_CompareState, presponse: *mut _DtcLu_CompareStates_Response, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::core::Result<()>;
    fn HandleConfirmationOfOurCompareStates(&mut self, confirmation: _DtcLu_CompareStates_Confirmation) -> ::windows::core::Result<()>;
    fn HandleErrorFromOurCompareStates(&mut self, error: _DtcLu_CompareStates_Error) -> ::windows::core::Result<()>;
    fn ConversationLost(&mut self) -> ::windows::core::Result<()>;
}
impl IDtcLuRecoveryInitiatedByLuWork_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuRecoveryInitiatedByLuWork_Vtbl {
        unsafe extern "system" fn HandleTheirXln<Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrecoveryseqnum: i32, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, pourlogname: *mut u8, cbourlogname: u32, dwprotocol: u32, presponse: *mut _DtcLu_Xln_Response) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandleTheirXln(::core::mem::transmute_copy(&lrecoveryseqnum), ::core::mem::transmute_copy(&xln), ::core::mem::transmute_copy(&premotelogname), ::core::mem::transmute_copy(&cbremotelogname), ::core::mem::transmute_copy(&pourlogname), ::core::mem::transmute_copy(&cbourlogname), ::core::mem::transmute_copy(&dwprotocol), ::core::mem::transmute_copy(&presponse)).into()
        }
        unsafe extern "system" fn GetOurLogNameSize<Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbourlogname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOurLogNameSize(::core::mem::transmute_copy(&pcbourlogname)).into()
        }
        unsafe extern "system" fn GetOurXln<Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOurXln(::core::mem::transmute_copy(&pxln), ::core::mem::transmute_copy(&pourlogname), ::core::mem::transmute_copy(&pdwprotocol)).into()
        }
        unsafe extern "system" fn HandleConfirmationOfOurXln<Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandleConfirmationOfOurXln(::core::mem::transmute_copy(&confirmation)).into()
        }
        unsafe extern "system" fn HandleTheirCompareStates<Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, premotetransid: *mut u8, cbremotetransid: u32, comparestate: _DtcLu_CompareState, presponse: *mut _DtcLu_CompareStates_Response, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandleTheirCompareStates(::core::mem::transmute_copy(&premotetransid), ::core::mem::transmute_copy(&cbremotetransid), ::core::mem::transmute_copy(&comparestate), ::core::mem::transmute_copy(&presponse), ::core::mem::transmute_copy(&pcomparestate)).into()
        }
        unsafe extern "system" fn HandleConfirmationOfOurCompareStates<Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confirmation: _DtcLu_CompareStates_Confirmation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandleConfirmationOfOurCompareStates(::core::mem::transmute_copy(&confirmation)).into()
        }
        unsafe extern "system" fn HandleErrorFromOurCompareStates<Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: _DtcLu_CompareStates_Error) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandleErrorFromOurCompareStates(::core::mem::transmute_copy(&error)).into()
        }
        unsafe extern "system" fn ConversationLost<Impl: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConversationLost().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            HandleTheirXln: HandleTheirXln::<Impl, IMPL_OFFSET>,
            GetOurLogNameSize: GetOurLogNameSize::<Impl, IMPL_OFFSET>,
            GetOurXln: GetOurXln::<Impl, IMPL_OFFSET>,
            HandleConfirmationOfOurXln: HandleConfirmationOfOurXln::<Impl, IMPL_OFFSET>,
            HandleTheirCompareStates: HandleTheirCompareStates::<Impl, IMPL_OFFSET>,
            HandleConfirmationOfOurCompareStates: HandleConfirmationOfOurCompareStates::<Impl, IMPL_OFFSET>,
            HandleErrorFromOurCompareStates: HandleErrorFromOurCompareStates::<Impl, IMPL_OFFSET>,
            ConversationLost: ConversationLost::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByLuWork as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcLuRmEnlistment_Impl: Sized {
    fn Unplug(&mut self, fconversationlost: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn BackedOut(&mut self) -> ::windows::core::Result<()>;
    fn BackOut(&mut self) -> ::windows::core::Result<()>;
    fn Committed(&mut self) -> ::windows::core::Result<()>;
    fn Forget(&mut self) -> ::windows::core::Result<()>;
    fn RequestCommit(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDtcLuRmEnlistment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuRmEnlistment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuRmEnlistment_Vtbl {
        unsafe extern "system" fn Unplug<Impl: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fconversationlost: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unplug(::core::mem::transmute_copy(&fconversationlost)).into()
        }
        unsafe extern "system" fn BackedOut<Impl: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BackedOut().into()
        }
        unsafe extern "system" fn BackOut<Impl: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BackOut().into()
        }
        unsafe extern "system" fn Committed<Impl: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Committed().into()
        }
        unsafe extern "system" fn Forget<Impl: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Forget().into()
        }
        unsafe extern "system" fn RequestCommit<Impl: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestCommit().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Unplug: Unplug::<Impl, IMPL_OFFSET>,
            BackedOut: BackedOut::<Impl, IMPL_OFFSET>,
            BackOut: BackOut::<Impl, IMPL_OFFSET>,
            Committed: Committed::<Impl, IMPL_OFFSET>,
            Forget: Forget::<Impl, IMPL_OFFSET>,
            RequestCommit: RequestCommit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRmEnlistment as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuRmEnlistmentFactory_Impl: Sized {
    fn Create(&mut self, puclupair: *mut u8, cblupair: u32, pitransaction: &::core::option::Option<ITransaction>, ptransid: *mut u8, cbtransid: u32, prmenlistmentsink: &::core::option::Option<IDtcLuRmEnlistmentSink>, pprmenlistment: *mut ::core::option::Option<IDtcLuRmEnlistment>) -> ::windows::core::Result<()>;
}
impl IDtcLuRmEnlistmentFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuRmEnlistmentFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuRmEnlistmentFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IDtcLuRmEnlistmentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puclupair: *mut u8, cblupair: u32, pitransaction: ::windows::core::RawPtr, ptransid: *mut u8, cbtransid: u32, prmenlistmentsink: ::windows::core::RawPtr, pprmenlistment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Create(::core::mem::transmute_copy(&puclupair), ::core::mem::transmute_copy(&cblupair), ::core::mem::transmute(&pitransaction), ::core::mem::transmute_copy(&ptransid), ::core::mem::transmute_copy(&cbtransid), ::core::mem::transmute(&prmenlistmentsink), ::core::mem::transmute_copy(&pprmenlistment)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRmEnlistmentFactory as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuRmEnlistmentSink_Impl: Sized {
    fn AckUnplug(&mut self) -> ::windows::core::Result<()>;
    fn TmDown(&mut self) -> ::windows::core::Result<()>;
    fn SessionLost(&mut self) -> ::windows::core::Result<()>;
    fn BackedOut(&mut self) -> ::windows::core::Result<()>;
    fn BackOut(&mut self) -> ::windows::core::Result<()>;
    fn Committed(&mut self) -> ::windows::core::Result<()>;
    fn Forget(&mut self) -> ::windows::core::Result<()>;
    fn Prepare(&mut self) -> ::windows::core::Result<()>;
    fn RequestCommit(&mut self) -> ::windows::core::Result<()>;
}
impl IDtcLuRmEnlistmentSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuRmEnlistmentSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuRmEnlistmentSink_Vtbl {
        unsafe extern "system" fn AckUnplug<Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AckUnplug().into()
        }
        unsafe extern "system" fn TmDown<Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TmDown().into()
        }
        unsafe extern "system" fn SessionLost<Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SessionLost().into()
        }
        unsafe extern "system" fn BackedOut<Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BackedOut().into()
        }
        unsafe extern "system" fn BackOut<Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BackOut().into()
        }
        unsafe extern "system" fn Committed<Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Committed().into()
        }
        unsafe extern "system" fn Forget<Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Forget().into()
        }
        unsafe extern "system" fn Prepare<Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Prepare().into()
        }
        unsafe extern "system" fn RequestCommit<Impl: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestCommit().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AckUnplug: AckUnplug::<Impl, IMPL_OFFSET>,
            TmDown: TmDown::<Impl, IMPL_OFFSET>,
            SessionLost: SessionLost::<Impl, IMPL_OFFSET>,
            BackedOut: BackedOut::<Impl, IMPL_OFFSET>,
            BackOut: BackOut::<Impl, IMPL_OFFSET>,
            Committed: Committed::<Impl, IMPL_OFFSET>,
            Forget: Forget::<Impl, IMPL_OFFSET>,
            Prepare: Prepare::<Impl, IMPL_OFFSET>,
            RequestCommit: RequestCommit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRmEnlistmentSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcLuSubordinateDtc_Impl: Sized {
    fn Unplug(&mut self, fconversationlost: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn BackedOut(&mut self) -> ::windows::core::Result<()>;
    fn BackOut(&mut self) -> ::windows::core::Result<()>;
    fn Committed(&mut self) -> ::windows::core::Result<()>;
    fn Forget(&mut self) -> ::windows::core::Result<()>;
    fn Prepare(&mut self) -> ::windows::core::Result<()>;
    fn RequestCommit(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDtcLuSubordinateDtc_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuSubordinateDtc_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuSubordinateDtc_Vtbl {
        unsafe extern "system" fn Unplug<Impl: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fconversationlost: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unplug(::core::mem::transmute_copy(&fconversationlost)).into()
        }
        unsafe extern "system" fn BackedOut<Impl: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BackedOut().into()
        }
        unsafe extern "system" fn BackOut<Impl: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BackOut().into()
        }
        unsafe extern "system" fn Committed<Impl: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Committed().into()
        }
        unsafe extern "system" fn Forget<Impl: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Forget().into()
        }
        unsafe extern "system" fn Prepare<Impl: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Prepare().into()
        }
        unsafe extern "system" fn RequestCommit<Impl: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestCommit().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Unplug: Unplug::<Impl, IMPL_OFFSET>,
            BackedOut: BackedOut::<Impl, IMPL_OFFSET>,
            BackOut: BackOut::<Impl, IMPL_OFFSET>,
            Committed: Committed::<Impl, IMPL_OFFSET>,
            Forget: Forget::<Impl, IMPL_OFFSET>,
            Prepare: Prepare::<Impl, IMPL_OFFSET>,
            RequestCommit: RequestCommit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuSubordinateDtc as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuSubordinateDtcFactory_Impl: Sized {
    fn Create(&mut self, puclupair: *mut u8, cblupair: u32, punktransactionouter: &::core::option::Option<::windows::core::IUnknown>, isolevel: i32, isoflags: u32, poptions: &::core::option::Option<ITransactionOptions>, pptransaction: *mut ::core::option::Option<ITransaction>, ptransid: *mut u8, cbtransid: u32, psubordinatedtcsink: &::core::option::Option<IDtcLuSubordinateDtcSink>, ppsubordinatedtc: *mut ::core::option::Option<IDtcLuSubordinateDtc>) -> ::windows::core::Result<()>;
}
impl IDtcLuSubordinateDtcFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuSubordinateDtcFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuSubordinateDtcFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IDtcLuSubordinateDtcFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puclupair: *mut u8, cblupair: u32, punktransactionouter: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, poptions: ::windows::core::RawPtr, pptransaction: *mut ::windows::core::RawPtr, ptransid: *mut u8, cbtransid: u32, psubordinatedtcsink: ::windows::core::RawPtr, ppsubordinatedtc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .Create(::core::mem::transmute_copy(&puclupair), ::core::mem::transmute_copy(&cblupair), ::core::mem::transmute(&punktransactionouter), ::core::mem::transmute_copy(&isolevel), ::core::mem::transmute_copy(&isoflags), ::core::mem::transmute(&poptions), ::core::mem::transmute_copy(&pptransaction), ::core::mem::transmute_copy(&ptransid), ::core::mem::transmute_copy(&cbtransid), ::core::mem::transmute(&psubordinatedtcsink), ::core::mem::transmute_copy(&ppsubordinatedtc))
                .into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuSubordinateDtcFactory as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuSubordinateDtcSink_Impl: Sized {
    fn AckUnplug(&mut self) -> ::windows::core::Result<()>;
    fn TmDown(&mut self) -> ::windows::core::Result<()>;
    fn SessionLost(&mut self) -> ::windows::core::Result<()>;
    fn BackedOut(&mut self) -> ::windows::core::Result<()>;
    fn BackOut(&mut self) -> ::windows::core::Result<()>;
    fn Committed(&mut self) -> ::windows::core::Result<()>;
    fn Forget(&mut self) -> ::windows::core::Result<()>;
    fn RequestCommit(&mut self) -> ::windows::core::Result<()>;
}
impl IDtcLuSubordinateDtcSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuSubordinateDtcSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuSubordinateDtcSink_Vtbl {
        unsafe extern "system" fn AckUnplug<Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AckUnplug().into()
        }
        unsafe extern "system" fn TmDown<Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TmDown().into()
        }
        unsafe extern "system" fn SessionLost<Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SessionLost().into()
        }
        unsafe extern "system" fn BackedOut<Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BackedOut().into()
        }
        unsafe extern "system" fn BackOut<Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BackOut().into()
        }
        unsafe extern "system" fn Committed<Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Committed().into()
        }
        unsafe extern "system" fn Forget<Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Forget().into()
        }
        unsafe extern "system" fn RequestCommit<Impl: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestCommit().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AckUnplug: AckUnplug::<Impl, IMPL_OFFSET>,
            TmDown: TmDown::<Impl, IMPL_OFFSET>,
            SessionLost: SessionLost::<Impl, IMPL_OFFSET>,
            BackedOut: BackedOut::<Impl, IMPL_OFFSET>,
            BackOut: BackOut::<Impl, IMPL_OFFSET>,
            Committed: Committed::<Impl, IMPL_OFFSET>,
            Forget: Forget::<Impl, IMPL_OFFSET>,
            RequestCommit: RequestCommit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuSubordinateDtcSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcNetworkAccessConfig_Impl: Sized {
    fn GetAnyNetworkAccess(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetAnyNetworkAccess(&mut self, banynetworkaccess: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetNetworkAdministrationAccess(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetNetworkAdministrationAccess(&mut self, bnetworkadministrationaccess: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetNetworkTransactionAccess(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetNetworkTransactionAccess(&mut self, bnetworktransactionaccess: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetNetworkClientAccess(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetNetworkClientAccess(&mut self, bnetworkclientaccess: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetNetworkTIPAccess(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetNetworkTIPAccess(&mut self, bnetworktipaccess: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetXAAccess(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetXAAccess(&mut self, bxaaccess: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn RestartDtcService(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDtcNetworkAccessConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcNetworkAccessConfig_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcNetworkAccessConfig_Vtbl {
        unsafe extern "system" fn GetAnyNetworkAccess<Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbanynetworkaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnyNetworkAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *pbanynetworkaccess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnyNetworkAccess<Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, banynetworkaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAnyNetworkAccess(::core::mem::transmute_copy(&banynetworkaccess)).into()
        }
        unsafe extern "system" fn GetNetworkAdministrationAccess<Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbnetworkadministrationaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetworkAdministrationAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *pbnetworkadministrationaccess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkAdministrationAccess<Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bnetworkadministrationaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNetworkAdministrationAccess(::core::mem::transmute_copy(&bnetworkadministrationaccess)).into()
        }
        unsafe extern "system" fn GetNetworkTransactionAccess<Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbnetworktransactionaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetworkTransactionAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *pbnetworktransactionaccess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkTransactionAccess<Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bnetworktransactionaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNetworkTransactionAccess(::core::mem::transmute_copy(&bnetworktransactionaccess)).into()
        }
        unsafe extern "system" fn GetNetworkClientAccess<Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbnetworkclientaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetworkClientAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *pbnetworkclientaccess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkClientAccess<Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bnetworkclientaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNetworkClientAccess(::core::mem::transmute_copy(&bnetworkclientaccess)).into()
        }
        unsafe extern "system" fn GetNetworkTIPAccess<Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbnetworktipaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetworkTIPAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *pbnetworktipaccess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkTIPAccess<Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bnetworktipaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNetworkTIPAccess(::core::mem::transmute_copy(&bnetworktipaccess)).into()
        }
        unsafe extern "system" fn GetXAAccess<Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbxaaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXAAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *pbxaaccess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXAAccess<Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bxaaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXAAccess(::core::mem::transmute_copy(&bxaaccess)).into()
        }
        unsafe extern "system" fn RestartDtcService<Impl: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestartDtcService().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAnyNetworkAccess: GetAnyNetworkAccess::<Impl, IMPL_OFFSET>,
            SetAnyNetworkAccess: SetAnyNetworkAccess::<Impl, IMPL_OFFSET>,
            GetNetworkAdministrationAccess: GetNetworkAdministrationAccess::<Impl, IMPL_OFFSET>,
            SetNetworkAdministrationAccess: SetNetworkAdministrationAccess::<Impl, IMPL_OFFSET>,
            GetNetworkTransactionAccess: GetNetworkTransactionAccess::<Impl, IMPL_OFFSET>,
            SetNetworkTransactionAccess: SetNetworkTransactionAccess::<Impl, IMPL_OFFSET>,
            GetNetworkClientAccess: GetNetworkClientAccess::<Impl, IMPL_OFFSET>,
            SetNetworkClientAccess: SetNetworkClientAccess::<Impl, IMPL_OFFSET>,
            GetNetworkTIPAccess: GetNetworkTIPAccess::<Impl, IMPL_OFFSET>,
            SetNetworkTIPAccess: SetNetworkTIPAccess::<Impl, IMPL_OFFSET>,
            GetXAAccess: GetXAAccess::<Impl, IMPL_OFFSET>,
            SetXAAccess: SetXAAccess::<Impl, IMPL_OFFSET>,
            RestartDtcService: RestartDtcService::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcNetworkAccessConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcNetworkAccessConfig2_Impl: Sized + IDtcNetworkAccessConfig_Impl {
    fn GetNetworkInboundAccess(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetNetworkOutboundAccess(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetNetworkInboundAccess(&mut self, binbound: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetNetworkOutboundAccess(&mut self, boutbound: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetAuthenticationLevel(&mut self) -> ::windows::core::Result<AUTHENTICATION_LEVEL>;
    fn SetAuthenticationLevel(&mut self, authlevel: AUTHENTICATION_LEVEL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDtcNetworkAccessConfig2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcNetworkAccessConfig2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcNetworkAccessConfig2_Vtbl {
        unsafe extern "system" fn GetNetworkInboundAccess<Impl: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbinbound: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetworkInboundAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *pbinbound = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkOutboundAccess<Impl: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboutbound: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetworkOutboundAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *pboutbound = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkInboundAccess<Impl: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binbound: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNetworkInboundAccess(::core::mem::transmute_copy(&binbound)).into()
        }
        unsafe extern "system" fn SetNetworkOutboundAccess<Impl: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boutbound: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNetworkOutboundAccess(::core::mem::transmute_copy(&boutbound)).into()
        }
        unsafe extern "system" fn GetAuthenticationLevel<Impl: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauthlevel: *mut AUTHENTICATION_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAuthenticationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *pauthlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationLevel<Impl: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, authlevel: AUTHENTICATION_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticationLevel(::core::mem::transmute_copy(&authlevel)).into()
        }
        Self {
            base: IDtcNetworkAccessConfig_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetNetworkInboundAccess: GetNetworkInboundAccess::<Impl, IMPL_OFFSET>,
            GetNetworkOutboundAccess: GetNetworkOutboundAccess::<Impl, IMPL_OFFSET>,
            SetNetworkInboundAccess: SetNetworkInboundAccess::<Impl, IMPL_OFFSET>,
            SetNetworkOutboundAccess: SetNetworkOutboundAccess::<Impl, IMPL_OFFSET>,
            GetAuthenticationLevel: GetAuthenticationLevel::<Impl, IMPL_OFFSET>,
            SetAuthenticationLevel: SetAuthenticationLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcNetworkAccessConfig2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcNetworkAccessConfig3_Impl: Sized + IDtcNetworkAccessConfig_Impl + IDtcNetworkAccessConfig2_Impl {
    fn GetLUAccess(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetLUAccess(&mut self, bluaccess: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDtcNetworkAccessConfig3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcNetworkAccessConfig3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcNetworkAccessConfig3_Vtbl {
        unsafe extern "system" fn GetLUAccess<Impl: IDtcNetworkAccessConfig3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbluaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLUAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *pbluaccess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLUAccess<Impl: IDtcNetworkAccessConfig3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLUAccess(::core::mem::transmute_copy(&bluaccess)).into()
        }
        Self {
            base: IDtcNetworkAccessConfig2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetLUAccess: GetLUAccess::<Impl, IMPL_OFFSET>,
            SetLUAccess: SetLUAccess::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcNetworkAccessConfig3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcToXaHelper_Impl: Sized {
    fn Close(&mut self, i_fdorecovery: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn TranslateTridToXid(&mut self, pitransaction: &::core::option::Option<ITransaction>, pguidbqual: *const ::windows::core::GUID) -> ::windows::core::Result<xid_t>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDtcToXaHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcToXaHelper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcToXaHelper_Vtbl {
        unsafe extern "system" fn Close<Impl: IDtcToXaHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i_fdorecovery: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close(::core::mem::transmute_copy(&i_fdorecovery)).into()
        }
        unsafe extern "system" fn TranslateTridToXid<Impl: IDtcToXaHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitransaction: ::windows::core::RawPtr, pguidbqual: *const ::windows::core::GUID, pxid: *mut xid_t) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslateTridToXid(::core::mem::transmute(&pitransaction), ::core::mem::transmute_copy(&pguidbqual)) {
                ::core::result::Result::Ok(ok__) => {
                    *pxid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Close: Close::<Impl, IMPL_OFFSET>,
            TranslateTridToXid: TranslateTridToXid::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcToXaHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcToXaHelperFactory_Impl: Sized {
    fn Create(&mut self, pszdsn: super::super::Foundation::PSTR, pszclientdllname: super::super::Foundation::PSTR, pguidrm: *mut ::windows::core::GUID, ppxahelper: *mut ::core::option::Option<IDtcToXaHelper>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDtcToXaHelperFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcToXaHelperFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcToXaHelperFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IDtcToXaHelperFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdsn: super::super::Foundation::PSTR, pszclientdllname: super::super::Foundation::PSTR, pguidrm: *mut ::windows::core::GUID, ppxahelper: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Create(::core::mem::transmute_copy(&pszdsn), ::core::mem::transmute_copy(&pszclientdllname), ::core::mem::transmute_copy(&pguidrm), ::core::mem::transmute_copy(&ppxahelper)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcToXaHelperFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcToXaHelperSinglePipe_Impl: Sized {
    fn XARMCreate(&mut self, pszdsn: super::super::Foundation::PSTR, pszclientdll: super::super::Foundation::PSTR, pdwrmcookie: *mut u32) -> ::windows::core::Result<()>;
    fn ConvertTridToXID(&mut self, pdwitrans: *mut u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::core::Result<()>;
    fn EnlistWithRM(&mut self, dwrmcookie: u32, i_pitransaction: &::core::option::Option<ITransaction>, i_pitransres: &::core::option::Option<ITransactionResourceAsync>) -> ::windows::core::Result<ITransactionEnlistmentAsync>;
    fn ReleaseRMCookie(&mut self, i_dwrmcookie: u32, i_fnormal: super::super::Foundation::BOOL);
}
#[cfg(feature = "Win32_Foundation")]
impl IDtcToXaHelperSinglePipe_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcToXaHelperSinglePipe_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcToXaHelperSinglePipe_Vtbl {
        unsafe extern "system" fn XARMCreate<Impl: IDtcToXaHelperSinglePipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdsn: super::super::Foundation::PSTR, pszclientdll: super::super::Foundation::PSTR, pdwrmcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).XARMCreate(::core::mem::transmute_copy(&pszdsn), ::core::mem::transmute_copy(&pszclientdll), ::core::mem::transmute_copy(&pdwrmcookie)).into()
        }
        unsafe extern "system" fn ConvertTridToXID<Impl: IDtcToXaHelperSinglePipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwitrans: *mut u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertTridToXID(::core::mem::transmute_copy(&pdwitrans), ::core::mem::transmute_copy(&dwrmcookie), ::core::mem::transmute_copy(&pxid)).into()
        }
        unsafe extern "system" fn EnlistWithRM<Impl: IDtcToXaHelperSinglePipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwrmcookie: u32, i_pitransaction: ::windows::core::RawPtr, i_pitransres: ::windows::core::RawPtr, o_ppitransenslitment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnlistWithRM(::core::mem::transmute_copy(&dwrmcookie), ::core::mem::transmute(&i_pitransaction), ::core::mem::transmute(&i_pitransres)) {
                ::core::result::Result::Ok(ok__) => {
                    *o_ppitransenslitment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseRMCookie<Impl: IDtcToXaHelperSinglePipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i_dwrmcookie: u32, i_fnormal: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseRMCookie(::core::mem::transmute_copy(&i_dwrmcookie), ::core::mem::transmute_copy(&i_fnormal))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            XARMCreate: XARMCreate::<Impl, IMPL_OFFSET>,
            ConvertTridToXID: ConvertTridToXID::<Impl, IMPL_OFFSET>,
            EnlistWithRM: EnlistWithRM::<Impl, IMPL_OFFSET>,
            ReleaseRMCookie: ReleaseRMCookie::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcToXaHelperSinglePipe as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcToXaMapper_Impl: Sized {
    fn RequestNewResourceManager(&mut self, pszdsn: super::super::Foundation::PSTR, pszclientdllname: super::super::Foundation::PSTR, pdwrmcookie: *mut u32) -> ::windows::core::Result<()>;
    fn TranslateTridToXid(&mut self, pdwitransaction: *const u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::core::Result<()>;
    fn EnlistResourceManager(&mut self, dwrmcookie: u32, pdwitransaction: *const u32) -> ::windows::core::Result<()>;
    fn ReleaseResourceManager(&mut self, dwrmcookie: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDtcToXaMapper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcToXaMapper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcToXaMapper_Vtbl {
        unsafe extern "system" fn RequestNewResourceManager<Impl: IDtcToXaMapper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdsn: super::super::Foundation::PSTR, pszclientdllname: super::super::Foundation::PSTR, pdwrmcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestNewResourceManager(::core::mem::transmute_copy(&pszdsn), ::core::mem::transmute_copy(&pszclientdllname), ::core::mem::transmute_copy(&pdwrmcookie)).into()
        }
        unsafe extern "system" fn TranslateTridToXid<Impl: IDtcToXaMapper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwitransaction: *const u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TranslateTridToXid(::core::mem::transmute_copy(&pdwitransaction), ::core::mem::transmute_copy(&dwrmcookie), ::core::mem::transmute_copy(&pxid)).into()
        }
        unsafe extern "system" fn EnlistResourceManager<Impl: IDtcToXaMapper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwrmcookie: u32, pdwitransaction: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnlistResourceManager(::core::mem::transmute_copy(&dwrmcookie), ::core::mem::transmute_copy(&pdwitransaction)).into()
        }
        unsafe extern "system" fn ReleaseResourceManager<Impl: IDtcToXaMapper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwrmcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseResourceManager(::core::mem::transmute_copy(&dwrmcookie)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RequestNewResourceManager: RequestNewResourceManager::<Impl, IMPL_OFFSET>,
            TranslateTridToXid: TranslateTridToXid::<Impl, IMPL_OFFSET>,
            EnlistResourceManager: EnlistResourceManager::<Impl, IMPL_OFFSET>,
            ReleaseResourceManager: ReleaseResourceManager::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcToXaMapper as ::windows::core::Interface>::IID
    }
}
pub trait IGetDispenser_Impl: Sized {
    fn GetDispenser(&mut self, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IGetDispenser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetDispenser_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetDispenser_Vtbl {
        unsafe extern "system" fn GetDispenser<Impl: IGetDispenser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDispenser(::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvobject)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetDispenser: GetDispenser::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetDispenser as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IKernelTransaction_Impl: Sized {
    fn GetHandle(&mut self) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Foundation")]
impl IKernelTransaction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKernelTransaction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKernelTransaction_Vtbl {
        unsafe extern "system" fn GetHandle<Impl: IKernelTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *phandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetHandle: GetHandle::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKernelTransaction as ::windows::core::Interface>::IID
    }
}
pub trait ILastResourceManager_Impl: Sized {
    fn TransactionCommitted(&mut self, pprepinfo: *const u8, cbprepinfo: u32) -> ::windows::core::Result<()>;
    fn RecoveryDone(&mut self) -> ::windows::core::Result<()>;
}
impl ILastResourceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILastResourceManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILastResourceManager_Vtbl {
        unsafe extern "system" fn TransactionCommitted<Impl: ILastResourceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransactionCommitted(::core::mem::transmute_copy(&pprepinfo), ::core::mem::transmute_copy(&cbprepinfo)).into()
        }
        unsafe extern "system" fn RecoveryDone<Impl: ILastResourceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RecoveryDone().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            TransactionCommitted: TransactionCommitted::<Impl, IMPL_OFFSET>,
            RecoveryDone: RecoveryDone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILastResourceManager as ::windows::core::Interface>::IID
    }
}
pub trait IPrepareInfo_Impl: Sized {
    fn GetPrepareInfoSize(&mut self, pcbprepinfo: *mut u32) -> ::windows::core::Result<()>;
    fn GetPrepareInfo(&mut self, pprepinfo: *mut u8) -> ::windows::core::Result<()>;
}
impl IPrepareInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrepareInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrepareInfo_Vtbl {
        unsafe extern "system" fn GetPrepareInfoSize<Impl: IPrepareInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbprepinfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPrepareInfoSize(::core::mem::transmute_copy(&pcbprepinfo)).into()
        }
        unsafe extern "system" fn GetPrepareInfo<Impl: IPrepareInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprepinfo: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPrepareInfo(::core::mem::transmute_copy(&pprepinfo)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPrepareInfoSize: GetPrepareInfoSize::<Impl, IMPL_OFFSET>,
            GetPrepareInfo: GetPrepareInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrepareInfo as ::windows::core::Interface>::IID
    }
}
pub trait IPrepareInfo2_Impl: Sized {
    fn GetPrepareInfoSize(&mut self) -> ::windows::core::Result<u32>;
    fn GetPrepareInfo(&mut self, cbprepareinfo: u32, pprepinfo: *mut u8) -> ::windows::core::Result<()>;
}
impl IPrepareInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrepareInfo2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrepareInfo2_Vtbl {
        unsafe extern "system" fn GetPrepareInfoSize<Impl: IPrepareInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbprepinfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrepareInfoSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbprepinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrepareInfo<Impl: IPrepareInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbprepareinfo: u32, pprepinfo: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPrepareInfo(::core::mem::transmute_copy(&cbprepareinfo), ::core::mem::transmute_copy(&pprepinfo)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPrepareInfoSize: GetPrepareInfoSize::<Impl, IMPL_OFFSET>,
            GetPrepareInfo: GetPrepareInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrepareInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRMHelper_Impl: Sized {
    fn RMCount(&mut self, dwctotalnumberofrms: u32) -> ::windows::core::Result<()>;
    fn RMInfo(&mut self, pxa_switch: *mut xa_switch_t, fcdeclcallingconv: super::super::Foundation::BOOL, pszopenstring: super::super::Foundation::PSTR, pszclosestring: super::super::Foundation::PSTR, guidrmrecovery: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRMHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRMHelper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRMHelper_Vtbl {
        unsafe extern "system" fn RMCount<Impl: IRMHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwctotalnumberofrms: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RMCount(::core::mem::transmute_copy(&dwctotalnumberofrms)).into()
        }
        unsafe extern "system" fn RMInfo<Impl: IRMHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxa_switch: *mut xa_switch_t, fcdeclcallingconv: super::super::Foundation::BOOL, pszopenstring: super::super::Foundation::PSTR, pszclosestring: super::super::Foundation::PSTR, guidrmrecovery: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RMInfo(::core::mem::transmute_copy(&pxa_switch), ::core::mem::transmute_copy(&fcdeclcallingconv), ::core::mem::transmute_copy(&pszopenstring), ::core::mem::transmute_copy(&pszclosestring), ::core::mem::transmute_copy(&guidrmrecovery)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), RMCount: RMCount::<Impl, IMPL_OFFSET>, RMInfo: RMInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRMHelper as ::windows::core::Interface>::IID
    }
}
pub trait IResourceManager_Impl: Sized {
    fn Enlist(&mut self, ptransaction: &::core::option::Option<ITransaction>, pres: &::core::option::Option<ITransactionResourceAsync>, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows::core::Result<()>;
    fn Reenlist(&mut self, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32) -> ::windows::core::Result<XACTSTAT>;
    fn ReenlistmentComplete(&mut self) -> ::windows::core::Result<()>;
    fn GetDistributedTransactionManager(&mut self, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IResourceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceManager_Vtbl {
        unsafe extern "system" fn Enlist<Impl: IResourceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: ::windows::core::RawPtr, pres: ::windows::core::RawPtr, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enlist(::core::mem::transmute(&ptransaction), ::core::mem::transmute(&pres), ::core::mem::transmute_copy(&puow), ::core::mem::transmute_copy(&pisolevel), ::core::mem::transmute_copy(&ppenlist)).into()
        }
        unsafe extern "system" fn Reenlist<Impl: IResourceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reenlist(::core::mem::transmute_copy(&pprepinfo), ::core::mem::transmute_copy(&cbprepinfo), ::core::mem::transmute_copy(&ltimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *pxactstat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReenlistmentComplete<Impl: IResourceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReenlistmentComplete().into()
        }
        unsafe extern "system" fn GetDistributedTransactionManager<Impl: IResourceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDistributedTransactionManager(::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvobject)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Enlist: Enlist::<Impl, IMPL_OFFSET>,
            Reenlist: Reenlist::<Impl, IMPL_OFFSET>,
            ReenlistmentComplete: ReenlistmentComplete::<Impl, IMPL_OFFSET>,
            GetDistributedTransactionManager: GetDistributedTransactionManager::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IResourceManager2_Impl: Sized + IResourceManager_Impl {
    fn Enlist2(&mut self, ptransaction: &::core::option::Option<ITransaction>, presasync: &::core::option::Option<ITransactionResourceAsync>, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut xid_t, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows::core::Result<()>;
    fn Reenlist2(&mut self, pxid: *const xid_t, dwtimeout: u32) -> ::windows::core::Result<XACTSTAT>;
}
#[cfg(feature = "Win32_Foundation")]
impl IResourceManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceManager2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceManager2_Vtbl {
        unsafe extern "system" fn Enlist2<Impl: IResourceManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: ::windows::core::RawPtr, presasync: ::windows::core::RawPtr, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut xid_t, ppenlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enlist2(::core::mem::transmute(&ptransaction), ::core::mem::transmute(&presasync), ::core::mem::transmute_copy(&puow), ::core::mem::transmute_copy(&pisolevel), ::core::mem::transmute_copy(&pxid), ::core::mem::transmute_copy(&ppenlist)).into()
        }
        unsafe extern "system" fn Reenlist2<Impl: IResourceManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxid: *const xid_t, dwtimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reenlist2(::core::mem::transmute_copy(&pxid), ::core::mem::transmute_copy(&dwtimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *pxactstat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IResourceManager_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Enlist2: Enlist2::<Impl, IMPL_OFFSET>,
            Reenlist2: Reenlist2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IResourceManagerFactory_Impl: Sized {
    fn Create(&mut self, pguidrm: *const ::windows::core::GUID, pszrmname: super::super::Foundation::PSTR, piresmgrsink: &::core::option::Option<IResourceManagerSink>) -> ::windows::core::Result<IResourceManager>;
}
#[cfg(feature = "Win32_Foundation")]
impl IResourceManagerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceManagerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceManagerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IResourceManagerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidrm: *const ::windows::core::GUID, pszrmname: super::super::Foundation::PSTR, piresmgrsink: ::windows::core::RawPtr, ppresmgr: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(::core::mem::transmute_copy(&pguidrm), ::core::mem::transmute_copy(&pszrmname), ::core::mem::transmute(&piresmgrsink)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresmgr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceManagerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IResourceManagerFactory2_Impl: Sized + IResourceManagerFactory_Impl {
    fn CreateEx(&mut self, pguidrm: *const ::windows::core::GUID, pszrmname: super::super::Foundation::PSTR, piresmgrsink: &::core::option::Option<IResourceManagerSink>, riidrequested: *const ::windows::core::GUID, ppvresmgr: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IResourceManagerFactory2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceManagerFactory2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceManagerFactory2_Vtbl {
        unsafe extern "system" fn CreateEx<Impl: IResourceManagerFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidrm: *const ::windows::core::GUID, pszrmname: super::super::Foundation::PSTR, piresmgrsink: ::windows::core::RawPtr, riidrequested: *const ::windows::core::GUID, ppvresmgr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateEx(::core::mem::transmute_copy(&pguidrm), ::core::mem::transmute_copy(&pszrmname), ::core::mem::transmute(&piresmgrsink), ::core::mem::transmute_copy(&riidrequested), ::core::mem::transmute_copy(&ppvresmgr)).into()
        }
        Self { base: IResourceManagerFactory_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), CreateEx: CreateEx::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceManagerFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IResourceManagerRejoinable_Impl: Sized + IResourceManager_Impl + IResourceManager2_Impl {
    fn Rejoin(&mut self, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32) -> ::windows::core::Result<XACTSTAT>;
}
#[cfg(feature = "Win32_Foundation")]
impl IResourceManagerRejoinable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceManagerRejoinable_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceManagerRejoinable_Vtbl {
        unsafe extern "system" fn Rejoin<Impl: IResourceManagerRejoinable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rejoin(::core::mem::transmute_copy(&pprepinfo), ::core::mem::transmute_copy(&cbprepinfo), ::core::mem::transmute_copy(&ltimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *pxactstat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IResourceManager2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Rejoin: Rejoin::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceManagerRejoinable as ::windows::core::Interface>::IID
    }
}
pub trait IResourceManagerSink_Impl: Sized {
    fn TMDown(&mut self) -> ::windows::core::Result<()>;
}
impl IResourceManagerSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceManagerSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceManagerSink_Vtbl {
        unsafe extern "system" fn TMDown<Impl: IResourceManagerSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TMDown().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), TMDown: TMDown::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceManagerSink as ::windows::core::Interface>::IID
    }
}
pub trait ITipHelper_Impl: Sized {
    fn Pull(&mut self, i_psztxurl: *const u8) -> ::windows::core::Result<ITransaction>;
    fn PullAsync(&mut self, i_psztxurl: *const u8, i_ptippullsink: &::core::option::Option<ITipPullSink>) -> ::windows::core::Result<ITransaction>;
    fn GetLocalTmUrl(&mut self) -> ::windows::core::Result<*mut u8>;
}
impl ITipHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITipHelper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITipHelper_Vtbl {
        unsafe extern "system" fn Pull<Impl: ITipHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i_psztxurl: *const u8, o_ppitransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pull(::core::mem::transmute_copy(&i_psztxurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *o_ppitransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PullAsync<Impl: ITipHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i_psztxurl: *const u8, i_ptippullsink: ::windows::core::RawPtr, o_ppitransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PullAsync(::core::mem::transmute_copy(&i_psztxurl), ::core::mem::transmute(&i_ptippullsink)) {
                ::core::result::Result::Ok(ok__) => {
                    *o_ppitransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalTmUrl<Impl: ITipHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, o_ppszlocaltmurl: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalTmUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *o_ppszlocaltmurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Pull: Pull::<Impl, IMPL_OFFSET>,
            PullAsync: PullAsync::<Impl, IMPL_OFFSET>,
            GetLocalTmUrl: GetLocalTmUrl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITipHelper as ::windows::core::Interface>::IID
    }
}
pub trait ITipPullSink_Impl: Sized {
    fn PullComplete(&mut self, i_hrpull: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ITipPullSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITipPullSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITipPullSink_Vtbl {
        unsafe extern "system" fn PullComplete<Impl: ITipPullSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i_hrpull: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PullComplete(::core::mem::transmute_copy(&i_hrpull)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), PullComplete: PullComplete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITipPullSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITipTransaction_Impl: Sized {
    fn Push(&mut self, i_pszremotetmurl: *const u8) -> ::windows::core::Result<super::super::Foundation::PSTR>;
    fn GetTransactionUrl(&mut self) -> ::windows::core::Result<super::super::Foundation::PSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITipTransaction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITipTransaction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITipTransaction_Vtbl {
        unsafe extern "system" fn Push<Impl: ITipTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i_pszremotetmurl: *const u8, o_ppszremotetxurl: *mut super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Push(::core::mem::transmute_copy(&i_pszremotetmurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *o_ppszremotetxurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransactionUrl<Impl: ITipTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, o_ppszlocaltxurl: *mut super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransactionUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *o_ppszlocaltxurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Push: Push::<Impl, IMPL_OFFSET>,
            GetTransactionUrl: GetTransactionUrl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITipTransaction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITmNodeName_Impl: Sized {
    fn GetNodeNameSize(&mut self) -> ::windows::core::Result<u32>;
    fn GetNodeName(&mut self, cbnodenamebuffersize: u32, pnodenamebuffer: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITmNodeName_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITmNodeName_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITmNodeName_Vtbl {
        unsafe extern "system" fn GetNodeNameSize<Impl: ITmNodeName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbnodenamesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNodeNameSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbnodenamesize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNodeName<Impl: ITmNodeName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbnodenamebuffersize: u32, pnodenamebuffer: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNodeName(::core::mem::transmute_copy(&cbnodenamebuffersize), ::core::mem::transmute_copy(&pnodenamebuffer)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetNodeNameSize: GetNodeNameSize::<Impl, IMPL_OFFSET>,
            GetNodeName: GetNodeName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITmNodeName as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITransaction_Impl: Sized {
    fn Commit(&mut self, fretaining: super::super::Foundation::BOOL, grftc: u32, grfrm: u32) -> ::windows::core::Result<()>;
    fn Abort(&mut self, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, fasync: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetTransactionInfo(&mut self) -> ::windows::core::Result<XACTTRANSINFO>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITransaction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransaction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransaction_Vtbl {
        unsafe extern "system" fn Commit<Impl: ITransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grftc: u32, grfrm: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit(::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&grftc), ::core::mem::transmute_copy(&grfrm)).into()
        }
        unsafe extern "system" fn Abort<Impl: ITransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, fasync: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Abort(::core::mem::transmute_copy(&pboidreason), ::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&fasync)).into()
        }
        unsafe extern "system" fn GetTransactionInfo<Impl: ITransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut XACTTRANSINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransactionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *pinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Commit: Commit::<Impl, IMPL_OFFSET>,
            Abort: Abort::<Impl, IMPL_OFFSET>,
            GetTransactionInfo: GetTransactionInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransaction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITransaction2_Impl: Sized + ITransaction_Impl + ITransactionCloner_Impl {
    fn GetTransactionInfo2(&mut self) -> ::windows::core::Result<XACTTRANSINFO>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITransaction2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransaction2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransaction2_Vtbl {
        unsafe extern "system" fn GetTransactionInfo2<Impl: ITransaction2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut XACTTRANSINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransactionInfo2() {
                ::core::result::Result::Ok(ok__) => {
                    *pinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ITransactionCloner_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetTransactionInfo2: GetTransactionInfo2::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransaction2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITransactionCloner_Impl: Sized + ITransaction_Impl {
    fn CloneWithCommitDisabled(&mut self) -> ::windows::core::Result<ITransaction>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITransactionCloner_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionCloner_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionCloner_Vtbl {
        unsafe extern "system" fn CloneWithCommitDisabled<Impl: ITransactionCloner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CloneWithCommitDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *ppitransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ITransaction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CloneWithCommitDisabled: CloneWithCommitDisabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionCloner as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionDispenser_Impl: Sized {
    fn GetOptionsObject(&mut self) -> ::windows::core::Result<ITransactionOptions>;
    fn BeginTransaction(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, isolevel: i32, isoflags: u32, poptions: &::core::option::Option<ITransactionOptions>) -> ::windows::core::Result<ITransaction>;
}
impl ITransactionDispenser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionDispenser_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionDispenser_Vtbl {
        unsafe extern "system" fn GetOptionsObject<Impl: ITransactionDispenser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOptionsObject() {
                ::core::result::Result::Ok(ok__) => {
                    *ppoptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginTransaction<Impl: ITransactionDispenser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, poptions: ::windows::core::RawPtr, pptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginTransaction(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&isolevel), ::core::mem::transmute_copy(&isoflags), ::core::mem::transmute(&poptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetOptionsObject: GetOptionsObject::<Impl, IMPL_OFFSET>,
            BeginTransaction: BeginTransaction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionDispenser as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITransactionEnlistmentAsync_Impl: Sized {
    fn PrepareRequestDone(&mut self, hr: ::windows::core::HRESULT, pmk: &::core::option::Option<super::Com::IMoniker>, pboidreason: *const BOID) -> ::windows::core::Result<()>;
    fn CommitRequestDone(&mut self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn AbortRequestDone(&mut self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITransactionEnlistmentAsync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionEnlistmentAsync_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionEnlistmentAsync_Vtbl {
        unsafe extern "system" fn PrepareRequestDone<Impl: ITransactionEnlistmentAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, pmk: ::windows::core::RawPtr, pboidreason: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrepareRequestDone(::core::mem::transmute_copy(&hr), ::core::mem::transmute(&pmk), ::core::mem::transmute_copy(&pboidreason)).into()
        }
        unsafe extern "system" fn CommitRequestDone<Impl: ITransactionEnlistmentAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CommitRequestDone(::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn AbortRequestDone<Impl: ITransactionEnlistmentAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AbortRequestDone(::core::mem::transmute_copy(&hr)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            PrepareRequestDone: PrepareRequestDone::<Impl, IMPL_OFFSET>,
            CommitRequestDone: CommitRequestDone::<Impl, IMPL_OFFSET>,
            AbortRequestDone: AbortRequestDone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionEnlistmentAsync as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionExport_Impl: Sized {
    fn Export(&mut self, punktransaction: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<u32>;
    fn GetTransactionCookie(&mut self, punktransaction: &::core::option::Option<::windows::core::IUnknown>, cbtransactioncookie: u32, rgbtransactioncookie: *mut u8, pcbused: *mut u32) -> ::windows::core::Result<()>;
}
impl ITransactionExport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionExport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionExport_Vtbl {
        unsafe extern "system" fn Export<Impl: ITransactionExport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punktransaction: *mut ::core::ffi::c_void, pcbtransactioncookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Export(::core::mem::transmute(&punktransaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbtransactioncookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransactionCookie<Impl: ITransactionExport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punktransaction: *mut ::core::ffi::c_void, cbtransactioncookie: u32, rgbtransactioncookie: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTransactionCookie(::core::mem::transmute(&punktransaction), ::core::mem::transmute_copy(&cbtransactioncookie), ::core::mem::transmute_copy(&rgbtransactioncookie), ::core::mem::transmute_copy(&pcbused)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Export: Export::<Impl, IMPL_OFFSET>,
            GetTransactionCookie: GetTransactionCookie::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionExport as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionExportFactory_Impl: Sized {
    fn GetRemoteClassId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Create(&mut self, cbwhereabouts: u32, rgbwhereabouts: *const u8) -> ::windows::core::Result<ITransactionExport>;
}
impl ITransactionExportFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionExportFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionExportFactory_Vtbl {
        unsafe extern "system" fn GetRemoteClassId<Impl: ITransactionExportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRemoteClassId() {
                ::core::result::Result::Ok(ok__) => {
                    *pclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: ITransactionExportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbwhereabouts: u32, rgbwhereabouts: *const u8, ppexport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(::core::mem::transmute_copy(&cbwhereabouts), ::core::mem::transmute_copy(&rgbwhereabouts)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppexport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetRemoteClassId: GetRemoteClassId::<Impl, IMPL_OFFSET>,
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionExportFactory as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionImport_Impl: Sized {
    fn Import(&mut self, cbtransactioncookie: u32, rgbtransactioncookie: *const u8, piid: *const ::windows::core::GUID, ppvtransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ITransactionImport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionImport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionImport_Vtbl {
        unsafe extern "system" fn Import<Impl: ITransactionImport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbtransactioncookie: u32, rgbtransactioncookie: *const u8, piid: *const ::windows::core::GUID, ppvtransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Import(::core::mem::transmute_copy(&cbtransactioncookie), ::core::mem::transmute_copy(&rgbtransactioncookie), ::core::mem::transmute_copy(&piid), ::core::mem::transmute_copy(&ppvtransaction)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Import: Import::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionImport as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionImportWhereabouts_Impl: Sized {
    fn GetWhereaboutsSize(&mut self) -> ::windows::core::Result<u32>;
    fn GetWhereabouts(&mut self, cbwhereabouts: u32, rgbwhereabouts: *mut u8, pcbused: *mut u32) -> ::windows::core::Result<()>;
}
impl ITransactionImportWhereabouts_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionImportWhereabouts_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionImportWhereabouts_Vtbl {
        unsafe extern "system" fn GetWhereaboutsSize<Impl: ITransactionImportWhereabouts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbwhereabouts: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWhereaboutsSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbwhereabouts = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWhereabouts<Impl: ITransactionImportWhereabouts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbwhereabouts: u32, rgbwhereabouts: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWhereabouts(::core::mem::transmute_copy(&cbwhereabouts), ::core::mem::transmute_copy(&rgbwhereabouts), ::core::mem::transmute_copy(&pcbused)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetWhereaboutsSize: GetWhereaboutsSize::<Impl, IMPL_OFFSET>,
            GetWhereabouts: GetWhereabouts::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionImportWhereabouts as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionLastEnlistmentAsync_Impl: Sized {
    fn TransactionOutcome(&mut self, xactstat: XACTSTAT, pboidreason: *const BOID) -> ::windows::core::Result<()>;
}
impl ITransactionLastEnlistmentAsync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionLastEnlistmentAsync_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionLastEnlistmentAsync_Vtbl {
        unsafe extern "system" fn TransactionOutcome<Impl: ITransactionLastEnlistmentAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xactstat: XACTSTAT, pboidreason: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransactionOutcome(::core::mem::transmute_copy(&xactstat), ::core::mem::transmute_copy(&pboidreason)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), TransactionOutcome: TransactionOutcome::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionLastEnlistmentAsync as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionLastResourceAsync_Impl: Sized {
    fn DelegateCommit(&mut self, grfrm: u32) -> ::windows::core::Result<()>;
    fn ForgetRequest(&mut self, pnewuow: *const BOID) -> ::windows::core::Result<()>;
}
impl ITransactionLastResourceAsync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionLastResourceAsync_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionLastResourceAsync_Vtbl {
        unsafe extern "system" fn DelegateCommit<Impl: ITransactionLastResourceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfrm: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DelegateCommit(::core::mem::transmute_copy(&grfrm)).into()
        }
        unsafe extern "system" fn ForgetRequest<Impl: ITransactionLastResourceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewuow: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ForgetRequest(::core::mem::transmute_copy(&pnewuow)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            DelegateCommit: DelegateCommit::<Impl, IMPL_OFFSET>,
            ForgetRequest: ForgetRequest::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionLastResourceAsync as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionOptions_Impl: Sized {
    fn SetOptions(&mut self, poptions: *const XACTOPT) -> ::windows::core::Result<()>;
    fn GetOptions(&mut self, poptions: *mut XACTOPT) -> ::windows::core::Result<()>;
}
impl ITransactionOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionOptions_Vtbl {
        unsafe extern "system" fn SetOptions<Impl: ITransactionOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poptions: *const XACTOPT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOptions(::core::mem::transmute_copy(&poptions)).into()
        }
        unsafe extern "system" fn GetOptions<Impl: ITransactionOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poptions: *mut XACTOPT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOptions(::core::mem::transmute_copy(&poptions)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetOptions: SetOptions::<Impl, IMPL_OFFSET>,
            GetOptions: GetOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITransactionOutcomeEvents_Impl: Sized {
    fn Committed(&mut self, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn Aborted(&mut self, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn HeuristicDecision(&mut self, dwdecision: u32, pboidreason: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn Indoubt(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITransactionOutcomeEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionOutcomeEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionOutcomeEvents_Vtbl {
        unsafe extern "system" fn Committed<Impl: ITransactionOutcomeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Committed(::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&pnewuow), ::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn Aborted<Impl: ITransactionOutcomeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Aborted(::core::mem::transmute_copy(&pboidreason), ::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&pnewuow), ::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn HeuristicDecision<Impl: ITransactionOutcomeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdecision: u32, pboidreason: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HeuristicDecision(::core::mem::transmute_copy(&dwdecision), ::core::mem::transmute_copy(&pboidreason), ::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn Indoubt<Impl: ITransactionOutcomeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Indoubt().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Committed: Committed::<Impl, IMPL_OFFSET>,
            Aborted: Aborted::<Impl, IMPL_OFFSET>,
            HeuristicDecision: HeuristicDecision::<Impl, IMPL_OFFSET>,
            Indoubt: Indoubt::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionOutcomeEvents as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionPhase0EnlistmentAsync_Impl: Sized {
    fn Enable(&mut self) -> ::windows::core::Result<()>;
    fn WaitForEnlistment(&mut self) -> ::windows::core::Result<()>;
    fn Phase0Done(&mut self) -> ::windows::core::Result<()>;
    fn Unenlist(&mut self) -> ::windows::core::Result<()>;
    fn GetTransaction(&mut self) -> ::windows::core::Result<ITransaction>;
}
impl ITransactionPhase0EnlistmentAsync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionPhase0EnlistmentAsync_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionPhase0EnlistmentAsync_Vtbl {
        unsafe extern "system" fn Enable<Impl: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enable().into()
        }
        unsafe extern "system" fn WaitForEnlistment<Impl: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WaitForEnlistment().into()
        }
        unsafe extern "system" fn Phase0Done<Impl: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Phase0Done().into()
        }
        unsafe extern "system" fn Unenlist<Impl: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unenlist().into()
        }
        unsafe extern "system" fn GetTransaction<Impl: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *ppitransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Enable: Enable::<Impl, IMPL_OFFSET>,
            WaitForEnlistment: WaitForEnlistment::<Impl, IMPL_OFFSET>,
            Phase0Done: Phase0Done::<Impl, IMPL_OFFSET>,
            Unenlist: Unenlist::<Impl, IMPL_OFFSET>,
            GetTransaction: GetTransaction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionPhase0EnlistmentAsync as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionPhase0Factory_Impl: Sized {
    fn Create(&mut self, pphase0notify: &::core::option::Option<ITransactionPhase0NotifyAsync>) -> ::windows::core::Result<ITransactionPhase0EnlistmentAsync>;
}
impl ITransactionPhase0Factory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionPhase0Factory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionPhase0Factory_Vtbl {
        unsafe extern "system" fn Create<Impl: ITransactionPhase0Factory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphase0notify: ::windows::core::RawPtr, ppphase0enlistment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(::core::mem::transmute(&pphase0notify)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppphase0enlistment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionPhase0Factory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITransactionPhase0NotifyAsync_Impl: Sized {
    fn Phase0Request(&mut self, fabortinghint: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn EnlistCompleted(&mut self, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITransactionPhase0NotifyAsync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionPhase0NotifyAsync_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionPhase0NotifyAsync_Vtbl {
        unsafe extern "system" fn Phase0Request<Impl: ITransactionPhase0NotifyAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fabortinghint: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Phase0Request(::core::mem::transmute_copy(&fabortinghint)).into()
        }
        unsafe extern "system" fn EnlistCompleted<Impl: ITransactionPhase0NotifyAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnlistCompleted(::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Phase0Request: Phase0Request::<Impl, IMPL_OFFSET>,
            EnlistCompleted: EnlistCompleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionPhase0NotifyAsync as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionReceiver_Impl: Sized {
    fn UnmarshalPropagationToken(&mut self, cbtoken: u32, rgbtoken: *const u8) -> ::windows::core::Result<ITransaction>;
    fn GetReturnTokenSize(&mut self) -> ::windows::core::Result<u32>;
    fn MarshalReturnToken(&mut self, cbreturntoken: u32, rgbreturntoken: *mut u8, pcbused: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
}
impl ITransactionReceiver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionReceiver_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionReceiver_Vtbl {
        unsafe extern "system" fn UnmarshalPropagationToken<Impl: ITransactionReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbtoken: u32, rgbtoken: *const u8, pptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnmarshalPropagationToken(::core::mem::transmute_copy(&cbtoken), ::core::mem::transmute_copy(&rgbtoken)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReturnTokenSize<Impl: ITransactionReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbreturntoken: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReturnTokenSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbreturntoken = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarshalReturnToken<Impl: ITransactionReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbreturntoken: u32, rgbreturntoken: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MarshalReturnToken(::core::mem::transmute_copy(&cbreturntoken), ::core::mem::transmute_copy(&rgbreturntoken), ::core::mem::transmute_copy(&pcbused)).into()
        }
        unsafe extern "system" fn Reset<Impl: ITransactionReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            UnmarshalPropagationToken: UnmarshalPropagationToken::<Impl, IMPL_OFFSET>,
            GetReturnTokenSize: GetReturnTokenSize::<Impl, IMPL_OFFSET>,
            MarshalReturnToken: MarshalReturnToken::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionReceiver as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionReceiverFactory_Impl: Sized {
    fn Create(&mut self) -> ::windows::core::Result<ITransactionReceiver>;
}
impl ITransactionReceiverFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionReceiverFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionReceiverFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: ITransactionReceiverFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppreceiver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create() {
                ::core::result::Result::Ok(ok__) => {
                    *ppreceiver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionReceiverFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITransactionResource_Impl: Sized {
    fn PrepareRequest(&mut self, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CommitRequest(&mut self, grfrm: u32, pnewuow: *const BOID) -> ::windows::core::Result<()>;
    fn AbortRequest(&mut self, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows::core::Result<()>;
    fn TMDown(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITransactionResource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionResource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionResource_Vtbl {
        unsafe extern "system" fn PrepareRequest<Impl: ITransactionResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrepareRequest(::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&grfrm), ::core::mem::transmute_copy(&fwantmoniker), ::core::mem::transmute_copy(&fsinglephase)).into()
        }
        unsafe extern "system" fn CommitRequest<Impl: ITransactionResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfrm: u32, pnewuow: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CommitRequest(::core::mem::transmute_copy(&grfrm), ::core::mem::transmute_copy(&pnewuow)).into()
        }
        unsafe extern "system" fn AbortRequest<Impl: ITransactionResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AbortRequest(::core::mem::transmute_copy(&pboidreason), ::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&pnewuow)).into()
        }
        unsafe extern "system" fn TMDown<Impl: ITransactionResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TMDown().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            PrepareRequest: PrepareRequest::<Impl, IMPL_OFFSET>,
            CommitRequest: CommitRequest::<Impl, IMPL_OFFSET>,
            AbortRequest: AbortRequest::<Impl, IMPL_OFFSET>,
            TMDown: TMDown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionResource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITransactionResourceAsync_Impl: Sized {
    fn PrepareRequest(&mut self, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CommitRequest(&mut self, grfrm: u32, pnewuow: *const BOID) -> ::windows::core::Result<()>;
    fn AbortRequest(&mut self, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows::core::Result<()>;
    fn TMDown(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITransactionResourceAsync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionResourceAsync_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionResourceAsync_Vtbl {
        unsafe extern "system" fn PrepareRequest<Impl: ITransactionResourceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrepareRequest(::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&grfrm), ::core::mem::transmute_copy(&fwantmoniker), ::core::mem::transmute_copy(&fsinglephase)).into()
        }
        unsafe extern "system" fn CommitRequest<Impl: ITransactionResourceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfrm: u32, pnewuow: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CommitRequest(::core::mem::transmute_copy(&grfrm), ::core::mem::transmute_copy(&pnewuow)).into()
        }
        unsafe extern "system" fn AbortRequest<Impl: ITransactionResourceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AbortRequest(::core::mem::transmute_copy(&pboidreason), ::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&pnewuow)).into()
        }
        unsafe extern "system" fn TMDown<Impl: ITransactionResourceAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TMDown().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            PrepareRequest: PrepareRequest::<Impl, IMPL_OFFSET>,
            CommitRequest: CommitRequest::<Impl, IMPL_OFFSET>,
            AbortRequest: AbortRequest::<Impl, IMPL_OFFSET>,
            TMDown: TMDown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionResourceAsync as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionTransmitter_Impl: Sized {
    fn Set(&mut self, ptransaction: &::core::option::Option<ITransaction>) -> ::windows::core::Result<()>;
    fn GetPropagationTokenSize(&mut self) -> ::windows::core::Result<u32>;
    fn MarshalPropagationToken(&mut self, cbtoken: u32, rgbtoken: *mut u8, pcbused: *mut u32) -> ::windows::core::Result<()>;
    fn UnmarshalReturnToken(&mut self, cbreturntoken: u32, rgbreturntoken: *const u8) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
}
impl ITransactionTransmitter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionTransmitter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionTransmitter_Vtbl {
        unsafe extern "system" fn Set<Impl: ITransactionTransmitter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Set(::core::mem::transmute(&ptransaction)).into()
        }
        unsafe extern "system" fn GetPropagationTokenSize<Impl: ITransactionTransmitter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbtoken: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropagationTokenSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbtoken = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarshalPropagationToken<Impl: ITransactionTransmitter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbtoken: u32, rgbtoken: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MarshalPropagationToken(::core::mem::transmute_copy(&cbtoken), ::core::mem::transmute_copy(&rgbtoken), ::core::mem::transmute_copy(&pcbused)).into()
        }
        unsafe extern "system" fn UnmarshalReturnToken<Impl: ITransactionTransmitter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbreturntoken: u32, rgbreturntoken: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnmarshalReturnToken(::core::mem::transmute_copy(&cbreturntoken), ::core::mem::transmute_copy(&rgbreturntoken)).into()
        }
        unsafe extern "system" fn Reset<Impl: ITransactionTransmitter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Set: Set::<Impl, IMPL_OFFSET>,
            GetPropagationTokenSize: GetPropagationTokenSize::<Impl, IMPL_OFFSET>,
            MarshalPropagationToken: MarshalPropagationToken::<Impl, IMPL_OFFSET>,
            UnmarshalReturnToken: UnmarshalReturnToken::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionTransmitter as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionTransmitterFactory_Impl: Sized {
    fn Create(&mut self) -> ::windows::core::Result<ITransactionTransmitter>;
}
impl ITransactionTransmitterFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionTransmitterFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionTransmitterFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: ITransactionTransmitterFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptransmitter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create() {
                ::core::result::Result::Ok(ok__) => {
                    *pptransmitter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionTransmitterFactory as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionVoterBallotAsync2_Impl: Sized {
    fn VoteRequestDone(&mut self, hr: ::windows::core::HRESULT, pboidreason: *const BOID) -> ::windows::core::Result<()>;
}
impl ITransactionVoterBallotAsync2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionVoterBallotAsync2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionVoterBallotAsync2_Vtbl {
        unsafe extern "system" fn VoteRequestDone<Impl: ITransactionVoterBallotAsync2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, pboidreason: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VoteRequestDone(::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&pboidreason)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), VoteRequestDone: VoteRequestDone::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionVoterBallotAsync2 as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionVoterFactory2_Impl: Sized {
    fn Create(&mut self, ptransaction: &::core::option::Option<ITransaction>, pvoternotify: &::core::option::Option<ITransactionVoterNotifyAsync2>) -> ::windows::core::Result<ITransactionVoterBallotAsync2>;
}
impl ITransactionVoterFactory2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionVoterFactory2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionVoterFactory2_Vtbl {
        unsafe extern "system" fn Create<Impl: ITransactionVoterFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: ::windows::core::RawPtr, pvoternotify: ::windows::core::RawPtr, ppvoterballot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(::core::mem::transmute(&ptransaction), ::core::mem::transmute(&pvoternotify)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvoterballot = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionVoterFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITransactionVoterNotifyAsync2_Impl: Sized + ITransactionOutcomeEvents_Impl {
    fn VoteRequest(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITransactionVoterNotifyAsync2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionVoterNotifyAsync2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionVoterNotifyAsync2_Vtbl {
        unsafe extern "system" fn VoteRequest<Impl: ITransactionVoterNotifyAsync2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VoteRequest().into()
        }
        Self { base: ITransactionOutcomeEvents_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), VoteRequest: VoteRequest::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionVoterNotifyAsync2 as ::windows::core::Interface>::IID
    }
}
pub trait IXAConfig_Impl: Sized {
    fn Initialize(&mut self, clsidhelperdll: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Terminate(&mut self) -> ::windows::core::Result<()>;
}
impl IXAConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAConfig_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXAConfig_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IXAConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidhelperdll: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&clsidhelperdll)).into()
        }
        unsafe extern "system" fn Terminate<Impl: IXAConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Terminate().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Terminate: Terminate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAConfig as ::windows::core::Interface>::IID
    }
}
pub trait IXAObtainRMInfo_Impl: Sized {
    fn ObtainRMInfo(&mut self, pirmhelper: &::core::option::Option<IRMHelper>) -> ::windows::core::Result<()>;
}
impl IXAObtainRMInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAObtainRMInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXAObtainRMInfo_Vtbl {
        unsafe extern "system" fn ObtainRMInfo<Impl: IXAObtainRMInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirmhelper: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ObtainRMInfo(::core::mem::transmute(&pirmhelper)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ObtainRMInfo: ObtainRMInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAObtainRMInfo as ::windows::core::Interface>::IID
    }
}
pub trait IXATransLookup_Impl: Sized {
    fn Lookup(&mut self) -> ::windows::core::Result<ITransaction>;
}
impl IXATransLookup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXATransLookup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXATransLookup_Vtbl {
        unsafe extern "system" fn Lookup<Impl: IXATransLookup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Lookup() {
                ::core::result::Result::Ok(ok__) => {
                    *pptransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Lookup: Lookup::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXATransLookup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXATransLookup2_Impl: Sized {
    fn Lookup(&mut self, pxid: *const xid_t) -> ::windows::core::Result<ITransaction>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXATransLookup2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXATransLookup2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXATransLookup2_Vtbl {
        unsafe extern "system" fn Lookup<Impl: IXATransLookup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxid: *const xid_t, pptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Lookup(::core::mem::transmute_copy(&pxid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Lookup: Lookup::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXATransLookup2 as ::windows::core::Interface>::IID
    }
}
