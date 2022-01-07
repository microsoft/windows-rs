pub trait IDtcLuConfigureImpl: Sized {
    fn Add();
    fn Delete();
}
impl ::windows::core::RuntimeName for IDtcLuConfigure {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IDtcLuConfigure";
}
impl IDtcLuConfigureVtbl {
    pub const fn new<Impl: IDtcLuConfigureImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDtcLuConfigureVtbl {
        unsafe extern "system" fn Add<Impl: IDtcLuConfigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puclupair: *const u8, cblupair: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Add(puclupair, cblupair) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IDtcLuConfigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puclupair: *const u8, cblupair: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delete(puclupair, cblupair) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDtcLuConfigure>, base.5, Add::<Impl, OFFSET>, Delete::<Impl, OFFSET>)
    }
}
pub trait IDtcLuRecoveryImpl: Sized {}
impl ::windows::core::RuntimeName for IDtcLuRecovery {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IDtcLuRecovery";
}
impl IDtcLuRecoveryVtbl {
    pub const fn new<Impl: IDtcLuRecoveryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDtcLuRecoveryVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDtcLuRecovery>, base.5)
    }
}
pub trait IDtcLuRecoveryFactoryImpl: Sized {
    fn Create();
}
impl ::windows::core::RuntimeName for IDtcLuRecoveryFactory {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IDtcLuRecoveryFactory";
}
impl IDtcLuRecoveryFactoryVtbl {
    pub const fn new<Impl: IDtcLuRecoveryFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDtcLuRecoveryFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IDtcLuRecoveryFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puclupair: *const u8, cblupair: u32, pprecovery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(puclupair, cblupair, ::core::mem::transmute_copy(&pprecovery)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDtcLuRecoveryFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
pub trait IDtcLuRecoveryInitiatedByDtcImpl: Sized {
    fn GetWork();
}
impl ::windows::core::RuntimeName for IDtcLuRecoveryInitiatedByDtc {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IDtcLuRecoveryInitiatedByDtc";
}
impl IDtcLuRecoveryInitiatedByDtcVtbl {
    pub const fn new<Impl: IDtcLuRecoveryInitiatedByDtcImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDtcLuRecoveryInitiatedByDtcVtbl {
        unsafe extern "system" fn GetWork<Impl: IDtcLuRecoveryInitiatedByDtcImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwork: *mut _DtcLu_LocalRecovery_Work, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetWork(pwork, &*(&ppv as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDtcLuRecoveryInitiatedByDtc>, base.5, GetWork::<Impl, OFFSET>)
    }
}
pub trait IDtcLuRecoveryInitiatedByDtcStatusWorkImpl: Sized {
    fn HandleCheckLuStatus();
}
impl ::windows::core::RuntimeName for IDtcLuRecoveryInitiatedByDtcStatusWork {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IDtcLuRecoveryInitiatedByDtcStatusWork";
}
impl IDtcLuRecoveryInitiatedByDtcStatusWorkVtbl {
    pub const fn new<Impl: IDtcLuRecoveryInitiatedByDtcStatusWorkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDtcLuRecoveryInitiatedByDtcStatusWorkVtbl {
        unsafe extern "system" fn HandleCheckLuStatus<Impl: IDtcLuRecoveryInitiatedByDtcStatusWorkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lrecoveryseqnum: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HandleCheckLuStatus(lrecoveryseqnum) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDtcLuRecoveryInitiatedByDtcStatusWork>, base.5, HandleCheckLuStatus::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDtcLuRecoveryInitiatedByDtcTransWork {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IDtcLuRecoveryInitiatedByDtcTransWork";
}
impl IDtcLuRecoveryInitiatedByDtcTransWorkVtbl {
    pub const fn new<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDtcLuRecoveryInitiatedByDtcTransWorkVtbl {
        unsafe extern "system" fn GetLogNameSizes<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbourlogname: *mut u32, pcbremotelogname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLogNameSizes(pcbourlogname, pcbremotelogname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOurXln<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOurXln(pxln, pourlogname, premotelogname, pdwprotocol) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HandleConfirmationFromOurXln<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HandleConfirmationFromOurXln(confirmation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HandleTheirXlnResponse<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, dwprotocol: u32, pconfirmation: *mut _DtcLu_Xln_Confirmation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HandleTheirXlnResponse(xln, premotelogname, cbremotelogname, dwprotocol, pconfirmation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HandleErrorFromOurXln<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, error: _DtcLu_Xln_Error) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HandleErrorFromOurXln(error) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckForCompareStates<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fcomparestates: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckForCompareStates(&*(&fcomparestates as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOurTransIdSize<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbourtransid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOurTransIdSize(pcbourtransid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOurCompareStates<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pourtransid: *mut u8, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOurCompareStates(pourtransid, pcomparestate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HandleTheirCompareStatesResponse<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, comparestate: _DtcLu_CompareState, pconfirmation: *mut _DtcLu_CompareStates_Confirmation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HandleTheirCompareStatesResponse(comparestate, pconfirmation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HandleErrorFromOurCompareStates<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, error: _DtcLu_CompareStates_Error) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HandleErrorFromOurCompareStates(error) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConversationLost<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConversationLost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecoverySeqNum<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plrecoveryseqnum: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRecoverySeqNum(plrecoveryseqnum) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ObsoleteRecoverySeqNum<Impl: IDtcLuRecoveryInitiatedByDtcTransWorkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnewrecoveryseqnum: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ObsoleteRecoverySeqNum(lnewrecoveryseqnum) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDtcLuRecoveryInitiatedByDtcTransWork>,
            base.5,
            GetLogNameSizes::<Impl, OFFSET>,
            GetOurXln::<Impl, OFFSET>,
            HandleConfirmationFromOurXln::<Impl, OFFSET>,
            HandleTheirXlnResponse::<Impl, OFFSET>,
            HandleErrorFromOurXln::<Impl, OFFSET>,
            CheckForCompareStates::<Impl, OFFSET>,
            GetOurTransIdSize::<Impl, OFFSET>,
            GetOurCompareStates::<Impl, OFFSET>,
            HandleTheirCompareStatesResponse::<Impl, OFFSET>,
            HandleErrorFromOurCompareStates::<Impl, OFFSET>,
            ConversationLost::<Impl, OFFSET>,
            GetRecoverySeqNum::<Impl, OFFSET>,
            ObsoleteRecoverySeqNum::<Impl, OFFSET>,
        )
    }
}
pub trait IDtcLuRecoveryInitiatedByLuImpl: Sized {
    fn GetObjectToHandleWorkFromLu();
}
impl ::windows::core::RuntimeName for IDtcLuRecoveryInitiatedByLu {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IDtcLuRecoveryInitiatedByLu";
}
impl IDtcLuRecoveryInitiatedByLuVtbl {
    pub const fn new<Impl: IDtcLuRecoveryInitiatedByLuImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDtcLuRecoveryInitiatedByLuVtbl {
        unsafe extern "system" fn GetObjectToHandleWorkFromLu<Impl: IDtcLuRecoveryInitiatedByLuImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetObjectToHandleWorkFromLu(::core::mem::transmute_copy(&ppwork)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDtcLuRecoveryInitiatedByLu>, base.5, GetObjectToHandleWorkFromLu::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IDtcLuRecoveryInitiatedByLuWork {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IDtcLuRecoveryInitiatedByLuWork";
}
impl IDtcLuRecoveryInitiatedByLuWorkVtbl {
    pub const fn new<Impl: IDtcLuRecoveryInitiatedByLuWorkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDtcLuRecoveryInitiatedByLuWorkVtbl {
        unsafe extern "system" fn HandleTheirXln<Impl: IDtcLuRecoveryInitiatedByLuWorkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lrecoveryseqnum: i32, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, pourlogname: *mut u8, cbourlogname: u32, dwprotocol: u32, presponse: *mut _DtcLu_Xln_Response) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HandleTheirXln(lrecoveryseqnum, xln, premotelogname, cbremotelogname, pourlogname, cbourlogname, dwprotocol, presponse) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOurLogNameSize<Impl: IDtcLuRecoveryInitiatedByLuWorkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbourlogname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOurLogNameSize(pcbourlogname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOurXln<Impl: IDtcLuRecoveryInitiatedByLuWorkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOurXln(pxln, pourlogname, pdwprotocol) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HandleConfirmationOfOurXln<Impl: IDtcLuRecoveryInitiatedByLuWorkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HandleConfirmationOfOurXln(confirmation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HandleTheirCompareStates<Impl: IDtcLuRecoveryInitiatedByLuWorkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, premotetransid: *mut u8, cbremotetransid: u32, comparestate: _DtcLu_CompareState, presponse: *mut _DtcLu_CompareStates_Response, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HandleTheirCompareStates(premotetransid, cbremotetransid, comparestate, presponse, pcomparestate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HandleConfirmationOfOurCompareStates<Impl: IDtcLuRecoveryInitiatedByLuWorkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, confirmation: _DtcLu_CompareStates_Confirmation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HandleConfirmationOfOurCompareStates(confirmation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HandleErrorFromOurCompareStates<Impl: IDtcLuRecoveryInitiatedByLuWorkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, error: _DtcLu_CompareStates_Error) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HandleErrorFromOurCompareStates(error) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConversationLost<Impl: IDtcLuRecoveryInitiatedByLuWorkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConversationLost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDtcLuRecoveryInitiatedByLuWork>, base.5, HandleTheirXln::<Impl, OFFSET>, GetOurLogNameSize::<Impl, OFFSET>, GetOurXln::<Impl, OFFSET>, HandleConfirmationOfOurXln::<Impl, OFFSET>, HandleTheirCompareStates::<Impl, OFFSET>, HandleConfirmationOfOurCompareStates::<Impl, OFFSET>, HandleErrorFromOurCompareStates::<Impl, OFFSET>, ConversationLost::<Impl, OFFSET>)
    }
}
pub trait IDtcLuRmEnlistmentImpl: Sized {
    fn Unplug();
    fn BackedOut();
    fn BackOut();
    fn Committed();
    fn Forget();
    fn RequestCommit();
}
impl ::windows::core::RuntimeName for IDtcLuRmEnlistment {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IDtcLuRmEnlistment";
}
impl IDtcLuRmEnlistmentVtbl {
    pub const fn new<Impl: IDtcLuRmEnlistmentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDtcLuRmEnlistmentVtbl {
        unsafe extern "system" fn Unplug<Impl: IDtcLuRmEnlistmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fconversationlost: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unplug(&*(&fconversationlost as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackedOut<Impl: IDtcLuRmEnlistmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BackedOut() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackOut<Impl: IDtcLuRmEnlistmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BackOut() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Committed<Impl: IDtcLuRmEnlistmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Committed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Forget<Impl: IDtcLuRmEnlistmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Forget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCommit<Impl: IDtcLuRmEnlistmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestCommit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDtcLuRmEnlistment>, base.5, Unplug::<Impl, OFFSET>, BackedOut::<Impl, OFFSET>, BackOut::<Impl, OFFSET>, Committed::<Impl, OFFSET>, Forget::<Impl, OFFSET>, RequestCommit::<Impl, OFFSET>)
    }
}
pub trait IDtcLuRmEnlistmentFactoryImpl: Sized {
    fn Create();
}
impl ::windows::core::RuntimeName for IDtcLuRmEnlistmentFactory {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IDtcLuRmEnlistmentFactory";
}
impl IDtcLuRmEnlistmentFactoryVtbl {
    pub const fn new<Impl: IDtcLuRmEnlistmentFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDtcLuRmEnlistmentFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IDtcLuRmEnlistmentFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puclupair: *mut u8, cblupair: u32, pitransaction: ::windows::core::RawPtr, ptransid: *mut u8, cbtransid: u32, prmenlistmentsink: ::windows::core::RawPtr, pprmenlistment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(puclupair, cblupair, &*(&pitransaction as *const <ITransaction as ::windows::core::Abi>::Abi as *const <ITransaction as ::windows::core::DefaultType>::DefaultType), ptransid, cbtransid, &*(&prmenlistmentsink as *const <IDtcLuRmEnlistmentSink as ::windows::core::Abi>::Abi as *const <IDtcLuRmEnlistmentSink as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprmenlistment)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDtcLuRmEnlistmentFactory>, base.5, Create::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IDtcLuRmEnlistmentSink {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IDtcLuRmEnlistmentSink";
}
impl IDtcLuRmEnlistmentSinkVtbl {
    pub const fn new<Impl: IDtcLuRmEnlistmentSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDtcLuRmEnlistmentSinkVtbl {
        unsafe extern "system" fn AckUnplug<Impl: IDtcLuRmEnlistmentSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AckUnplug() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TmDown<Impl: IDtcLuRmEnlistmentSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TmDown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionLost<Impl: IDtcLuRmEnlistmentSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SessionLost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackedOut<Impl: IDtcLuRmEnlistmentSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BackedOut() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackOut<Impl: IDtcLuRmEnlistmentSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BackOut() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Committed<Impl: IDtcLuRmEnlistmentSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Committed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Forget<Impl: IDtcLuRmEnlistmentSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Forget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Prepare<Impl: IDtcLuRmEnlistmentSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Prepare() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCommit<Impl: IDtcLuRmEnlistmentSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestCommit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDtcLuRmEnlistmentSink>, base.5, AckUnplug::<Impl, OFFSET>, TmDown::<Impl, OFFSET>, SessionLost::<Impl, OFFSET>, BackedOut::<Impl, OFFSET>, BackOut::<Impl, OFFSET>, Committed::<Impl, OFFSET>, Forget::<Impl, OFFSET>, Prepare::<Impl, OFFSET>, RequestCommit::<Impl, OFFSET>)
    }
}
pub trait IDtcLuSubordinateDtcImpl: Sized {
    fn Unplug();
    fn BackedOut();
    fn BackOut();
    fn Committed();
    fn Forget();
    fn Prepare();
    fn RequestCommit();
}
impl ::windows::core::RuntimeName for IDtcLuSubordinateDtc {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IDtcLuSubordinateDtc";
}
impl IDtcLuSubordinateDtcVtbl {
    pub const fn new<Impl: IDtcLuSubordinateDtcImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDtcLuSubordinateDtcVtbl {
        unsafe extern "system" fn Unplug<Impl: IDtcLuSubordinateDtcImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fconversationlost: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unplug(&*(&fconversationlost as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackedOut<Impl: IDtcLuSubordinateDtcImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BackedOut() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackOut<Impl: IDtcLuSubordinateDtcImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BackOut() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Committed<Impl: IDtcLuSubordinateDtcImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Committed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Forget<Impl: IDtcLuSubordinateDtcImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Forget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Prepare<Impl: IDtcLuSubordinateDtcImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Prepare() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCommit<Impl: IDtcLuSubordinateDtcImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestCommit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDtcLuSubordinateDtc>, base.5, Unplug::<Impl, OFFSET>, BackedOut::<Impl, OFFSET>, BackOut::<Impl, OFFSET>, Committed::<Impl, OFFSET>, Forget::<Impl, OFFSET>, Prepare::<Impl, OFFSET>, RequestCommit::<Impl, OFFSET>)
    }
}
pub trait IDtcLuSubordinateDtcFactoryImpl: Sized {
    fn Create();
}
impl ::windows::core::RuntimeName for IDtcLuSubordinateDtcFactory {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IDtcLuSubordinateDtcFactory";
}
impl IDtcLuSubordinateDtcFactoryVtbl {
    pub const fn new<Impl: IDtcLuSubordinateDtcFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDtcLuSubordinateDtcFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IDtcLuSubordinateDtcFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puclupair: *mut u8, cblupair: u32, punktransactionouter: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, poptions: ::windows::core::RawPtr, pptransaction: *mut ::windows::core::RawPtr, ptransid: *mut u8, cbtransid: u32, psubordinatedtcsink: ::windows::core::RawPtr, ppsubordinatedtc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(
                puclupair,
                cblupair,
                &*(&punktransactionouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                isolevel,
                isoflags,
                &*(&poptions as *const <ITransactionOptions as ::windows::core::Abi>::Abi as *const <ITransactionOptions as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pptransaction),
                ptransid,
                cbtransid,
                &*(&psubordinatedtcsink as *const <IDtcLuSubordinateDtcSink as ::windows::core::Abi>::Abi as *const <IDtcLuSubordinateDtcSink as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppsubordinatedtc),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDtcLuSubordinateDtcFactory>, base.5, Create::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IDtcLuSubordinateDtcSink {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IDtcLuSubordinateDtcSink";
}
impl IDtcLuSubordinateDtcSinkVtbl {
    pub const fn new<Impl: IDtcLuSubordinateDtcSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDtcLuSubordinateDtcSinkVtbl {
        unsafe extern "system" fn AckUnplug<Impl: IDtcLuSubordinateDtcSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AckUnplug() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TmDown<Impl: IDtcLuSubordinateDtcSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TmDown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionLost<Impl: IDtcLuSubordinateDtcSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SessionLost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackedOut<Impl: IDtcLuSubordinateDtcSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BackedOut() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackOut<Impl: IDtcLuSubordinateDtcSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BackOut() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Committed<Impl: IDtcLuSubordinateDtcSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Committed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Forget<Impl: IDtcLuSubordinateDtcSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Forget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCommit<Impl: IDtcLuSubordinateDtcSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestCommit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDtcLuSubordinateDtcSink>, base.5, AckUnplug::<Impl, OFFSET>, TmDown::<Impl, OFFSET>, SessionLost::<Impl, OFFSET>, BackedOut::<Impl, OFFSET>, BackOut::<Impl, OFFSET>, Committed::<Impl, OFFSET>, Forget::<Impl, OFFSET>, RequestCommit::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDtcNetworkAccessConfig {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IDtcNetworkAccessConfig";
}
impl IDtcNetworkAccessConfigVtbl {
    pub const fn new<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDtcNetworkAccessConfigVtbl {
        unsafe extern "system" fn GetAnyNetworkAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbanynetworkaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAnyNetworkAccess(::core::mem::transmute_copy(&pbanynetworkaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnyNetworkAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, banynetworkaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAnyNetworkAccess(&*(&banynetworkaccess as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkAdministrationAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbnetworkadministrationaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNetworkAdministrationAccess(::core::mem::transmute_copy(&pbnetworkadministrationaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkAdministrationAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bnetworkadministrationaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNetworkAdministrationAccess(&*(&bnetworkadministrationaccess as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkTransactionAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbnetworktransactionaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNetworkTransactionAccess(::core::mem::transmute_copy(&pbnetworktransactionaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkTransactionAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bnetworktransactionaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNetworkTransactionAccess(&*(&bnetworktransactionaccess as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkClientAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbnetworkclientaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNetworkClientAccess(::core::mem::transmute_copy(&pbnetworkclientaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkClientAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bnetworkclientaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNetworkClientAccess(&*(&bnetworkclientaccess as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkTIPAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbnetworktipaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNetworkTIPAccess(::core::mem::transmute_copy(&pbnetworktipaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkTIPAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bnetworktipaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNetworkTIPAccess(&*(&bnetworktipaccess as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXAAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbxaaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetXAAccess(::core::mem::transmute_copy(&pbxaaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXAAccess<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bxaaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetXAAccess(&*(&bxaaccess as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestartDtcService<Impl: IDtcNetworkAccessConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RestartDtcService() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDtcNetworkAccessConfig>,
            base.5,
            GetAnyNetworkAccess::<Impl, OFFSET>,
            SetAnyNetworkAccess::<Impl, OFFSET>,
            GetNetworkAdministrationAccess::<Impl, OFFSET>,
            SetNetworkAdministrationAccess::<Impl, OFFSET>,
            GetNetworkTransactionAccess::<Impl, OFFSET>,
            SetNetworkTransactionAccess::<Impl, OFFSET>,
            GetNetworkClientAccess::<Impl, OFFSET>,
            SetNetworkClientAccess::<Impl, OFFSET>,
            GetNetworkTIPAccess::<Impl, OFFSET>,
            SetNetworkTIPAccess::<Impl, OFFSET>,
            GetXAAccess::<Impl, OFFSET>,
            SetXAAccess::<Impl, OFFSET>,
            RestartDtcService::<Impl, OFFSET>,
        )
    }
}
pub trait IDtcNetworkAccessConfig2Impl: Sized + IDtcNetworkAccessConfigImpl {
    fn GetNetworkInboundAccess();
    fn GetNetworkOutboundAccess();
    fn SetNetworkInboundAccess();
    fn SetNetworkOutboundAccess();
    fn GetAuthenticationLevel();
    fn SetAuthenticationLevel();
}
impl ::windows::core::RuntimeName for IDtcNetworkAccessConfig2 {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IDtcNetworkAccessConfig2";
}
impl IDtcNetworkAccessConfig2Vtbl {
    pub const fn new<Impl: IDtcNetworkAccessConfig2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDtcNetworkAccessConfig2Vtbl {
        unsafe extern "system" fn GetNetworkInboundAccess<Impl: IDtcNetworkAccessConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbinbound: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNetworkInboundAccess(::core::mem::transmute_copy(&pbinbound)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkOutboundAccess<Impl: IDtcNetworkAccessConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pboutbound: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNetworkOutboundAccess(::core::mem::transmute_copy(&pboutbound)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkInboundAccess<Impl: IDtcNetworkAccessConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, binbound: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNetworkInboundAccess(&*(&binbound as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkOutboundAccess<Impl: IDtcNetworkAccessConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, boutbound: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNetworkOutboundAccess(&*(&boutbound as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthenticationLevel<Impl: IDtcNetworkAccessConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pauthlevel: *mut AUTHENTICATION_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAuthenticationLevel(::core::mem::transmute_copy(&pauthlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationLevel<Impl: IDtcNetworkAccessConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, authlevel: AUTHENTICATION_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAuthenticationLevel(authlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDtcNetworkAccessConfig2>, base.5, GetNetworkInboundAccess::<Impl, OFFSET>, GetNetworkOutboundAccess::<Impl, OFFSET>, SetNetworkInboundAccess::<Impl, OFFSET>, SetNetworkOutboundAccess::<Impl, OFFSET>, GetAuthenticationLevel::<Impl, OFFSET>, SetAuthenticationLevel::<Impl, OFFSET>)
    }
}
pub trait IDtcNetworkAccessConfig3Impl: Sized + IDtcNetworkAccessConfig2Impl + IDtcNetworkAccessConfigImpl {
    fn GetLUAccess();
    fn SetLUAccess();
}
impl ::windows::core::RuntimeName for IDtcNetworkAccessConfig3 {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IDtcNetworkAccessConfig3";
}
impl IDtcNetworkAccessConfig3Vtbl {
    pub const fn new<Impl: IDtcNetworkAccessConfig3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDtcNetworkAccessConfig3Vtbl {
        unsafe extern "system" fn GetLUAccess<Impl: IDtcNetworkAccessConfig3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbluaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLUAccess(::core::mem::transmute_copy(&pbluaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLUAccess<Impl: IDtcNetworkAccessConfig3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bluaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLUAccess(&*(&bluaccess as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDtcNetworkAccessConfig3>, base.5, GetLUAccess::<Impl, OFFSET>, SetLUAccess::<Impl, OFFSET>)
    }
}
pub trait IDtcToXaHelperImpl: Sized {
    fn Close();
    fn TranslateTridToXid();
}
impl ::windows::core::RuntimeName for IDtcToXaHelper {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IDtcToXaHelper";
}
impl IDtcToXaHelperVtbl {
    pub const fn new<Impl: IDtcToXaHelperImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDtcToXaHelperVtbl {
        unsafe extern "system" fn Close<Impl: IDtcToXaHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i_fdorecovery: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Close(&*(&i_fdorecovery as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateTridToXid<Impl: IDtcToXaHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitransaction: ::windows::core::RawPtr, pguidbqual: *const ::windows::core::GUID, pxid: *mut xid_t) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TranslateTridToXid(&*(&pitransaction as *const <ITransaction as ::windows::core::Abi>::Abi as *const <ITransaction as ::windows::core::DefaultType>::DefaultType), &*(&pguidbqual as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pxid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDtcToXaHelper>, base.5, Close::<Impl, OFFSET>, TranslateTridToXid::<Impl, OFFSET>)
    }
}
pub trait IDtcToXaHelperFactoryImpl: Sized {
    fn Create();
}
impl ::windows::core::RuntimeName for IDtcToXaHelperFactory {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IDtcToXaHelperFactory";
}
impl IDtcToXaHelperFactoryVtbl {
    pub const fn new<Impl: IDtcToXaHelperFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDtcToXaHelperFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IDtcToXaHelperFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdsn: super::super::Foundation::PSTR, pszclientdllname: super::super::Foundation::PSTR, pguidrm: *mut ::windows::core::GUID, ppxahelper: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&pszdsn as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszclientdllname as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pguidrm), ::core::mem::transmute_copy(&ppxahelper)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDtcToXaHelperFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
pub trait IDtcToXaHelperSinglePipeImpl: Sized {
    fn XARMCreate();
    fn ConvertTridToXID();
    fn EnlistWithRM();
    fn ReleaseRMCookie();
}
impl ::windows::core::RuntimeName for IDtcToXaHelperSinglePipe {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IDtcToXaHelperSinglePipe";
}
impl IDtcToXaHelperSinglePipeVtbl {
    pub const fn new<Impl: IDtcToXaHelperSinglePipeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDtcToXaHelperSinglePipeVtbl {
        unsafe extern "system" fn XARMCreate<Impl: IDtcToXaHelperSinglePipeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdsn: super::super::Foundation::PSTR, pszclientdll: super::super::Foundation::PSTR, pdwrmcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).XARMCreate(&*(&pszdsn as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszclientdll as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), pdwrmcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertTridToXID<Impl: IDtcToXaHelperSinglePipeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwitrans: *mut u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConvertTridToXID(pdwitrans, dwrmcookie, &*(&pxid as *const <xid_t as ::windows::core::Abi>::Abi as *const <xid_t as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnlistWithRM<Impl: IDtcToXaHelperSinglePipeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwrmcookie: u32, i_pitransaction: ::windows::core::RawPtr, i_pitransres: ::windows::core::RawPtr, o_ppitransenslitment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnlistWithRM(dwrmcookie, &*(&i_pitransaction as *const <ITransaction as ::windows::core::Abi>::Abi as *const <ITransaction as ::windows::core::DefaultType>::DefaultType), &*(&i_pitransres as *const <ITransactionResourceAsync as ::windows::core::Abi>::Abi as *const <ITransactionResourceAsync as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&o_ppitransenslitment)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseRMCookie<Impl: IDtcToXaHelperSinglePipeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i_dwrmcookie: u32, i_fnormal: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReleaseRMCookie(i_dwrmcookie, &*(&i_fnormal as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDtcToXaHelperSinglePipe>, base.5, XARMCreate::<Impl, OFFSET>, ConvertTridToXID::<Impl, OFFSET>, EnlistWithRM::<Impl, OFFSET>, ReleaseRMCookie::<Impl, OFFSET>)
    }
}
pub trait IDtcToXaMapperImpl: Sized {
    fn RequestNewResourceManager();
    fn TranslateTridToXid();
    fn EnlistResourceManager();
    fn ReleaseResourceManager();
}
impl ::windows::core::RuntimeName for IDtcToXaMapper {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IDtcToXaMapper";
}
impl IDtcToXaMapperVtbl {
    pub const fn new<Impl: IDtcToXaMapperImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDtcToXaMapperVtbl {
        unsafe extern "system" fn RequestNewResourceManager<Impl: IDtcToXaMapperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdsn: super::super::Foundation::PSTR, pszclientdllname: super::super::Foundation::PSTR, pdwrmcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestNewResourceManager(&*(&pszdsn as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszclientdllname as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), pdwrmcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateTridToXid<Impl: IDtcToXaMapperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwitransaction: *const u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TranslateTridToXid(pdwitransaction, dwrmcookie, &*(&pxid as *const <xid_t as ::windows::core::Abi>::Abi as *const <xid_t as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnlistResourceManager<Impl: IDtcToXaMapperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwrmcookie: u32, pdwitransaction: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnlistResourceManager(dwrmcookie, pdwitransaction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseResourceManager<Impl: IDtcToXaMapperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwrmcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseResourceManager(dwrmcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDtcToXaMapper>, base.5, RequestNewResourceManager::<Impl, OFFSET>, TranslateTridToXid::<Impl, OFFSET>, EnlistResourceManager::<Impl, OFFSET>, ReleaseResourceManager::<Impl, OFFSET>)
    }
}
pub trait IGetDispenserImpl: Sized {
    fn GetDispenser();
}
impl ::windows::core::RuntimeName for IGetDispenser {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IGetDispenser";
}
impl IGetDispenserVtbl {
    pub const fn new<Impl: IGetDispenserImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGetDispenserVtbl {
        unsafe extern "system" fn GetDispenser<Impl: IGetDispenserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDispenser(&*(&iid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGetDispenser>, base.5, GetDispenser::<Impl, OFFSET>)
    }
}
pub trait IKernelTransactionImpl: Sized {
    fn GetHandle();
}
impl ::windows::core::RuntimeName for IKernelTransaction {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IKernelTransaction";
}
impl IKernelTransactionVtbl {
    pub const fn new<Impl: IKernelTransactionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKernelTransactionVtbl {
        unsafe extern "system" fn GetHandle<Impl: IKernelTransactionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHandle(::core::mem::transmute_copy(&phandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKernelTransaction>, base.5, GetHandle::<Impl, OFFSET>)
    }
}
pub trait ILastResourceManagerImpl: Sized {
    fn TransactionCommitted();
    fn RecoveryDone();
}
impl ::windows::core::RuntimeName for ILastResourceManager {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ILastResourceManager";
}
impl ILastResourceManagerVtbl {
    pub const fn new<Impl: ILastResourceManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILastResourceManagerVtbl {
        unsafe extern "system" fn TransactionCommitted<Impl: ILastResourceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransactionCommitted(pprepinfo, cbprepinfo) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecoveryDone<Impl: ILastResourceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecoveryDone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILastResourceManager>, base.5, TransactionCommitted::<Impl, OFFSET>, RecoveryDone::<Impl, OFFSET>)
    }
}
pub trait IPrepareInfoImpl: Sized {
    fn GetPrepareInfoSize();
    fn GetPrepareInfo();
}
impl ::windows::core::RuntimeName for IPrepareInfo {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IPrepareInfo";
}
impl IPrepareInfoVtbl {
    pub const fn new<Impl: IPrepareInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrepareInfoVtbl {
        unsafe extern "system" fn GetPrepareInfoSize<Impl: IPrepareInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbprepinfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPrepareInfoSize(pcbprepinfo) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrepareInfo<Impl: IPrepareInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprepinfo: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPrepareInfo(pprepinfo) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrepareInfo>, base.5, GetPrepareInfoSize::<Impl, OFFSET>, GetPrepareInfo::<Impl, OFFSET>)
    }
}
pub trait IPrepareInfo2Impl: Sized {
    fn GetPrepareInfoSize();
    fn GetPrepareInfo();
}
impl ::windows::core::RuntimeName for IPrepareInfo2 {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IPrepareInfo2";
}
impl IPrepareInfo2Vtbl {
    pub const fn new<Impl: IPrepareInfo2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrepareInfo2Vtbl {
        unsafe extern "system" fn GetPrepareInfoSize<Impl: IPrepareInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbprepinfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPrepareInfoSize(::core::mem::transmute_copy(&pcbprepinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrepareInfo<Impl: IPrepareInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbprepareinfo: u32, pprepinfo: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPrepareInfo(cbprepareinfo, ::core::mem::transmute_copy(&pprepinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrepareInfo2>, base.5, GetPrepareInfoSize::<Impl, OFFSET>, GetPrepareInfo::<Impl, OFFSET>)
    }
}
pub trait IRMHelperImpl: Sized {
    fn RMCount();
    fn RMInfo();
}
impl ::windows::core::RuntimeName for IRMHelper {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IRMHelper";
}
impl IRMHelperVtbl {
    pub const fn new<Impl: IRMHelperImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRMHelperVtbl {
        unsafe extern "system" fn RMCount<Impl: IRMHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwctotalnumberofrms: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RMCount(dwctotalnumberofrms) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RMInfo<Impl: IRMHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxa_switch: *mut xa_switch_t, fcdeclcallingconv: super::super::Foundation::BOOL, pszopenstring: super::super::Foundation::PSTR, pszclosestring: super::super::Foundation::PSTR, guidrmrecovery: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RMInfo(
                &*(&pxa_switch as *const <xa_switch_t as ::windows::core::Abi>::Abi as *const <xa_switch_t as ::windows::core::DefaultType>::DefaultType),
                &*(&fcdeclcallingconv as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&pszopenstring as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszclosestring as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&guidrmrecovery as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRMHelper>, base.5, RMCount::<Impl, OFFSET>, RMInfo::<Impl, OFFSET>)
    }
}
pub trait IResourceManagerImpl: Sized {
    fn Enlist();
    fn Reenlist();
    fn ReenlistmentComplete();
    fn GetDistributedTransactionManager();
}
impl ::windows::core::RuntimeName for IResourceManager {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IResourceManager";
}
impl IResourceManagerVtbl {
    pub const fn new<Impl: IResourceManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceManagerVtbl {
        unsafe extern "system" fn Enlist<Impl: IResourceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptransaction: ::windows::core::RawPtr, pres: ::windows::core::RawPtr, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Enlist(&*(&ptransaction as *const <ITransaction as ::windows::core::Abi>::Abi as *const <ITransaction as ::windows::core::DefaultType>::DefaultType), &*(&pres as *const <ITransactionResourceAsync as ::windows::core::Abi>::Abi as *const <ITransactionResourceAsync as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&puow), ::core::mem::transmute_copy(&pisolevel), ::core::mem::transmute_copy(&ppenlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reenlist<Impl: IResourceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reenlist(pprepinfo, cbprepinfo, ltimeout, ::core::mem::transmute_copy(&pxactstat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReenlistmentComplete<Impl: IResourceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReenlistmentComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDistributedTransactionManager<Impl: IResourceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDistributedTransactionManager(&*(&iid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceManager>, base.5, Enlist::<Impl, OFFSET>, Reenlist::<Impl, OFFSET>, ReenlistmentComplete::<Impl, OFFSET>, GetDistributedTransactionManager::<Impl, OFFSET>)
    }
}
pub trait IResourceManager2Impl: Sized + IResourceManagerImpl {
    fn Enlist2();
    fn Reenlist2();
}
impl ::windows::core::RuntimeName for IResourceManager2 {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IResourceManager2";
}
impl IResourceManager2Vtbl {
    pub const fn new<Impl: IResourceManager2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceManager2Vtbl {
        unsafe extern "system" fn Enlist2<Impl: IResourceManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptransaction: ::windows::core::RawPtr, presasync: ::windows::core::RawPtr, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut xid_t, ppenlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Enlist2(&*(&ptransaction as *const <ITransaction as ::windows::core::Abi>::Abi as *const <ITransaction as ::windows::core::DefaultType>::DefaultType), &*(&presasync as *const <ITransactionResourceAsync as ::windows::core::Abi>::Abi as *const <ITransactionResourceAsync as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&puow), ::core::mem::transmute_copy(&pisolevel), ::core::mem::transmute_copy(&pxid), ::core::mem::transmute_copy(&ppenlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reenlist2<Impl: IResourceManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxid: *const xid_t, dwtimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reenlist2(&*(&pxid as *const <xid_t as ::windows::core::Abi>::Abi as *const <xid_t as ::windows::core::DefaultType>::DefaultType), dwtimeout, ::core::mem::transmute_copy(&pxactstat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceManager2>, base.5, Enlist2::<Impl, OFFSET>, Reenlist2::<Impl, OFFSET>)
    }
}
pub trait IResourceManagerFactoryImpl: Sized {
    fn Create();
}
impl ::windows::core::RuntimeName for IResourceManagerFactory {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IResourceManagerFactory";
}
impl IResourceManagerFactoryVtbl {
    pub const fn new<Impl: IResourceManagerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceManagerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IResourceManagerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidrm: *const ::windows::core::GUID, pszrmname: super::super::Foundation::PSTR, piresmgrsink: ::windows::core::RawPtr, ppresmgr: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(
                &*(&pguidrm as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pszrmname as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&piresmgrsink as *const <IResourceManagerSink as ::windows::core::Abi>::Abi as *const <IResourceManagerSink as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppresmgr),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceManagerFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
pub trait IResourceManagerFactory2Impl: Sized + IResourceManagerFactoryImpl {
    fn CreateEx();
}
impl ::windows::core::RuntimeName for IResourceManagerFactory2 {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IResourceManagerFactory2";
}
impl IResourceManagerFactory2Vtbl {
    pub const fn new<Impl: IResourceManagerFactory2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceManagerFactory2Vtbl {
        unsafe extern "system" fn CreateEx<Impl: IResourceManagerFactory2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidrm: *const ::windows::core::GUID, pszrmname: super::super::Foundation::PSTR, piresmgrsink: ::windows::core::RawPtr, riidrequested: *const ::windows::core::GUID, ppvresmgr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateEx(
                &*(&pguidrm as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pszrmname as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&piresmgrsink as *const <IResourceManagerSink as ::windows::core::Abi>::Abi as *const <IResourceManagerSink as ::windows::core::DefaultType>::DefaultType),
                &*(&riidrequested as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppvresmgr),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceManagerFactory2>, base.5, CreateEx::<Impl, OFFSET>)
    }
}
pub trait IResourceManagerRejoinableImpl: Sized + IResourceManager2Impl + IResourceManagerImpl {
    fn Rejoin();
}
impl ::windows::core::RuntimeName for IResourceManagerRejoinable {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IResourceManagerRejoinable";
}
impl IResourceManagerRejoinableVtbl {
    pub const fn new<Impl: IResourceManagerRejoinableImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceManagerRejoinableVtbl {
        unsafe extern "system" fn Rejoin<Impl: IResourceManagerRejoinableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Rejoin(pprepinfo, cbprepinfo, ltimeout, ::core::mem::transmute_copy(&pxactstat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceManagerRejoinable>, base.5, Rejoin::<Impl, OFFSET>)
    }
}
pub trait IResourceManagerSinkImpl: Sized {
    fn TMDown();
}
impl ::windows::core::RuntimeName for IResourceManagerSink {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IResourceManagerSink";
}
impl IResourceManagerSinkVtbl {
    pub const fn new<Impl: IResourceManagerSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceManagerSinkVtbl {
        unsafe extern "system" fn TMDown<Impl: IResourceManagerSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TMDown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceManagerSink>, base.5, TMDown::<Impl, OFFSET>)
    }
}
pub trait ITipHelperImpl: Sized {
    fn Pull();
    fn PullAsync();
    fn GetLocalTmUrl();
}
impl ::windows::core::RuntimeName for ITipHelper {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITipHelper";
}
impl ITipHelperVtbl {
    pub const fn new<Impl: ITipHelperImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITipHelperVtbl {
        unsafe extern "system" fn Pull<Impl: ITipHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i_psztxurl: *const u8, o_ppitransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pull(i_psztxurl, ::core::mem::transmute_copy(&o_ppitransaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PullAsync<Impl: ITipHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i_psztxurl: *const u8, i_ptippullsink: ::windows::core::RawPtr, o_ppitransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PullAsync(i_psztxurl, &*(&i_ptippullsink as *const <ITipPullSink as ::windows::core::Abi>::Abi as *const <ITipPullSink as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&o_ppitransaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalTmUrl<Impl: ITipHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, o_ppszlocaltmurl: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLocalTmUrl(::core::mem::transmute_copy(&o_ppszlocaltmurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITipHelper>, base.5, Pull::<Impl, OFFSET>, PullAsync::<Impl, OFFSET>, GetLocalTmUrl::<Impl, OFFSET>)
    }
}
pub trait ITipPullSinkImpl: Sized {
    fn PullComplete();
}
impl ::windows::core::RuntimeName for ITipPullSink {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITipPullSink";
}
impl ITipPullSinkVtbl {
    pub const fn new<Impl: ITipPullSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITipPullSinkVtbl {
        unsafe extern "system" fn PullComplete<Impl: ITipPullSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i_hrpull: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PullComplete(i_hrpull) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITipPullSink>, base.5, PullComplete::<Impl, OFFSET>)
    }
}
pub trait ITipTransactionImpl: Sized {
    fn Push();
    fn GetTransactionUrl();
}
impl ::windows::core::RuntimeName for ITipTransaction {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITipTransaction";
}
impl ITipTransactionVtbl {
    pub const fn new<Impl: ITipTransactionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITipTransactionVtbl {
        unsafe extern "system" fn Push<Impl: ITipTransactionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i_pszremotetmurl: *const u8, o_ppszremotetxurl: *mut super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Push(i_pszremotetmurl, ::core::mem::transmute_copy(&o_ppszremotetxurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransactionUrl<Impl: ITipTransactionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, o_ppszlocaltxurl: *mut super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransactionUrl(::core::mem::transmute_copy(&o_ppszlocaltxurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITipTransaction>, base.5, Push::<Impl, OFFSET>, GetTransactionUrl::<Impl, OFFSET>)
    }
}
pub trait ITmNodeNameImpl: Sized {
    fn GetNodeNameSize();
    fn GetNodeName();
}
impl ::windows::core::RuntimeName for ITmNodeName {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITmNodeName";
}
impl ITmNodeNameVtbl {
    pub const fn new<Impl: ITmNodeNameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITmNodeNameVtbl {
        unsafe extern "system" fn GetNodeNameSize<Impl: ITmNodeNameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbnodenamesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNodeNameSize(::core::mem::transmute_copy(&pcbnodenamesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNodeName<Impl: ITmNodeNameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbnodenamebuffersize: u32, pnodenamebuffer: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNodeName(cbnodenamebuffersize, &*(&pnodenamebuffer as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITmNodeName>, base.5, GetNodeNameSize::<Impl, OFFSET>, GetNodeName::<Impl, OFFSET>)
    }
}
pub trait ITransactionImpl: Sized {
    fn Commit();
    fn Abort();
    fn GetTransactionInfo();
}
impl ::windows::core::RuntimeName for ITransaction {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransaction";
}
impl ITransactionVtbl {
    pub const fn new<Impl: ITransactionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionVtbl {
        unsafe extern "system" fn Commit<Impl: ITransactionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grftc: u32, grfrm: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Commit(&*(&fretaining as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), grftc, grfrm) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Impl: ITransactionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, fasync: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Abort(&*(&pboidreason as *const <BOID as ::windows::core::Abi>::Abi as *const <BOID as ::windows::core::DefaultType>::DefaultType), &*(&fretaining as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&fasync as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransactionInfo<Impl: ITransactionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *mut XACTTRANSINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransactionInfo(::core::mem::transmute_copy(&pinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransaction>, base.5, Commit::<Impl, OFFSET>, Abort::<Impl, OFFSET>, GetTransactionInfo::<Impl, OFFSET>)
    }
}
pub trait ITransaction2Impl: Sized + ITransactionClonerImpl + ITransactionImpl {
    fn GetTransactionInfo2();
}
impl ::windows::core::RuntimeName for ITransaction2 {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransaction2";
}
impl ITransaction2Vtbl {
    pub const fn new<Impl: ITransaction2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransaction2Vtbl {
        unsafe extern "system" fn GetTransactionInfo2<Impl: ITransaction2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *mut XACTTRANSINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransactionInfo2(::core::mem::transmute_copy(&pinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransaction2>, base.5, GetTransactionInfo2::<Impl, OFFSET>)
    }
}
pub trait ITransactionClonerImpl: Sized + ITransactionImpl {
    fn CloneWithCommitDisabled();
}
impl ::windows::core::RuntimeName for ITransactionCloner {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransactionCloner";
}
impl ITransactionClonerVtbl {
    pub const fn new<Impl: ITransactionClonerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionClonerVtbl {
        unsafe extern "system" fn CloneWithCommitDisabled<Impl: ITransactionClonerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppitransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CloneWithCommitDisabled(::core::mem::transmute_copy(&ppitransaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionCloner>, base.5, CloneWithCommitDisabled::<Impl, OFFSET>)
    }
}
pub trait ITransactionDispenserImpl: Sized {
    fn GetOptionsObject();
    fn BeginTransaction();
}
impl ::windows::core::RuntimeName for ITransactionDispenser {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransactionDispenser";
}
impl ITransactionDispenserVtbl {
    pub const fn new<Impl: ITransactionDispenserImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionDispenserVtbl {
        unsafe extern "system" fn GetOptionsObject<Impl: ITransactionDispenserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOptionsObject(::core::mem::transmute_copy(&ppoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginTransaction<Impl: ITransactionDispenserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, poptions: ::windows::core::RawPtr, pptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginTransaction(&*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), isolevel, isoflags, &*(&poptions as *const <ITransactionOptions as ::windows::core::Abi>::Abi as *const <ITransactionOptions as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pptransaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionDispenser>, base.5, GetOptionsObject::<Impl, OFFSET>, BeginTransaction::<Impl, OFFSET>)
    }
}
pub trait ITransactionEnlistmentAsyncImpl: Sized {
    fn PrepareRequestDone();
    fn CommitRequestDone();
    fn AbortRequestDone();
}
impl ::windows::core::RuntimeName for ITransactionEnlistmentAsync {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransactionEnlistmentAsync";
}
impl ITransactionEnlistmentAsyncVtbl {
    pub const fn new<Impl: ITransactionEnlistmentAsyncImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionEnlistmentAsyncVtbl {
        unsafe extern "system" fn PrepareRequestDone<Impl: ITransactionEnlistmentAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, pmk: ::windows::core::RawPtr, pboidreason: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrepareRequestDone(hr, &*(&pmk as *const <super::Com::IMoniker as ::windows::core::Abi>::Abi as *const <super::Com::IMoniker as ::windows::core::DefaultType>::DefaultType), &*(&pboidreason as *const <BOID as ::windows::core::Abi>::Abi as *const <BOID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitRequestDone<Impl: ITransactionEnlistmentAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommitRequestDone(hr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbortRequestDone<Impl: ITransactionEnlistmentAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AbortRequestDone(hr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionEnlistmentAsync>, base.5, PrepareRequestDone::<Impl, OFFSET>, CommitRequestDone::<Impl, OFFSET>, AbortRequestDone::<Impl, OFFSET>)
    }
}
pub trait ITransactionExportImpl: Sized {
    fn Export();
    fn GetTransactionCookie();
}
impl ::windows::core::RuntimeName for ITransactionExport {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransactionExport";
}
impl ITransactionExportVtbl {
    pub const fn new<Impl: ITransactionExportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionExportVtbl {
        unsafe extern "system" fn Export<Impl: ITransactionExportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punktransaction: *mut ::core::ffi::c_void, pcbtransactioncookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Export(&*(&punktransaction as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcbtransactioncookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransactionCookie<Impl: ITransactionExportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punktransaction: *mut ::core::ffi::c_void, cbtransactioncookie: u32, rgbtransactioncookie: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransactionCookie(&*(&punktransaction as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), cbtransactioncookie, ::core::mem::transmute_copy(&rgbtransactioncookie), ::core::mem::transmute_copy(&pcbused)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionExport>, base.5, Export::<Impl, OFFSET>, GetTransactionCookie::<Impl, OFFSET>)
    }
}
pub trait ITransactionExportFactoryImpl: Sized {
    fn GetRemoteClassId();
    fn Create();
}
impl ::windows::core::RuntimeName for ITransactionExportFactory {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransactionExportFactory";
}
impl ITransactionExportFactoryVtbl {
    pub const fn new<Impl: ITransactionExportFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionExportFactoryVtbl {
        unsafe extern "system" fn GetRemoteClassId<Impl: ITransactionExportFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRemoteClassId(::core::mem::transmute_copy(&pclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: ITransactionExportFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbwhereabouts: u32, rgbwhereabouts: *const u8, ppexport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(cbwhereabouts, rgbwhereabouts, ::core::mem::transmute_copy(&ppexport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionExportFactory>, base.5, GetRemoteClassId::<Impl, OFFSET>, Create::<Impl, OFFSET>)
    }
}
pub trait ITransactionImportImpl: Sized {
    fn Import();
}
impl ::windows::core::RuntimeName for ITransactionImport {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransactionImport";
}
impl ITransactionImportVtbl {
    pub const fn new<Impl: ITransactionImportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionImportVtbl {
        unsafe extern "system" fn Import<Impl: ITransactionImportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbtransactioncookie: u32, rgbtransactioncookie: *const u8, piid: *const ::windows::core::GUID, ppvtransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Import(cbtransactioncookie, rgbtransactioncookie, &*(&piid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvtransaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionImport>, base.5, Import::<Impl, OFFSET>)
    }
}
pub trait ITransactionImportWhereaboutsImpl: Sized {
    fn GetWhereaboutsSize();
    fn GetWhereabouts();
}
impl ::windows::core::RuntimeName for ITransactionImportWhereabouts {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransactionImportWhereabouts";
}
impl ITransactionImportWhereaboutsVtbl {
    pub const fn new<Impl: ITransactionImportWhereaboutsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionImportWhereaboutsVtbl {
        unsafe extern "system" fn GetWhereaboutsSize<Impl: ITransactionImportWhereaboutsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbwhereabouts: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetWhereaboutsSize(::core::mem::transmute_copy(&pcbwhereabouts)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWhereabouts<Impl: ITransactionImportWhereaboutsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbwhereabouts: u32, rgbwhereabouts: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetWhereabouts(cbwhereabouts, ::core::mem::transmute_copy(&rgbwhereabouts), ::core::mem::transmute_copy(&pcbused)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionImportWhereabouts>, base.5, GetWhereaboutsSize::<Impl, OFFSET>, GetWhereabouts::<Impl, OFFSET>)
    }
}
pub trait ITransactionLastEnlistmentAsyncImpl: Sized {
    fn TransactionOutcome();
}
impl ::windows::core::RuntimeName for ITransactionLastEnlistmentAsync {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransactionLastEnlistmentAsync";
}
impl ITransactionLastEnlistmentAsyncVtbl {
    pub const fn new<Impl: ITransactionLastEnlistmentAsyncImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionLastEnlistmentAsyncVtbl {
        unsafe extern "system" fn TransactionOutcome<Impl: ITransactionLastEnlistmentAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xactstat: XACTSTAT, pboidreason: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransactionOutcome(xactstat, &*(&pboidreason as *const <BOID as ::windows::core::Abi>::Abi as *const <BOID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionLastEnlistmentAsync>, base.5, TransactionOutcome::<Impl, OFFSET>)
    }
}
pub trait ITransactionLastResourceAsyncImpl: Sized {
    fn DelegateCommit();
    fn ForgetRequest();
}
impl ::windows::core::RuntimeName for ITransactionLastResourceAsync {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransactionLastResourceAsync";
}
impl ITransactionLastResourceAsyncVtbl {
    pub const fn new<Impl: ITransactionLastResourceAsyncImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionLastResourceAsyncVtbl {
        unsafe extern "system" fn DelegateCommit<Impl: ITransactionLastResourceAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfrm: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DelegateCommit(grfrm) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForgetRequest<Impl: ITransactionLastResourceAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnewuow: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ForgetRequest(&*(&pnewuow as *const <BOID as ::windows::core::Abi>::Abi as *const <BOID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionLastResourceAsync>, base.5, DelegateCommit::<Impl, OFFSET>, ForgetRequest::<Impl, OFFSET>)
    }
}
pub trait ITransactionOptionsImpl: Sized {
    fn SetOptions();
    fn GetOptions();
}
impl ::windows::core::RuntimeName for ITransactionOptions {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransactionOptions";
}
impl ITransactionOptionsVtbl {
    pub const fn new<Impl: ITransactionOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionOptionsVtbl {
        unsafe extern "system" fn SetOptions<Impl: ITransactionOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poptions: *const XACTOPT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOptions(&*(&poptions as *const <XACTOPT as ::windows::core::Abi>::Abi as *const <XACTOPT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptions<Impl: ITransactionOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poptions: *mut XACTOPT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOptions(&*(&poptions as *const <XACTOPT as ::windows::core::Abi>::Abi as *const <XACTOPT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionOptions>, base.5, SetOptions::<Impl, OFFSET>, GetOptions::<Impl, OFFSET>)
    }
}
pub trait ITransactionOutcomeEventsImpl: Sized {
    fn Committed();
    fn Aborted();
    fn HeuristicDecision();
    fn Indoubt();
}
impl ::windows::core::RuntimeName for ITransactionOutcomeEvents {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransactionOutcomeEvents";
}
impl ITransactionOutcomeEventsVtbl {
    pub const fn new<Impl: ITransactionOutcomeEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionOutcomeEventsVtbl {
        unsafe extern "system" fn Committed<Impl: ITransactionOutcomeEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Committed(&*(&fretaining as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&pnewuow as *const <BOID as ::windows::core::Abi>::Abi as *const <BOID as ::windows::core::DefaultType>::DefaultType), hr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Aborted<Impl: ITransactionOutcomeEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Aborted(&*(&pboidreason as *const <BOID as ::windows::core::Abi>::Abi as *const <BOID as ::windows::core::DefaultType>::DefaultType), &*(&fretaining as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&pnewuow as *const <BOID as ::windows::core::Abi>::Abi as *const <BOID as ::windows::core::DefaultType>::DefaultType), hr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeuristicDecision<Impl: ITransactionOutcomeEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdecision: u32, pboidreason: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HeuristicDecision(dwdecision, &*(&pboidreason as *const <BOID as ::windows::core::Abi>::Abi as *const <BOID as ::windows::core::DefaultType>::DefaultType), hr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Indoubt<Impl: ITransactionOutcomeEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Indoubt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionOutcomeEvents>, base.5, Committed::<Impl, OFFSET>, Aborted::<Impl, OFFSET>, HeuristicDecision::<Impl, OFFSET>, Indoubt::<Impl, OFFSET>)
    }
}
pub trait ITransactionPhase0EnlistmentAsyncImpl: Sized {
    fn Enable();
    fn WaitForEnlistment();
    fn Phase0Done();
    fn Unenlist();
    fn GetTransaction();
}
impl ::windows::core::RuntimeName for ITransactionPhase0EnlistmentAsync {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransactionPhase0EnlistmentAsync";
}
impl ITransactionPhase0EnlistmentAsyncVtbl {
    pub const fn new<Impl: ITransactionPhase0EnlistmentAsyncImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionPhase0EnlistmentAsyncVtbl {
        unsafe extern "system" fn Enable<Impl: ITransactionPhase0EnlistmentAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Enable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitForEnlistment<Impl: ITransactionPhase0EnlistmentAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WaitForEnlistment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Phase0Done<Impl: ITransactionPhase0EnlistmentAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Phase0Done() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unenlist<Impl: ITransactionPhase0EnlistmentAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unenlist() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransaction<Impl: ITransactionPhase0EnlistmentAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppitransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransaction(::core::mem::transmute_copy(&ppitransaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionPhase0EnlistmentAsync>, base.5, Enable::<Impl, OFFSET>, WaitForEnlistment::<Impl, OFFSET>, Phase0Done::<Impl, OFFSET>, Unenlist::<Impl, OFFSET>, GetTransaction::<Impl, OFFSET>)
    }
}
pub trait ITransactionPhase0FactoryImpl: Sized {
    fn Create();
}
impl ::windows::core::RuntimeName for ITransactionPhase0Factory {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransactionPhase0Factory";
}
impl ITransactionPhase0FactoryVtbl {
    pub const fn new<Impl: ITransactionPhase0FactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionPhase0FactoryVtbl {
        unsafe extern "system" fn Create<Impl: ITransactionPhase0FactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphase0notify: ::windows::core::RawPtr, ppphase0enlistment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&pphase0notify as *const <ITransactionPhase0NotifyAsync as ::windows::core::Abi>::Abi as *const <ITransactionPhase0NotifyAsync as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppphase0enlistment)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionPhase0Factory>, base.5, Create::<Impl, OFFSET>)
    }
}
pub trait ITransactionPhase0NotifyAsyncImpl: Sized {
    fn Phase0Request();
    fn EnlistCompleted();
}
impl ::windows::core::RuntimeName for ITransactionPhase0NotifyAsync {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransactionPhase0NotifyAsync";
}
impl ITransactionPhase0NotifyAsyncVtbl {
    pub const fn new<Impl: ITransactionPhase0NotifyAsyncImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionPhase0NotifyAsyncVtbl {
        unsafe extern "system" fn Phase0Request<Impl: ITransactionPhase0NotifyAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fabortinghint: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Phase0Request(&*(&fabortinghint as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnlistCompleted<Impl: ITransactionPhase0NotifyAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnlistCompleted(status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionPhase0NotifyAsync>, base.5, Phase0Request::<Impl, OFFSET>, EnlistCompleted::<Impl, OFFSET>)
    }
}
pub trait ITransactionReceiverImpl: Sized {
    fn UnmarshalPropagationToken();
    fn GetReturnTokenSize();
    fn MarshalReturnToken();
    fn Reset();
}
impl ::windows::core::RuntimeName for ITransactionReceiver {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransactionReceiver";
}
impl ITransactionReceiverVtbl {
    pub const fn new<Impl: ITransactionReceiverImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionReceiverVtbl {
        unsafe extern "system" fn UnmarshalPropagationToken<Impl: ITransactionReceiverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbtoken: u32, rgbtoken: *const u8, pptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnmarshalPropagationToken(cbtoken, rgbtoken, ::core::mem::transmute_copy(&pptransaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReturnTokenSize<Impl: ITransactionReceiverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbreturntoken: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReturnTokenSize(::core::mem::transmute_copy(&pcbreturntoken)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarshalReturnToken<Impl: ITransactionReceiverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbreturntoken: u32, rgbreturntoken: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MarshalReturnToken(cbreturntoken, ::core::mem::transmute_copy(&rgbreturntoken), ::core::mem::transmute_copy(&pcbused)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: ITransactionReceiverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionReceiver>, base.5, UnmarshalPropagationToken::<Impl, OFFSET>, GetReturnTokenSize::<Impl, OFFSET>, MarshalReturnToken::<Impl, OFFSET>, Reset::<Impl, OFFSET>)
    }
}
pub trait ITransactionReceiverFactoryImpl: Sized {
    fn Create();
}
impl ::windows::core::RuntimeName for ITransactionReceiverFactory {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransactionReceiverFactory";
}
impl ITransactionReceiverFactoryVtbl {
    pub const fn new<Impl: ITransactionReceiverFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionReceiverFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ITransactionReceiverFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppreceiver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(::core::mem::transmute_copy(&ppreceiver)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionReceiverFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
pub trait ITransactionResourceImpl: Sized {
    fn PrepareRequest();
    fn CommitRequest();
    fn AbortRequest();
    fn TMDown();
}
impl ::windows::core::RuntimeName for ITransactionResource {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransactionResource";
}
impl ITransactionResourceVtbl {
    pub const fn new<Impl: ITransactionResourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionResourceVtbl {
        unsafe extern "system" fn PrepareRequest<Impl: ITransactionResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrepareRequest(
                &*(&fretaining as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                grfrm,
                &*(&fwantmoniker as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&fsinglephase as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitRequest<Impl: ITransactionResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfrm: u32, pnewuow: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommitRequest(grfrm, &*(&pnewuow as *const <BOID as ::windows::core::Abi>::Abi as *const <BOID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbortRequest<Impl: ITransactionResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AbortRequest(&*(&pboidreason as *const <BOID as ::windows::core::Abi>::Abi as *const <BOID as ::windows::core::DefaultType>::DefaultType), &*(&fretaining as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&pnewuow as *const <BOID as ::windows::core::Abi>::Abi as *const <BOID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TMDown<Impl: ITransactionResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TMDown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionResource>, base.5, PrepareRequest::<Impl, OFFSET>, CommitRequest::<Impl, OFFSET>, AbortRequest::<Impl, OFFSET>, TMDown::<Impl, OFFSET>)
    }
}
pub trait ITransactionResourceAsyncImpl: Sized {
    fn PrepareRequest();
    fn CommitRequest();
    fn AbortRequest();
    fn TMDown();
}
impl ::windows::core::RuntimeName for ITransactionResourceAsync {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransactionResourceAsync";
}
impl ITransactionResourceAsyncVtbl {
    pub const fn new<Impl: ITransactionResourceAsyncImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionResourceAsyncVtbl {
        unsafe extern "system" fn PrepareRequest<Impl: ITransactionResourceAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrepareRequest(
                &*(&fretaining as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                grfrm,
                &*(&fwantmoniker as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&fsinglephase as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitRequest<Impl: ITransactionResourceAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfrm: u32, pnewuow: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommitRequest(grfrm, &*(&pnewuow as *const <BOID as ::windows::core::Abi>::Abi as *const <BOID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbortRequest<Impl: ITransactionResourceAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AbortRequest(&*(&pboidreason as *const <BOID as ::windows::core::Abi>::Abi as *const <BOID as ::windows::core::DefaultType>::DefaultType), &*(&fretaining as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&pnewuow as *const <BOID as ::windows::core::Abi>::Abi as *const <BOID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TMDown<Impl: ITransactionResourceAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TMDown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionResourceAsync>, base.5, PrepareRequest::<Impl, OFFSET>, CommitRequest::<Impl, OFFSET>, AbortRequest::<Impl, OFFSET>, TMDown::<Impl, OFFSET>)
    }
}
pub trait ITransactionTransmitterImpl: Sized {
    fn Set();
    fn GetPropagationTokenSize();
    fn MarshalPropagationToken();
    fn UnmarshalReturnToken();
    fn Reset();
}
impl ::windows::core::RuntimeName for ITransactionTransmitter {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransactionTransmitter";
}
impl ITransactionTransmitterVtbl {
    pub const fn new<Impl: ITransactionTransmitterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionTransmitterVtbl {
        unsafe extern "system" fn Set<Impl: ITransactionTransmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptransaction: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Set(&*(&ptransaction as *const <ITransaction as ::windows::core::Abi>::Abi as *const <ITransaction as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropagationTokenSize<Impl: ITransactionTransmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbtoken: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropagationTokenSize(::core::mem::transmute_copy(&pcbtoken)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarshalPropagationToken<Impl: ITransactionTransmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbtoken: u32, rgbtoken: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MarshalPropagationToken(cbtoken, ::core::mem::transmute_copy(&rgbtoken), ::core::mem::transmute_copy(&pcbused)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnmarshalReturnToken<Impl: ITransactionTransmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbreturntoken: u32, rgbreturntoken: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnmarshalReturnToken(cbreturntoken, rgbreturntoken) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: ITransactionTransmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionTransmitter>, base.5, Set::<Impl, OFFSET>, GetPropagationTokenSize::<Impl, OFFSET>, MarshalPropagationToken::<Impl, OFFSET>, UnmarshalReturnToken::<Impl, OFFSET>, Reset::<Impl, OFFSET>)
    }
}
pub trait ITransactionTransmitterFactoryImpl: Sized {
    fn Create();
}
impl ::windows::core::RuntimeName for ITransactionTransmitterFactory {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransactionTransmitterFactory";
}
impl ITransactionTransmitterFactoryVtbl {
    pub const fn new<Impl: ITransactionTransmitterFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionTransmitterFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ITransactionTransmitterFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptransmitter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(::core::mem::transmute_copy(&pptransmitter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionTransmitterFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
pub trait ITransactionVoterBallotAsync2Impl: Sized {
    fn VoteRequestDone();
}
impl ::windows::core::RuntimeName for ITransactionVoterBallotAsync2 {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransactionVoterBallotAsync2";
}
impl ITransactionVoterBallotAsync2Vtbl {
    pub const fn new<Impl: ITransactionVoterBallotAsync2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionVoterBallotAsync2Vtbl {
        unsafe extern "system" fn VoteRequestDone<Impl: ITransactionVoterBallotAsync2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, pboidreason: *const BOID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VoteRequestDone(hr, &*(&pboidreason as *const <BOID as ::windows::core::Abi>::Abi as *const <BOID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionVoterBallotAsync2>, base.5, VoteRequestDone::<Impl, OFFSET>)
    }
}
pub trait ITransactionVoterFactory2Impl: Sized {
    fn Create();
}
impl ::windows::core::RuntimeName for ITransactionVoterFactory2 {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransactionVoterFactory2";
}
impl ITransactionVoterFactory2Vtbl {
    pub const fn new<Impl: ITransactionVoterFactory2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionVoterFactory2Vtbl {
        unsafe extern "system" fn Create<Impl: ITransactionVoterFactory2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptransaction: ::windows::core::RawPtr, pvoternotify: ::windows::core::RawPtr, ppvoterballot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&ptransaction as *const <ITransaction as ::windows::core::Abi>::Abi as *const <ITransaction as ::windows::core::DefaultType>::DefaultType), &*(&pvoternotify as *const <ITransactionVoterNotifyAsync2 as ::windows::core::Abi>::Abi as *const <ITransactionVoterNotifyAsync2 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvoterballot)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionVoterFactory2>, base.5, Create::<Impl, OFFSET>)
    }
}
pub trait ITransactionVoterNotifyAsync2Impl: Sized + ITransactionOutcomeEventsImpl {
    fn VoteRequest();
}
impl ::windows::core::RuntimeName for ITransactionVoterNotifyAsync2 {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.ITransactionVoterNotifyAsync2";
}
impl ITransactionVoterNotifyAsync2Vtbl {
    pub const fn new<Impl: ITransactionVoterNotifyAsync2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionVoterNotifyAsync2Vtbl {
        unsafe extern "system" fn VoteRequest<Impl: ITransactionVoterNotifyAsync2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VoteRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionVoterNotifyAsync2>, base.5, VoteRequest::<Impl, OFFSET>)
    }
}
pub trait IXAConfigImpl: Sized {
    fn Initialize();
    fn Terminate();
}
impl ::windows::core::RuntimeName for IXAConfig {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IXAConfig";
}
impl IXAConfigVtbl {
    pub const fn new<Impl: IXAConfigImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXAConfigVtbl {
        unsafe extern "system" fn Initialize<Impl: IXAConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsidhelperdll: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&clsidhelperdll as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminate<Impl: IXAConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Terminate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXAConfig>, base.5, Initialize::<Impl, OFFSET>, Terminate::<Impl, OFFSET>)
    }
}
pub trait IXAObtainRMInfoImpl: Sized {
    fn ObtainRMInfo();
}
impl ::windows::core::RuntimeName for IXAObtainRMInfo {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IXAObtainRMInfo";
}
impl IXAObtainRMInfoVtbl {
    pub const fn new<Impl: IXAObtainRMInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXAObtainRMInfoVtbl {
        unsafe extern "system" fn ObtainRMInfo<Impl: IXAObtainRMInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pirmhelper: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ObtainRMInfo(&*(&pirmhelper as *const <IRMHelper as ::windows::core::Abi>::Abi as *const <IRMHelper as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXAObtainRMInfo>, base.5, ObtainRMInfo::<Impl, OFFSET>)
    }
}
pub trait IXATransLookupImpl: Sized {
    fn Lookup();
}
impl ::windows::core::RuntimeName for IXATransLookup {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IXATransLookup";
}
impl IXATransLookupVtbl {
    pub const fn new<Impl: IXATransLookupImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXATransLookupVtbl {
        unsafe extern "system" fn Lookup<Impl: IXATransLookupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Lookup(::core::mem::transmute_copy(&pptransaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXATransLookup>, base.5, Lookup::<Impl, OFFSET>)
    }
}
pub trait IXATransLookup2Impl: Sized {
    fn Lookup();
}
impl ::windows::core::RuntimeName for IXATransLookup2 {
    const NAME: &'static str = "Windows.Win32.System.DistributedTransactionCoordinator.IXATransLookup2";
}
impl IXATransLookup2Vtbl {
    pub const fn new<Impl: IXATransLookup2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXATransLookup2Vtbl {
        unsafe extern "system" fn Lookup<Impl: IXATransLookup2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxid: *const xid_t, pptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Lookup(&*(&pxid as *const <xid_t as ::windows::core::Abi>::Abi as *const <xid_t as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pptransaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXATransLookup2>, base.5, Lookup::<Impl, OFFSET>)
    }
}
