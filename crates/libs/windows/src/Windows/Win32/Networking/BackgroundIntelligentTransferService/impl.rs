pub trait AsyncIBackgroundCopyCallbackImpl: Sized {
    fn Begin_JobTransferred();
    fn Finish_JobTransferred();
    fn Begin_JobError();
    fn Finish_JobError();
    fn Begin_JobModification();
    fn Finish_JobModification();
}
impl ::windows::core::RuntimeName for AsyncIBackgroundCopyCallback {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.AsyncIBackgroundCopyCallback";
}
impl AsyncIBackgroundCopyCallbackVtbl {
    pub const fn new<Impl: AsyncIBackgroundCopyCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> AsyncIBackgroundCopyCallbackVtbl {
        unsafe extern "system" fn Begin_JobTransferred<Impl: AsyncIBackgroundCopyCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Begin_JobTransferred(&*(&pjob as *const <IBackgroundCopyJob as ::windows::core::Abi>::Abi as *const <IBackgroundCopyJob as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_JobTransferred<Impl: AsyncIBackgroundCopyCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Finish_JobTransferred() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_JobError<Impl: AsyncIBackgroundCopyCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr, perror: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Begin_JobError(&*(&pjob as *const <IBackgroundCopyJob as ::windows::core::Abi>::Abi as *const <IBackgroundCopyJob as ::windows::core::DefaultType>::DefaultType), &*(&perror as *const <IBackgroundCopyError as ::windows::core::Abi>::Abi as *const <IBackgroundCopyError as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_JobError<Impl: AsyncIBackgroundCopyCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Finish_JobError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_JobModification<Impl: AsyncIBackgroundCopyCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Begin_JobModification(&*(&pjob as *const <IBackgroundCopyJob as ::windows::core::Abi>::Abi as *const <IBackgroundCopyJob as ::windows::core::DefaultType>::DefaultType), dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_JobModification<Impl: AsyncIBackgroundCopyCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Finish_JobModification() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<AsyncIBackgroundCopyCallback>, base.5, Begin_JobTransferred::<Impl, OFFSET>, Finish_JobTransferred::<Impl, OFFSET>, Begin_JobError::<Impl, OFFSET>, Finish_JobError::<Impl, OFFSET>, Begin_JobModification::<Impl, OFFSET>, Finish_JobModification::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBITSExtensionSetupImpl: Sized + IDispatchImpl {
    fn EnableBITSUploads();
    fn DisableBITSUploads();
    fn GetCleanupTaskName();
    fn GetCleanupTask();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IBITSExtensionSetup {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBITSExtensionSetup";
}
#[cfg(feature = "Win32_System_Com")]
impl IBITSExtensionSetupVtbl {
    pub const fn new<Impl: IBITSExtensionSetupImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBITSExtensionSetupVtbl {
        unsafe extern "system" fn EnableBITSUploads<Impl: IBITSExtensionSetupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableBITSUploads() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableBITSUploads<Impl: IBITSExtensionSetupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisableBITSUploads() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCleanupTaskName<Impl: IBITSExtensionSetupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptaskname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCleanupTaskName(::core::mem::transmute_copy(&ptaskname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCleanupTask<Impl: IBITSExtensionSetupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCleanupTask(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBITSExtensionSetup>, base.5, EnableBITSUploads::<Impl, OFFSET>, DisableBITSUploads::<Impl, OFFSET>, GetCleanupTaskName::<Impl, OFFSET>, GetCleanupTask::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBITSExtensionSetupFactoryImpl: Sized + IDispatchImpl {
    fn GetObject();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IBITSExtensionSetupFactory {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBITSExtensionSetupFactory";
}
#[cfg(feature = "Win32_System_Com")]
impl IBITSExtensionSetupFactoryVtbl {
    pub const fn new<Impl: IBITSExtensionSetupFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBITSExtensionSetupFactoryVtbl {
        unsafe extern "system" fn GetObject<Impl: IBITSExtensionSetupFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppextensionsetup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetObject(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppextensionsetup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBITSExtensionSetupFactory>, base.5, GetObject::<Impl, OFFSET>)
    }
}
pub trait IBackgroundCopyCallbackImpl: Sized {
    fn JobTransferred();
    fn JobError();
    fn JobModification();
}
impl ::windows::core::RuntimeName for IBackgroundCopyCallback {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyCallback";
}
impl IBackgroundCopyCallbackVtbl {
    pub const fn new<Impl: IBackgroundCopyCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyCallbackVtbl {
        unsafe extern "system" fn JobTransferred<Impl: IBackgroundCopyCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).JobTransferred(&*(&pjob as *const <IBackgroundCopyJob as ::windows::core::Abi>::Abi as *const <IBackgroundCopyJob as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobError<Impl: IBackgroundCopyCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr, perror: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).JobError(&*(&pjob as *const <IBackgroundCopyJob as ::windows::core::Abi>::Abi as *const <IBackgroundCopyJob as ::windows::core::DefaultType>::DefaultType), &*(&perror as *const <IBackgroundCopyError as ::windows::core::Abi>::Abi as *const <IBackgroundCopyError as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobModification<Impl: IBackgroundCopyCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).JobModification(&*(&pjob as *const <IBackgroundCopyJob as ::windows::core::Abi>::Abi as *const <IBackgroundCopyJob as ::windows::core::DefaultType>::DefaultType), dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundCopyCallback>, base.5, JobTransferred::<Impl, OFFSET>, JobError::<Impl, OFFSET>, JobModification::<Impl, OFFSET>)
    }
}
pub trait IBackgroundCopyCallback1Impl: Sized {
    fn OnStatus();
    fn OnProgress();
    fn OnProgressEx();
}
impl ::windows::core::RuntimeName for IBackgroundCopyCallback1 {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyCallback1";
}
impl IBackgroundCopyCallback1Vtbl {
    pub const fn new<Impl: IBackgroundCopyCallback1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyCallback1Vtbl {
        unsafe extern "system" fn OnStatus<Impl: IBackgroundCopyCallback1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgroup: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, dwfileindex: u32, dwstatus: u32, dwnumofretries: u32, dwwin32result: u32, dwtransportresult: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnStatus(&*(&pgroup as *const <IBackgroundCopyGroup as ::windows::core::Abi>::Abi as *const <IBackgroundCopyGroup as ::windows::core::DefaultType>::DefaultType), &*(&pjob as *const <IBackgroundCopyJob1 as ::windows::core::Abi>::Abi as *const <IBackgroundCopyJob1 as ::windows::core::DefaultType>::DefaultType), dwfileindex, dwstatus, dwnumofretries, dwwin32result, dwtransportresult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnProgress<Impl: IBackgroundCopyCallback1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, progresstype: u32, pgroup: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, dwfileindex: u32, dwprogressvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnProgress(progresstype, &*(&pgroup as *const <IBackgroundCopyGroup as ::windows::core::Abi>::Abi as *const <IBackgroundCopyGroup as ::windows::core::DefaultType>::DefaultType), &*(&pjob as *const <IBackgroundCopyJob1 as ::windows::core::Abi>::Abi as *const <IBackgroundCopyJob1 as ::windows::core::DefaultType>::DefaultType), dwfileindex, dwprogressvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnProgressEx<Impl: IBackgroundCopyCallback1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, progresstype: u32, pgroup: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, dwfileindex: u32, dwprogressvalue: u32, dwbytearraysize: u32, pbyte: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnProgressEx(progresstype, &*(&pgroup as *const <IBackgroundCopyGroup as ::windows::core::Abi>::Abi as *const <IBackgroundCopyGroup as ::windows::core::DefaultType>::DefaultType), &*(&pjob as *const <IBackgroundCopyJob1 as ::windows::core::Abi>::Abi as *const <IBackgroundCopyJob1 as ::windows::core::DefaultType>::DefaultType), dwfileindex, dwprogressvalue, dwbytearraysize, pbyte) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundCopyCallback1>, base.5, OnStatus::<Impl, OFFSET>, OnProgress::<Impl, OFFSET>, OnProgressEx::<Impl, OFFSET>)
    }
}
pub trait IBackgroundCopyCallback2Impl: Sized + IBackgroundCopyCallbackImpl {
    fn FileTransferred();
}
impl ::windows::core::RuntimeName for IBackgroundCopyCallback2 {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyCallback2";
}
impl IBackgroundCopyCallback2Vtbl {
    pub const fn new<Impl: IBackgroundCopyCallback2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyCallback2Vtbl {
        unsafe extern "system" fn FileTransferred<Impl: IBackgroundCopyCallback2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr, pfile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FileTransferred(&*(&pjob as *const <IBackgroundCopyJob as ::windows::core::Abi>::Abi as *const <IBackgroundCopyJob as ::windows::core::DefaultType>::DefaultType), &*(&pfile as *const <IBackgroundCopyFile as ::windows::core::Abi>::Abi as *const <IBackgroundCopyFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundCopyCallback2>, base.5, FileTransferred::<Impl, OFFSET>)
    }
}
pub trait IBackgroundCopyCallback3Impl: Sized + IBackgroundCopyCallback2Impl + IBackgroundCopyCallbackImpl {
    fn FileRangesTransferred();
}
impl ::windows::core::RuntimeName for IBackgroundCopyCallback3 {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyCallback3";
}
impl IBackgroundCopyCallback3Vtbl {
    pub const fn new<Impl: IBackgroundCopyCallback3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyCallback3Vtbl {
        unsafe extern "system" fn FileRangesTransferred<Impl: IBackgroundCopyCallback3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, job: ::windows::core::RawPtr, file: ::windows::core::RawPtr, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FileRangesTransferred(&*(&job as *const <IBackgroundCopyJob as ::windows::core::Abi>::Abi as *const <IBackgroundCopyJob as ::windows::core::DefaultType>::DefaultType), &*(&file as *const <IBackgroundCopyFile as ::windows::core::Abi>::Abi as *const <IBackgroundCopyFile as ::windows::core::DefaultType>::DefaultType), rangecount, &*(&ranges as *const <BG_FILE_RANGE as ::windows::core::Abi>::Abi as *const <BG_FILE_RANGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundCopyCallback3>, base.5, FileRangesTransferred::<Impl, OFFSET>)
    }
}
pub trait IBackgroundCopyErrorImpl: Sized {
    fn GetError();
    fn GetFile();
    fn GetErrorDescription();
    fn GetErrorContextDescription();
    fn GetProtocol();
}
impl ::windows::core::RuntimeName for IBackgroundCopyError {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyError";
}
impl IBackgroundCopyErrorVtbl {
    pub const fn new<Impl: IBackgroundCopyErrorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyErrorVtbl {
        unsafe extern "system" fn GetError<Impl: IBackgroundCopyErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontext: *mut BG_ERROR_CONTEXT, pcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetError(::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&pcode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFile<Impl: IBackgroundCopyErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFile(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorDescription<Impl: IBackgroundCopyErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languageid: u32, perrordescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetErrorDescription(languageid, ::core::mem::transmute_copy(&perrordescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorContextDescription<Impl: IBackgroundCopyErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languageid: u32, pcontextdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetErrorContextDescription(languageid, ::core::mem::transmute_copy(&pcontextdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProtocol<Impl: IBackgroundCopyErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprotocol: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProtocol(::core::mem::transmute_copy(&pprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundCopyError>, base.5, GetError::<Impl, OFFSET>, GetFile::<Impl, OFFSET>, GetErrorDescription::<Impl, OFFSET>, GetErrorContextDescription::<Impl, OFFSET>, GetProtocol::<Impl, OFFSET>)
    }
}
pub trait IBackgroundCopyFileImpl: Sized {
    fn GetRemoteName();
    fn GetLocalName();
    fn GetProgress();
}
impl ::windows::core::RuntimeName for IBackgroundCopyFile {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyFile";
}
impl IBackgroundCopyFileVtbl {
    pub const fn new<Impl: IBackgroundCopyFileImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyFileVtbl {
        unsafe extern "system" fn GetRemoteName<Impl: IBackgroundCopyFileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRemoteName(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalName<Impl: IBackgroundCopyFileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLocalName(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProgress<Impl: IBackgroundCopyFileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut BG_FILE_PROGRESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProgress(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundCopyFile>, base.5, GetRemoteName::<Impl, OFFSET>, GetLocalName::<Impl, OFFSET>, GetProgress::<Impl, OFFSET>)
    }
}
pub trait IBackgroundCopyFile2Impl: Sized + IBackgroundCopyFileImpl {
    fn GetFileRanges();
    fn SetRemoteName();
}
impl ::windows::core::RuntimeName for IBackgroundCopyFile2 {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyFile2";
}
impl IBackgroundCopyFile2Vtbl {
    pub const fn new<Impl: IBackgroundCopyFile2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyFile2Vtbl {
        unsafe extern "system" fn GetFileRanges<Impl: IBackgroundCopyFile2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFileRanges(rangecount, ::core::mem::transmute_copy(&ranges)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteName<Impl: IBackgroundCopyFile2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRemoteName(&*(&val as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundCopyFile2>, base.5, GetFileRanges::<Impl, OFFSET>, SetRemoteName::<Impl, OFFSET>)
    }
}
pub trait IBackgroundCopyFile3Impl: Sized + IBackgroundCopyFile2Impl + IBackgroundCopyFileImpl {
    fn GetTemporaryName();
    fn SetValidationState();
    fn GetValidationState();
    fn IsDownloadedFromPeer();
}
impl ::windows::core::RuntimeName for IBackgroundCopyFile3 {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyFile3";
}
impl IBackgroundCopyFile3Vtbl {
    pub const fn new<Impl: IBackgroundCopyFile3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyFile3Vtbl {
        unsafe extern "system" fn GetTemporaryName<Impl: IBackgroundCopyFile3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTemporaryName(::core::mem::transmute_copy(&pfilename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValidationState<Impl: IBackgroundCopyFile3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetValidationState(&*(&state as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValidationState<Impl: IBackgroundCopyFile3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetValidationState(::core::mem::transmute_copy(&pstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDownloadedFromPeer<Impl: IBackgroundCopyFile3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDownloadedFromPeer(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundCopyFile3>, base.5, GetTemporaryName::<Impl, OFFSET>, SetValidationState::<Impl, OFFSET>, GetValidationState::<Impl, OFFSET>, IsDownloadedFromPeer::<Impl, OFFSET>)
    }
}
pub trait IBackgroundCopyFile4Impl: Sized + IBackgroundCopyFile3Impl + IBackgroundCopyFile2Impl + IBackgroundCopyFileImpl {
    fn GetPeerDownloadStats();
}
impl ::windows::core::RuntimeName for IBackgroundCopyFile4 {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyFile4";
}
impl IBackgroundCopyFile4Vtbl {
    pub const fn new<Impl: IBackgroundCopyFile4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyFile4Vtbl {
        unsafe extern "system" fn GetPeerDownloadStats<Impl: IBackgroundCopyFile4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfromorigin: *mut u64, pfrompeers: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPeerDownloadStats(::core::mem::transmute_copy(&pfromorigin), ::core::mem::transmute_copy(&pfrompeers)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundCopyFile4>, base.5, GetPeerDownloadStats::<Impl, OFFSET>)
    }
}
pub trait IBackgroundCopyFile5Impl: Sized + IBackgroundCopyFile4Impl + IBackgroundCopyFile3Impl + IBackgroundCopyFile2Impl + IBackgroundCopyFileImpl {
    fn SetProperty();
    fn GetProperty();
}
impl ::windows::core::RuntimeName for IBackgroundCopyFile5 {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyFile5";
}
impl IBackgroundCopyFile5Vtbl {
    pub const fn new<Impl: IBackgroundCopyFile5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyFile5Vtbl {
        unsafe extern "system" fn SetProperty<Impl: IBackgroundCopyFile5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: BITS_FILE_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProperty(propertyid, &*(&propertyvalue as *const <BITS_FILE_PROPERTY_VALUE as ::windows::core::Abi>::Abi as *const <BITS_FILE_PROPERTY_VALUE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IBackgroundCopyFile5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: *mut BITS_FILE_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProperty(propertyid, ::core::mem::transmute_copy(&propertyvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundCopyFile5>, base.5, SetProperty::<Impl, OFFSET>, GetProperty::<Impl, OFFSET>)
    }
}
pub trait IBackgroundCopyFile6Impl: Sized + IBackgroundCopyFile5Impl + IBackgroundCopyFile4Impl + IBackgroundCopyFile3Impl + IBackgroundCopyFile2Impl + IBackgroundCopyFileImpl {
    fn UpdateDownloadPosition();
    fn RequestFileRanges();
    fn GetFilledFileRanges();
}
impl ::windows::core::RuntimeName for IBackgroundCopyFile6 {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyFile6";
}
impl IBackgroundCopyFile6Vtbl {
    pub const fn new<Impl: IBackgroundCopyFile6Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyFile6Vtbl {
        unsafe extern "system" fn UpdateDownloadPosition<Impl: IBackgroundCopyFile6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateDownloadPosition(offset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestFileRanges<Impl: IBackgroundCopyFile6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestFileRanges(rangecount, &*(&ranges as *const <BG_FILE_RANGE as ::windows::core::Abi>::Abi as *const <BG_FILE_RANGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilledFileRanges<Impl: IBackgroundCopyFile6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFilledFileRanges(::core::mem::transmute_copy(&rangecount), ::core::mem::transmute_copy(&ranges)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundCopyFile6>, base.5, UpdateDownloadPosition::<Impl, OFFSET>, RequestFileRanges::<Impl, OFFSET>, GetFilledFileRanges::<Impl, OFFSET>)
    }
}
pub trait IBackgroundCopyGroupImpl: Sized {
    fn GetProp();
    fn SetProp();
    fn GetProgress();
    fn GetStatus();
    fn GetJob();
    fn SuspendGroup();
    fn ResumeGroup();
    fn CancelGroup();
    fn Size();
    fn GroupID();
    fn CreateJob();
    fn EnumJobs();
    fn SwitchToForeground();
    fn QueryNewJobInterface();
    fn SetNotificationPointer();
}
impl ::windows::core::RuntimeName for IBackgroundCopyGroup {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyGroup";
}
impl IBackgroundCopyGroupVtbl {
    pub const fn new<Impl: IBackgroundCopyGroupImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyGroupVtbl {
        unsafe extern "system" fn GetProp<Impl: IBackgroundCopyGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propid: GROUPPROP, pvarval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProp(propid, ::core::mem::transmute_copy(&pvarval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProp<Impl: IBackgroundCopyGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propid: GROUPPROP, pvarval: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProp(propid, &*(&pvarval as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProgress<Impl: IBackgroundCopyGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdwprogress: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProgress(dwflags, ::core::mem::transmute_copy(&pdwprogress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IBackgroundCopyGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32, pdwjobindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&pdwstatus), ::core::mem::transmute_copy(&pdwjobindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJob<Impl: IBackgroundCopyGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, jobid: ::windows::core::GUID, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetJob(&*(&jobid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppjob)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuspendGroup<Impl: IBackgroundCopyGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SuspendGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResumeGroup<Impl: IBackgroundCopyGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResumeGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelGroup<Impl: IBackgroundCopyGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CancelGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IBackgroundCopyGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Size(::core::mem::transmute_copy(&pdwsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GroupID<Impl: IBackgroundCopyGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidgroupid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GroupID(::core::mem::transmute_copy(&pguidgroupid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateJob<Impl: IBackgroundCopyGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidjobid: ::windows::core::GUID, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateJob(&*(&guidjobid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppjob)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumJobs<Impl: IBackgroundCopyGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppenumjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumJobs(dwflags, ::core::mem::transmute_copy(&ppenumjobs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SwitchToForeground<Impl: IBackgroundCopyGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SwitchToForeground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryNewJobInterface<Impl: IBackgroundCopyGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryNewJobInterface(&*(&iid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&punk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationPointer<Impl: IBackgroundCopyGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNotificationPointer(&*(&iid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IBackgroundCopyGroup>,
            base.5,
            GetProp::<Impl, OFFSET>,
            SetProp::<Impl, OFFSET>,
            GetProgress::<Impl, OFFSET>,
            GetStatus::<Impl, OFFSET>,
            GetJob::<Impl, OFFSET>,
            SuspendGroup::<Impl, OFFSET>,
            ResumeGroup::<Impl, OFFSET>,
            CancelGroup::<Impl, OFFSET>,
            Size::<Impl, OFFSET>,
            GroupID::<Impl, OFFSET>,
            CreateJob::<Impl, OFFSET>,
            EnumJobs::<Impl, OFFSET>,
            SwitchToForeground::<Impl, OFFSET>,
            QueryNewJobInterface::<Impl, OFFSET>,
            SetNotificationPointer::<Impl, OFFSET>,
        )
    }
}
pub trait IBackgroundCopyJobImpl: Sized {
    fn AddFileSet();
    fn AddFile();
    fn EnumFiles();
    fn Suspend();
    fn Resume();
    fn Cancel();
    fn Complete();
    fn GetId();
    fn GetType();
    fn GetProgress();
    fn GetTimes();
    fn GetState();
    fn GetError();
    fn GetOwner();
    fn SetDisplayName();
    fn GetDisplayName();
    fn SetDescription();
    fn GetDescription();
    fn SetPriority();
    fn GetPriority();
    fn SetNotifyFlags();
    fn GetNotifyFlags();
    fn SetNotifyInterface();
    fn GetNotifyInterface();
    fn SetMinimumRetryDelay();
    fn GetMinimumRetryDelay();
    fn SetNoProgressTimeout();
    fn GetNoProgressTimeout();
    fn GetErrorCount();
    fn SetProxySettings();
    fn GetProxySettings();
    fn TakeOwnership();
}
impl ::windows::core::RuntimeName for IBackgroundCopyJob {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyJob";
}
impl IBackgroundCopyJobVtbl {
    pub const fn new<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyJobVtbl {
        unsafe extern "system" fn AddFileSet<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddFileSet(cfilecount, &*(&pfileset as *const <BG_FILE_INFO as ::windows::core::Abi>::Abi as *const <BG_FILE_INFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddFile<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remoteurl: super::super::Foundation::PWSTR, localname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddFile(&*(&remoteurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&localname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFiles<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumFiles(::core::mem::transmute_copy(&penum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Suspend<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Suspend() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Complete<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Complete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetId<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetId(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetType(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProgress<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_PROGRESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProgress(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimes<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_TIMES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTimes(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetState<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetState(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetError<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pperror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetError(::core::mem::transmute_copy(&pperror)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOwner<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOwner(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDisplayName(&*(&val as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayName<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDisplayName(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDescription(&*(&val as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDescription(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: BG_JOB_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPriority(val) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPriority<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPriority(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotifyFlags<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNotifyFlags(val) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNotifyFlags<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNotifyFlags(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotifyInterface<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNotifyInterface(&*(&val as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNotifyInterface<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNotifyInterface(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumRetryDelay<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMinimumRetryDelay(seconds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinimumRetryDelay<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMinimumRetryDelay(::core::mem::transmute_copy(&seconds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoProgressTimeout<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNoProgressTimeout(seconds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNoProgressTimeout<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNoProgressTimeout(::core::mem::transmute_copy(&seconds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorCount<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errors: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetErrorCount(::core::mem::transmute_copy(&errors)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxySettings<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, proxyusage: BG_JOB_PROXY_USAGE, proxylist: super::super::Foundation::PWSTR, proxybypasslist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProxySettings(proxyusage, &*(&proxylist as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&proxybypasslist as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProxySettings<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut super::super::Foundation::PWSTR, pproxybypasslist: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProxySettings(::core::mem::transmute_copy(&pproxyusage), ::core::mem::transmute_copy(&pproxylist), ::core::mem::transmute_copy(&pproxybypasslist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TakeOwnership<Impl: IBackgroundCopyJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TakeOwnership() {
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
            ::windows::core::GetRuntimeClassName::<IBackgroundCopyJob>,
            base.5,
            AddFileSet::<Impl, OFFSET>,
            AddFile::<Impl, OFFSET>,
            EnumFiles::<Impl, OFFSET>,
            Suspend::<Impl, OFFSET>,
            Resume::<Impl, OFFSET>,
            Cancel::<Impl, OFFSET>,
            Complete::<Impl, OFFSET>,
            GetId::<Impl, OFFSET>,
            GetType::<Impl, OFFSET>,
            GetProgress::<Impl, OFFSET>,
            GetTimes::<Impl, OFFSET>,
            GetState::<Impl, OFFSET>,
            GetError::<Impl, OFFSET>,
            GetOwner::<Impl, OFFSET>,
            SetDisplayName::<Impl, OFFSET>,
            GetDisplayName::<Impl, OFFSET>,
            SetDescription::<Impl, OFFSET>,
            GetDescription::<Impl, OFFSET>,
            SetPriority::<Impl, OFFSET>,
            GetPriority::<Impl, OFFSET>,
            SetNotifyFlags::<Impl, OFFSET>,
            GetNotifyFlags::<Impl, OFFSET>,
            SetNotifyInterface::<Impl, OFFSET>,
            GetNotifyInterface::<Impl, OFFSET>,
            SetMinimumRetryDelay::<Impl, OFFSET>,
            GetMinimumRetryDelay::<Impl, OFFSET>,
            SetNoProgressTimeout::<Impl, OFFSET>,
            GetNoProgressTimeout::<Impl, OFFSET>,
            GetErrorCount::<Impl, OFFSET>,
            SetProxySettings::<Impl, OFFSET>,
            GetProxySettings::<Impl, OFFSET>,
            TakeOwnership::<Impl, OFFSET>,
        )
    }
}
pub trait IBackgroundCopyJob1Impl: Sized {
    fn CancelJob();
    fn GetProgress();
    fn GetStatus();
    fn AddFiles();
    fn GetFile();
    fn GetFileCount();
    fn SwitchToForeground();
    fn JobID();
}
impl ::windows::core::RuntimeName for IBackgroundCopyJob1 {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyJob1";
}
impl IBackgroundCopyJob1Vtbl {
    pub const fn new<Impl: IBackgroundCopyJob1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyJob1Vtbl {
        unsafe extern "system" fn CancelJob<Impl: IBackgroundCopyJob1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CancelJob() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProgress<Impl: IBackgroundCopyJob1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdwprogress: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProgress(dwflags, ::core::mem::transmute_copy(&pdwprogress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IBackgroundCopyJob1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32, pdwwin32result: *mut u32, pdwtransportresult: *mut u32, pdwnumofretries: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&pdwstatus), ::core::mem::transmute_copy(&pdwwin32result), ::core::mem::transmute_copy(&pdwtransportresult), ::core::mem::transmute_copy(&pdwnumofretries)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddFiles<Impl: IBackgroundCopyJob1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cfilecount: u32, ppfileset: *const *const FILESETINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddFiles(cfilecount, &*(&ppfileset as *const <FILESETINFO as ::windows::core::Abi>::Abi as *const <FILESETINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFile<Impl: IBackgroundCopyJob1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cfileindex: u32, pfileinfo: *mut FILESETINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFile(cfileindex, ::core::mem::transmute_copy(&pfileinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileCount<Impl: IBackgroundCopyJob1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwfilecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFileCount(::core::mem::transmute_copy(&pdwfilecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SwitchToForeground<Impl: IBackgroundCopyJob1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SwitchToForeground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobID<Impl: IBackgroundCopyJob1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidjobid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).JobID(::core::mem::transmute_copy(&pguidjobid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundCopyJob1>, base.5, CancelJob::<Impl, OFFSET>, GetProgress::<Impl, OFFSET>, GetStatus::<Impl, OFFSET>, AddFiles::<Impl, OFFSET>, GetFile::<Impl, OFFSET>, GetFileCount::<Impl, OFFSET>, SwitchToForeground::<Impl, OFFSET>, JobID::<Impl, OFFSET>)
    }
}
pub trait IBackgroundCopyJob2Impl: Sized + IBackgroundCopyJobImpl {
    fn SetNotifyCmdLine();
    fn GetNotifyCmdLine();
    fn GetReplyProgress();
    fn GetReplyData();
    fn SetReplyFileName();
    fn GetReplyFileName();
    fn SetCredentials();
    fn RemoveCredentials();
}
impl ::windows::core::RuntimeName for IBackgroundCopyJob2 {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyJob2";
}
impl IBackgroundCopyJob2Vtbl {
    pub const fn new<Impl: IBackgroundCopyJob2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyJob2Vtbl {
        unsafe extern "system" fn SetNotifyCmdLine<Impl: IBackgroundCopyJob2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, program: super::super::Foundation::PWSTR, parameters: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNotifyCmdLine(&*(&program as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&parameters as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNotifyCmdLine<Impl: IBackgroundCopyJob2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprogram: *mut super::super::Foundation::PWSTR, pparameters: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNotifyCmdLine(::core::mem::transmute_copy(&pprogram), ::core::mem::transmute_copy(&pparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReplyProgress<Impl: IBackgroundCopyJob2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReplyProgress(&*(&pprogress as *const <BG_JOB_REPLY_PROGRESS as ::windows::core::Abi>::Abi as *const <BG_JOB_REPLY_PROGRESS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReplyData<Impl: IBackgroundCopyJob2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbuffer: *mut *mut u8, plength: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReplyData(::core::mem::transmute_copy(&ppbuffer), plength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReplyFileName<Impl: IBackgroundCopyJob2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, replyfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetReplyFileName(&*(&replyfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReplyFileName<Impl: IBackgroundCopyJob2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preplyfilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReplyFileName(::core::mem::transmute_copy(&preplyfilename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Impl: IBackgroundCopyJob2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, credentials: *const BG_AUTH_CREDENTIALS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCredentials(&*(&credentials as *const <BG_AUTH_CREDENTIALS as ::windows::core::Abi>::Abi as *const <BG_AUTH_CREDENTIALS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCredentials<Impl: IBackgroundCopyJob2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveCredentials(target, scheme) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundCopyJob2>, base.5, SetNotifyCmdLine::<Impl, OFFSET>, GetNotifyCmdLine::<Impl, OFFSET>, GetReplyProgress::<Impl, OFFSET>, GetReplyData::<Impl, OFFSET>, SetReplyFileName::<Impl, OFFSET>, GetReplyFileName::<Impl, OFFSET>, SetCredentials::<Impl, OFFSET>, RemoveCredentials::<Impl, OFFSET>)
    }
}
pub trait IBackgroundCopyJob3Impl: Sized + IBackgroundCopyJob2Impl + IBackgroundCopyJobImpl {
    fn ReplaceRemotePrefix();
    fn AddFileWithRanges();
    fn SetFileACLFlags();
    fn GetFileACLFlags();
}
impl ::windows::core::RuntimeName for IBackgroundCopyJob3 {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyJob3";
}
impl IBackgroundCopyJob3Vtbl {
    pub const fn new<Impl: IBackgroundCopyJob3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyJob3Vtbl {
        unsafe extern "system" fn ReplaceRemotePrefix<Impl: IBackgroundCopyJob3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oldprefix: super::super::Foundation::PWSTR, newprefix: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReplaceRemotePrefix(&*(&oldprefix as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&newprefix as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddFileWithRanges<Impl: IBackgroundCopyJob3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remoteurl: super::super::Foundation::PWSTR, localname: super::super::Foundation::PWSTR, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddFileWithRanges(
                &*(&remoteurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&localname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                rangecount,
                &*(&ranges as *const <BG_FILE_RANGE as ::windows::core::Abi>::Abi as *const <BG_FILE_RANGE as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileACLFlags<Impl: IBackgroundCopyJob3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFileACLFlags(flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileACLFlags<Impl: IBackgroundCopyJob3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFileACLFlags(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundCopyJob3>, base.5, ReplaceRemotePrefix::<Impl, OFFSET>, AddFileWithRanges::<Impl, OFFSET>, SetFileACLFlags::<Impl, OFFSET>, GetFileACLFlags::<Impl, OFFSET>)
    }
}
pub trait IBackgroundCopyJob4Impl: Sized + IBackgroundCopyJob3Impl + IBackgroundCopyJob2Impl + IBackgroundCopyJobImpl {
    fn SetPeerCachingFlags();
    fn GetPeerCachingFlags();
    fn GetOwnerIntegrityLevel();
    fn GetOwnerElevationState();
    fn SetMaximumDownloadTime();
    fn GetMaximumDownloadTime();
}
impl ::windows::core::RuntimeName for IBackgroundCopyJob4 {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyJob4";
}
impl IBackgroundCopyJob4Vtbl {
    pub const fn new<Impl: IBackgroundCopyJob4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyJob4Vtbl {
        unsafe extern "system" fn SetPeerCachingFlags<Impl: IBackgroundCopyJob4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPeerCachingFlags(flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPeerCachingFlags<Impl: IBackgroundCopyJob4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPeerCachingFlags(::core::mem::transmute_copy(&pflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOwnerIntegrityLevel<Impl: IBackgroundCopyJob4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOwnerIntegrityLevel(::core::mem::transmute_copy(&plevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOwnerElevationState<Impl: IBackgroundCopyJob4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pelevated: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOwnerElevationState(::core::mem::transmute_copy(&pelevated)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumDownloadTime<Impl: IBackgroundCopyJob4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMaximumDownloadTime(timeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaximumDownloadTime<Impl: IBackgroundCopyJob4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMaximumDownloadTime(::core::mem::transmute_copy(&ptimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundCopyJob4>, base.5, SetPeerCachingFlags::<Impl, OFFSET>, GetPeerCachingFlags::<Impl, OFFSET>, GetOwnerIntegrityLevel::<Impl, OFFSET>, GetOwnerElevationState::<Impl, OFFSET>, SetMaximumDownloadTime::<Impl, OFFSET>, GetMaximumDownloadTime::<Impl, OFFSET>)
    }
}
pub trait IBackgroundCopyJob5Impl: Sized + IBackgroundCopyJob4Impl + IBackgroundCopyJob3Impl + IBackgroundCopyJob2Impl + IBackgroundCopyJobImpl {
    fn SetProperty();
    fn GetProperty();
}
impl ::windows::core::RuntimeName for IBackgroundCopyJob5 {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyJob5";
}
impl IBackgroundCopyJob5Vtbl {
    pub const fn new<Impl: IBackgroundCopyJob5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyJob5Vtbl {
        unsafe extern "system" fn SetProperty<Impl: IBackgroundCopyJob5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: BITS_JOB_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProperty(propertyid, &*(&propertyvalue as *const <BITS_JOB_PROPERTY_VALUE as ::windows::core::Abi>::Abi as *const <BITS_JOB_PROPERTY_VALUE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IBackgroundCopyJob5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: *mut BITS_JOB_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProperty(propertyid, ::core::mem::transmute_copy(&propertyvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundCopyJob5>, base.5, SetProperty::<Impl, OFFSET>, GetProperty::<Impl, OFFSET>)
    }
}
pub trait IBackgroundCopyJobHttpOptionsImpl: Sized {
    fn SetClientCertificateByID();
    fn SetClientCertificateByName();
    fn RemoveClientCertificate();
    fn GetClientCertificate();
    fn SetCustomHeaders();
    fn GetCustomHeaders();
    fn SetSecurityFlags();
    fn GetSecurityFlags();
}
impl ::windows::core::RuntimeName for IBackgroundCopyJobHttpOptions {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyJobHttpOptions";
}
impl IBackgroundCopyJobHttpOptionsVtbl {
    pub const fn new<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyJobHttpOptionsVtbl {
        unsafe extern "system" fn SetClientCertificateByID<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storelocation: BG_CERT_STORE_LOCATION, storename: super::super::Foundation::PWSTR, pcerthashblob: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClientCertificateByID(storelocation, &*(&storename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pcerthashblob) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientCertificateByName<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storelocation: BG_CERT_STORE_LOCATION, storename: super::super::Foundation::PWSTR, subjectname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClientCertificateByName(storelocation, &*(&storename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&subjectname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClientCertificate<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveClientCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClientCertificate<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut super::super::Foundation::PWSTR, ppcerthashblob: *mut *mut u8, psubjectname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClientCertificate(::core::mem::transmute_copy(&pstorelocation), ::core::mem::transmute_copy(&pstorename), ::core::mem::transmute_copy(&ppcerthashblob), ::core::mem::transmute_copy(&psubjectname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomHeaders<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestheaders: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCustomHeaders(&*(&requestheaders as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomHeaders<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prequestheaders: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCustomHeaders(::core::mem::transmute_copy(&prequestheaders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityFlags<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSecurityFlags(flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityFlags<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSecurityFlags(::core::mem::transmute_copy(&pflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundCopyJobHttpOptions>, base.5, SetClientCertificateByID::<Impl, OFFSET>, SetClientCertificateByName::<Impl, OFFSET>, RemoveClientCertificate::<Impl, OFFSET>, GetClientCertificate::<Impl, OFFSET>, SetCustomHeaders::<Impl, OFFSET>, GetCustomHeaders::<Impl, OFFSET>, SetSecurityFlags::<Impl, OFFSET>, GetSecurityFlags::<Impl, OFFSET>)
    }
}
pub trait IBackgroundCopyJobHttpOptions2Impl: Sized + IBackgroundCopyJobHttpOptionsImpl {
    fn SetHttpMethod();
    fn GetHttpMethod();
}
impl ::windows::core::RuntimeName for IBackgroundCopyJobHttpOptions2 {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyJobHttpOptions2";
}
impl IBackgroundCopyJobHttpOptions2Vtbl {
    pub const fn new<Impl: IBackgroundCopyJobHttpOptions2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyJobHttpOptions2Vtbl {
        unsafe extern "system" fn SetHttpMethod<Impl: IBackgroundCopyJobHttpOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, method: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetHttpMethod(&*(&method as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHttpMethod<Impl: IBackgroundCopyJobHttpOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, method: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHttpMethod(::core::mem::transmute_copy(&method)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundCopyJobHttpOptions2>, base.5, SetHttpMethod::<Impl, OFFSET>, GetHttpMethod::<Impl, OFFSET>)
    }
}
pub trait IBackgroundCopyJobHttpOptions3Impl: Sized + IBackgroundCopyJobHttpOptions2Impl + IBackgroundCopyJobHttpOptionsImpl {
    fn SetServerCertificateValidationInterface();
    fn MakeCustomHeadersWriteOnly();
}
impl ::windows::core::RuntimeName for IBackgroundCopyJobHttpOptions3 {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyJobHttpOptions3";
}
impl IBackgroundCopyJobHttpOptions3Vtbl {
    pub const fn new<Impl: IBackgroundCopyJobHttpOptions3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyJobHttpOptions3Vtbl {
        unsafe extern "system" fn SetServerCertificateValidationInterface<Impl: IBackgroundCopyJobHttpOptions3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certvalidationcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetServerCertificateValidationInterface(&*(&certvalidationcallback as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeCustomHeadersWriteOnly<Impl: IBackgroundCopyJobHttpOptions3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MakeCustomHeadersWriteOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundCopyJobHttpOptions3>, base.5, SetServerCertificateValidationInterface::<Impl, OFFSET>, MakeCustomHeadersWriteOnly::<Impl, OFFSET>)
    }
}
pub trait IBackgroundCopyManagerImpl: Sized {
    fn CreateJob();
    fn GetJob();
    fn EnumJobs();
    fn GetErrorDescription();
}
impl ::windows::core::RuntimeName for IBackgroundCopyManager {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyManager";
}
impl IBackgroundCopyManagerVtbl {
    pub const fn new<Impl: IBackgroundCopyManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyManagerVtbl {
        unsafe extern "system" fn CreateJob<Impl: IBackgroundCopyManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displayname: super::super::Foundation::PWSTR, r#type: BG_JOB_TYPE, pjobid: *mut ::windows::core::GUID, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateJob(&*(&displayname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), r#type, ::core::mem::transmute_copy(&pjobid), ::core::mem::transmute_copy(&ppjob)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJob<Impl: IBackgroundCopyManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, jobid: *const ::windows::core::GUID, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetJob(&*(&jobid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppjob)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumJobs<Impl: IBackgroundCopyManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumJobs(dwflags, ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorDescription<Impl: IBackgroundCopyManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, languageid: u32, perrordescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetErrorDescription(hresult, languageid, ::core::mem::transmute_copy(&perrordescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundCopyManager>, base.5, CreateJob::<Impl, OFFSET>, GetJob::<Impl, OFFSET>, EnumJobs::<Impl, OFFSET>, GetErrorDescription::<Impl, OFFSET>)
    }
}
pub trait IBackgroundCopyQMgrImpl: Sized {
    fn CreateGroup();
    fn GetGroup();
    fn EnumGroups();
}
impl ::windows::core::RuntimeName for IBackgroundCopyQMgr {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyQMgr";
}
impl IBackgroundCopyQMgrVtbl {
    pub const fn new<Impl: IBackgroundCopyQMgrImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyQMgrVtbl {
        unsafe extern "system" fn CreateGroup<Impl: IBackgroundCopyQMgrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidgroupid: ::windows::core::GUID, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateGroup(&*(&guidgroupid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGroup<Impl: IBackgroundCopyQMgrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, groupid: ::windows::core::GUID, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGroup(&*(&groupid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumGroups<Impl: IBackgroundCopyQMgrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppenumgroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumGroups(dwflags, ::core::mem::transmute_copy(&ppenumgroups)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundCopyQMgr>, base.5, CreateGroup::<Impl, OFFSET>, GetGroup::<Impl, OFFSET>, EnumGroups::<Impl, OFFSET>)
    }
}
pub trait IBackgroundCopyServerCertificateValidationCallbackImpl: Sized {
    fn ValidateServerCertificate();
}
impl ::windows::core::RuntimeName for IBackgroundCopyServerCertificateValidationCallback {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBackgroundCopyServerCertificateValidationCallback";
}
impl IBackgroundCopyServerCertificateValidationCallbackVtbl {
    pub const fn new<Impl: IBackgroundCopyServerCertificateValidationCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundCopyServerCertificateValidationCallbackVtbl {
        unsafe extern "system" fn ValidateServerCertificate<Impl: IBackgroundCopyServerCertificateValidationCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, job: ::windows::core::RawPtr, file: ::windows::core::RawPtr, certlength: u32, certdata: *const u8, certencodingtype: u32, certstorelength: u32, certstoredata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ValidateServerCertificate(&*(&job as *const <IBackgroundCopyJob as ::windows::core::Abi>::Abi as *const <IBackgroundCopyJob as ::windows::core::DefaultType>::DefaultType), &*(&file as *const <IBackgroundCopyFile as ::windows::core::Abi>::Abi as *const <IBackgroundCopyFile as ::windows::core::DefaultType>::DefaultType), certlength, certdata, certencodingtype, certstorelength, certstoredata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundCopyServerCertificateValidationCallback>, base.5, ValidateServerCertificate::<Impl, OFFSET>)
    }
}
pub trait IBitsPeerImpl: Sized {
    fn GetPeerName();
    fn IsAuthenticated();
    fn IsAvailable();
}
impl ::windows::core::RuntimeName for IBitsPeer {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBitsPeer";
}
impl IBitsPeerVtbl {
    pub const fn new<Impl: IBitsPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBitsPeerVtbl {
        unsafe extern "system" fn GetPeerName<Impl: IBitsPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPeerName(::core::mem::transmute_copy(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAuthenticated<Impl: IBitsPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pauth: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAuthenticated(::core::mem::transmute_copy(&pauth)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAvailable<Impl: IBitsPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ponline: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAvailable(::core::mem::transmute_copy(&ponline)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBitsPeer>, base.5, GetPeerName::<Impl, OFFSET>, IsAuthenticated::<Impl, OFFSET>, IsAvailable::<Impl, OFFSET>)
    }
}
pub trait IBitsPeerCacheAdministrationImpl: Sized {
    fn GetMaximumCacheSize();
    fn SetMaximumCacheSize();
    fn GetMaximumContentAge();
    fn SetMaximumContentAge();
    fn GetConfigurationFlags();
    fn SetConfigurationFlags();
    fn EnumRecords();
    fn GetRecord();
    fn ClearRecords();
    fn DeleteRecord();
    fn DeleteUrl();
    fn EnumPeers();
    fn ClearPeers();
    fn DiscoverPeers();
}
impl ::windows::core::RuntimeName for IBitsPeerCacheAdministration {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBitsPeerCacheAdministration";
}
impl IBitsPeerCacheAdministrationVtbl {
    pub const fn new<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBitsPeerCacheAdministrationVtbl {
        unsafe extern "system" fn GetMaximumCacheSize<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbytes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMaximumCacheSize(::core::mem::transmute_copy(&pbytes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumCacheSize<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bytes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMaximumCacheSize(bytes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaximumContentAge<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pseconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMaximumContentAge(::core::mem::transmute_copy(&pseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumContentAge<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMaximumContentAge(seconds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConfigurationFlags<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConfigurationFlags(::core::mem::transmute_copy(&pflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConfigurationFlags<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetConfigurationFlags(flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRecords<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumRecords(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecord<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: *const ::windows::core::GUID, pprecord: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRecord(&*(&id as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprecord)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearRecords<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClearRecords() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRecord<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteRecord(&*(&id as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteUrl<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, url: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteUrl(&*(&url as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumPeers<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumPeers(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearPeers<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClearPeers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscoverPeers<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DiscoverPeers() {
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
            ::windows::core::GetRuntimeClassName::<IBitsPeerCacheAdministration>,
            base.5,
            GetMaximumCacheSize::<Impl, OFFSET>,
            SetMaximumCacheSize::<Impl, OFFSET>,
            GetMaximumContentAge::<Impl, OFFSET>,
            SetMaximumContentAge::<Impl, OFFSET>,
            GetConfigurationFlags::<Impl, OFFSET>,
            SetConfigurationFlags::<Impl, OFFSET>,
            EnumRecords::<Impl, OFFSET>,
            GetRecord::<Impl, OFFSET>,
            ClearRecords::<Impl, OFFSET>,
            DeleteRecord::<Impl, OFFSET>,
            DeleteUrl::<Impl, OFFSET>,
            EnumPeers::<Impl, OFFSET>,
            ClearPeers::<Impl, OFFSET>,
            DiscoverPeers::<Impl, OFFSET>,
        )
    }
}
pub trait IBitsPeerCacheRecordImpl: Sized {
    fn GetId();
    fn GetOriginUrl();
    fn GetFileSize();
    fn GetFileModificationTime();
    fn GetLastAccessTime();
    fn IsFileValidated();
    fn GetFileRanges();
}
impl ::windows::core::RuntimeName for IBitsPeerCacheRecord {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBitsPeerCacheRecord";
}
impl IBitsPeerCacheRecordVtbl {
    pub const fn new<Impl: IBitsPeerCacheRecordImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBitsPeerCacheRecordVtbl {
        unsafe extern "system" fn GetId<Impl: IBitsPeerCacheRecordImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetId(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginUrl<Impl: IBitsPeerCacheRecordImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOriginUrl(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileSize<Impl: IBitsPeerCacheRecordImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFileSize(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileModificationTime<Impl: IBitsPeerCacheRecordImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFileModificationTime(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastAccessTime<Impl: IBitsPeerCacheRecordImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLastAccessTime(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFileValidated<Impl: IBitsPeerCacheRecordImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsFileValidated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileRanges<Impl: IBitsPeerCacheRecordImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prangecount: *mut u32, ppranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFileRanges(::core::mem::transmute_copy(&prangecount), ::core::mem::transmute_copy(&ppranges)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBitsPeerCacheRecord>, base.5, GetId::<Impl, OFFSET>, GetOriginUrl::<Impl, OFFSET>, GetFileSize::<Impl, OFFSET>, GetFileModificationTime::<Impl, OFFSET>, GetLastAccessTime::<Impl, OFFSET>, IsFileValidated::<Impl, OFFSET>, GetFileRanges::<Impl, OFFSET>)
    }
}
pub trait IBitsTokenOptionsImpl: Sized {
    fn SetHelperTokenFlags();
    fn GetHelperTokenFlags();
    fn SetHelperToken();
    fn ClearHelperToken();
    fn GetHelperTokenSid();
}
impl ::windows::core::RuntimeName for IBitsTokenOptions {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IBitsTokenOptions";
}
impl IBitsTokenOptionsVtbl {
    pub const fn new<Impl: IBitsTokenOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBitsTokenOptionsVtbl {
        unsafe extern "system" fn SetHelperTokenFlags<Impl: IBitsTokenOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, usageflags: BG_TOKEN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetHelperTokenFlags(usageflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelperTokenFlags<Impl: IBitsTokenOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflags: *mut BG_TOKEN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHelperTokenFlags(::core::mem::transmute_copy(&pflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHelperToken<Impl: IBitsTokenOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetHelperToken() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearHelperToken<Impl: IBitsTokenOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClearHelperToken() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelperTokenSid<Impl: IBitsTokenOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHelperTokenSid(::core::mem::transmute_copy(&psid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBitsTokenOptions>, base.5, SetHelperTokenFlags::<Impl, OFFSET>, GetHelperTokenFlags::<Impl, OFFSET>, SetHelperToken::<Impl, OFFSET>, ClearHelperToken::<Impl, OFFSET>, GetHelperTokenSid::<Impl, OFFSET>)
    }
}
pub trait IEnumBackgroundCopyFilesImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
impl ::windows::core::RuntimeName for IEnumBackgroundCopyFiles {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IEnumBackgroundCopyFiles";
}
impl IEnumBackgroundCopyFilesVtbl {
    pub const fn new<Impl: IEnumBackgroundCopyFilesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumBackgroundCopyFilesVtbl {
        unsafe extern "system" fn Next<Impl: IEnumBackgroundCopyFilesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumBackgroundCopyFilesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumBackgroundCopyFilesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Clone<Impl: IEnumBackgroundCopyFilesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IEnumBackgroundCopyFilesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pucount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumBackgroundCopyFiles>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>, GetCount::<Impl, OFFSET>)
    }
}
pub trait IEnumBackgroundCopyGroupsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
impl ::windows::core::RuntimeName for IEnumBackgroundCopyGroups {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IEnumBackgroundCopyGroups";
}
impl IEnumBackgroundCopyGroupsVtbl {
    pub const fn new<Impl: IEnumBackgroundCopyGroupsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumBackgroundCopyGroupsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumBackgroundCopyGroupsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumBackgroundCopyGroupsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumBackgroundCopyGroupsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Clone<Impl: IEnumBackgroundCopyGroupsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IEnumBackgroundCopyGroupsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pucount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumBackgroundCopyGroups>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>, GetCount::<Impl, OFFSET>)
    }
}
pub trait IEnumBackgroundCopyJobsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
impl ::windows::core::RuntimeName for IEnumBackgroundCopyJobs {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IEnumBackgroundCopyJobs";
}
impl IEnumBackgroundCopyJobsVtbl {
    pub const fn new<Impl: IEnumBackgroundCopyJobsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumBackgroundCopyJobsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumBackgroundCopyJobsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumBackgroundCopyJobsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumBackgroundCopyJobsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Clone<Impl: IEnumBackgroundCopyJobsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IEnumBackgroundCopyJobsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pucount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumBackgroundCopyJobs>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>, GetCount::<Impl, OFFSET>)
    }
}
pub trait IEnumBackgroundCopyJobs1Impl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
impl ::windows::core::RuntimeName for IEnumBackgroundCopyJobs1 {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IEnumBackgroundCopyJobs1";
}
impl IEnumBackgroundCopyJobs1Vtbl {
    pub const fn new<Impl: IEnumBackgroundCopyJobs1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumBackgroundCopyJobs1Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumBackgroundCopyJobs1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumBackgroundCopyJobs1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumBackgroundCopyJobs1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Clone<Impl: IEnumBackgroundCopyJobs1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IEnumBackgroundCopyJobs1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pucount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumBackgroundCopyJobs1>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>, GetCount::<Impl, OFFSET>)
    }
}
pub trait IEnumBitsPeerCacheRecordsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
impl ::windows::core::RuntimeName for IEnumBitsPeerCacheRecords {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IEnumBitsPeerCacheRecords";
}
impl IEnumBitsPeerCacheRecordsVtbl {
    pub const fn new<Impl: IEnumBitsPeerCacheRecordsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumBitsPeerCacheRecordsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumBitsPeerCacheRecordsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumBitsPeerCacheRecordsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumBitsPeerCacheRecordsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Clone<Impl: IEnumBitsPeerCacheRecordsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IEnumBitsPeerCacheRecordsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pucount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumBitsPeerCacheRecords>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>, GetCount::<Impl, OFFSET>)
    }
}
pub trait IEnumBitsPeersImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
impl ::windows::core::RuntimeName for IEnumBitsPeers {
    const NAME: &'static str = "Windows.Win32.Networking.BackgroundIntelligentTransferService.IEnumBitsPeers";
}
impl IEnumBitsPeersVtbl {
    pub const fn new<Impl: IEnumBitsPeersImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumBitsPeersVtbl {
        unsafe extern "system" fn Next<Impl: IEnumBitsPeersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumBitsPeersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumBitsPeersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Clone<Impl: IEnumBitsPeersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IEnumBitsPeersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pucount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumBitsPeers>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>, GetCount::<Impl, OFFSET>)
    }
}
