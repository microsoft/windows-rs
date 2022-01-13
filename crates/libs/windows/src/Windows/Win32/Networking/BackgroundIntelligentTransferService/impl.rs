pub trait AsyncIBackgroundCopyCallbackImpl: Sized {
    fn Begin_JobTransferred(&mut self, pjob: ::core::option::Option<IBackgroundCopyJob>) -> ::windows::core::Result<()>;
    fn Finish_JobTransferred(&mut self) -> ::windows::core::Result<()>;
    fn Begin_JobError(&mut self, pjob: ::core::option::Option<IBackgroundCopyJob>, perror: ::core::option::Option<IBackgroundCopyError>) -> ::windows::core::Result<()>;
    fn Finish_JobError(&mut self) -> ::windows::core::Result<()>;
    fn Begin_JobModification(&mut self, pjob: ::core::option::Option<IBackgroundCopyJob>, dwreserved: u32) -> ::windows::core::Result<()>;
    fn Finish_JobModification(&mut self) -> ::windows::core::Result<()>;
}
impl AsyncIBackgroundCopyCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIBackgroundCopyCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIBackgroundCopyCallbackVtbl {
        unsafe extern "system" fn Begin_JobTransferred<Impl: AsyncIBackgroundCopyCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_JobTransferred(::core::mem::transmute(&pjob)).into()
        }
        unsafe extern "system" fn Finish_JobTransferred<Impl: AsyncIBackgroundCopyCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_JobTransferred().into()
        }
        unsafe extern "system" fn Begin_JobError<Impl: AsyncIBackgroundCopyCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr, perror: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_JobError(::core::mem::transmute(&pjob), ::core::mem::transmute(&perror)).into()
        }
        unsafe extern "system" fn Finish_JobError<Impl: AsyncIBackgroundCopyCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_JobError().into()
        }
        unsafe extern "system" fn Begin_JobModification<Impl: AsyncIBackgroundCopyCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_JobModification(::core::mem::transmute(&pjob), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn Finish_JobModification<Impl: AsyncIBackgroundCopyCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_JobModification().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Begin_JobTransferred: Begin_JobTransferred::<Impl, IMPL_OFFSET>,
            Finish_JobTransferred: Finish_JobTransferred::<Impl, IMPL_OFFSET>,
            Begin_JobError: Begin_JobError::<Impl, IMPL_OFFSET>,
            Finish_JobError: Finish_JobError::<Impl, IMPL_OFFSET>,
            Begin_JobModification: Begin_JobModification::<Impl, IMPL_OFFSET>,
            Finish_JobModification: Finish_JobModification::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIBackgroundCopyCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBITSExtensionSetupImpl: Sized + IDispatchImpl {
    fn EnableBITSUploads(&mut self) -> ::windows::core::Result<()>;
    fn DisableBITSUploads(&mut self) -> ::windows::core::Result<()>;
    fn GetCleanupTaskName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetCleanupTask(&mut self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBITSExtensionSetupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBITSExtensionSetupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBITSExtensionSetupVtbl {
        unsafe extern "system" fn EnableBITSUploads<Impl: IBITSExtensionSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableBITSUploads().into()
        }
        unsafe extern "system" fn DisableBITSUploads<Impl: IBITSExtensionSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableBITSUploads().into()
        }
        unsafe extern "system" fn GetCleanupTaskName<Impl: IBITSExtensionSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptaskname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCleanupTaskName() {
                ::core::result::Result::Ok(ok__) => {
                    *ptaskname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCleanupTask<Impl: IBITSExtensionSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCleanupTask(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EnableBITSUploads: EnableBITSUploads::<Impl, IMPL_OFFSET>,
            DisableBITSUploads: DisableBITSUploads::<Impl, IMPL_OFFSET>,
            GetCleanupTaskName: GetCleanupTaskName::<Impl, IMPL_OFFSET>,
            GetCleanupTask: GetCleanupTask::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBITSExtensionSetup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBITSExtensionSetupFactoryImpl: Sized + IDispatchImpl {
    fn GetObject(&mut self, path: super::super::Foundation::BSTR) -> ::windows::core::Result<IBITSExtensionSetup>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBITSExtensionSetupFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBITSExtensionSetupFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBITSExtensionSetupFactoryVtbl {
        unsafe extern "system" fn GetObject<Impl: IBITSExtensionSetupFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppextensionsetup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObject(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppextensionsetup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetObject: GetObject::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBITSExtensionSetupFactory as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundCopyCallbackImpl: Sized {
    fn JobTransferred(&mut self, pjob: ::core::option::Option<IBackgroundCopyJob>) -> ::windows::core::Result<()>;
    fn JobError(&mut self, pjob: ::core::option::Option<IBackgroundCopyJob>, perror: ::core::option::Option<IBackgroundCopyError>) -> ::windows::core::Result<()>;
    fn JobModification(&mut self, pjob: ::core::option::Option<IBackgroundCopyJob>, dwreserved: u32) -> ::windows::core::Result<()>;
}
impl IBackgroundCopyCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyCallbackVtbl {
        unsafe extern "system" fn JobTransferred<Impl: IBackgroundCopyCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).JobTransferred(::core::mem::transmute(&pjob)).into()
        }
        unsafe extern "system" fn JobError<Impl: IBackgroundCopyCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr, perror: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).JobError(::core::mem::transmute(&pjob), ::core::mem::transmute(&perror)).into()
        }
        unsafe extern "system" fn JobModification<Impl: IBackgroundCopyCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).JobModification(::core::mem::transmute(&pjob), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            JobTransferred: JobTransferred::<Impl, IMPL_OFFSET>,
            JobError: JobError::<Impl, IMPL_OFFSET>,
            JobModification: JobModification::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyCallback as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundCopyCallback1Impl: Sized {
    fn OnStatus(&mut self, pgroup: ::core::option::Option<IBackgroundCopyGroup>, pjob: ::core::option::Option<IBackgroundCopyJob1>, dwfileindex: u32, dwstatus: u32, dwnumofretries: u32, dwwin32result: u32, dwtransportresult: u32) -> ::windows::core::Result<()>;
    fn OnProgress(&mut self, progresstype: u32, pgroup: ::core::option::Option<IBackgroundCopyGroup>, pjob: ::core::option::Option<IBackgroundCopyJob1>, dwfileindex: u32, dwprogressvalue: u32) -> ::windows::core::Result<()>;
    fn OnProgressEx(&mut self, progresstype: u32, pgroup: ::core::option::Option<IBackgroundCopyGroup>, pjob: ::core::option::Option<IBackgroundCopyJob1>, dwfileindex: u32, dwprogressvalue: u32, dwbytearraysize: u32, pbyte: *const u8) -> ::windows::core::Result<()>;
}
impl IBackgroundCopyCallback1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyCallback1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyCallback1Vtbl {
        unsafe extern "system" fn OnStatus<Impl: IBackgroundCopyCallback1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroup: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, dwfileindex: u32, dwstatus: u32, dwnumofretries: u32, dwwin32result: u32, dwtransportresult: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStatus(::core::mem::transmute(&pgroup), ::core::mem::transmute(&pjob), ::core::mem::transmute_copy(&dwfileindex), ::core::mem::transmute_copy(&dwstatus), ::core::mem::transmute_copy(&dwnumofretries), ::core::mem::transmute_copy(&dwwin32result), ::core::mem::transmute_copy(&dwtransportresult)).into()
        }
        unsafe extern "system" fn OnProgress<Impl: IBackgroundCopyCallback1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progresstype: u32, pgroup: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, dwfileindex: u32, dwprogressvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnProgress(::core::mem::transmute_copy(&progresstype), ::core::mem::transmute(&pgroup), ::core::mem::transmute(&pjob), ::core::mem::transmute_copy(&dwfileindex), ::core::mem::transmute_copy(&dwprogressvalue)).into()
        }
        unsafe extern "system" fn OnProgressEx<Impl: IBackgroundCopyCallback1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progresstype: u32, pgroup: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, dwfileindex: u32, dwprogressvalue: u32, dwbytearraysize: u32, pbyte: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnProgressEx(::core::mem::transmute_copy(&progresstype), ::core::mem::transmute(&pgroup), ::core::mem::transmute(&pjob), ::core::mem::transmute_copy(&dwfileindex), ::core::mem::transmute_copy(&dwprogressvalue), ::core::mem::transmute_copy(&dwbytearraysize), ::core::mem::transmute_copy(&pbyte)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnStatus: OnStatus::<Impl, IMPL_OFFSET>,
            OnProgress: OnProgress::<Impl, IMPL_OFFSET>,
            OnProgressEx: OnProgressEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyCallback1 as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundCopyCallback2Impl: Sized + IBackgroundCopyCallbackImpl {
    fn FileTransferred(&mut self, pjob: ::core::option::Option<IBackgroundCopyJob>, pfile: ::core::option::Option<IBackgroundCopyFile>) -> ::windows::core::Result<()>;
}
impl IBackgroundCopyCallback2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyCallback2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyCallback2Vtbl {
        unsafe extern "system" fn FileTransferred<Impl: IBackgroundCopyCallback2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr, pfile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FileTransferred(::core::mem::transmute(&pjob), ::core::mem::transmute(&pfile)).into()
        }
        Self { base: IBackgroundCopyCallbackVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), FileTransferred: FileTransferred::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyCallback2 as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundCopyCallback3Impl: Sized + IBackgroundCopyCallbackImpl + IBackgroundCopyCallback2Impl {
    fn FileRangesTransferred(&mut self, job: ::core::option::Option<IBackgroundCopyJob>, file: ::core::option::Option<IBackgroundCopyFile>, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::Result<()>;
}
impl IBackgroundCopyCallback3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyCallback3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyCallback3Vtbl {
        unsafe extern "system" fn FileRangesTransferred<Impl: IBackgroundCopyCallback3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, job: ::windows::core::RawPtr, file: ::windows::core::RawPtr, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FileRangesTransferred(::core::mem::transmute(&job), ::core::mem::transmute(&file), ::core::mem::transmute_copy(&rangecount), ::core::mem::transmute_copy(&ranges)).into()
        }
        Self {
            base: IBackgroundCopyCallback2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            FileRangesTransferred: FileRangesTransferred::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyCallback3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyErrorImpl: Sized {
    fn GetError(&mut self, pcontext: *mut BG_ERROR_CONTEXT, pcode: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn GetFile(&mut self) -> ::windows::core::Result<IBackgroundCopyFile>;
    fn GetErrorDescription(&mut self, languageid: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetErrorContextDescription(&mut self, languageid: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetProtocol(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyErrorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyErrorVtbl {
        unsafe extern "system" fn GetError<Impl: IBackgroundCopyErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut BG_ERROR_CONTEXT, pcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetError(::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&pcode)).into()
        }
        unsafe extern "system" fn GetFile<Impl: IBackgroundCopyErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFile() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorDescription<Impl: IBackgroundCopyErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageid: u32, perrordescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorDescription(::core::mem::transmute_copy(&languageid)) {
                ::core::result::Result::Ok(ok__) => {
                    *perrordescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorContextDescription<Impl: IBackgroundCopyErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageid: u32, pcontextdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorContextDescription(::core::mem::transmute_copy(&languageid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcontextdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProtocol<Impl: IBackgroundCopyErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocol: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProtocol() {
                ::core::result::Result::Ok(ok__) => {
                    *pprotocol = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetError: GetError::<Impl, IMPL_OFFSET>,
            GetFile: GetFile::<Impl, IMPL_OFFSET>,
            GetErrorDescription: GetErrorDescription::<Impl, IMPL_OFFSET>,
            GetErrorContextDescription: GetErrorContextDescription::<Impl, IMPL_OFFSET>,
            GetProtocol: GetProtocol::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyError as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFileImpl: Sized {
    fn GetRemoteName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetLocalName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetProgress(&mut self) -> ::windows::core::Result<BG_FILE_PROGRESS>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyFileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyFileVtbl {
        unsafe extern "system" fn GetRemoteName<Impl: IBackgroundCopyFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRemoteName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalName<Impl: IBackgroundCopyFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProgress<Impl: IBackgroundCopyFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut BG_FILE_PROGRESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetRemoteName: GetRemoteName::<Impl, IMPL_OFFSET>,
            GetLocalName: GetLocalName::<Impl, IMPL_OFFSET>,
            GetProgress: GetProgress::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyFile as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFile2Impl: Sized + IBackgroundCopyFileImpl {
    fn GetFileRanges(&mut self, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::Result<()>;
    fn SetRemoteName(&mut self, val: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyFile2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyFile2Vtbl {
        unsafe extern "system" fn GetFileRanges<Impl: IBackgroundCopyFile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFileRanges(::core::mem::transmute_copy(&rangecount), ::core::mem::transmute_copy(&ranges)).into()
        }
        unsafe extern "system" fn SetRemoteName<Impl: IBackgroundCopyFile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteName(::core::mem::transmute_copy(&val)).into()
        }
        Self {
            base: IBackgroundCopyFileVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetFileRanges: GetFileRanges::<Impl, IMPL_OFFSET>,
            SetRemoteName: SetRemoteName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyFile2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFile3Impl: Sized + IBackgroundCopyFileImpl + IBackgroundCopyFile2Impl {
    fn GetTemporaryName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetValidationState(&mut self, state: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetValidationState(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsDownloadedFromPeer(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyFile3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyFile3Vtbl {
        unsafe extern "system" fn GetTemporaryName<Impl: IBackgroundCopyFile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTemporaryName() {
                ::core::result::Result::Ok(ok__) => {
                    *pfilename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValidationState<Impl: IBackgroundCopyFile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValidationState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn GetValidationState<Impl: IBackgroundCopyFile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValidationState() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDownloadedFromPeer<Impl: IBackgroundCopyFile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDownloadedFromPeer() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IBackgroundCopyFile2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetTemporaryName: GetTemporaryName::<Impl, IMPL_OFFSET>,
            SetValidationState: SetValidationState::<Impl, IMPL_OFFSET>,
            GetValidationState: GetValidationState::<Impl, IMPL_OFFSET>,
            IsDownloadedFromPeer: IsDownloadedFromPeer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyFile3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFile4Impl: Sized + IBackgroundCopyFileImpl + IBackgroundCopyFile2Impl + IBackgroundCopyFile3Impl {
    fn GetPeerDownloadStats(&mut self, pfromorigin: *mut u64, pfrompeers: *mut u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyFile4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyFile4Vtbl {
        unsafe extern "system" fn GetPeerDownloadStats<Impl: IBackgroundCopyFile4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfromorigin: *mut u64, pfrompeers: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPeerDownloadStats(::core::mem::transmute_copy(&pfromorigin), ::core::mem::transmute_copy(&pfrompeers)).into()
        }
        Self {
            base: IBackgroundCopyFile3Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetPeerDownloadStats: GetPeerDownloadStats::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyFile4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFile5Impl: Sized + IBackgroundCopyFileImpl + IBackgroundCopyFile2Impl + IBackgroundCopyFile3Impl + IBackgroundCopyFile4Impl {
    fn SetProperty(&mut self, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: BITS_FILE_PROPERTY_VALUE) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, propertyid: BITS_FILE_PROPERTY_ID) -> ::windows::core::Result<BITS_FILE_PROPERTY_VALUE>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyFile5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyFile5Vtbl {
        unsafe extern "system" fn SetProperty<Impl: IBackgroundCopyFile5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: BITS_FILE_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: IBackgroundCopyFile5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: *mut BITS_FILE_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IBackgroundCopyFile4Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyFile5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFile6Impl: Sized + IBackgroundCopyFileImpl + IBackgroundCopyFile2Impl + IBackgroundCopyFile3Impl + IBackgroundCopyFile4Impl + IBackgroundCopyFile5Impl {
    fn UpdateDownloadPosition(&mut self, offset: u64) -> ::windows::core::Result<()>;
    fn RequestFileRanges(&mut self, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::Result<()>;
    fn GetFilledFileRanges(&mut self, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyFile6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyFile6Vtbl {
        unsafe extern "system" fn UpdateDownloadPosition<Impl: IBackgroundCopyFile6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateDownloadPosition(::core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn RequestFileRanges<Impl: IBackgroundCopyFile6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestFileRanges(::core::mem::transmute_copy(&rangecount), ::core::mem::transmute_copy(&ranges)).into()
        }
        unsafe extern "system" fn GetFilledFileRanges<Impl: IBackgroundCopyFile6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFilledFileRanges(::core::mem::transmute_copy(&rangecount), ::core::mem::transmute_copy(&ranges)).into()
        }
        Self {
            base: IBackgroundCopyFile5Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            UpdateDownloadPosition: UpdateDownloadPosition::<Impl, IMPL_OFFSET>,
            RequestFileRanges: RequestFileRanges::<Impl, IMPL_OFFSET>,
            GetFilledFileRanges: GetFilledFileRanges::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyFile6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBackgroundCopyGroupImpl: Sized {
    fn GetProp(&mut self, propid: GROUPPROP) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProp(&mut self, propid: GROUPPROP, pvarval: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetProgress(&mut self, dwflags: u32) -> ::windows::core::Result<u32>;
    fn GetStatus(&mut self, pdwstatus: *mut u32, pdwjobindex: *mut u32) -> ::windows::core::Result<()>;
    fn GetJob(&mut self, jobid: ::windows::core::GUID) -> ::windows::core::Result<IBackgroundCopyJob1>;
    fn SuspendGroup(&mut self) -> ::windows::core::Result<()>;
    fn ResumeGroup(&mut self) -> ::windows::core::Result<()>;
    fn CancelGroup(&mut self) -> ::windows::core::Result<()>;
    fn Size(&mut self) -> ::windows::core::Result<u32>;
    fn GroupID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CreateJob(&mut self, guidjobid: ::windows::core::GUID) -> ::windows::core::Result<IBackgroundCopyJob1>;
    fn EnumJobs(&mut self, dwflags: u32) -> ::windows::core::Result<IEnumBackgroundCopyJobs1>;
    fn SwitchToForeground(&mut self) -> ::windows::core::Result<()>;
    fn QueryNewJobInterface(&mut self, iid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn SetNotificationPointer(&mut self, iid: *const ::windows::core::GUID, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBackgroundCopyGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyGroupVtbl {
        unsafe extern "system" fn GetProp<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: GROUPPROP, pvarval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProp(::core::mem::transmute_copy(&propid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProp<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: GROUPPROP, pvarval: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProp(::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn GetProgress<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdwprogress: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProgress(::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwprogress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32, pdwjobindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStatus(::core::mem::transmute_copy(&pdwstatus), ::core::mem::transmute_copy(&pdwjobindex)).into()
        }
        unsafe extern "system" fn GetJob<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobid: ::windows::core::GUID, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJob(::core::mem::transmute_copy(&jobid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppjob = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuspendGroup<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SuspendGroup().into()
        }
        unsafe extern "system" fn ResumeGroup<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResumeGroup().into()
        }
        unsafe extern "system" fn CancelGroup<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelGroup().into()
        }
        unsafe extern "system" fn Size<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GroupID<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidgroupid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GroupID() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidgroupid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateJob<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidjobid: ::windows::core::GUID, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateJob(::core::mem::transmute_copy(&guidjobid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppjob = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumJobs<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppenumjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumJobs(::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumjobs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SwitchToForeground<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SwitchToForeground().into()
        }
        unsafe extern "system" fn QueryNewJobInterface<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryNewJobInterface(::core::mem::transmute_copy(&iid)) {
                ::core::result::Result::Ok(ok__) => {
                    *punk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationPointer<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotificationPointer(::core::mem::transmute_copy(&iid), ::core::mem::transmute(&punk)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetProp: GetProp::<Impl, IMPL_OFFSET>,
            SetProp: SetProp::<Impl, IMPL_OFFSET>,
            GetProgress: GetProgress::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            GetJob: GetJob::<Impl, IMPL_OFFSET>,
            SuspendGroup: SuspendGroup::<Impl, IMPL_OFFSET>,
            ResumeGroup: ResumeGroup::<Impl, IMPL_OFFSET>,
            CancelGroup: CancelGroup::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
            GroupID: GroupID::<Impl, IMPL_OFFSET>,
            CreateJob: CreateJob::<Impl, IMPL_OFFSET>,
            EnumJobs: EnumJobs::<Impl, IMPL_OFFSET>,
            SwitchToForeground: SwitchToForeground::<Impl, IMPL_OFFSET>,
            QueryNewJobInterface: QueryNewJobInterface::<Impl, IMPL_OFFSET>,
            SetNotificationPointer: SetNotificationPointer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJobImpl: Sized {
    fn AddFileSet(&mut self, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> ::windows::core::Result<()>;
    fn AddFile(&mut self, remoteurl: super::super::Foundation::PWSTR, localname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn EnumFiles(&mut self) -> ::windows::core::Result<IEnumBackgroundCopyFiles>;
    fn Suspend(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
    fn Complete(&mut self) -> ::windows::core::Result<()>;
    fn GetId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetType(&mut self) -> ::windows::core::Result<BG_JOB_TYPE>;
    fn GetProgress(&mut self) -> ::windows::core::Result<BG_JOB_PROGRESS>;
    fn GetTimes(&mut self) -> ::windows::core::Result<BG_JOB_TIMES>;
    fn GetState(&mut self) -> ::windows::core::Result<BG_JOB_STATE>;
    fn GetError(&mut self) -> ::windows::core::Result<IBackgroundCopyError>;
    fn GetOwner(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetDisplayName(&mut self, val: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetDisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetDescription(&mut self, val: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetPriority(&mut self, val: BG_JOB_PRIORITY) -> ::windows::core::Result<()>;
    fn GetPriority(&mut self) -> ::windows::core::Result<BG_JOB_PRIORITY>;
    fn SetNotifyFlags(&mut self, val: u32) -> ::windows::core::Result<()>;
    fn GetNotifyFlags(&mut self) -> ::windows::core::Result<u32>;
    fn SetNotifyInterface(&mut self, val: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetNotifyInterface(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn SetMinimumRetryDelay(&mut self, seconds: u32) -> ::windows::core::Result<()>;
    fn GetMinimumRetryDelay(&mut self) -> ::windows::core::Result<u32>;
    fn SetNoProgressTimeout(&mut self, seconds: u32) -> ::windows::core::Result<()>;
    fn GetNoProgressTimeout(&mut self) -> ::windows::core::Result<u32>;
    fn GetErrorCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetProxySettings(&mut self, proxyusage: BG_JOB_PROXY_USAGE, proxylist: super::super::Foundation::PWSTR, proxybypasslist: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetProxySettings(&mut self, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut super::super::Foundation::PWSTR, pproxybypasslist: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn TakeOwnership(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyJobVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJobImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyJobVtbl {
        unsafe extern "system" fn AddFileSet<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddFileSet(::core::mem::transmute_copy(&cfilecount), ::core::mem::transmute_copy(&pfileset)).into()
        }
        unsafe extern "system" fn AddFile<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteurl: super::super::Foundation::PWSTR, localname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddFile(::core::mem::transmute_copy(&remoteurl), ::core::mem::transmute_copy(&localname)).into()
        }
        unsafe extern "system" fn EnumFiles<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumFiles() {
                ::core::result::Result::Ok(ok__) => {
                    *penum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Suspend<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Suspend().into()
        }
        unsafe extern "system" fn Resume<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Cancel<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn Complete<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        unsafe extern "system" fn GetId<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetId() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProgress<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_PROGRESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimes<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_TIMES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTimes() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetState<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetState() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetError<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetError() {
                ::core::result::Result::Ok(ok__) => {
                    *pperror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOwner<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn GetDisplayName<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn GetDescription<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: BG_JOB_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn GetPriority<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPriority() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotifyFlags<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotifyFlags(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn GetNotifyFlags<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNotifyFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotifyInterface<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotifyInterface(::core::mem::transmute(&val)).into()
        }
        unsafe extern "system" fn GetNotifyInterface<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNotifyInterface() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumRetryDelay<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinimumRetryDelay(::core::mem::transmute_copy(&seconds)).into()
        }
        unsafe extern "system" fn GetMinimumRetryDelay<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMinimumRetryDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *seconds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoProgressTimeout<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNoProgressTimeout(::core::mem::transmute_copy(&seconds)).into()
        }
        unsafe extern "system" fn GetNoProgressTimeout<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNoProgressTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *seconds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorCount<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errors: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorCount() {
                ::core::result::Result::Ok(ok__) => {
                    *errors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxySettings<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, proxyusage: BG_JOB_PROXY_USAGE, proxylist: super::super::Foundation::PWSTR, proxybypasslist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProxySettings(::core::mem::transmute_copy(&proxyusage), ::core::mem::transmute_copy(&proxylist), ::core::mem::transmute_copy(&proxybypasslist)).into()
        }
        unsafe extern "system" fn GetProxySettings<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut super::super::Foundation::PWSTR, pproxybypasslist: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProxySettings(::core::mem::transmute_copy(&pproxyusage), ::core::mem::transmute_copy(&pproxylist), ::core::mem::transmute_copy(&pproxybypasslist)).into()
        }
        unsafe extern "system" fn TakeOwnership<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TakeOwnership().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddFileSet: AddFileSet::<Impl, IMPL_OFFSET>,
            AddFile: AddFile::<Impl, IMPL_OFFSET>,
            EnumFiles: EnumFiles::<Impl, IMPL_OFFSET>,
            Suspend: Suspend::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            Complete: Complete::<Impl, IMPL_OFFSET>,
            GetId: GetId::<Impl, IMPL_OFFSET>,
            GetType: GetType::<Impl, IMPL_OFFSET>,
            GetProgress: GetProgress::<Impl, IMPL_OFFSET>,
            GetTimes: GetTimes::<Impl, IMPL_OFFSET>,
            GetState: GetState::<Impl, IMPL_OFFSET>,
            GetError: GetError::<Impl, IMPL_OFFSET>,
            GetOwner: GetOwner::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            GetDisplayName: GetDisplayName::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            GetDescription: GetDescription::<Impl, IMPL_OFFSET>,
            SetPriority: SetPriority::<Impl, IMPL_OFFSET>,
            GetPriority: GetPriority::<Impl, IMPL_OFFSET>,
            SetNotifyFlags: SetNotifyFlags::<Impl, IMPL_OFFSET>,
            GetNotifyFlags: GetNotifyFlags::<Impl, IMPL_OFFSET>,
            SetNotifyInterface: SetNotifyInterface::<Impl, IMPL_OFFSET>,
            GetNotifyInterface: GetNotifyInterface::<Impl, IMPL_OFFSET>,
            SetMinimumRetryDelay: SetMinimumRetryDelay::<Impl, IMPL_OFFSET>,
            GetMinimumRetryDelay: GetMinimumRetryDelay::<Impl, IMPL_OFFSET>,
            SetNoProgressTimeout: SetNoProgressTimeout::<Impl, IMPL_OFFSET>,
            GetNoProgressTimeout: GetNoProgressTimeout::<Impl, IMPL_OFFSET>,
            GetErrorCount: GetErrorCount::<Impl, IMPL_OFFSET>,
            SetProxySettings: SetProxySettings::<Impl, IMPL_OFFSET>,
            GetProxySettings: GetProxySettings::<Impl, IMPL_OFFSET>,
            TakeOwnership: TakeOwnership::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJob as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJob1Impl: Sized {
    fn CancelJob(&mut self) -> ::windows::core::Result<()>;
    fn GetProgress(&mut self, dwflags: u32) -> ::windows::core::Result<u32>;
    fn GetStatus(&mut self, pdwstatus: *mut u32, pdwwin32result: *mut u32, pdwtransportresult: *mut u32, pdwnumofretries: *mut u32) -> ::windows::core::Result<()>;
    fn AddFiles(&mut self, cfilecount: u32, ppfileset: *const *const FILESETINFO) -> ::windows::core::Result<()>;
    fn GetFile(&mut self, cfileindex: u32) -> ::windows::core::Result<FILESETINFO>;
    fn GetFileCount(&mut self) -> ::windows::core::Result<u32>;
    fn SwitchToForeground(&mut self) -> ::windows::core::Result<()>;
    fn JobID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyJob1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyJob1Vtbl {
        unsafe extern "system" fn CancelJob<Impl: IBackgroundCopyJob1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelJob().into()
        }
        unsafe extern "system" fn GetProgress<Impl: IBackgroundCopyJob1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdwprogress: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProgress(::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwprogress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IBackgroundCopyJob1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32, pdwwin32result: *mut u32, pdwtransportresult: *mut u32, pdwnumofretries: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStatus(::core::mem::transmute_copy(&pdwstatus), ::core::mem::transmute_copy(&pdwwin32result), ::core::mem::transmute_copy(&pdwtransportresult), ::core::mem::transmute_copy(&pdwnumofretries)).into()
        }
        unsafe extern "system" fn AddFiles<Impl: IBackgroundCopyJob1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfilecount: u32, ppfileset: *const *const FILESETINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddFiles(::core::mem::transmute_copy(&cfilecount), ::core::mem::transmute_copy(&ppfileset)).into()
        }
        unsafe extern "system" fn GetFile<Impl: IBackgroundCopyJob1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfileindex: u32, pfileinfo: *mut FILESETINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFile(::core::mem::transmute_copy(&cfileindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfileinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileCount<Impl: IBackgroundCopyJob1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfilecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwfilecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SwitchToForeground<Impl: IBackgroundCopyJob1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SwitchToForeground().into()
        }
        unsafe extern "system" fn JobID<Impl: IBackgroundCopyJob1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidjobid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JobID() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidjobid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CancelJob: CancelJob::<Impl, IMPL_OFFSET>,
            GetProgress: GetProgress::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            AddFiles: AddFiles::<Impl, IMPL_OFFSET>,
            GetFile: GetFile::<Impl, IMPL_OFFSET>,
            GetFileCount: GetFileCount::<Impl, IMPL_OFFSET>,
            SwitchToForeground: SwitchToForeground::<Impl, IMPL_OFFSET>,
            JobID: JobID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJob1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJob2Impl: Sized + IBackgroundCopyJobImpl {
    fn SetNotifyCmdLine(&mut self, program: super::super::Foundation::PWSTR, parameters: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetNotifyCmdLine(&mut self, pprogram: *mut super::super::Foundation::PWSTR, pparameters: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetReplyProgress(&mut self, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> ::windows::core::Result<()>;
    fn GetReplyData(&mut self, ppbuffer: *mut *mut u8, plength: *mut u64) -> ::windows::core::Result<()>;
    fn SetReplyFileName(&mut self, replyfilename: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetReplyFileName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetCredentials(&mut self, credentials: *const BG_AUTH_CREDENTIALS) -> ::windows::core::Result<()>;
    fn RemoveCredentials(&mut self, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyJob2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyJob2Vtbl {
        unsafe extern "system" fn SetNotifyCmdLine<Impl: IBackgroundCopyJob2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, program: super::super::Foundation::PWSTR, parameters: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotifyCmdLine(::core::mem::transmute_copy(&program), ::core::mem::transmute_copy(&parameters)).into()
        }
        unsafe extern "system" fn GetNotifyCmdLine<Impl: IBackgroundCopyJob2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprogram: *mut super::super::Foundation::PWSTR, pparameters: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNotifyCmdLine(::core::mem::transmute_copy(&pprogram), ::core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn GetReplyProgress<Impl: IBackgroundCopyJob2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetReplyProgress(::core::mem::transmute_copy(&pprogress)).into()
        }
        unsafe extern "system" fn GetReplyData<Impl: IBackgroundCopyJob2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbuffer: *mut *mut u8, plength: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetReplyData(::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&plength)).into()
        }
        unsafe extern "system" fn SetReplyFileName<Impl: IBackgroundCopyJob2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, replyfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReplyFileName(::core::mem::transmute_copy(&replyfilename)).into()
        }
        unsafe extern "system" fn GetReplyFileName<Impl: IBackgroundCopyJob2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preplyfilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReplyFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *preplyfilename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Impl: IBackgroundCopyJob2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credentials: *const BG_AUTH_CREDENTIALS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCredentials(::core::mem::transmute_copy(&credentials)).into()
        }
        unsafe extern "system" fn RemoveCredentials<Impl: IBackgroundCopyJob2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCredentials(::core::mem::transmute_copy(&target), ::core::mem::transmute_copy(&scheme)).into()
        }
        Self {
            base: IBackgroundCopyJobVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetNotifyCmdLine: SetNotifyCmdLine::<Impl, IMPL_OFFSET>,
            GetNotifyCmdLine: GetNotifyCmdLine::<Impl, IMPL_OFFSET>,
            GetReplyProgress: GetReplyProgress::<Impl, IMPL_OFFSET>,
            GetReplyData: GetReplyData::<Impl, IMPL_OFFSET>,
            SetReplyFileName: SetReplyFileName::<Impl, IMPL_OFFSET>,
            GetReplyFileName: GetReplyFileName::<Impl, IMPL_OFFSET>,
            SetCredentials: SetCredentials::<Impl, IMPL_OFFSET>,
            RemoveCredentials: RemoveCredentials::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJob2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJob3Impl: Sized + IBackgroundCopyJobImpl + IBackgroundCopyJob2Impl {
    fn ReplaceRemotePrefix(&mut self, oldprefix: super::super::Foundation::PWSTR, newprefix: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn AddFileWithRanges(&mut self, remoteurl: super::super::Foundation::PWSTR, localname: super::super::Foundation::PWSTR, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::Result<()>;
    fn SetFileACLFlags(&mut self, flags: u32) -> ::windows::core::Result<()>;
    fn GetFileACLFlags(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyJob3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyJob3Vtbl {
        unsafe extern "system" fn ReplaceRemotePrefix<Impl: IBackgroundCopyJob3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldprefix: super::super::Foundation::PWSTR, newprefix: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReplaceRemotePrefix(::core::mem::transmute_copy(&oldprefix), ::core::mem::transmute_copy(&newprefix)).into()
        }
        unsafe extern "system" fn AddFileWithRanges<Impl: IBackgroundCopyJob3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteurl: super::super::Foundation::PWSTR, localname: super::super::Foundation::PWSTR, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddFileWithRanges(::core::mem::transmute_copy(&remoteurl), ::core::mem::transmute_copy(&localname), ::core::mem::transmute_copy(&rangecount), ::core::mem::transmute_copy(&ranges)).into()
        }
        unsafe extern "system" fn SetFileACLFlags<Impl: IBackgroundCopyJob3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFileACLFlags(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetFileACLFlags<Impl: IBackgroundCopyJob3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileACLFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IBackgroundCopyJob2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ReplaceRemotePrefix: ReplaceRemotePrefix::<Impl, IMPL_OFFSET>,
            AddFileWithRanges: AddFileWithRanges::<Impl, IMPL_OFFSET>,
            SetFileACLFlags: SetFileACLFlags::<Impl, IMPL_OFFSET>,
            GetFileACLFlags: GetFileACLFlags::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJob3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJob4Impl: Sized + IBackgroundCopyJobImpl + IBackgroundCopyJob2Impl + IBackgroundCopyJob3Impl {
    fn SetPeerCachingFlags(&mut self, flags: u32) -> ::windows::core::Result<()>;
    fn GetPeerCachingFlags(&mut self) -> ::windows::core::Result<u32>;
    fn GetOwnerIntegrityLevel(&mut self) -> ::windows::core::Result<u32>;
    fn GetOwnerElevationState(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetMaximumDownloadTime(&mut self, timeout: u32) -> ::windows::core::Result<()>;
    fn GetMaximumDownloadTime(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyJob4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyJob4Vtbl {
        unsafe extern "system" fn SetPeerCachingFlags<Impl: IBackgroundCopyJob4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPeerCachingFlags(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetPeerCachingFlags<Impl: IBackgroundCopyJob4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPeerCachingFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOwnerIntegrityLevel<Impl: IBackgroundCopyJob4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwnerIntegrityLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOwnerElevationState<Impl: IBackgroundCopyJob4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pelevated: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwnerElevationState() {
                ::core::result::Result::Ok(ok__) => {
                    *pelevated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumDownloadTime<Impl: IBackgroundCopyJob4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaximumDownloadTime(::core::mem::transmute_copy(&timeout)).into()
        }
        unsafe extern "system" fn GetMaximumDownloadTime<Impl: IBackgroundCopyJob4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaximumDownloadTime() {
                ::core::result::Result::Ok(ok__) => {
                    *ptimeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IBackgroundCopyJob3Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetPeerCachingFlags: SetPeerCachingFlags::<Impl, IMPL_OFFSET>,
            GetPeerCachingFlags: GetPeerCachingFlags::<Impl, IMPL_OFFSET>,
            GetOwnerIntegrityLevel: GetOwnerIntegrityLevel::<Impl, IMPL_OFFSET>,
            GetOwnerElevationState: GetOwnerElevationState::<Impl, IMPL_OFFSET>,
            SetMaximumDownloadTime: SetMaximumDownloadTime::<Impl, IMPL_OFFSET>,
            GetMaximumDownloadTime: GetMaximumDownloadTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJob4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJob5Impl: Sized + IBackgroundCopyJobImpl + IBackgroundCopyJob2Impl + IBackgroundCopyJob3Impl + IBackgroundCopyJob4Impl {
    fn SetProperty(&mut self, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: BITS_JOB_PROPERTY_VALUE) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, propertyid: BITS_JOB_PROPERTY_ID) -> ::windows::core::Result<BITS_JOB_PROPERTY_VALUE>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyJob5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyJob5Vtbl {
        unsafe extern "system" fn SetProperty<Impl: IBackgroundCopyJob5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: BITS_JOB_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: IBackgroundCopyJob5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: *mut BITS_JOB_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IBackgroundCopyJob4Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJob5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJobHttpOptionsImpl: Sized {
    fn SetClientCertificateByID(&mut self, storelocation: BG_CERT_STORE_LOCATION, storename: super::super::Foundation::PWSTR, pcerthashblob: *const u8) -> ::windows::core::Result<()>;
    fn SetClientCertificateByName(&mut self, storelocation: BG_CERT_STORE_LOCATION, storename: super::super::Foundation::PWSTR, subjectname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RemoveClientCertificate(&mut self) -> ::windows::core::Result<()>;
    fn GetClientCertificate(&mut self, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut super::super::Foundation::PWSTR, ppcerthashblob: *mut *mut u8, psubjectname: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetCustomHeaders(&mut self, requestheaders: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetCustomHeaders(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetSecurityFlags(&mut self, flags: u32) -> ::windows::core::Result<()>;
    fn GetSecurityFlags(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyJobHttpOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJobHttpOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyJobHttpOptionsVtbl {
        unsafe extern "system" fn SetClientCertificateByID<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storelocation: BG_CERT_STORE_LOCATION, storename: super::super::Foundation::PWSTR, pcerthashblob: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientCertificateByID(::core::mem::transmute_copy(&storelocation), ::core::mem::transmute_copy(&storename), ::core::mem::transmute_copy(&pcerthashblob)).into()
        }
        unsafe extern "system" fn SetClientCertificateByName<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storelocation: BG_CERT_STORE_LOCATION, storename: super::super::Foundation::PWSTR, subjectname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientCertificateByName(::core::mem::transmute_copy(&storelocation), ::core::mem::transmute_copy(&storename), ::core::mem::transmute_copy(&subjectname)).into()
        }
        unsafe extern "system" fn RemoveClientCertificate<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClientCertificate().into()
        }
        unsafe extern "system" fn GetClientCertificate<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut super::super::Foundation::PWSTR, ppcerthashblob: *mut *mut u8, psubjectname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClientCertificate(::core::mem::transmute_copy(&pstorelocation), ::core::mem::transmute_copy(&pstorename), ::core::mem::transmute_copy(&ppcerthashblob), ::core::mem::transmute_copy(&psubjectname)).into()
        }
        unsafe extern "system" fn SetCustomHeaders<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestheaders: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCustomHeaders(::core::mem::transmute_copy(&requestheaders)).into()
        }
        unsafe extern "system" fn GetCustomHeaders<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequestheaders: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustomHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *prequestheaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityFlags<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurityFlags(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetSecurityFlags<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecurityFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetClientCertificateByID: SetClientCertificateByID::<Impl, IMPL_OFFSET>,
            SetClientCertificateByName: SetClientCertificateByName::<Impl, IMPL_OFFSET>,
            RemoveClientCertificate: RemoveClientCertificate::<Impl, IMPL_OFFSET>,
            GetClientCertificate: GetClientCertificate::<Impl, IMPL_OFFSET>,
            SetCustomHeaders: SetCustomHeaders::<Impl, IMPL_OFFSET>,
            GetCustomHeaders: GetCustomHeaders::<Impl, IMPL_OFFSET>,
            SetSecurityFlags: SetSecurityFlags::<Impl, IMPL_OFFSET>,
            GetSecurityFlags: GetSecurityFlags::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJobHttpOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJobHttpOptions2Impl: Sized + IBackgroundCopyJobHttpOptionsImpl {
    fn SetHttpMethod(&mut self, method: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetHttpMethod(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyJobHttpOptions2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJobHttpOptions2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyJobHttpOptions2Vtbl {
        unsafe extern "system" fn SetHttpMethod<Impl: IBackgroundCopyJobHttpOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHttpMethod(::core::mem::transmute_copy(&method)).into()
        }
        unsafe extern "system" fn GetHttpMethod<Impl: IBackgroundCopyJobHttpOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHttpMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *method = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IBackgroundCopyJobHttpOptionsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetHttpMethod: SetHttpMethod::<Impl, IMPL_OFFSET>,
            GetHttpMethod: GetHttpMethod::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJobHttpOptions2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJobHttpOptions3Impl: Sized + IBackgroundCopyJobHttpOptionsImpl + IBackgroundCopyJobHttpOptions2Impl {
    fn SetServerCertificateValidationInterface(&mut self, certvalidationcallback: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn MakeCustomHeadersWriteOnly(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyJobHttpOptions3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJobHttpOptions3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyJobHttpOptions3Vtbl {
        unsafe extern "system" fn SetServerCertificateValidationInterface<Impl: IBackgroundCopyJobHttpOptions3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certvalidationcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServerCertificateValidationInterface(::core::mem::transmute(&certvalidationcallback)).into()
        }
        unsafe extern "system" fn MakeCustomHeadersWriteOnly<Impl: IBackgroundCopyJobHttpOptions3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MakeCustomHeadersWriteOnly().into()
        }
        Self {
            base: IBackgroundCopyJobHttpOptions2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetServerCertificateValidationInterface: SetServerCertificateValidationInterface::<Impl, IMPL_OFFSET>,
            MakeCustomHeadersWriteOnly: MakeCustomHeadersWriteOnly::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJobHttpOptions3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyManagerImpl: Sized {
    fn CreateJob(&mut self, displayname: super::super::Foundation::PWSTR, r#type: BG_JOB_TYPE, pjobid: *mut ::windows::core::GUID, ppjob: *mut ::core::option::Option<IBackgroundCopyJob>) -> ::windows::core::Result<()>;
    fn GetJob(&mut self, jobid: *const ::windows::core::GUID) -> ::windows::core::Result<IBackgroundCopyJob>;
    fn EnumJobs(&mut self, dwflags: u32) -> ::windows::core::Result<IEnumBackgroundCopyJobs>;
    fn GetErrorDescription(&mut self, hresult: ::windows::core::HRESULT, languageid: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyManagerVtbl {
        unsafe extern "system" fn CreateJob<Impl: IBackgroundCopyManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: super::super::Foundation::PWSTR, r#type: BG_JOB_TYPE, pjobid: *mut ::windows::core::GUID, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateJob(::core::mem::transmute_copy(&displayname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pjobid), ::core::mem::transmute_copy(&ppjob)).into()
        }
        unsafe extern "system" fn GetJob<Impl: IBackgroundCopyManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobid: *const ::windows::core::GUID, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJob(::core::mem::transmute_copy(&jobid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppjob = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumJobs<Impl: IBackgroundCopyManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumJobs(::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorDescription<Impl: IBackgroundCopyManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, languageid: u32, perrordescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorDescription(::core::mem::transmute_copy(&hresult), ::core::mem::transmute_copy(&languageid)) {
                ::core::result::Result::Ok(ok__) => {
                    *perrordescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateJob: CreateJob::<Impl, IMPL_OFFSET>,
            GetJob: GetJob::<Impl, IMPL_OFFSET>,
            EnumJobs: EnumJobs::<Impl, IMPL_OFFSET>,
            GetErrorDescription: GetErrorDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyManager as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundCopyQMgrImpl: Sized {
    fn CreateGroup(&mut self, guidgroupid: ::windows::core::GUID) -> ::windows::core::Result<IBackgroundCopyGroup>;
    fn GetGroup(&mut self, groupid: ::windows::core::GUID) -> ::windows::core::Result<IBackgroundCopyGroup>;
    fn EnumGroups(&mut self, dwflags: u32) -> ::windows::core::Result<IEnumBackgroundCopyGroups>;
}
impl IBackgroundCopyQMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyQMgrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyQMgrVtbl {
        unsafe extern "system" fn CreateGroup<Impl: IBackgroundCopyQMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidgroupid: ::windows::core::GUID, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGroup(::core::mem::transmute_copy(&guidgroupid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGroup<Impl: IBackgroundCopyQMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, groupid: ::windows::core::GUID, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGroup(::core::mem::transmute_copy(&groupid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumGroups<Impl: IBackgroundCopyQMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppenumgroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumGroups(::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumgroups = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateGroup: CreateGroup::<Impl, IMPL_OFFSET>,
            GetGroup: GetGroup::<Impl, IMPL_OFFSET>,
            EnumGroups: EnumGroups::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyQMgr as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundCopyServerCertificateValidationCallbackImpl: Sized {
    fn ValidateServerCertificate(&mut self, job: ::core::option::Option<IBackgroundCopyJob>, file: ::core::option::Option<IBackgroundCopyFile>, certlength: u32, certdata: *const u8, certencodingtype: u32, certstorelength: u32, certstoredata: *const u8) -> ::windows::core::Result<()>;
}
impl IBackgroundCopyServerCertificateValidationCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyServerCertificateValidationCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyServerCertificateValidationCallbackVtbl {
        unsafe extern "system" fn ValidateServerCertificate<Impl: IBackgroundCopyServerCertificateValidationCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, job: ::windows::core::RawPtr, file: ::windows::core::RawPtr, certlength: u32, certdata: *const u8, certencodingtype: u32, certstorelength: u32, certstoredata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ValidateServerCertificate(::core::mem::transmute(&job), ::core::mem::transmute(&file), ::core::mem::transmute_copy(&certlength), ::core::mem::transmute_copy(&certdata), ::core::mem::transmute_copy(&certencodingtype), ::core::mem::transmute_copy(&certstorelength), ::core::mem::transmute_copy(&certstoredata)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ValidateServerCertificate: ValidateServerCertificate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyServerCertificateValidationCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBitsPeerImpl: Sized {
    fn GetPeerName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn IsAuthenticated(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsAvailable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBitsPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitsPeerVtbl {
        unsafe extern "system" fn GetPeerName<Impl: IBitsPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPeerName() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAuthenticated<Impl: IBitsPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauth: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAuthenticated() {
                ::core::result::Result::Ok(ok__) => {
                    *pauth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAvailable<Impl: IBitsPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ponline: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *ponline = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPeerName: GetPeerName::<Impl, IMPL_OFFSET>,
            IsAuthenticated: IsAuthenticated::<Impl, IMPL_OFFSET>,
            IsAvailable: IsAvailable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitsPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBitsPeerCacheAdministrationImpl: Sized {
    fn GetMaximumCacheSize(&mut self) -> ::windows::core::Result<u32>;
    fn SetMaximumCacheSize(&mut self, bytes: u32) -> ::windows::core::Result<()>;
    fn GetMaximumContentAge(&mut self) -> ::windows::core::Result<u32>;
    fn SetMaximumContentAge(&mut self, seconds: u32) -> ::windows::core::Result<()>;
    fn GetConfigurationFlags(&mut self) -> ::windows::core::Result<u32>;
    fn SetConfigurationFlags(&mut self, flags: u32) -> ::windows::core::Result<()>;
    fn EnumRecords(&mut self) -> ::windows::core::Result<IEnumBitsPeerCacheRecords>;
    fn GetRecord(&mut self, id: *const ::windows::core::GUID) -> ::windows::core::Result<IBitsPeerCacheRecord>;
    fn ClearRecords(&mut self) -> ::windows::core::Result<()>;
    fn DeleteRecord(&mut self, id: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn DeleteUrl(&mut self, url: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn EnumPeers(&mut self) -> ::windows::core::Result<IEnumBitsPeers>;
    fn ClearPeers(&mut self) -> ::windows::core::Result<()>;
    fn DiscoverPeers(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBitsPeerCacheAdministrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheAdministrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitsPeerCacheAdministrationVtbl {
        unsafe extern "system" fn GetMaximumCacheSize<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbytes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaximumCacheSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pbytes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumCacheSize<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bytes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaximumCacheSize(::core::mem::transmute_copy(&bytes)).into()
        }
        unsafe extern "system" fn GetMaximumContentAge<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pseconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaximumContentAge() {
                ::core::result::Result::Ok(ok__) => {
                    *pseconds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumContentAge<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaximumContentAge(::core::mem::transmute_copy(&seconds)).into()
        }
        unsafe extern "system" fn GetConfigurationFlags<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConfigurationFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConfigurationFlags<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConfigurationFlags(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn EnumRecords<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumRecords() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecord<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *const ::windows::core::GUID, pprecord: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecord(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprecord = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearRecords<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearRecords().into()
        }
        unsafe extern "system" fn DeleteRecord<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteRecord(::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn DeleteUrl<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, url: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteUrl(::core::mem::transmute_copy(&url)).into()
        }
        unsafe extern "system" fn EnumPeers<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumPeers() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearPeers<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearPeers().into()
        }
        unsafe extern "system" fn DiscoverPeers<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DiscoverPeers().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetMaximumCacheSize: GetMaximumCacheSize::<Impl, IMPL_OFFSET>,
            SetMaximumCacheSize: SetMaximumCacheSize::<Impl, IMPL_OFFSET>,
            GetMaximumContentAge: GetMaximumContentAge::<Impl, IMPL_OFFSET>,
            SetMaximumContentAge: SetMaximumContentAge::<Impl, IMPL_OFFSET>,
            GetConfigurationFlags: GetConfigurationFlags::<Impl, IMPL_OFFSET>,
            SetConfigurationFlags: SetConfigurationFlags::<Impl, IMPL_OFFSET>,
            EnumRecords: EnumRecords::<Impl, IMPL_OFFSET>,
            GetRecord: GetRecord::<Impl, IMPL_OFFSET>,
            ClearRecords: ClearRecords::<Impl, IMPL_OFFSET>,
            DeleteRecord: DeleteRecord::<Impl, IMPL_OFFSET>,
            DeleteUrl: DeleteUrl::<Impl, IMPL_OFFSET>,
            EnumPeers: EnumPeers::<Impl, IMPL_OFFSET>,
            ClearPeers: ClearPeers::<Impl, IMPL_OFFSET>,
            DiscoverPeers: DiscoverPeers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitsPeerCacheAdministration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBitsPeerCacheRecordImpl: Sized {
    fn GetId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetOriginUrl(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetFileSize(&mut self) -> ::windows::core::Result<u64>;
    fn GetFileModificationTime(&mut self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn GetLastAccessTime(&mut self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn IsFileValidated(&mut self) -> ::windows::core::Result<()>;
    fn GetFileRanges(&mut self, prangecount: *mut u32, ppranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBitsPeerCacheRecordVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheRecordImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitsPeerCacheRecordVtbl {
        unsafe extern "system" fn GetId<Impl: IBitsPeerCacheRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetId() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginUrl<Impl: IBitsPeerCacheRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOriginUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileSize<Impl: IBitsPeerCacheRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileModificationTime<Impl: IBitsPeerCacheRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileModificationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastAccessTime<Impl: IBitsPeerCacheRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastAccessTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFileValidated<Impl: IBitsPeerCacheRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsFileValidated().into()
        }
        unsafe extern "system" fn GetFileRanges<Impl: IBitsPeerCacheRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangecount: *mut u32, ppranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFileRanges(::core::mem::transmute_copy(&prangecount), ::core::mem::transmute_copy(&ppranges)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetId: GetId::<Impl, IMPL_OFFSET>,
            GetOriginUrl: GetOriginUrl::<Impl, IMPL_OFFSET>,
            GetFileSize: GetFileSize::<Impl, IMPL_OFFSET>,
            GetFileModificationTime: GetFileModificationTime::<Impl, IMPL_OFFSET>,
            GetLastAccessTime: GetLastAccessTime::<Impl, IMPL_OFFSET>,
            IsFileValidated: IsFileValidated::<Impl, IMPL_OFFSET>,
            GetFileRanges: GetFileRanges::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitsPeerCacheRecord as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBitsTokenOptionsImpl: Sized {
    fn SetHelperTokenFlags(&mut self, usageflags: BG_TOKEN) -> ::windows::core::Result<()>;
    fn GetHelperTokenFlags(&mut self) -> ::windows::core::Result<BG_TOKEN>;
    fn SetHelperToken(&mut self) -> ::windows::core::Result<()>;
    fn ClearHelperToken(&mut self) -> ::windows::core::Result<()>;
    fn GetHelperTokenSid(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBitsTokenOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitsTokenOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitsTokenOptionsVtbl {
        unsafe extern "system" fn SetHelperTokenFlags<Impl: IBitsTokenOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usageflags: BG_TOKEN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHelperTokenFlags(::core::mem::transmute_copy(&usageflags)).into()
        }
        unsafe extern "system" fn GetHelperTokenFlags<Impl: IBitsTokenOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut BG_TOKEN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHelperTokenFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHelperToken<Impl: IBitsTokenOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHelperToken().into()
        }
        unsafe extern "system" fn ClearHelperToken<Impl: IBitsTokenOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearHelperToken().into()
        }
        unsafe extern "system" fn GetHelperTokenSid<Impl: IBitsTokenOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHelperTokenSid() {
                ::core::result::Result::Ok(ok__) => {
                    *psid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetHelperTokenFlags: SetHelperTokenFlags::<Impl, IMPL_OFFSET>,
            GetHelperTokenFlags: GetHelperTokenFlags::<Impl, IMPL_OFFSET>,
            SetHelperToken: SetHelperToken::<Impl, IMPL_OFFSET>,
            ClearHelperToken: ClearHelperToken::<Impl, IMPL_OFFSET>,
            GetHelperTokenSid: GetHelperTokenSid::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitsTokenOptions as ::windows::core::Interface>::IID
    }
}
pub trait IEnumBackgroundCopyFilesImpl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<IBackgroundCopyFile>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumBackgroundCopyFiles>;
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
}
impl IEnumBackgroundCopyFilesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyFilesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumBackgroundCopyFilesVtbl {
        unsafe extern "system" fn Next<Impl: IEnumBackgroundCopyFilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumBackgroundCopyFilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumBackgroundCopyFilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumBackgroundCopyFilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IEnumBackgroundCopyFilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pucount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumBackgroundCopyFiles as ::windows::core::Interface>::IID
    }
}
pub trait IEnumBackgroundCopyGroupsImpl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumBackgroundCopyGroups>;
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
}
impl IEnumBackgroundCopyGroupsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyGroupsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumBackgroundCopyGroupsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumBackgroundCopyGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumBackgroundCopyGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumBackgroundCopyGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumBackgroundCopyGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IEnumBackgroundCopyGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pucount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumBackgroundCopyGroups as ::windows::core::Interface>::IID
    }
}
pub trait IEnumBackgroundCopyJobsImpl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<IBackgroundCopyJob>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumBackgroundCopyJobs>;
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
}
impl IEnumBackgroundCopyJobsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyJobsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumBackgroundCopyJobsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumBackgroundCopyJobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumBackgroundCopyJobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumBackgroundCopyJobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumBackgroundCopyJobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IEnumBackgroundCopyJobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pucount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumBackgroundCopyJobs as ::windows::core::Interface>::IID
    }
}
pub trait IEnumBackgroundCopyJobs1Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumBackgroundCopyJobs1>;
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
}
impl IEnumBackgroundCopyJobs1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyJobs1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumBackgroundCopyJobs1Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumBackgroundCopyJobs1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumBackgroundCopyJobs1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumBackgroundCopyJobs1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumBackgroundCopyJobs1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IEnumBackgroundCopyJobs1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pucount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumBackgroundCopyJobs1 as ::windows::core::Interface>::IID
    }
}
pub trait IEnumBitsPeerCacheRecordsImpl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<IBitsPeerCacheRecord>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumBitsPeerCacheRecords>;
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
}
impl IEnumBitsPeerCacheRecordsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBitsPeerCacheRecordsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumBitsPeerCacheRecordsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumBitsPeerCacheRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumBitsPeerCacheRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumBitsPeerCacheRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumBitsPeerCacheRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IEnumBitsPeerCacheRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pucount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumBitsPeerCacheRecords as ::windows::core::Interface>::IID
    }
}
pub trait IEnumBitsPeersImpl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<IBitsPeer>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumBitsPeers>;
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
}
impl IEnumBitsPeersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBitsPeersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumBitsPeersVtbl {
        unsafe extern "system" fn Next<Impl: IEnumBitsPeersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumBitsPeersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumBitsPeersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumBitsPeersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IEnumBitsPeersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pucount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumBitsPeers as ::windows::core::Interface>::IID
    }
}
