pub trait AsyncIBackgroundCopyCallback_Impl: Sized {
    fn Begin_JobTransferred(&mut self, pjob: &::core::option::Option<IBackgroundCopyJob>) -> ::windows::core::Result<()>;
    fn Finish_JobTransferred(&mut self) -> ::windows::core::Result<()>;
    fn Begin_JobError(&mut self, pjob: &::core::option::Option<IBackgroundCopyJob>, perror: &::core::option::Option<IBackgroundCopyError>) -> ::windows::core::Result<()>;
    fn Finish_JobError(&mut self) -> ::windows::core::Result<()>;
    fn Begin_JobModification(&mut self, pjob: &::core::option::Option<IBackgroundCopyJob>, dwreserved: u32) -> ::windows::core::Result<()>;
    fn Finish_JobModification(&mut self) -> ::windows::core::Result<()>;
}
impl AsyncIBackgroundCopyCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>() -> AsyncIBackgroundCopyCallback_Vtbl {
        unsafe extern "system" fn Begin_JobTransferred<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_JobTransferred(::core::mem::transmute(&pjob)).into()
        }
        unsafe extern "system" fn Finish_JobTransferred<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_JobTransferred().into()
        }
        unsafe extern "system" fn Begin_JobError<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr, perror: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_JobError(::core::mem::transmute(&pjob), ::core::mem::transmute(&perror)).into()
        }
        unsafe extern "system" fn Finish_JobError<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_JobError().into()
        }
        unsafe extern "system" fn Begin_JobModification<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_JobModification(::core::mem::transmute(&pjob), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn Finish_JobModification<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_JobModification().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin_JobTransferred: Begin_JobTransferred::<Identity, Impl, OFFSET>,
            Finish_JobTransferred: Finish_JobTransferred::<Identity, Impl, OFFSET>,
            Begin_JobError: Begin_JobError::<Identity, Impl, OFFSET>,
            Finish_JobError: Finish_JobError::<Identity, Impl, OFFSET>,
            Begin_JobModification: Begin_JobModification::<Identity, Impl, OFFSET>,
            Finish_JobModification: Finish_JobModification::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIBackgroundCopyCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBITSExtensionSetup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn EnableBITSUploads(&mut self) -> ::windows::core::Result<()>;
    fn DisableBITSUploads(&mut self) -> ::windows::core::Result<()>;
    fn GetCleanupTaskName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetCleanupTask(&mut self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBITSExtensionSetup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBITSExtensionSetup_Impl, const OFFSET: isize>() -> IBITSExtensionSetup_Vtbl {
        unsafe extern "system" fn EnableBITSUploads<Identity: ::windows::core::IUnknownImpl, Impl: IBITSExtensionSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableBITSUploads().into()
        }
        unsafe extern "system" fn DisableBITSUploads<Identity: ::windows::core::IUnknownImpl, Impl: IBITSExtensionSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisableBITSUploads().into()
        }
        unsafe extern "system" fn GetCleanupTaskName<Identity: ::windows::core::IUnknownImpl, Impl: IBITSExtensionSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptaskname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCleanupTaskName() {
                ::core::result::Result::Ok(ok__) => {
                    *ptaskname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCleanupTask<Identity: ::windows::core::IUnknownImpl, Impl: IBITSExtensionSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCleanupTask(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnableBITSUploads: EnableBITSUploads::<Identity, Impl, OFFSET>,
            DisableBITSUploads: DisableBITSUploads::<Identity, Impl, OFFSET>,
            GetCleanupTaskName: GetCleanupTaskName::<Identity, Impl, OFFSET>,
            GetCleanupTask: GetCleanupTask::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBITSExtensionSetup as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBITSExtensionSetupFactory_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetObject(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<IBITSExtensionSetup>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBITSExtensionSetupFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBITSExtensionSetupFactory_Impl, const OFFSET: isize>() -> IBITSExtensionSetupFactory_Vtbl {
        unsafe extern "system" fn GetObject<Identity: ::windows::core::IUnknownImpl, Impl: IBITSExtensionSetupFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppextensionsetup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetObject(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppextensionsetup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), GetObject: GetObject::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBITSExtensionSetupFactory as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundCopyCallback_Impl: Sized {
    fn JobTransferred(&mut self, pjob: &::core::option::Option<IBackgroundCopyJob>) -> ::windows::core::Result<()>;
    fn JobError(&mut self, pjob: &::core::option::Option<IBackgroundCopyJob>, perror: &::core::option::Option<IBackgroundCopyError>) -> ::windows::core::Result<()>;
    fn JobModification(&mut self, pjob: &::core::option::Option<IBackgroundCopyJob>, dwreserved: u32) -> ::windows::core::Result<()>;
}
impl IBackgroundCopyCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyCallback_Impl, const OFFSET: isize>() -> IBackgroundCopyCallback_Vtbl {
        unsafe extern "system" fn JobTransferred<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).JobTransferred(::core::mem::transmute(&pjob)).into()
        }
        unsafe extern "system" fn JobError<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr, perror: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).JobError(::core::mem::transmute(&pjob), ::core::mem::transmute(&perror)).into()
        }
        unsafe extern "system" fn JobModification<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).JobModification(::core::mem::transmute(&pjob), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            JobTransferred: JobTransferred::<Identity, Impl, OFFSET>,
            JobError: JobError::<Identity, Impl, OFFSET>,
            JobModification: JobModification::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyCallback as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundCopyCallback1_Impl: Sized {
    fn OnStatus(&mut self, pgroup: &::core::option::Option<IBackgroundCopyGroup>, pjob: &::core::option::Option<IBackgroundCopyJob1>, dwfileindex: u32, dwstatus: u32, dwnumofretries: u32, dwwin32result: u32, dwtransportresult: u32) -> ::windows::core::Result<()>;
    fn OnProgress(&mut self, progresstype: u32, pgroup: &::core::option::Option<IBackgroundCopyGroup>, pjob: &::core::option::Option<IBackgroundCopyJob1>, dwfileindex: u32, dwprogressvalue: u32) -> ::windows::core::Result<()>;
    fn OnProgressEx(&mut self, progresstype: u32, pgroup: &::core::option::Option<IBackgroundCopyGroup>, pjob: &::core::option::Option<IBackgroundCopyJob1>, dwfileindex: u32, dwprogressvalue: u32, dwbytearraysize: u32, pbyte: *const u8) -> ::windows::core::Result<()>;
}
impl IBackgroundCopyCallback1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyCallback1_Impl, const OFFSET: isize>() -> IBackgroundCopyCallback1_Vtbl {
        unsafe extern "system" fn OnStatus<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyCallback1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroup: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, dwfileindex: u32, dwstatus: u32, dwnumofretries: u32, dwwin32result: u32, dwtransportresult: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnStatus(::core::mem::transmute(&pgroup), ::core::mem::transmute(&pjob), ::core::mem::transmute_copy(&dwfileindex), ::core::mem::transmute_copy(&dwstatus), ::core::mem::transmute_copy(&dwnumofretries), ::core::mem::transmute_copy(&dwwin32result), ::core::mem::transmute_copy(&dwtransportresult)).into()
        }
        unsafe extern "system" fn OnProgress<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyCallback1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progresstype: u32, pgroup: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, dwfileindex: u32, dwprogressvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnProgress(::core::mem::transmute_copy(&progresstype), ::core::mem::transmute(&pgroup), ::core::mem::transmute(&pjob), ::core::mem::transmute_copy(&dwfileindex), ::core::mem::transmute_copy(&dwprogressvalue)).into()
        }
        unsafe extern "system" fn OnProgressEx<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyCallback1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progresstype: u32, pgroup: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, dwfileindex: u32, dwprogressvalue: u32, dwbytearraysize: u32, pbyte: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnProgressEx(::core::mem::transmute_copy(&progresstype), ::core::mem::transmute(&pgroup), ::core::mem::transmute(&pjob), ::core::mem::transmute_copy(&dwfileindex), ::core::mem::transmute_copy(&dwprogressvalue), ::core::mem::transmute_copy(&dwbytearraysize), ::core::mem::transmute_copy(&pbyte)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnStatus: OnStatus::<Identity, Impl, OFFSET>,
            OnProgress: OnProgress::<Identity, Impl, OFFSET>,
            OnProgressEx: OnProgressEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyCallback1 as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundCopyCallback2_Impl: Sized + IBackgroundCopyCallback_Impl {
    fn FileTransferred(&mut self, pjob: &::core::option::Option<IBackgroundCopyJob>, pfile: &::core::option::Option<IBackgroundCopyFile>) -> ::windows::core::Result<()>;
}
impl IBackgroundCopyCallback2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyCallback2_Impl, const OFFSET: isize>() -> IBackgroundCopyCallback2_Vtbl {
        unsafe extern "system" fn FileTransferred<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyCallback2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr, pfile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FileTransferred(::core::mem::transmute(&pjob), ::core::mem::transmute(&pfile)).into()
        }
        Self { base: IBackgroundCopyCallback_Vtbl::new::<Identity, Impl, OFFSET>(), FileTransferred: FileTransferred::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyCallback2 as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyCallback as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundCopyCallback3_Impl: Sized + IBackgroundCopyCallback_Impl + IBackgroundCopyCallback2_Impl {
    fn FileRangesTransferred(&mut self, job: &::core::option::Option<IBackgroundCopyJob>, file: &::core::option::Option<IBackgroundCopyFile>, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::Result<()>;
}
impl IBackgroundCopyCallback3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyCallback3_Impl, const OFFSET: isize>() -> IBackgroundCopyCallback3_Vtbl {
        unsafe extern "system" fn FileRangesTransferred<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyCallback3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, job: ::windows::core::RawPtr, file: ::windows::core::RawPtr, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FileRangesTransferred(::core::mem::transmute(&job), ::core::mem::transmute(&file), ::core::mem::transmute_copy(&rangecount), ::core::mem::transmute_copy(&ranges)).into()
        }
        Self { base: IBackgroundCopyCallback2_Vtbl::new::<Identity, Impl, OFFSET>(), FileRangesTransferred: FileRangesTransferred::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyCallback3 as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyCallback as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyCallback2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyError_Impl: Sized {
    fn GetError(&mut self, pcontext: *mut BG_ERROR_CONTEXT, pcode: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn GetFile(&mut self) -> ::windows::core::Result<IBackgroundCopyFile>;
    fn GetErrorDescription(&mut self, languageid: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetErrorContextDescription(&mut self, languageid: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetProtocol(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyError_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyError_Impl, const OFFSET: isize>() -> IBackgroundCopyError_Vtbl {
        unsafe extern "system" fn GetError<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut BG_ERROR_CONTEXT, pcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetError(::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&pcode)).into()
        }
        unsafe extern "system" fn GetFile<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFile() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorDescription<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageid: u32, perrordescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetErrorDescription(::core::mem::transmute_copy(&languageid)) {
                ::core::result::Result::Ok(ok__) => {
                    *perrordescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorContextDescription<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageid: u32, pcontextdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetErrorContextDescription(::core::mem::transmute_copy(&languageid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcontextdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProtocol<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocol: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProtocol() {
                ::core::result::Result::Ok(ok__) => {
                    *pprotocol = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetError: GetError::<Identity, Impl, OFFSET>,
            GetFile: GetFile::<Identity, Impl, OFFSET>,
            GetErrorDescription: GetErrorDescription::<Identity, Impl, OFFSET>,
            GetErrorContextDescription: GetErrorContextDescription::<Identity, Impl, OFFSET>,
            GetProtocol: GetProtocol::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyError as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFile_Impl: Sized {
    fn GetRemoteName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetLocalName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetProgress(&mut self) -> ::windows::core::Result<BG_FILE_PROGRESS>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyFile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile_Impl, const OFFSET: isize>() -> IBackgroundCopyFile_Vtbl {
        unsafe extern "system" fn GetRemoteName<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRemoteName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalName<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLocalName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProgress<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut BG_FILE_PROGRESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetRemoteName: GetRemoteName::<Identity, Impl, OFFSET>,
            GetLocalName: GetLocalName::<Identity, Impl, OFFSET>,
            GetProgress: GetProgress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyFile as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFile2_Impl: Sized + IBackgroundCopyFile_Impl {
    fn GetFileRanges(&mut self, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::Result<()>;
    fn SetRemoteName(&mut self, val: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyFile2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile2_Impl, const OFFSET: isize>() -> IBackgroundCopyFile2_Vtbl {
        unsafe extern "system" fn GetFileRanges<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFileRanges(::core::mem::transmute_copy(&rangecount), ::core::mem::transmute_copy(&ranges)).into()
        }
        unsafe extern "system" fn SetRemoteName<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRemoteName(::core::mem::transmute_copy(&val)).into()
        }
        Self {
            base: IBackgroundCopyFile_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFileRanges: GetFileRanges::<Identity, Impl, OFFSET>,
            SetRemoteName: SetRemoteName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyFile2 as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyFile as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFile3_Impl: Sized + IBackgroundCopyFile_Impl + IBackgroundCopyFile2_Impl {
    fn GetTemporaryName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetValidationState(&mut self, state: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetValidationState(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsDownloadedFromPeer(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyFile3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile3_Impl, const OFFSET: isize>() -> IBackgroundCopyFile3_Vtbl {
        unsafe extern "system" fn GetTemporaryName<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTemporaryName() {
                ::core::result::Result::Ok(ok__) => {
                    *pfilename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValidationState<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValidationState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn GetValidationState<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetValidationState() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDownloadedFromPeer<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsDownloadedFromPeer() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IBackgroundCopyFile2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetTemporaryName: GetTemporaryName::<Identity, Impl, OFFSET>,
            SetValidationState: SetValidationState::<Identity, Impl, OFFSET>,
            GetValidationState: GetValidationState::<Identity, Impl, OFFSET>,
            IsDownloadedFromPeer: IsDownloadedFromPeer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyFile3 as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyFile as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyFile2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFile4_Impl: Sized + IBackgroundCopyFile_Impl + IBackgroundCopyFile2_Impl + IBackgroundCopyFile3_Impl {
    fn GetPeerDownloadStats(&mut self, pfromorigin: *mut u64, pfrompeers: *mut u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyFile4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile4_Impl, const OFFSET: isize>() -> IBackgroundCopyFile4_Vtbl {
        unsafe extern "system" fn GetPeerDownloadStats<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfromorigin: *mut u64, pfrompeers: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPeerDownloadStats(::core::mem::transmute_copy(&pfromorigin), ::core::mem::transmute_copy(&pfrompeers)).into()
        }
        Self { base: IBackgroundCopyFile3_Vtbl::new::<Identity, Impl, OFFSET>(), GetPeerDownloadStats: GetPeerDownloadStats::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyFile4 as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyFile as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyFile2 as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyFile3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFile5_Impl: Sized + IBackgroundCopyFile_Impl + IBackgroundCopyFile2_Impl + IBackgroundCopyFile3_Impl + IBackgroundCopyFile4_Impl {
    fn SetProperty(&mut self, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: &BITS_FILE_PROPERTY_VALUE) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, propertyid: BITS_FILE_PROPERTY_ID) -> ::windows::core::Result<BITS_FILE_PROPERTY_VALUE>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyFile5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile5_Impl, const OFFSET: isize>() -> IBackgroundCopyFile5_Vtbl {
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: BITS_FILE_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: *mut BITS_FILE_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IBackgroundCopyFile4_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyFile5 as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyFile as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyFile2 as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyFile3 as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyFile4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFile6_Impl: Sized + IBackgroundCopyFile_Impl + IBackgroundCopyFile2_Impl + IBackgroundCopyFile3_Impl + IBackgroundCopyFile4_Impl + IBackgroundCopyFile5_Impl {
    fn UpdateDownloadPosition(&mut self, offset: u64) -> ::windows::core::Result<()>;
    fn RequestFileRanges(&mut self, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::Result<()>;
    fn GetFilledFileRanges(&mut self, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyFile6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile6_Impl, const OFFSET: isize>() -> IBackgroundCopyFile6_Vtbl {
        unsafe extern "system" fn UpdateDownloadPosition<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateDownloadPosition(::core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn RequestFileRanges<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RequestFileRanges(::core::mem::transmute_copy(&rangecount), ::core::mem::transmute_copy(&ranges)).into()
        }
        unsafe extern "system" fn GetFilledFileRanges<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFilledFileRanges(::core::mem::transmute_copy(&rangecount), ::core::mem::transmute_copy(&ranges)).into()
        }
        Self {
            base: IBackgroundCopyFile5_Vtbl::new::<Identity, Impl, OFFSET>(),
            UpdateDownloadPosition: UpdateDownloadPosition::<Identity, Impl, OFFSET>,
            RequestFileRanges: RequestFileRanges::<Identity, Impl, OFFSET>,
            GetFilledFileRanges: GetFilledFileRanges::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyFile6 as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyFile as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyFile2 as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyFile3 as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyFile4 as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyFile5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBackgroundCopyGroup_Impl: Sized {
    fn GetProp(&mut self, propid: GROUPPROP) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProp(&mut self, propid: GROUPPROP, pvarval: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetProgress(&mut self, dwflags: u32) -> ::windows::core::Result<u32>;
    fn GetStatus(&mut self, pdwstatus: *mut u32, pdwjobindex: *mut u32) -> ::windows::core::Result<()>;
    fn GetJob(&mut self, jobid: &::windows::core::GUID) -> ::windows::core::Result<IBackgroundCopyJob1>;
    fn SuspendGroup(&mut self) -> ::windows::core::Result<()>;
    fn ResumeGroup(&mut self) -> ::windows::core::Result<()>;
    fn CancelGroup(&mut self) -> ::windows::core::Result<()>;
    fn Size(&mut self) -> ::windows::core::Result<u32>;
    fn GroupID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CreateJob(&mut self, guidjobid: &::windows::core::GUID) -> ::windows::core::Result<IBackgroundCopyJob1>;
    fn EnumJobs(&mut self, dwflags: u32) -> ::windows::core::Result<IEnumBackgroundCopyJobs1>;
    fn SwitchToForeground(&mut self) -> ::windows::core::Result<()>;
    fn QueryNewJobInterface(&mut self, iid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn SetNotificationPointer(&mut self, iid: *const ::windows::core::GUID, punk: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBackgroundCopyGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>() -> IBackgroundCopyGroup_Vtbl {
        unsafe extern "system" fn GetProp<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: GROUPPROP, pvarval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProp(::core::mem::transmute_copy(&propid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProp<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: GROUPPROP, pvarval: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProp(::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn GetProgress<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdwprogress: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProgress(::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwprogress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32, pdwjobindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStatus(::core::mem::transmute_copy(&pdwstatus), ::core::mem::transmute_copy(&pdwjobindex)).into()
        }
        unsafe extern "system" fn GetJob<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobid: ::windows::core::GUID, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetJob(::core::mem::transmute_copy(&jobid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppjob = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuspendGroup<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SuspendGroup().into()
        }
        unsafe extern "system" fn ResumeGroup<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResumeGroup().into()
        }
        unsafe extern "system" fn CancelGroup<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelGroup().into()
        }
        unsafe extern "system" fn Size<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GroupID<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidgroupid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GroupID() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidgroupid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateJob<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidjobid: ::windows::core::GUID, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateJob(::core::mem::transmute_copy(&guidjobid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppjob = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumJobs<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppenumjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumJobs(::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumjobs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SwitchToForeground<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SwitchToForeground().into()
        }
        unsafe extern "system" fn QueryNewJobInterface<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryNewJobInterface(::core::mem::transmute_copy(&iid)) {
                ::core::result::Result::Ok(ok__) => {
                    *punk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationPointer<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNotificationPointer(::core::mem::transmute_copy(&iid), ::core::mem::transmute(&punk)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProp: GetProp::<Identity, Impl, OFFSET>,
            SetProp: SetProp::<Identity, Impl, OFFSET>,
            GetProgress: GetProgress::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetJob: GetJob::<Identity, Impl, OFFSET>,
            SuspendGroup: SuspendGroup::<Identity, Impl, OFFSET>,
            ResumeGroup: ResumeGroup::<Identity, Impl, OFFSET>,
            CancelGroup: CancelGroup::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            GroupID: GroupID::<Identity, Impl, OFFSET>,
            CreateJob: CreateJob::<Identity, Impl, OFFSET>,
            EnumJobs: EnumJobs::<Identity, Impl, OFFSET>,
            SwitchToForeground: SwitchToForeground::<Identity, Impl, OFFSET>,
            QueryNewJobInterface: QueryNewJobInterface::<Identity, Impl, OFFSET>,
            SetNotificationPointer: SetNotificationPointer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJob_Impl: Sized {
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
    fn SetNotifyInterface(&mut self, val: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
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
impl IBackgroundCopyJob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>() -> IBackgroundCopyJob_Vtbl {
        unsafe extern "system" fn AddFileSet<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddFileSet(::core::mem::transmute_copy(&cfilecount), ::core::mem::transmute_copy(&pfileset)).into()
        }
        unsafe extern "system" fn AddFile<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteurl: super::super::Foundation::PWSTR, localname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddFile(::core::mem::transmute_copy(&remoteurl), ::core::mem::transmute_copy(&localname)).into()
        }
        unsafe extern "system" fn EnumFiles<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumFiles() {
                ::core::result::Result::Ok(ok__) => {
                    *penum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Suspend<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Suspend().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn Complete<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Complete().into()
        }
        unsafe extern "system" fn GetId<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetId() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProgress<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_PROGRESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimes<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_TIMES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTimes() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetState<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetState() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetError<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetError() {
                ::core::result::Result::Ok(ok__) => {
                    *pperror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOwner<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn GetDisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: BG_JOB_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn GetPriority<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPriority() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotifyFlags<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNotifyFlags(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn GetNotifyFlags<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNotifyFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotifyInterface<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNotifyInterface(::core::mem::transmute(&val)).into()
        }
        unsafe extern "system" fn GetNotifyInterface<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNotifyInterface() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumRetryDelay<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMinimumRetryDelay(::core::mem::transmute_copy(&seconds)).into()
        }
        unsafe extern "system" fn GetMinimumRetryDelay<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMinimumRetryDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *seconds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoProgressTimeout<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNoProgressTimeout(::core::mem::transmute_copy(&seconds)).into()
        }
        unsafe extern "system" fn GetNoProgressTimeout<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNoProgressTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *seconds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorCount<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errors: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetErrorCount() {
                ::core::result::Result::Ok(ok__) => {
                    *errors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxySettings<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, proxyusage: BG_JOB_PROXY_USAGE, proxylist: super::super::Foundation::PWSTR, proxybypasslist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProxySettings(::core::mem::transmute_copy(&proxyusage), ::core::mem::transmute_copy(&proxylist), ::core::mem::transmute_copy(&proxybypasslist)).into()
        }
        unsafe extern "system" fn GetProxySettings<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut super::super::Foundation::PWSTR, pproxybypasslist: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProxySettings(::core::mem::transmute_copy(&pproxyusage), ::core::mem::transmute_copy(&pproxylist), ::core::mem::transmute_copy(&pproxybypasslist)).into()
        }
        unsafe extern "system" fn TakeOwnership<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TakeOwnership().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddFileSet: AddFileSet::<Identity, Impl, OFFSET>,
            AddFile: AddFile::<Identity, Impl, OFFSET>,
            EnumFiles: EnumFiles::<Identity, Impl, OFFSET>,
            Suspend: Suspend::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Complete: Complete::<Identity, Impl, OFFSET>,
            GetId: GetId::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetProgress: GetProgress::<Identity, Impl, OFFSET>,
            GetTimes: GetTimes::<Identity, Impl, OFFSET>,
            GetState: GetState::<Identity, Impl, OFFSET>,
            GetError: GetError::<Identity, Impl, OFFSET>,
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            GetPriority: GetPriority::<Identity, Impl, OFFSET>,
            SetNotifyFlags: SetNotifyFlags::<Identity, Impl, OFFSET>,
            GetNotifyFlags: GetNotifyFlags::<Identity, Impl, OFFSET>,
            SetNotifyInterface: SetNotifyInterface::<Identity, Impl, OFFSET>,
            GetNotifyInterface: GetNotifyInterface::<Identity, Impl, OFFSET>,
            SetMinimumRetryDelay: SetMinimumRetryDelay::<Identity, Impl, OFFSET>,
            GetMinimumRetryDelay: GetMinimumRetryDelay::<Identity, Impl, OFFSET>,
            SetNoProgressTimeout: SetNoProgressTimeout::<Identity, Impl, OFFSET>,
            GetNoProgressTimeout: GetNoProgressTimeout::<Identity, Impl, OFFSET>,
            GetErrorCount: GetErrorCount::<Identity, Impl, OFFSET>,
            SetProxySettings: SetProxySettings::<Identity, Impl, OFFSET>,
            GetProxySettings: GetProxySettings::<Identity, Impl, OFFSET>,
            TakeOwnership: TakeOwnership::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJob as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJob1_Impl: Sized {
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
impl IBackgroundCopyJob1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob1_Impl, const OFFSET: isize>() -> IBackgroundCopyJob1_Vtbl {
        unsafe extern "system" fn CancelJob<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelJob().into()
        }
        unsafe extern "system" fn GetProgress<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdwprogress: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProgress(::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwprogress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32, pdwwin32result: *mut u32, pdwtransportresult: *mut u32, pdwnumofretries: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStatus(::core::mem::transmute_copy(&pdwstatus), ::core::mem::transmute_copy(&pdwwin32result), ::core::mem::transmute_copy(&pdwtransportresult), ::core::mem::transmute_copy(&pdwnumofretries)).into()
        }
        unsafe extern "system" fn AddFiles<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfilecount: u32, ppfileset: *const *const FILESETINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddFiles(::core::mem::transmute_copy(&cfilecount), ::core::mem::transmute_copy(&ppfileset)).into()
        }
        unsafe extern "system" fn GetFile<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfileindex: u32, pfileinfo: *mut FILESETINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFile(::core::mem::transmute_copy(&cfileindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfileinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileCount<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfilecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFileCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwfilecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SwitchToForeground<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SwitchToForeground().into()
        }
        unsafe extern "system" fn JobID<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidjobid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).JobID() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidjobid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CancelJob: CancelJob::<Identity, Impl, OFFSET>,
            GetProgress: GetProgress::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            AddFiles: AddFiles::<Identity, Impl, OFFSET>,
            GetFile: GetFile::<Identity, Impl, OFFSET>,
            GetFileCount: GetFileCount::<Identity, Impl, OFFSET>,
            SwitchToForeground: SwitchToForeground::<Identity, Impl, OFFSET>,
            JobID: JobID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJob1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJob2_Impl: Sized + IBackgroundCopyJob_Impl {
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
impl IBackgroundCopyJob2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob2_Impl, const OFFSET: isize>() -> IBackgroundCopyJob2_Vtbl {
        unsafe extern "system" fn SetNotifyCmdLine<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, program: super::super::Foundation::PWSTR, parameters: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNotifyCmdLine(::core::mem::transmute_copy(&program), ::core::mem::transmute_copy(&parameters)).into()
        }
        unsafe extern "system" fn GetNotifyCmdLine<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprogram: *mut super::super::Foundation::PWSTR, pparameters: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNotifyCmdLine(::core::mem::transmute_copy(&pprogram), ::core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn GetReplyProgress<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetReplyProgress(::core::mem::transmute_copy(&pprogress)).into()
        }
        unsafe extern "system" fn GetReplyData<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbuffer: *mut *mut u8, plength: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetReplyData(::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&plength)).into()
        }
        unsafe extern "system" fn SetReplyFileName<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, replyfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetReplyFileName(::core::mem::transmute_copy(&replyfilename)).into()
        }
        unsafe extern "system" fn GetReplyFileName<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preplyfilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetReplyFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *preplyfilename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credentials: *const BG_AUTH_CREDENTIALS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCredentials(::core::mem::transmute_copy(&credentials)).into()
        }
        unsafe extern "system" fn RemoveCredentials<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveCredentials(::core::mem::transmute_copy(&target), ::core::mem::transmute_copy(&scheme)).into()
        }
        Self {
            base: IBackgroundCopyJob_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetNotifyCmdLine: SetNotifyCmdLine::<Identity, Impl, OFFSET>,
            GetNotifyCmdLine: GetNotifyCmdLine::<Identity, Impl, OFFSET>,
            GetReplyProgress: GetReplyProgress::<Identity, Impl, OFFSET>,
            GetReplyData: GetReplyData::<Identity, Impl, OFFSET>,
            SetReplyFileName: SetReplyFileName::<Identity, Impl, OFFSET>,
            GetReplyFileName: GetReplyFileName::<Identity, Impl, OFFSET>,
            SetCredentials: SetCredentials::<Identity, Impl, OFFSET>,
            RemoveCredentials: RemoveCredentials::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJob2 as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyJob as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJob3_Impl: Sized + IBackgroundCopyJob_Impl + IBackgroundCopyJob2_Impl {
    fn ReplaceRemotePrefix(&mut self, oldprefix: super::super::Foundation::PWSTR, newprefix: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn AddFileWithRanges(&mut self, remoteurl: super::super::Foundation::PWSTR, localname: super::super::Foundation::PWSTR, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::Result<()>;
    fn SetFileACLFlags(&mut self, flags: u32) -> ::windows::core::Result<()>;
    fn GetFileACLFlags(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyJob3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob3_Impl, const OFFSET: isize>() -> IBackgroundCopyJob3_Vtbl {
        unsafe extern "system" fn ReplaceRemotePrefix<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldprefix: super::super::Foundation::PWSTR, newprefix: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReplaceRemotePrefix(::core::mem::transmute_copy(&oldprefix), ::core::mem::transmute_copy(&newprefix)).into()
        }
        unsafe extern "system" fn AddFileWithRanges<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteurl: super::super::Foundation::PWSTR, localname: super::super::Foundation::PWSTR, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddFileWithRanges(::core::mem::transmute_copy(&remoteurl), ::core::mem::transmute_copy(&localname), ::core::mem::transmute_copy(&rangecount), ::core::mem::transmute_copy(&ranges)).into()
        }
        unsafe extern "system" fn SetFileACLFlags<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFileACLFlags(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetFileACLFlags<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFileACLFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IBackgroundCopyJob2_Vtbl::new::<Identity, Impl, OFFSET>(),
            ReplaceRemotePrefix: ReplaceRemotePrefix::<Identity, Impl, OFFSET>,
            AddFileWithRanges: AddFileWithRanges::<Identity, Impl, OFFSET>,
            SetFileACLFlags: SetFileACLFlags::<Identity, Impl, OFFSET>,
            GetFileACLFlags: GetFileACLFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJob3 as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyJob as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyJob2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJob4_Impl: Sized + IBackgroundCopyJob_Impl + IBackgroundCopyJob2_Impl + IBackgroundCopyJob3_Impl {
    fn SetPeerCachingFlags(&mut self, flags: u32) -> ::windows::core::Result<()>;
    fn GetPeerCachingFlags(&mut self) -> ::windows::core::Result<u32>;
    fn GetOwnerIntegrityLevel(&mut self) -> ::windows::core::Result<u32>;
    fn GetOwnerElevationState(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetMaximumDownloadTime(&mut self, timeout: u32) -> ::windows::core::Result<()>;
    fn GetMaximumDownloadTime(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyJob4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob4_Impl, const OFFSET: isize>() -> IBackgroundCopyJob4_Vtbl {
        unsafe extern "system" fn SetPeerCachingFlags<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPeerCachingFlags(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetPeerCachingFlags<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPeerCachingFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOwnerIntegrityLevel<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOwnerIntegrityLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOwnerElevationState<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pelevated: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOwnerElevationState() {
                ::core::result::Result::Ok(ok__) => {
                    *pelevated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumDownloadTime<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaximumDownloadTime(::core::mem::transmute_copy(&timeout)).into()
        }
        unsafe extern "system" fn GetMaximumDownloadTime<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaximumDownloadTime() {
                ::core::result::Result::Ok(ok__) => {
                    *ptimeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IBackgroundCopyJob3_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetPeerCachingFlags: SetPeerCachingFlags::<Identity, Impl, OFFSET>,
            GetPeerCachingFlags: GetPeerCachingFlags::<Identity, Impl, OFFSET>,
            GetOwnerIntegrityLevel: GetOwnerIntegrityLevel::<Identity, Impl, OFFSET>,
            GetOwnerElevationState: GetOwnerElevationState::<Identity, Impl, OFFSET>,
            SetMaximumDownloadTime: SetMaximumDownloadTime::<Identity, Impl, OFFSET>,
            GetMaximumDownloadTime: GetMaximumDownloadTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJob4 as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyJob as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyJob2 as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyJob3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJob5_Impl: Sized + IBackgroundCopyJob_Impl + IBackgroundCopyJob2_Impl + IBackgroundCopyJob3_Impl + IBackgroundCopyJob4_Impl {
    fn SetProperty(&mut self, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: &BITS_JOB_PROPERTY_VALUE) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, propertyid: BITS_JOB_PROPERTY_ID) -> ::windows::core::Result<BITS_JOB_PROPERTY_VALUE>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyJob5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob5_Impl, const OFFSET: isize>() -> IBackgroundCopyJob5_Vtbl {
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: BITS_JOB_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: *mut BITS_JOB_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IBackgroundCopyJob4_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJob5 as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyJob as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyJob2 as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyJob3 as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyJob4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJobHttpOptions_Impl: Sized {
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
impl IBackgroundCopyJobHttpOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>() -> IBackgroundCopyJobHttpOptions_Vtbl {
        unsafe extern "system" fn SetClientCertificateByID<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storelocation: BG_CERT_STORE_LOCATION, storename: super::super::Foundation::PWSTR, pcerthashblob: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClientCertificateByID(::core::mem::transmute_copy(&storelocation), ::core::mem::transmute_copy(&storename), ::core::mem::transmute_copy(&pcerthashblob)).into()
        }
        unsafe extern "system" fn SetClientCertificateByName<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storelocation: BG_CERT_STORE_LOCATION, storename: super::super::Foundation::PWSTR, subjectname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClientCertificateByName(::core::mem::transmute_copy(&storelocation), ::core::mem::transmute_copy(&storename), ::core::mem::transmute_copy(&subjectname)).into()
        }
        unsafe extern "system" fn RemoveClientCertificate<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveClientCertificate().into()
        }
        unsafe extern "system" fn GetClientCertificate<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut super::super::Foundation::PWSTR, ppcerthashblob: *mut *mut u8, psubjectname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClientCertificate(::core::mem::transmute_copy(&pstorelocation), ::core::mem::transmute_copy(&pstorename), ::core::mem::transmute_copy(&ppcerthashblob), ::core::mem::transmute_copy(&psubjectname)).into()
        }
        unsafe extern "system" fn SetCustomHeaders<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestheaders: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCustomHeaders(::core::mem::transmute_copy(&requestheaders)).into()
        }
        unsafe extern "system" fn GetCustomHeaders<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequestheaders: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCustomHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *prequestheaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityFlags<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSecurityFlags(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetSecurityFlags<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSecurityFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetClientCertificateByID: SetClientCertificateByID::<Identity, Impl, OFFSET>,
            SetClientCertificateByName: SetClientCertificateByName::<Identity, Impl, OFFSET>,
            RemoveClientCertificate: RemoveClientCertificate::<Identity, Impl, OFFSET>,
            GetClientCertificate: GetClientCertificate::<Identity, Impl, OFFSET>,
            SetCustomHeaders: SetCustomHeaders::<Identity, Impl, OFFSET>,
            GetCustomHeaders: GetCustomHeaders::<Identity, Impl, OFFSET>,
            SetSecurityFlags: SetSecurityFlags::<Identity, Impl, OFFSET>,
            GetSecurityFlags: GetSecurityFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJobHttpOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJobHttpOptions2_Impl: Sized + IBackgroundCopyJobHttpOptions_Impl {
    fn SetHttpMethod(&mut self, method: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetHttpMethod(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyJobHttpOptions2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJobHttpOptions2_Impl, const OFFSET: isize>() -> IBackgroundCopyJobHttpOptions2_Vtbl {
        unsafe extern "system" fn SetHttpMethod<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJobHttpOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHttpMethod(::core::mem::transmute_copy(&method)).into()
        }
        unsafe extern "system" fn GetHttpMethod<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJobHttpOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetHttpMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *method = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IBackgroundCopyJobHttpOptions_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetHttpMethod: SetHttpMethod::<Identity, Impl, OFFSET>,
            GetHttpMethod: GetHttpMethod::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJobHttpOptions2 as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyJobHttpOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJobHttpOptions3_Impl: Sized + IBackgroundCopyJobHttpOptions_Impl + IBackgroundCopyJobHttpOptions2_Impl {
    fn SetServerCertificateValidationInterface(&mut self, certvalidationcallback: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn MakeCustomHeadersWriteOnly(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyJobHttpOptions3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJobHttpOptions3_Impl, const OFFSET: isize>() -> IBackgroundCopyJobHttpOptions3_Vtbl {
        unsafe extern "system" fn SetServerCertificateValidationInterface<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJobHttpOptions3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certvalidationcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetServerCertificateValidationInterface(::core::mem::transmute(&certvalidationcallback)).into()
        }
        unsafe extern "system" fn MakeCustomHeadersWriteOnly<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJobHttpOptions3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MakeCustomHeadersWriteOnly().into()
        }
        Self {
            base: IBackgroundCopyJobHttpOptions2_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetServerCertificateValidationInterface: SetServerCertificateValidationInterface::<Identity, Impl, OFFSET>,
            MakeCustomHeadersWriteOnly: MakeCustomHeadersWriteOnly::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJobHttpOptions3 as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyJobHttpOptions as ::windows::core::Interface>::IID || iid == &<IBackgroundCopyJobHttpOptions2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyManager_Impl: Sized {
    fn CreateJob(&mut self, displayname: super::super::Foundation::PWSTR, r#type: BG_JOB_TYPE, pjobid: *mut ::windows::core::GUID, ppjob: *mut ::core::option::Option<IBackgroundCopyJob>) -> ::windows::core::Result<()>;
    fn GetJob(&mut self, jobid: *const ::windows::core::GUID) -> ::windows::core::Result<IBackgroundCopyJob>;
    fn EnumJobs(&mut self, dwflags: u32) -> ::windows::core::Result<IEnumBackgroundCopyJobs>;
    fn GetErrorDescription(&mut self, hresult: ::windows::core::HRESULT, languageid: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyManager_Impl, const OFFSET: isize>() -> IBackgroundCopyManager_Vtbl {
        unsafe extern "system" fn CreateJob<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: super::super::Foundation::PWSTR, r#type: BG_JOB_TYPE, pjobid: *mut ::windows::core::GUID, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateJob(::core::mem::transmute_copy(&displayname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pjobid), ::core::mem::transmute_copy(&ppjob)).into()
        }
        unsafe extern "system" fn GetJob<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobid: *const ::windows::core::GUID, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetJob(::core::mem::transmute_copy(&jobid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppjob = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumJobs<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumJobs(::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorDescription<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, languageid: u32, perrordescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetErrorDescription(::core::mem::transmute_copy(&hresult), ::core::mem::transmute_copy(&languageid)) {
                ::core::result::Result::Ok(ok__) => {
                    *perrordescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateJob: CreateJob::<Identity, Impl, OFFSET>,
            GetJob: GetJob::<Identity, Impl, OFFSET>,
            EnumJobs: EnumJobs::<Identity, Impl, OFFSET>,
            GetErrorDescription: GetErrorDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyManager as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundCopyQMgr_Impl: Sized {
    fn CreateGroup(&mut self, guidgroupid: &::windows::core::GUID) -> ::windows::core::Result<IBackgroundCopyGroup>;
    fn GetGroup(&mut self, groupid: &::windows::core::GUID) -> ::windows::core::Result<IBackgroundCopyGroup>;
    fn EnumGroups(&mut self, dwflags: u32) -> ::windows::core::Result<IEnumBackgroundCopyGroups>;
}
impl IBackgroundCopyQMgr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyQMgr_Impl, const OFFSET: isize>() -> IBackgroundCopyQMgr_Vtbl {
        unsafe extern "system" fn CreateGroup<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyQMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidgroupid: ::windows::core::GUID, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateGroup(::core::mem::transmute_copy(&guidgroupid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGroup<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyQMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, groupid: ::windows::core::GUID, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGroup(::core::mem::transmute_copy(&groupid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumGroups<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyQMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppenumgroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumGroups(::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumgroups = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateGroup: CreateGroup::<Identity, Impl, OFFSET>,
            GetGroup: GetGroup::<Identity, Impl, OFFSET>,
            EnumGroups: EnumGroups::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyQMgr as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundCopyServerCertificateValidationCallback_Impl: Sized {
    fn ValidateServerCertificate(&mut self, job: &::core::option::Option<IBackgroundCopyJob>, file: &::core::option::Option<IBackgroundCopyFile>, certlength: u32, certdata: *const u8, certencodingtype: u32, certstorelength: u32, certstoredata: *const u8) -> ::windows::core::Result<()>;
}
impl IBackgroundCopyServerCertificateValidationCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyServerCertificateValidationCallback_Impl, const OFFSET: isize>() -> IBackgroundCopyServerCertificateValidationCallback_Vtbl {
        unsafe extern "system" fn ValidateServerCertificate<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyServerCertificateValidationCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, job: ::windows::core::RawPtr, file: ::windows::core::RawPtr, certlength: u32, certdata: *const u8, certencodingtype: u32, certstorelength: u32, certstoredata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ValidateServerCertificate(::core::mem::transmute(&job), ::core::mem::transmute(&file), ::core::mem::transmute_copy(&certlength), ::core::mem::transmute_copy(&certdata), ::core::mem::transmute_copy(&certencodingtype), ::core::mem::transmute_copy(&certstorelength), ::core::mem::transmute_copy(&certstoredata)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ValidateServerCertificate: ValidateServerCertificate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyServerCertificateValidationCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBitsPeer_Impl: Sized {
    fn GetPeerName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn IsAuthenticated(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsAvailable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBitsPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeer_Impl, const OFFSET: isize>() -> IBitsPeer_Vtbl {
        unsafe extern "system" fn GetPeerName<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPeerName() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAuthenticated<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauth: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsAuthenticated() {
                ::core::result::Result::Ok(ok__) => {
                    *pauth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAvailable<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ponline: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *ponline = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPeerName: GetPeerName::<Identity, Impl, OFFSET>,
            IsAuthenticated: IsAuthenticated::<Identity, Impl, OFFSET>,
            IsAvailable: IsAvailable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitsPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBitsPeerCacheAdministration_Impl: Sized {
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
impl IBitsPeerCacheAdministration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>() -> IBitsPeerCacheAdministration_Vtbl {
        unsafe extern "system" fn GetMaximumCacheSize<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbytes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaximumCacheSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pbytes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumCacheSize<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bytes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaximumCacheSize(::core::mem::transmute_copy(&bytes)).into()
        }
        unsafe extern "system" fn GetMaximumContentAge<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pseconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaximumContentAge() {
                ::core::result::Result::Ok(ok__) => {
                    *pseconds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumContentAge<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaximumContentAge(::core::mem::transmute_copy(&seconds)).into()
        }
        unsafe extern "system" fn GetConfigurationFlags<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetConfigurationFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConfigurationFlags<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetConfigurationFlags(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn EnumRecords<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumRecords() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecord<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *const ::windows::core::GUID, pprecord: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRecord(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprecord = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearRecords<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearRecords().into()
        }
        unsafe extern "system" fn DeleteRecord<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteRecord(::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn DeleteUrl<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, url: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteUrl(::core::mem::transmute_copy(&url)).into()
        }
        unsafe extern "system" fn EnumPeers<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumPeers() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearPeers<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearPeers().into()
        }
        unsafe extern "system" fn DiscoverPeers<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DiscoverPeers().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetMaximumCacheSize: GetMaximumCacheSize::<Identity, Impl, OFFSET>,
            SetMaximumCacheSize: SetMaximumCacheSize::<Identity, Impl, OFFSET>,
            GetMaximumContentAge: GetMaximumContentAge::<Identity, Impl, OFFSET>,
            SetMaximumContentAge: SetMaximumContentAge::<Identity, Impl, OFFSET>,
            GetConfigurationFlags: GetConfigurationFlags::<Identity, Impl, OFFSET>,
            SetConfigurationFlags: SetConfigurationFlags::<Identity, Impl, OFFSET>,
            EnumRecords: EnumRecords::<Identity, Impl, OFFSET>,
            GetRecord: GetRecord::<Identity, Impl, OFFSET>,
            ClearRecords: ClearRecords::<Identity, Impl, OFFSET>,
            DeleteRecord: DeleteRecord::<Identity, Impl, OFFSET>,
            DeleteUrl: DeleteUrl::<Identity, Impl, OFFSET>,
            EnumPeers: EnumPeers::<Identity, Impl, OFFSET>,
            ClearPeers: ClearPeers::<Identity, Impl, OFFSET>,
            DiscoverPeers: DiscoverPeers::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitsPeerCacheAdministration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBitsPeerCacheRecord_Impl: Sized {
    fn GetId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetOriginUrl(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetFileSize(&mut self) -> ::windows::core::Result<u64>;
    fn GetFileModificationTime(&mut self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn GetLastAccessTime(&mut self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn IsFileValidated(&mut self) -> ::windows::core::Result<()>;
    fn GetFileRanges(&mut self, prangecount: *mut u32, ppranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBitsPeerCacheRecord_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: isize>() -> IBitsPeerCacheRecord_Vtbl {
        unsafe extern "system" fn GetId<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetId() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginUrl<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOriginUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileSize<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFileSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileModificationTime<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFileModificationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastAccessTime<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLastAccessTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFileValidated<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsFileValidated().into()
        }
        unsafe extern "system" fn GetFileRanges<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangecount: *mut u32, ppranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFileRanges(::core::mem::transmute_copy(&prangecount), ::core::mem::transmute_copy(&ppranges)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetId: GetId::<Identity, Impl, OFFSET>,
            GetOriginUrl: GetOriginUrl::<Identity, Impl, OFFSET>,
            GetFileSize: GetFileSize::<Identity, Impl, OFFSET>,
            GetFileModificationTime: GetFileModificationTime::<Identity, Impl, OFFSET>,
            GetLastAccessTime: GetLastAccessTime::<Identity, Impl, OFFSET>,
            IsFileValidated: IsFileValidated::<Identity, Impl, OFFSET>,
            GetFileRanges: GetFileRanges::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitsPeerCacheRecord as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBitsTokenOptions_Impl: Sized {
    fn SetHelperTokenFlags(&mut self, usageflags: BG_TOKEN) -> ::windows::core::Result<()>;
    fn GetHelperTokenFlags(&mut self) -> ::windows::core::Result<BG_TOKEN>;
    fn SetHelperToken(&mut self) -> ::windows::core::Result<()>;
    fn ClearHelperToken(&mut self) -> ::windows::core::Result<()>;
    fn GetHelperTokenSid(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBitsTokenOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitsTokenOptions_Impl, const OFFSET: isize>() -> IBitsTokenOptions_Vtbl {
        unsafe extern "system" fn SetHelperTokenFlags<Identity: ::windows::core::IUnknownImpl, Impl: IBitsTokenOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usageflags: BG_TOKEN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHelperTokenFlags(::core::mem::transmute_copy(&usageflags)).into()
        }
        unsafe extern "system" fn GetHelperTokenFlags<Identity: ::windows::core::IUnknownImpl, Impl: IBitsTokenOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut BG_TOKEN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetHelperTokenFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHelperToken<Identity: ::windows::core::IUnknownImpl, Impl: IBitsTokenOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHelperToken().into()
        }
        unsafe extern "system" fn ClearHelperToken<Identity: ::windows::core::IUnknownImpl, Impl: IBitsTokenOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearHelperToken().into()
        }
        unsafe extern "system" fn GetHelperTokenSid<Identity: ::windows::core::IUnknownImpl, Impl: IBitsTokenOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetHelperTokenSid() {
                ::core::result::Result::Ok(ok__) => {
                    *psid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetHelperTokenFlags: SetHelperTokenFlags::<Identity, Impl, OFFSET>,
            GetHelperTokenFlags: GetHelperTokenFlags::<Identity, Impl, OFFSET>,
            SetHelperToken: SetHelperToken::<Identity, Impl, OFFSET>,
            ClearHelperToken: ClearHelperToken::<Identity, Impl, OFFSET>,
            GetHelperTokenSid: GetHelperTokenSid::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitsTokenOptions as ::windows::core::Interface>::IID
    }
}
pub trait IEnumBackgroundCopyFiles_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<IBackgroundCopyFile>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumBackgroundCopyFiles>;
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
}
impl IEnumBackgroundCopyFiles_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>() -> IEnumBackgroundCopyFiles_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pucount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumBackgroundCopyFiles as ::windows::core::Interface>::IID
    }
}
pub trait IEnumBackgroundCopyGroups_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumBackgroundCopyGroups>;
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
}
impl IEnumBackgroundCopyGroups_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyGroups_Impl, const OFFSET: isize>() -> IEnumBackgroundCopyGroups_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pucount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumBackgroundCopyGroups as ::windows::core::Interface>::IID
    }
}
pub trait IEnumBackgroundCopyJobs_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<IBackgroundCopyJob>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumBackgroundCopyJobs>;
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
}
impl IEnumBackgroundCopyJobs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>() -> IEnumBackgroundCopyJobs_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pucount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumBackgroundCopyJobs as ::windows::core::Interface>::IID
    }
}
pub trait IEnumBackgroundCopyJobs1_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumBackgroundCopyJobs1>;
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
}
impl IEnumBackgroundCopyJobs1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyJobs1_Impl, const OFFSET: isize>() -> IEnumBackgroundCopyJobs1_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyJobs1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyJobs1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyJobs1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyJobs1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyJobs1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pucount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumBackgroundCopyJobs1 as ::windows::core::Interface>::IID
    }
}
pub trait IEnumBitsPeerCacheRecords_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<IBitsPeerCacheRecord>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumBitsPeerCacheRecords>;
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
}
impl IEnumBitsPeerCacheRecords_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>() -> IEnumBitsPeerCacheRecords_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pucount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumBitsPeerCacheRecords as ::windows::core::Interface>::IID
    }
}
pub trait IEnumBitsPeers_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<IBitsPeer>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumBitsPeers>;
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
}
impl IEnumBitsPeers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBitsPeers_Impl, const OFFSET: isize>() -> IEnumBitsPeers_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBitsPeers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBitsPeers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBitsPeers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBitsPeers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBitsPeers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pucount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumBitsPeers as ::windows::core::Interface>::IID
    }
}
