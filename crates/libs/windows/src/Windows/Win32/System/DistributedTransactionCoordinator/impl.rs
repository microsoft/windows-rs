pub trait IDtcLuConfigureImpl: Sized {
    fn Add();
    fn Delete();
}
impl IDtcLuConfigureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuConfigureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuConfigureVtbl {
        unsafe extern "system" fn Add<Impl: IDtcLuConfigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puclupair: *const u8, cblupair: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IDtcLuConfigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puclupair: *const u8, cblupair: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Add: Add::<Impl, IMPL_OFFSET>, Delete: Delete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuConfigure as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuRecoveryImpl: Sized {}
impl IDtcLuRecoveryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuRecoveryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuRecoveryVtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRecovery as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuRecoveryFactoryImpl: Sized {
    fn Create();
}
impl IDtcLuRecoveryFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuRecoveryFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuRecoveryFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IDtcLuRecoveryFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puclupair: *const u8, cblupair: u32, pprecovery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRecoveryFactory as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuRecoveryInitiatedByDtcImpl: Sized {
    fn GetWork();
}
impl IDtcLuRecoveryInitiatedByDtcVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuRecoveryInitiatedByDtcImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuRecoveryInitiatedByDtcVtbl {
        unsafe extern "system" fn GetWork<Impl: IDtcLuRecoveryInitiatedByDtcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwork: *mut _DtcLu_LocalRecovery_Work, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetWork: GetWork::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByDtc as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuRecoveryInitiatedByDtcStatusWorkImpl: Sized {
    fn HandleCheckLuStatus();
}
impl IDtcLuRecoveryInitiatedByDtcStatusWorkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuRecoveryInitiatedByDtcStatusWorkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuRecoveryInitiatedByDtcStatusWorkVtbl {
        unsafe extern "system" fn HandleCheckLuStatus<Impl: IDtcLuRecoveryInitiatedByDtcStatusWorkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrecoveryseqnum: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), HandleCheckLuStatus: HandleCheckLuStatus::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByDtcStatusWork as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcLuRecoveryInitiatedByDtcTransWorkImpl: Sized {
    fn GetLogNameSizes();
    fn GetOurXln();
    fn HandleConfirmationFromOurXln();
    fn HandleTheirXlnResponse();
    fn HandleErrorFromOurXln();
    fn CheckForCompareStates();
    fn GetOurTransIdSize();
    fn GetOurCompareStates();
    fn HandleTheirCompareStatesResponse();
    fn HandleErrorFromOurCompareStates();
    fn ConversationLost();
    fn GetRecoverySeqNum();
    fn ObsoleteRecoverySeqNum();
}
#[cfg(feature = "Win32_Foundation")]
impl IDtcLuRecoveryInitiatedByDtcTransWorkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuRecoveryInitiatedByDtcTransWorkVtbl {
        unsafe extern "system" fn GetLogNameSizes<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbourlogname: *mut u32, pcbremotelogname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOurXln<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HandleConfirmationFromOurXln<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HandleTheirXlnResponse<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, dwprotocol: u32, pconfirmation: *mut _DtcLu_Xln_Confirmation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HandleErrorFromOurXln<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: _DtcLu_Xln_Error) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckForCompareStates<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcomparestates: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOurTransIdSize<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbourtransid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOurCompareStates<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pourtransid: *mut u8, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HandleTheirCompareStatesResponse<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparestate: _DtcLu_CompareState, pconfirmation: *mut _DtcLu_CompareStates_Confirmation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HandleErrorFromOurCompareStates<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: _DtcLu_CompareStates_Error) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConversationLost<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRecoverySeqNum<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plrecoveryseqnum: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ObsoleteRecoverySeqNum<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnewrecoveryseqnum: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IDtcLuRecoveryInitiatedByLuImpl: Sized {
    fn GetObjectToHandleWorkFromLu();
}
impl IDtcLuRecoveryInitiatedByLuVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuRecoveryInitiatedByLuImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuRecoveryInitiatedByLuVtbl {
        unsafe extern "system" fn GetObjectToHandleWorkFromLu<Impl: IDtcLuRecoveryInitiatedByLuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IDtcLuRecoveryInitiatedByLuWorkImpl: Sized {
    fn HandleTheirXln();
    fn GetOurLogNameSize();
    fn GetOurXln();
    fn HandleConfirmationOfOurXln();
    fn HandleTheirCompareStates();
    fn HandleConfirmationOfOurCompareStates();
    fn HandleErrorFromOurCompareStates();
    fn ConversationLost();
}
impl IDtcLuRecoveryInitiatedByLuWorkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuRecoveryInitiatedByLuWorkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuRecoveryInitiatedByLuWorkVtbl {
        unsafe extern "system" fn HandleTheirXln<Impl: IDtcLuRecoveryInitiatedByLuWorkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrecoveryseqnum: i32, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, pourlogname: *mut u8, cbourlogname: u32, dwprotocol: u32, presponse: *mut _DtcLu_Xln_Response) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOurLogNameSize<Impl: IDtcLuRecoveryInitiatedByLuWorkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbourlogname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOurXln<Impl: IDtcLuRecoveryInitiatedByLuWorkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HandleConfirmationOfOurXln<Impl: IDtcLuRecoveryInitiatedByLuWorkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HandleTheirCompareStates<Impl: IDtcLuRecoveryInitiatedByLuWorkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, premotetransid: *mut u8, cbremotetransid: u32, comparestate: _DtcLu_CompareState, presponse: *mut _DtcLu_CompareStates_Response, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HandleConfirmationOfOurCompareStates<Impl: IDtcLuRecoveryInitiatedByLuWorkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confirmation: _DtcLu_CompareStates_Confirmation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HandleErrorFromOurCompareStates<Impl: IDtcLuRecoveryInitiatedByLuWorkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: _DtcLu_CompareStates_Error) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConversationLost<Impl: IDtcLuRecoveryInitiatedByLuWorkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IDtcLuRmEnlistmentImpl: Sized {
    fn Unplug();
    fn BackedOut();
    fn BackOut();
    fn Committed();
    fn Forget();
    fn RequestCommit();
}
#[cfg(feature = "Win32_Foundation")]
impl IDtcLuRmEnlistmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuRmEnlistmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuRmEnlistmentVtbl {
        unsafe extern "system" fn Unplug<Impl: IDtcLuRmEnlistmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fconversationlost: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackedOut<Impl: IDtcLuRmEnlistmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackOut<Impl: IDtcLuRmEnlistmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Committed<Impl: IDtcLuRmEnlistmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Forget<Impl: IDtcLuRmEnlistmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestCommit<Impl: IDtcLuRmEnlistmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IDtcLuRmEnlistmentFactoryImpl: Sized {
    fn Create();
}
impl IDtcLuRmEnlistmentFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuRmEnlistmentFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuRmEnlistmentFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IDtcLuRmEnlistmentFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puclupair: *mut u8, cblupair: u32, pitransaction: ::windows::core::RawPtr, ptransid: *mut u8, cbtransid: u32, prmenlistmentsink: ::windows::core::RawPtr, pprmenlistment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuRmEnlistmentFactory as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuRmEnlistmentSinkImpl: Sized {
    fn AckUnplug();
    fn TmDown();
    fn SessionLost();
    fn BackedOut();
    fn BackOut();
    fn Committed();
    fn Forget();
    fn Prepare();
    fn RequestCommit();
}
impl IDtcLuRmEnlistmentSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuRmEnlistmentSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuRmEnlistmentSinkVtbl {
        unsafe extern "system" fn AckUnplug<Impl: IDtcLuRmEnlistmentSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TmDown<Impl: IDtcLuRmEnlistmentSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionLost<Impl: IDtcLuRmEnlistmentSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackedOut<Impl: IDtcLuRmEnlistmentSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackOut<Impl: IDtcLuRmEnlistmentSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Committed<Impl: IDtcLuRmEnlistmentSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Forget<Impl: IDtcLuRmEnlistmentSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Prepare<Impl: IDtcLuRmEnlistmentSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestCommit<Impl: IDtcLuRmEnlistmentSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IDtcLuSubordinateDtcImpl: Sized {
    fn Unplug();
    fn BackedOut();
    fn BackOut();
    fn Committed();
    fn Forget();
    fn Prepare();
    fn RequestCommit();
}
#[cfg(feature = "Win32_Foundation")]
impl IDtcLuSubordinateDtcVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuSubordinateDtcImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuSubordinateDtcVtbl {
        unsafe extern "system" fn Unplug<Impl: IDtcLuSubordinateDtcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fconversationlost: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackedOut<Impl: IDtcLuSubordinateDtcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackOut<Impl: IDtcLuSubordinateDtcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Committed<Impl: IDtcLuSubordinateDtcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Forget<Impl: IDtcLuSubordinateDtcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Prepare<Impl: IDtcLuSubordinateDtcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestCommit<Impl: IDtcLuSubordinateDtcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IDtcLuSubordinateDtcFactoryImpl: Sized {
    fn Create();
}
impl IDtcLuSubordinateDtcFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuSubordinateDtcFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuSubordinateDtcFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IDtcLuSubordinateDtcFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puclupair: *mut u8, cblupair: u32, punktransactionouter: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, poptions: ::windows::core::RawPtr, pptransaction: *mut ::windows::core::RawPtr, ptransid: *mut u8, cbtransid: u32, psubordinatedtcsink: ::windows::core::RawPtr, ppsubordinatedtc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcLuSubordinateDtcFactory as ::windows::core::Interface>::IID
    }
}
pub trait IDtcLuSubordinateDtcSinkImpl: Sized {
    fn AckUnplug();
    fn TmDown();
    fn SessionLost();
    fn BackedOut();
    fn BackOut();
    fn Committed();
    fn Forget();
    fn RequestCommit();
}
impl IDtcLuSubordinateDtcSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcLuSubordinateDtcSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcLuSubordinateDtcSinkVtbl {
        unsafe extern "system" fn AckUnplug<Impl: IDtcLuSubordinateDtcSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TmDown<Impl: IDtcLuSubordinateDtcSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionLost<Impl: IDtcLuSubordinateDtcSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackedOut<Impl: IDtcLuSubordinateDtcSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackOut<Impl: IDtcLuSubordinateDtcSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Committed<Impl: IDtcLuSubordinateDtcSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Forget<Impl: IDtcLuSubordinateDtcSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestCommit<Impl: IDtcLuSubordinateDtcSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IDtcNetworkAccessConfigImpl: Sized {
    fn GetAnyNetworkAccess();
    fn SetAnyNetworkAccess();
    fn GetNetworkAdministrationAccess();
    fn SetNetworkAdministrationAccess();
    fn GetNetworkTransactionAccess();
    fn SetNetworkTransactionAccess();
    fn GetNetworkClientAccess();
    fn SetNetworkClientAccess();
    fn GetNetworkTIPAccess();
    fn SetNetworkTIPAccess();
    fn GetXAAccess();
    fn SetXAAccess();
    fn RestartDtcService();
}
#[cfg(feature = "Win32_Foundation")]
impl IDtcNetworkAccessConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcNetworkAccessConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcNetworkAccessConfigVtbl {
        unsafe extern "system" fn GetAnyNetworkAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbanynetworkaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAnyNetworkAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, banynetworkaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNetworkAdministrationAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbnetworkadministrationaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNetworkAdministrationAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bnetworkadministrationaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNetworkTransactionAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbnetworktransactionaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNetworkTransactionAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bnetworktransactionaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNetworkClientAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbnetworkclientaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNetworkClientAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bnetworkclientaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNetworkTIPAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbnetworktipaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNetworkTIPAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bnetworktipaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetXAAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbxaaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetXAAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bxaaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RestartDtcService<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IDtcNetworkAccessConfig2Impl: Sized + IDtcNetworkAccessConfigImpl {
    fn GetNetworkInboundAccess();
    fn GetNetworkOutboundAccess();
    fn SetNetworkInboundAccess();
    fn SetNetworkOutboundAccess();
    fn GetAuthenticationLevel();
    fn SetAuthenticationLevel();
}
#[cfg(feature = "Win32_Foundation")]
impl IDtcNetworkAccessConfig2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcNetworkAccessConfig2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcNetworkAccessConfig2Vtbl {
        unsafe extern "system" fn GetNetworkInboundAccess<Impl: IDtcNetworkAccessConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbinbound: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNetworkOutboundAccess<Impl: IDtcNetworkAccessConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboutbound: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNetworkInboundAccess<Impl: IDtcNetworkAccessConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binbound: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNetworkOutboundAccess<Impl: IDtcNetworkAccessConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boutbound: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAuthenticationLevel<Impl: IDtcNetworkAccessConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauthlevel: *mut AUTHENTICATION_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAuthenticationLevel<Impl: IDtcNetworkAccessConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, authlevel: AUTHENTICATION_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDtcNetworkAccessConfigVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IDtcNetworkAccessConfig3Impl: Sized + IDtcNetworkAccessConfigImpl + IDtcNetworkAccessConfig2Impl {
    fn GetLUAccess();
    fn SetLUAccess();
}
#[cfg(feature = "Win32_Foundation")]
impl IDtcNetworkAccessConfig3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcNetworkAccessConfig3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcNetworkAccessConfig3Vtbl {
        unsafe extern "system" fn GetLUAccess<Impl: IDtcNetworkAccessConfig3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbluaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLUAccess<Impl: IDtcNetworkAccessConfig3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDtcNetworkAccessConfig2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetLUAccess: GetLUAccess::<Impl, IMPL_OFFSET>,
            SetLUAccess: SetLUAccess::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcNetworkAccessConfig3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcToXaHelperImpl: Sized {
    fn Close();
    fn TranslateTridToXid();
}
#[cfg(feature = "Win32_Foundation")]
impl IDtcToXaHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcToXaHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcToXaHelperVtbl {
        unsafe extern "system" fn Close<Impl: IDtcToXaHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i_fdorecovery: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TranslateTridToXid<Impl: IDtcToXaHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitransaction: ::windows::core::RawPtr, pguidbqual: *const ::windows::core::GUID, pxid: *mut xid_t) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IDtcToXaHelperFactoryImpl: Sized {
    fn Create();
}
#[cfg(feature = "Win32_Foundation")]
impl IDtcToXaHelperFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcToXaHelperFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcToXaHelperFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IDtcToXaHelperFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdsn: super::super::Foundation::PSTR, pszclientdllname: super::super::Foundation::PSTR, pguidrm: *mut ::windows::core::GUID, ppxahelper: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDtcToXaHelperFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDtcToXaHelperSinglePipeImpl: Sized {
    fn XARMCreate();
    fn ConvertTridToXID();
    fn EnlistWithRM();
    fn ReleaseRMCookie();
}
#[cfg(feature = "Win32_Foundation")]
impl IDtcToXaHelperSinglePipeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcToXaHelperSinglePipeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcToXaHelperSinglePipeVtbl {
        unsafe extern "system" fn XARMCreate<Impl: IDtcToXaHelperSinglePipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdsn: super::super::Foundation::PSTR, pszclientdll: super::super::Foundation::PSTR, pdwrmcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertTridToXID<Impl: IDtcToXaHelperSinglePipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwitrans: *mut u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnlistWithRM<Impl: IDtcToXaHelperSinglePipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwrmcookie: u32, i_pitransaction: ::windows::core::RawPtr, i_pitransres: ::windows::core::RawPtr, o_ppitransenslitment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseRMCookie<Impl: IDtcToXaHelperSinglePipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i_dwrmcookie: u32, i_fnormal: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IDtcToXaMapperImpl: Sized {
    fn RequestNewResourceManager();
    fn TranslateTridToXid();
    fn EnlistResourceManager();
    fn ReleaseResourceManager();
}
#[cfg(feature = "Win32_Foundation")]
impl IDtcToXaMapperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDtcToXaMapperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDtcToXaMapperVtbl {
        unsafe extern "system" fn RequestNewResourceManager<Impl: IDtcToXaMapperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdsn: super::super::Foundation::PSTR, pszclientdllname: super::super::Foundation::PSTR, pdwrmcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TranslateTridToXid<Impl: IDtcToXaMapperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwitransaction: *const u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnlistResourceManager<Impl: IDtcToXaMapperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwrmcookie: u32, pdwitransaction: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseResourceManager<Impl: IDtcToXaMapperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwrmcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IGetDispenserImpl: Sized {
    fn GetDispenser();
}
impl IGetDispenserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetDispenserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetDispenserVtbl {
        unsafe extern "system" fn GetDispenser<Impl: IGetDispenserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetDispenser: GetDispenser::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetDispenser as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IKernelTransactionImpl: Sized {
    fn GetHandle();
}
#[cfg(feature = "Win32_Foundation")]
impl IKernelTransactionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKernelTransactionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKernelTransactionVtbl {
        unsafe extern "system" fn GetHandle<Impl: IKernelTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetHandle: GetHandle::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKernelTransaction as ::windows::core::Interface>::IID
    }
}
pub trait ILastResourceManagerImpl: Sized {
    fn TransactionCommitted();
    fn RecoveryDone();
}
impl ILastResourceManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILastResourceManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILastResourceManagerVtbl {
        unsafe extern "system" fn TransactionCommitted<Impl: ILastResourceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RecoveryDone<Impl: ILastResourceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IPrepareInfoImpl: Sized {
    fn GetPrepareInfoSize();
    fn GetPrepareInfo();
}
impl IPrepareInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrepareInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrepareInfoVtbl {
        unsafe extern "system" fn GetPrepareInfoSize<Impl: IPrepareInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbprepinfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrepareInfo<Impl: IPrepareInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprepinfo: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IPrepareInfo2Impl: Sized {
    fn GetPrepareInfoSize();
    fn GetPrepareInfo();
}
impl IPrepareInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrepareInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrepareInfo2Vtbl {
        unsafe extern "system" fn GetPrepareInfoSize<Impl: IPrepareInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbprepinfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrepareInfo<Impl: IPrepareInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbprepareinfo: u32, pprepinfo: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IRMHelperImpl: Sized {
    fn RMCount();
    fn RMInfo();
}
#[cfg(feature = "Win32_Foundation")]
impl IRMHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRMHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRMHelperVtbl {
        unsafe extern "system" fn RMCount<Impl: IRMHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwctotalnumberofrms: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RMInfo<Impl: IRMHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxa_switch: *mut xa_switch_t, fcdeclcallingconv: super::super::Foundation::BOOL, pszopenstring: super::super::Foundation::PSTR, pszclosestring: super::super::Foundation::PSTR, guidrmrecovery: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), RMCount: RMCount::<Impl, IMPL_OFFSET>, RMInfo: RMInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRMHelper as ::windows::core::Interface>::IID
    }
}
pub trait IResourceManagerImpl: Sized {
    fn Enlist();
    fn Reenlist();
    fn ReenlistmentComplete();
    fn GetDistributedTransactionManager();
}
impl IResourceManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceManagerVtbl {
        unsafe extern "system" fn Enlist<Impl: IResourceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: ::windows::core::RawPtr, pres: ::windows::core::RawPtr, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reenlist<Impl: IResourceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReenlistmentComplete<Impl: IResourceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDistributedTransactionManager<Impl: IResourceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IResourceManager2Impl: Sized + IResourceManagerImpl {
    fn Enlist2();
    fn Reenlist2();
}
#[cfg(feature = "Win32_Foundation")]
impl IResourceManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceManager2Vtbl {
        unsafe extern "system" fn Enlist2<Impl: IResourceManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: ::windows::core::RawPtr, presasync: ::windows::core::RawPtr, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut xid_t, ppenlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reenlist2<Impl: IResourceManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxid: *const xid_t, dwtimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IResourceManagerVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Enlist2: Enlist2::<Impl, IMPL_OFFSET>,
            Reenlist2: Reenlist2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IResourceManagerFactoryImpl: Sized {
    fn Create();
}
#[cfg(feature = "Win32_Foundation")]
impl IResourceManagerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceManagerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceManagerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IResourceManagerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidrm: *const ::windows::core::GUID, pszrmname: super::super::Foundation::PSTR, piresmgrsink: ::windows::core::RawPtr, ppresmgr: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceManagerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IResourceManagerFactory2Impl: Sized + IResourceManagerFactoryImpl {
    fn CreateEx();
}
#[cfg(feature = "Win32_Foundation")]
impl IResourceManagerFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceManagerFactory2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceManagerFactory2Vtbl {
        unsafe extern "system" fn CreateEx<Impl: IResourceManagerFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidrm: *const ::windows::core::GUID, pszrmname: super::super::Foundation::PSTR, piresmgrsink: ::windows::core::RawPtr, riidrequested: *const ::windows::core::GUID, ppvresmgr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IResourceManagerFactoryVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), CreateEx: CreateEx::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceManagerFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IResourceManagerRejoinableImpl: Sized + IResourceManagerImpl + IResourceManager2Impl {
    fn Rejoin();
}
#[cfg(feature = "Win32_Foundation")]
impl IResourceManagerRejoinableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceManagerRejoinableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceManagerRejoinableVtbl {
        unsafe extern "system" fn Rejoin<Impl: IResourceManagerRejoinableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IResourceManager2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Rejoin: Rejoin::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceManagerRejoinable as ::windows::core::Interface>::IID
    }
}
pub trait IResourceManagerSinkImpl: Sized {
    fn TMDown();
}
impl IResourceManagerSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceManagerSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceManagerSinkVtbl {
        unsafe extern "system" fn TMDown<Impl: IResourceManagerSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), TMDown: TMDown::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceManagerSink as ::windows::core::Interface>::IID
    }
}
pub trait ITipHelperImpl: Sized {
    fn Pull();
    fn PullAsync();
    fn GetLocalTmUrl();
}
impl ITipHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITipHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITipHelperVtbl {
        unsafe extern "system" fn Pull<Impl: ITipHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i_psztxurl: *const u8, o_ppitransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PullAsync<Impl: ITipHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i_psztxurl: *const u8, i_ptippullsink: ::windows::core::RawPtr, o_ppitransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocalTmUrl<Impl: ITipHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, o_ppszlocaltmurl: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITipPullSinkImpl: Sized {
    fn PullComplete();
}
impl ITipPullSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITipPullSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITipPullSinkVtbl {
        unsafe extern "system" fn PullComplete<Impl: ITipPullSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i_hrpull: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), PullComplete: PullComplete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITipPullSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITipTransactionImpl: Sized {
    fn Push();
    fn GetTransactionUrl();
}
#[cfg(feature = "Win32_Foundation")]
impl ITipTransactionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITipTransactionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITipTransactionVtbl {
        unsafe extern "system" fn Push<Impl: ITipTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i_pszremotetmurl: *const u8, o_ppszremotetxurl: *mut super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransactionUrl<Impl: ITipTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, o_ppszlocaltxurl: *mut super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITmNodeNameImpl: Sized {
    fn GetNodeNameSize();
    fn GetNodeName();
}
#[cfg(feature = "Win32_Foundation")]
impl ITmNodeNameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITmNodeNameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITmNodeNameVtbl {
        unsafe extern "system" fn GetNodeNameSize<Impl: ITmNodeNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbnodenamesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNodeName<Impl: ITmNodeNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbnodenamebuffersize: u32, pnodenamebuffer: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITransactionImpl: Sized {
    fn Commit();
    fn Abort();
    fn GetTransactionInfo();
}
#[cfg(feature = "Win32_Foundation")]
impl ITransactionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionVtbl {
        unsafe extern "system" fn Commit<Impl: ITransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grftc: u32, grfrm: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Abort<Impl: ITransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, fasync: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransactionInfo<Impl: ITransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut XACTTRANSINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITransaction2Impl: Sized + ITransactionImpl + ITransactionClonerImpl {
    fn GetTransactionInfo2();
}
#[cfg(feature = "Win32_Foundation")]
impl ITransaction2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransaction2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransaction2Vtbl {
        unsafe extern "system" fn GetTransactionInfo2<Impl: ITransaction2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut XACTTRANSINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ITransactionClonerVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetTransactionInfo2: GetTransactionInfo2::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransaction2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITransactionClonerImpl: Sized + ITransactionImpl {
    fn CloneWithCommitDisabled();
}
#[cfg(feature = "Win32_Foundation")]
impl ITransactionClonerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionClonerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionClonerVtbl {
        unsafe extern "system" fn CloneWithCommitDisabled<Impl: ITransactionClonerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ITransactionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), CloneWithCommitDisabled: CloneWithCommitDisabled::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionCloner as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionDispenserImpl: Sized {
    fn GetOptionsObject();
    fn BeginTransaction();
}
impl ITransactionDispenserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionDispenserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionDispenserVtbl {
        unsafe extern "system" fn GetOptionsObject<Impl: ITransactionDispenserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginTransaction<Impl: ITransactionDispenserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, poptions: ::windows::core::RawPtr, pptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITransactionEnlistmentAsyncImpl: Sized {
    fn PrepareRequestDone();
    fn CommitRequestDone();
    fn AbortRequestDone();
}
#[cfg(feature = "Win32_System_Com")]
impl ITransactionEnlistmentAsyncVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionEnlistmentAsyncImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionEnlistmentAsyncVtbl {
        unsafe extern "system" fn PrepareRequestDone<Impl: ITransactionEnlistmentAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, pmk: ::windows::core::RawPtr, pboidreason: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CommitRequestDone<Impl: ITransactionEnlistmentAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AbortRequestDone<Impl: ITransactionEnlistmentAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITransactionExportImpl: Sized {
    fn Export();
    fn GetTransactionCookie();
}
impl ITransactionExportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionExportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionExportVtbl {
        unsafe extern "system" fn Export<Impl: ITransactionExportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punktransaction: *mut ::core::ffi::c_void, pcbtransactioncookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransactionCookie<Impl: ITransactionExportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punktransaction: *mut ::core::ffi::c_void, cbtransactioncookie: u32, rgbtransactioncookie: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITransactionExportFactoryImpl: Sized {
    fn GetRemoteClassId();
    fn Create();
}
impl ITransactionExportFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionExportFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionExportFactoryVtbl {
        unsafe extern "system" fn GetRemoteClassId<Impl: ITransactionExportFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Create<Impl: ITransactionExportFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbwhereabouts: u32, rgbwhereabouts: *const u8, ppexport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITransactionImportImpl: Sized {
    fn Import();
}
impl ITransactionImportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionImportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionImportVtbl {
        unsafe extern "system" fn Import<Impl: ITransactionImportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbtransactioncookie: u32, rgbtransactioncookie: *const u8, piid: *const ::windows::core::GUID, ppvtransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Import: Import::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionImport as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionImportWhereaboutsImpl: Sized {
    fn GetWhereaboutsSize();
    fn GetWhereabouts();
}
impl ITransactionImportWhereaboutsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionImportWhereaboutsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionImportWhereaboutsVtbl {
        unsafe extern "system" fn GetWhereaboutsSize<Impl: ITransactionImportWhereaboutsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbwhereabouts: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWhereabouts<Impl: ITransactionImportWhereaboutsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbwhereabouts: u32, rgbwhereabouts: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITransactionLastEnlistmentAsyncImpl: Sized {
    fn TransactionOutcome();
}
impl ITransactionLastEnlistmentAsyncVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionLastEnlistmentAsyncImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionLastEnlistmentAsyncVtbl {
        unsafe extern "system" fn TransactionOutcome<Impl: ITransactionLastEnlistmentAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xactstat: XACTSTAT, pboidreason: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), TransactionOutcome: TransactionOutcome::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionLastEnlistmentAsync as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionLastResourceAsyncImpl: Sized {
    fn DelegateCommit();
    fn ForgetRequest();
}
impl ITransactionLastResourceAsyncVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionLastResourceAsyncImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionLastResourceAsyncVtbl {
        unsafe extern "system" fn DelegateCommit<Impl: ITransactionLastResourceAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfrm: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ForgetRequest<Impl: ITransactionLastResourceAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewuow: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITransactionOptionsImpl: Sized {
    fn SetOptions();
    fn GetOptions();
}
impl ITransactionOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionOptionsVtbl {
        unsafe extern "system" fn SetOptions<Impl: ITransactionOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poptions: *const XACTOPT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOptions<Impl: ITransactionOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poptions: *mut XACTOPT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITransactionOutcomeEventsImpl: Sized {
    fn Committed();
    fn Aborted();
    fn HeuristicDecision();
    fn Indoubt();
}
#[cfg(feature = "Win32_Foundation")]
impl ITransactionOutcomeEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionOutcomeEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionOutcomeEventsVtbl {
        unsafe extern "system" fn Committed<Impl: ITransactionOutcomeEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Aborted<Impl: ITransactionOutcomeEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HeuristicDecision<Impl: ITransactionOutcomeEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdecision: u32, pboidreason: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Indoubt<Impl: ITransactionOutcomeEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITransactionPhase0EnlistmentAsyncImpl: Sized {
    fn Enable();
    fn WaitForEnlistment();
    fn Phase0Done();
    fn Unenlist();
    fn GetTransaction();
}
impl ITransactionPhase0EnlistmentAsyncVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionPhase0EnlistmentAsyncImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionPhase0EnlistmentAsyncVtbl {
        unsafe extern "system" fn Enable<Impl: ITransactionPhase0EnlistmentAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WaitForEnlistment<Impl: ITransactionPhase0EnlistmentAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Phase0Done<Impl: ITransactionPhase0EnlistmentAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unenlist<Impl: ITransactionPhase0EnlistmentAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransaction<Impl: ITransactionPhase0EnlistmentAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITransactionPhase0FactoryImpl: Sized {
    fn Create();
}
impl ITransactionPhase0FactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionPhase0FactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionPhase0FactoryVtbl {
        unsafe extern "system" fn Create<Impl: ITransactionPhase0FactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphase0notify: ::windows::core::RawPtr, ppphase0enlistment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionPhase0Factory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITransactionPhase0NotifyAsyncImpl: Sized {
    fn Phase0Request();
    fn EnlistCompleted();
}
#[cfg(feature = "Win32_Foundation")]
impl ITransactionPhase0NotifyAsyncVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionPhase0NotifyAsyncImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionPhase0NotifyAsyncVtbl {
        unsafe extern "system" fn Phase0Request<Impl: ITransactionPhase0NotifyAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fabortinghint: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnlistCompleted<Impl: ITransactionPhase0NotifyAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITransactionReceiverImpl: Sized {
    fn UnmarshalPropagationToken();
    fn GetReturnTokenSize();
    fn MarshalReturnToken();
    fn Reset();
}
impl ITransactionReceiverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionReceiverImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionReceiverVtbl {
        unsafe extern "system" fn UnmarshalPropagationToken<Impl: ITransactionReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbtoken: u32, rgbtoken: *const u8, pptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReturnTokenSize<Impl: ITransactionReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbreturntoken: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MarshalReturnToken<Impl: ITransactionReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbreturntoken: u32, rgbreturntoken: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: ITransactionReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITransactionReceiverFactoryImpl: Sized {
    fn Create();
}
impl ITransactionReceiverFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionReceiverFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionReceiverFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ITransactionReceiverFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppreceiver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionReceiverFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITransactionResourceImpl: Sized {
    fn PrepareRequest();
    fn CommitRequest();
    fn AbortRequest();
    fn TMDown();
}
#[cfg(feature = "Win32_Foundation")]
impl ITransactionResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionResourceVtbl {
        unsafe extern "system" fn PrepareRequest<Impl: ITransactionResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CommitRequest<Impl: ITransactionResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfrm: u32, pnewuow: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AbortRequest<Impl: ITransactionResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TMDown<Impl: ITransactionResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITransactionResourceAsyncImpl: Sized {
    fn PrepareRequest();
    fn CommitRequest();
    fn AbortRequest();
    fn TMDown();
}
#[cfg(feature = "Win32_Foundation")]
impl ITransactionResourceAsyncVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionResourceAsyncImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionResourceAsyncVtbl {
        unsafe extern "system" fn PrepareRequest<Impl: ITransactionResourceAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CommitRequest<Impl: ITransactionResourceAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfrm: u32, pnewuow: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AbortRequest<Impl: ITransactionResourceAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TMDown<Impl: ITransactionResourceAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITransactionTransmitterImpl: Sized {
    fn Set();
    fn GetPropagationTokenSize();
    fn MarshalPropagationToken();
    fn UnmarshalReturnToken();
    fn Reset();
}
impl ITransactionTransmitterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionTransmitterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionTransmitterVtbl {
        unsafe extern "system" fn Set<Impl: ITransactionTransmitterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropagationTokenSize<Impl: ITransactionTransmitterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbtoken: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MarshalPropagationToken<Impl: ITransactionTransmitterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbtoken: u32, rgbtoken: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnmarshalReturnToken<Impl: ITransactionTransmitterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbreturntoken: u32, rgbreturntoken: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: ITransactionTransmitterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITransactionTransmitterFactoryImpl: Sized {
    fn Create();
}
impl ITransactionTransmitterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionTransmitterFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionTransmitterFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ITransactionTransmitterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptransmitter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionTransmitterFactory as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionVoterBallotAsync2Impl: Sized {
    fn VoteRequestDone();
}
impl ITransactionVoterBallotAsync2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionVoterBallotAsync2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionVoterBallotAsync2Vtbl {
        unsafe extern "system" fn VoteRequestDone<Impl: ITransactionVoterBallotAsync2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, pboidreason: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), VoteRequestDone: VoteRequestDone::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionVoterBallotAsync2 as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionVoterFactory2Impl: Sized {
    fn Create();
}
impl ITransactionVoterFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionVoterFactory2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionVoterFactory2Vtbl {
        unsafe extern "system" fn Create<Impl: ITransactionVoterFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: ::windows::core::RawPtr, pvoternotify: ::windows::core::RawPtr, ppvoterballot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionVoterFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITransactionVoterNotifyAsync2Impl: Sized + ITransactionOutcomeEventsImpl {
    fn VoteRequest();
}
#[cfg(feature = "Win32_Foundation")]
impl ITransactionVoterNotifyAsync2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionVoterNotifyAsync2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionVoterNotifyAsync2Vtbl {
        unsafe extern "system" fn VoteRequest<Impl: ITransactionVoterNotifyAsync2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ITransactionOutcomeEventsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), VoteRequest: VoteRequest::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionVoterNotifyAsync2 as ::windows::core::Interface>::IID
    }
}
pub trait IXAConfigImpl: Sized {
    fn Initialize();
    fn Terminate();
}
impl IXAConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXAConfigVtbl {
        unsafe extern "system" fn Initialize<Impl: IXAConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidhelperdll: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Terminate<Impl: IXAConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IXAObtainRMInfoImpl: Sized {
    fn ObtainRMInfo();
}
impl IXAObtainRMInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAObtainRMInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXAObtainRMInfoVtbl {
        unsafe extern "system" fn ObtainRMInfo<Impl: IXAObtainRMInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirmhelper: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ObtainRMInfo: ObtainRMInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAObtainRMInfo as ::windows::core::Interface>::IID
    }
}
pub trait IXATransLookupImpl: Sized {
    fn Lookup();
}
impl IXATransLookupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXATransLookupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXATransLookupVtbl {
        unsafe extern "system" fn Lookup<Impl: IXATransLookupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Lookup: Lookup::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXATransLookup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXATransLookup2Impl: Sized {
    fn Lookup();
}
#[cfg(feature = "Win32_Foundation")]
impl IXATransLookup2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXATransLookup2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXATransLookup2Vtbl {
        unsafe extern "system" fn Lookup<Impl: IXATransLookup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxid: *const xid_t, pptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Lookup: Lookup::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXATransLookup2 as ::windows::core::Interface>::IID
    }
}
