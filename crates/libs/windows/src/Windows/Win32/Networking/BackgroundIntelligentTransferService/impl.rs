pub trait AsyncIBackgroundCopyCallbackImpl: Sized {
    fn Begin_JobTransferred();
    fn Finish_JobTransferred();
    fn Begin_JobError();
    fn Finish_JobError();
    fn Begin_JobModification();
    fn Finish_JobModification();
}
impl AsyncIBackgroundCopyCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIBackgroundCopyCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIBackgroundCopyCallbackVtbl {
        unsafe extern "system" fn Begin_JobTransferred<Impl: AsyncIBackgroundCopyCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_JobTransferred<Impl: AsyncIBackgroundCopyCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_JobError<Impl: AsyncIBackgroundCopyCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr, perror: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_JobError<Impl: AsyncIBackgroundCopyCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_JobModification<Impl: AsyncIBackgroundCopyCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_JobModification<Impl: AsyncIBackgroundCopyCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Begin_JobTransferred::<Impl, IMPL_OFFSET>, Finish_JobTransferred::<Impl, IMPL_OFFSET>, Begin_JobError::<Impl, IMPL_OFFSET>, Finish_JobError::<Impl, IMPL_OFFSET>, Begin_JobModification::<Impl, IMPL_OFFSET>, Finish_JobModification::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIBackgroundCopyCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBITSExtensionSetupImpl: Sized + IDispatchImpl {
    fn EnableBITSUploads();
    fn DisableBITSUploads();
    fn GetCleanupTaskName();
    fn GetCleanupTask();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBITSExtensionSetupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBITSExtensionSetupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBITSExtensionSetupVtbl {
        unsafe extern "system" fn EnableBITSUploads<Impl: IBITSExtensionSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisableBITSUploads<Impl: IBITSExtensionSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCleanupTaskName<Impl: IBITSExtensionSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptaskname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCleanupTask<Impl: IBITSExtensionSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, EnableBITSUploads::<Impl, IMPL_OFFSET>, DisableBITSUploads::<Impl, IMPL_OFFSET>, GetCleanupTaskName::<Impl, IMPL_OFFSET>, GetCleanupTask::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBITSExtensionSetup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBITSExtensionSetupFactoryImpl: Sized + IDispatchImpl {
    fn GetObject();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBITSExtensionSetupFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBITSExtensionSetupFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBITSExtensionSetupFactoryVtbl {
        unsafe extern "system" fn GetObject<Impl: IBITSExtensionSetupFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppextensionsetup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, GetObject::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBITSExtensionSetupFactory as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundCopyCallbackImpl: Sized {
    fn JobTransferred();
    fn JobError();
    fn JobModification();
}
impl IBackgroundCopyCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyCallbackVtbl {
        unsafe extern "system" fn JobTransferred<Impl: IBackgroundCopyCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn JobError<Impl: IBackgroundCopyCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr, perror: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn JobModification<Impl: IBackgroundCopyCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, JobTransferred::<Impl, IMPL_OFFSET>, JobError::<Impl, IMPL_OFFSET>, JobModification::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyCallback as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundCopyCallback1Impl: Sized {
    fn OnStatus();
    fn OnProgress();
    fn OnProgressEx();
}
impl IBackgroundCopyCallback1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyCallback1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyCallback1Vtbl {
        unsafe extern "system" fn OnStatus<Impl: IBackgroundCopyCallback1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroup: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, dwfileindex: u32, dwstatus: u32, dwnumofretries: u32, dwwin32result: u32, dwtransportresult: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnProgress<Impl: IBackgroundCopyCallback1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progresstype: u32, pgroup: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, dwfileindex: u32, dwprogressvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnProgressEx<Impl: IBackgroundCopyCallback1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progresstype: u32, pgroup: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, dwfileindex: u32, dwprogressvalue: u32, dwbytearraysize: u32, pbyte: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnStatus::<Impl, IMPL_OFFSET>, OnProgress::<Impl, IMPL_OFFSET>, OnProgressEx::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyCallback1 as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundCopyCallback2Impl: Sized + IBackgroundCopyCallbackImpl {
    fn FileTransferred();
}
impl IBackgroundCopyCallback2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyCallback2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyCallback2Vtbl {
        unsafe extern "system" fn FileTransferred<Impl: IBackgroundCopyCallback2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjob: ::windows::core::RawPtr, pfile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, JobTransferred::<Impl, IMPL_OFFSET>, JobError::<Impl, IMPL_OFFSET>, JobModification::<Impl, IMPL_OFFSET>, FileTransferred::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyCallback2 as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundCopyCallback3Impl: Sized + IBackgroundCopyCallback2Impl + IBackgroundCopyCallbackImpl {
    fn FileRangesTransferred();
}
impl IBackgroundCopyCallback3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyCallback3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyCallback3Vtbl {
        unsafe extern "system" fn FileRangesTransferred<Impl: IBackgroundCopyCallback3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, job: ::windows::core::RawPtr, file: ::windows::core::RawPtr, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, JobTransferred::<Impl, IMPL_OFFSET>, JobError::<Impl, IMPL_OFFSET>, JobModification::<Impl, IMPL_OFFSET>, FileTransferred::<Impl, IMPL_OFFSET>, FileRangesTransferred::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyCallback3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyErrorImpl: Sized {
    fn GetError();
    fn GetFile();
    fn GetErrorDescription();
    fn GetErrorContextDescription();
    fn GetProtocol();
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyErrorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyErrorVtbl {
        unsafe extern "system" fn GetError<Impl: IBackgroundCopyErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut BG_ERROR_CONTEXT, pcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFile<Impl: IBackgroundCopyErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetErrorDescription<Impl: IBackgroundCopyErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageid: u32, perrordescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetErrorContextDescription<Impl: IBackgroundCopyErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageid: u32, pcontextdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProtocol<Impl: IBackgroundCopyErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocol: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetError::<Impl, IMPL_OFFSET>, GetFile::<Impl, IMPL_OFFSET>, GetErrorDescription::<Impl, IMPL_OFFSET>, GetErrorContextDescription::<Impl, IMPL_OFFSET>, GetProtocol::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyError as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFileImpl: Sized {
    fn GetRemoteName();
    fn GetLocalName();
    fn GetProgress();
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyFileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyFileVtbl {
        unsafe extern "system" fn GetRemoteName<Impl: IBackgroundCopyFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocalName<Impl: IBackgroundCopyFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProgress<Impl: IBackgroundCopyFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut BG_FILE_PROGRESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetRemoteName::<Impl, IMPL_OFFSET>, GetLocalName::<Impl, IMPL_OFFSET>, GetProgress::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyFile as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFile2Impl: Sized + IBackgroundCopyFileImpl {
    fn GetFileRanges();
    fn SetRemoteName();
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyFile2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyFile2Vtbl {
        unsafe extern "system" fn GetFileRanges<Impl: IBackgroundCopyFile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRemoteName<Impl: IBackgroundCopyFile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetRemoteName::<Impl, IMPL_OFFSET>, GetLocalName::<Impl, IMPL_OFFSET>, GetProgress::<Impl, IMPL_OFFSET>, GetFileRanges::<Impl, IMPL_OFFSET>, SetRemoteName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyFile2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFile3Impl: Sized + IBackgroundCopyFile2Impl + IBackgroundCopyFileImpl {
    fn GetTemporaryName();
    fn SetValidationState();
    fn GetValidationState();
    fn IsDownloadedFromPeer();
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyFile3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyFile3Vtbl {
        unsafe extern "system" fn GetTemporaryName<Impl: IBackgroundCopyFile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValidationState<Impl: IBackgroundCopyFile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValidationState<Impl: IBackgroundCopyFile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsDownloadedFromPeer<Impl: IBackgroundCopyFile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetRemoteName::<Impl, IMPL_OFFSET>,
            GetLocalName::<Impl, IMPL_OFFSET>,
            GetProgress::<Impl, IMPL_OFFSET>,
            GetFileRanges::<Impl, IMPL_OFFSET>,
            SetRemoteName::<Impl, IMPL_OFFSET>,
            GetTemporaryName::<Impl, IMPL_OFFSET>,
            SetValidationState::<Impl, IMPL_OFFSET>,
            GetValidationState::<Impl, IMPL_OFFSET>,
            IsDownloadedFromPeer::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyFile3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFile4Impl: Sized + IBackgroundCopyFile3Impl + IBackgroundCopyFile2Impl + IBackgroundCopyFileImpl {
    fn GetPeerDownloadStats();
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyFile4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyFile4Vtbl {
        unsafe extern "system" fn GetPeerDownloadStats<Impl: IBackgroundCopyFile4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfromorigin: *mut u64, pfrompeers: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetRemoteName::<Impl, IMPL_OFFSET>,
            GetLocalName::<Impl, IMPL_OFFSET>,
            GetProgress::<Impl, IMPL_OFFSET>,
            GetFileRanges::<Impl, IMPL_OFFSET>,
            SetRemoteName::<Impl, IMPL_OFFSET>,
            GetTemporaryName::<Impl, IMPL_OFFSET>,
            SetValidationState::<Impl, IMPL_OFFSET>,
            GetValidationState::<Impl, IMPL_OFFSET>,
            IsDownloadedFromPeer::<Impl, IMPL_OFFSET>,
            GetPeerDownloadStats::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyFile4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFile5Impl: Sized + IBackgroundCopyFile4Impl + IBackgroundCopyFile3Impl + IBackgroundCopyFile2Impl + IBackgroundCopyFileImpl {
    fn SetProperty();
    fn GetProperty();
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyFile5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyFile5Vtbl {
        unsafe extern "system" fn SetProperty<Impl: IBackgroundCopyFile5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: BITS_FILE_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: IBackgroundCopyFile5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: *mut BITS_FILE_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetRemoteName::<Impl, IMPL_OFFSET>,
            GetLocalName::<Impl, IMPL_OFFSET>,
            GetProgress::<Impl, IMPL_OFFSET>,
            GetFileRanges::<Impl, IMPL_OFFSET>,
            SetRemoteName::<Impl, IMPL_OFFSET>,
            GetTemporaryName::<Impl, IMPL_OFFSET>,
            SetValidationState::<Impl, IMPL_OFFSET>,
            GetValidationState::<Impl, IMPL_OFFSET>,
            IsDownloadedFromPeer::<Impl, IMPL_OFFSET>,
            GetPeerDownloadStats::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyFile5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFile6Impl: Sized + IBackgroundCopyFile5Impl + IBackgroundCopyFile4Impl + IBackgroundCopyFile3Impl + IBackgroundCopyFile2Impl + IBackgroundCopyFileImpl {
    fn UpdateDownloadPosition();
    fn RequestFileRanges();
    fn GetFilledFileRanges();
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyFile6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyFile6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyFile6Vtbl {
        unsafe extern "system" fn UpdateDownloadPosition<Impl: IBackgroundCopyFile6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestFileRanges<Impl: IBackgroundCopyFile6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilledFileRanges<Impl: IBackgroundCopyFile6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetRemoteName::<Impl, IMPL_OFFSET>,
            GetLocalName::<Impl, IMPL_OFFSET>,
            GetProgress::<Impl, IMPL_OFFSET>,
            GetFileRanges::<Impl, IMPL_OFFSET>,
            SetRemoteName::<Impl, IMPL_OFFSET>,
            GetTemporaryName::<Impl, IMPL_OFFSET>,
            SetValidationState::<Impl, IMPL_OFFSET>,
            GetValidationState::<Impl, IMPL_OFFSET>,
            IsDownloadedFromPeer::<Impl, IMPL_OFFSET>,
            GetPeerDownloadStats::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            UpdateDownloadPosition::<Impl, IMPL_OFFSET>,
            RequestFileRanges::<Impl, IMPL_OFFSET>,
            GetFilledFileRanges::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyFile6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBackgroundCopyGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyGroupVtbl {
        unsafe extern "system" fn GetProp<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: GROUPPROP, pvarval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProp<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: GROUPPROP, pvarval: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProgress<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdwprogress: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatus<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32, pdwjobindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetJob<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobid: ::windows::core::GUID, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SuspendGroup<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResumeGroup<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelGroup<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Size<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GroupID<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidgroupid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateJob<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidjobid: ::windows::core::GUID, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumJobs<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppenumjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SwitchToForeground<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryNewJobInterface<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNotificationPointer<Impl: IBackgroundCopyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetProp::<Impl, IMPL_OFFSET>,
            SetProp::<Impl, IMPL_OFFSET>,
            GetProgress::<Impl, IMPL_OFFSET>,
            GetStatus::<Impl, IMPL_OFFSET>,
            GetJob::<Impl, IMPL_OFFSET>,
            SuspendGroup::<Impl, IMPL_OFFSET>,
            ResumeGroup::<Impl, IMPL_OFFSET>,
            CancelGroup::<Impl, IMPL_OFFSET>,
            Size::<Impl, IMPL_OFFSET>,
            GroupID::<Impl, IMPL_OFFSET>,
            CreateJob::<Impl, IMPL_OFFSET>,
            EnumJobs::<Impl, IMPL_OFFSET>,
            SwitchToForeground::<Impl, IMPL_OFFSET>,
            QueryNewJobInterface::<Impl, IMPL_OFFSET>,
            SetNotificationPointer::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyJobVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJobImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyJobVtbl {
        unsafe extern "system" fn AddFileSet<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddFile<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteurl: super::super::Foundation::PWSTR, localname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumFiles<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Suspend<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Resume<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Complete<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetId<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetType<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProgress<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_PROGRESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTimes<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_TIMES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetState<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetError<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOwner<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisplayName<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayName<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDescription<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPriority<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: BG_JOB_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPriority<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNotifyFlags<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNotifyFlags<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNotifyInterface<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNotifyInterface<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMinimumRetryDelay<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMinimumRetryDelay<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNoProgressTimeout<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNoProgressTimeout<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetErrorCount<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errors: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProxySettings<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, proxyusage: BG_JOB_PROXY_USAGE, proxylist: super::super::Foundation::PWSTR, proxybypasslist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProxySettings<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut super::super::Foundation::PWSTR, pproxybypasslist: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TakeOwnership<Impl: IBackgroundCopyJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            AddFileSet::<Impl, IMPL_OFFSET>,
            AddFile::<Impl, IMPL_OFFSET>,
            EnumFiles::<Impl, IMPL_OFFSET>,
            Suspend::<Impl, IMPL_OFFSET>,
            Resume::<Impl, IMPL_OFFSET>,
            Cancel::<Impl, IMPL_OFFSET>,
            Complete::<Impl, IMPL_OFFSET>,
            GetId::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetProgress::<Impl, IMPL_OFFSET>,
            GetTimes::<Impl, IMPL_OFFSET>,
            GetState::<Impl, IMPL_OFFSET>,
            GetError::<Impl, IMPL_OFFSET>,
            GetOwner::<Impl, IMPL_OFFSET>,
            SetDisplayName::<Impl, IMPL_OFFSET>,
            GetDisplayName::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            GetDescription::<Impl, IMPL_OFFSET>,
            SetPriority::<Impl, IMPL_OFFSET>,
            GetPriority::<Impl, IMPL_OFFSET>,
            SetNotifyFlags::<Impl, IMPL_OFFSET>,
            GetNotifyFlags::<Impl, IMPL_OFFSET>,
            SetNotifyInterface::<Impl, IMPL_OFFSET>,
            GetNotifyInterface::<Impl, IMPL_OFFSET>,
            SetMinimumRetryDelay::<Impl, IMPL_OFFSET>,
            GetMinimumRetryDelay::<Impl, IMPL_OFFSET>,
            SetNoProgressTimeout::<Impl, IMPL_OFFSET>,
            GetNoProgressTimeout::<Impl, IMPL_OFFSET>,
            GetErrorCount::<Impl, IMPL_OFFSET>,
            SetProxySettings::<Impl, IMPL_OFFSET>,
            GetProxySettings::<Impl, IMPL_OFFSET>,
            TakeOwnership::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJob as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyJob1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyJob1Vtbl {
        unsafe extern "system" fn CancelJob<Impl: IBackgroundCopyJob1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProgress<Impl: IBackgroundCopyJob1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdwprogress: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatus<Impl: IBackgroundCopyJob1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32, pdwwin32result: *mut u32, pdwtransportresult: *mut u32, pdwnumofretries: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddFiles<Impl: IBackgroundCopyJob1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfilecount: u32, ppfileset: *const *const FILESETINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFile<Impl: IBackgroundCopyJob1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfileindex: u32, pfileinfo: *mut FILESETINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileCount<Impl: IBackgroundCopyJob1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfilecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SwitchToForeground<Impl: IBackgroundCopyJob1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn JobID<Impl: IBackgroundCopyJob1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidjobid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CancelJob::<Impl, IMPL_OFFSET>, GetProgress::<Impl, IMPL_OFFSET>, GetStatus::<Impl, IMPL_OFFSET>, AddFiles::<Impl, IMPL_OFFSET>, GetFile::<Impl, IMPL_OFFSET>, GetFileCount::<Impl, IMPL_OFFSET>, SwitchToForeground::<Impl, IMPL_OFFSET>, JobID::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJob1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyJob2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyJob2Vtbl {
        unsafe extern "system" fn SetNotifyCmdLine<Impl: IBackgroundCopyJob2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, program: super::super::Foundation::PWSTR, parameters: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNotifyCmdLine<Impl: IBackgroundCopyJob2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprogram: *mut super::super::Foundation::PWSTR, pparameters: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReplyProgress<Impl: IBackgroundCopyJob2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReplyData<Impl: IBackgroundCopyJob2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbuffer: *mut *mut u8, plength: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReplyFileName<Impl: IBackgroundCopyJob2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, replyfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReplyFileName<Impl: IBackgroundCopyJob2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preplyfilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCredentials<Impl: IBackgroundCopyJob2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credentials: *const BG_AUTH_CREDENTIALS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveCredentials<Impl: IBackgroundCopyJob2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            AddFileSet::<Impl, IMPL_OFFSET>,
            AddFile::<Impl, IMPL_OFFSET>,
            EnumFiles::<Impl, IMPL_OFFSET>,
            Suspend::<Impl, IMPL_OFFSET>,
            Resume::<Impl, IMPL_OFFSET>,
            Cancel::<Impl, IMPL_OFFSET>,
            Complete::<Impl, IMPL_OFFSET>,
            GetId::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetProgress::<Impl, IMPL_OFFSET>,
            GetTimes::<Impl, IMPL_OFFSET>,
            GetState::<Impl, IMPL_OFFSET>,
            GetError::<Impl, IMPL_OFFSET>,
            GetOwner::<Impl, IMPL_OFFSET>,
            SetDisplayName::<Impl, IMPL_OFFSET>,
            GetDisplayName::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            GetDescription::<Impl, IMPL_OFFSET>,
            SetPriority::<Impl, IMPL_OFFSET>,
            GetPriority::<Impl, IMPL_OFFSET>,
            SetNotifyFlags::<Impl, IMPL_OFFSET>,
            GetNotifyFlags::<Impl, IMPL_OFFSET>,
            SetNotifyInterface::<Impl, IMPL_OFFSET>,
            GetNotifyInterface::<Impl, IMPL_OFFSET>,
            SetMinimumRetryDelay::<Impl, IMPL_OFFSET>,
            GetMinimumRetryDelay::<Impl, IMPL_OFFSET>,
            SetNoProgressTimeout::<Impl, IMPL_OFFSET>,
            GetNoProgressTimeout::<Impl, IMPL_OFFSET>,
            GetErrorCount::<Impl, IMPL_OFFSET>,
            SetProxySettings::<Impl, IMPL_OFFSET>,
            GetProxySettings::<Impl, IMPL_OFFSET>,
            TakeOwnership::<Impl, IMPL_OFFSET>,
            SetNotifyCmdLine::<Impl, IMPL_OFFSET>,
            GetNotifyCmdLine::<Impl, IMPL_OFFSET>,
            GetReplyProgress::<Impl, IMPL_OFFSET>,
            GetReplyData::<Impl, IMPL_OFFSET>,
            SetReplyFileName::<Impl, IMPL_OFFSET>,
            GetReplyFileName::<Impl, IMPL_OFFSET>,
            SetCredentials::<Impl, IMPL_OFFSET>,
            RemoveCredentials::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJob2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJob3Impl: Sized + IBackgroundCopyJob2Impl + IBackgroundCopyJobImpl {
    fn ReplaceRemotePrefix();
    fn AddFileWithRanges();
    fn SetFileACLFlags();
    fn GetFileACLFlags();
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyJob3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyJob3Vtbl {
        unsafe extern "system" fn ReplaceRemotePrefix<Impl: IBackgroundCopyJob3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldprefix: super::super::Foundation::PWSTR, newprefix: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddFileWithRanges<Impl: IBackgroundCopyJob3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteurl: super::super::Foundation::PWSTR, localname: super::super::Foundation::PWSTR, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFileACLFlags<Impl: IBackgroundCopyJob3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileACLFlags<Impl: IBackgroundCopyJob3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            AddFileSet::<Impl, IMPL_OFFSET>,
            AddFile::<Impl, IMPL_OFFSET>,
            EnumFiles::<Impl, IMPL_OFFSET>,
            Suspend::<Impl, IMPL_OFFSET>,
            Resume::<Impl, IMPL_OFFSET>,
            Cancel::<Impl, IMPL_OFFSET>,
            Complete::<Impl, IMPL_OFFSET>,
            GetId::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetProgress::<Impl, IMPL_OFFSET>,
            GetTimes::<Impl, IMPL_OFFSET>,
            GetState::<Impl, IMPL_OFFSET>,
            GetError::<Impl, IMPL_OFFSET>,
            GetOwner::<Impl, IMPL_OFFSET>,
            SetDisplayName::<Impl, IMPL_OFFSET>,
            GetDisplayName::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            GetDescription::<Impl, IMPL_OFFSET>,
            SetPriority::<Impl, IMPL_OFFSET>,
            GetPriority::<Impl, IMPL_OFFSET>,
            SetNotifyFlags::<Impl, IMPL_OFFSET>,
            GetNotifyFlags::<Impl, IMPL_OFFSET>,
            SetNotifyInterface::<Impl, IMPL_OFFSET>,
            GetNotifyInterface::<Impl, IMPL_OFFSET>,
            SetMinimumRetryDelay::<Impl, IMPL_OFFSET>,
            GetMinimumRetryDelay::<Impl, IMPL_OFFSET>,
            SetNoProgressTimeout::<Impl, IMPL_OFFSET>,
            GetNoProgressTimeout::<Impl, IMPL_OFFSET>,
            GetErrorCount::<Impl, IMPL_OFFSET>,
            SetProxySettings::<Impl, IMPL_OFFSET>,
            GetProxySettings::<Impl, IMPL_OFFSET>,
            TakeOwnership::<Impl, IMPL_OFFSET>,
            SetNotifyCmdLine::<Impl, IMPL_OFFSET>,
            GetNotifyCmdLine::<Impl, IMPL_OFFSET>,
            GetReplyProgress::<Impl, IMPL_OFFSET>,
            GetReplyData::<Impl, IMPL_OFFSET>,
            SetReplyFileName::<Impl, IMPL_OFFSET>,
            GetReplyFileName::<Impl, IMPL_OFFSET>,
            SetCredentials::<Impl, IMPL_OFFSET>,
            RemoveCredentials::<Impl, IMPL_OFFSET>,
            ReplaceRemotePrefix::<Impl, IMPL_OFFSET>,
            AddFileWithRanges::<Impl, IMPL_OFFSET>,
            SetFileACLFlags::<Impl, IMPL_OFFSET>,
            GetFileACLFlags::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJob3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJob4Impl: Sized + IBackgroundCopyJob3Impl + IBackgroundCopyJob2Impl + IBackgroundCopyJobImpl {
    fn SetPeerCachingFlags();
    fn GetPeerCachingFlags();
    fn GetOwnerIntegrityLevel();
    fn GetOwnerElevationState();
    fn SetMaximumDownloadTime();
    fn GetMaximumDownloadTime();
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyJob4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyJob4Vtbl {
        unsafe extern "system" fn SetPeerCachingFlags<Impl: IBackgroundCopyJob4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPeerCachingFlags<Impl: IBackgroundCopyJob4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOwnerIntegrityLevel<Impl: IBackgroundCopyJob4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOwnerElevationState<Impl: IBackgroundCopyJob4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pelevated: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaximumDownloadTime<Impl: IBackgroundCopyJob4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaximumDownloadTime<Impl: IBackgroundCopyJob4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            AddFileSet::<Impl, IMPL_OFFSET>,
            AddFile::<Impl, IMPL_OFFSET>,
            EnumFiles::<Impl, IMPL_OFFSET>,
            Suspend::<Impl, IMPL_OFFSET>,
            Resume::<Impl, IMPL_OFFSET>,
            Cancel::<Impl, IMPL_OFFSET>,
            Complete::<Impl, IMPL_OFFSET>,
            GetId::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetProgress::<Impl, IMPL_OFFSET>,
            GetTimes::<Impl, IMPL_OFFSET>,
            GetState::<Impl, IMPL_OFFSET>,
            GetError::<Impl, IMPL_OFFSET>,
            GetOwner::<Impl, IMPL_OFFSET>,
            SetDisplayName::<Impl, IMPL_OFFSET>,
            GetDisplayName::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            GetDescription::<Impl, IMPL_OFFSET>,
            SetPriority::<Impl, IMPL_OFFSET>,
            GetPriority::<Impl, IMPL_OFFSET>,
            SetNotifyFlags::<Impl, IMPL_OFFSET>,
            GetNotifyFlags::<Impl, IMPL_OFFSET>,
            SetNotifyInterface::<Impl, IMPL_OFFSET>,
            GetNotifyInterface::<Impl, IMPL_OFFSET>,
            SetMinimumRetryDelay::<Impl, IMPL_OFFSET>,
            GetMinimumRetryDelay::<Impl, IMPL_OFFSET>,
            SetNoProgressTimeout::<Impl, IMPL_OFFSET>,
            GetNoProgressTimeout::<Impl, IMPL_OFFSET>,
            GetErrorCount::<Impl, IMPL_OFFSET>,
            SetProxySettings::<Impl, IMPL_OFFSET>,
            GetProxySettings::<Impl, IMPL_OFFSET>,
            TakeOwnership::<Impl, IMPL_OFFSET>,
            SetNotifyCmdLine::<Impl, IMPL_OFFSET>,
            GetNotifyCmdLine::<Impl, IMPL_OFFSET>,
            GetReplyProgress::<Impl, IMPL_OFFSET>,
            GetReplyData::<Impl, IMPL_OFFSET>,
            SetReplyFileName::<Impl, IMPL_OFFSET>,
            GetReplyFileName::<Impl, IMPL_OFFSET>,
            SetCredentials::<Impl, IMPL_OFFSET>,
            RemoveCredentials::<Impl, IMPL_OFFSET>,
            ReplaceRemotePrefix::<Impl, IMPL_OFFSET>,
            AddFileWithRanges::<Impl, IMPL_OFFSET>,
            SetFileACLFlags::<Impl, IMPL_OFFSET>,
            GetFileACLFlags::<Impl, IMPL_OFFSET>,
            SetPeerCachingFlags::<Impl, IMPL_OFFSET>,
            GetPeerCachingFlags::<Impl, IMPL_OFFSET>,
            GetOwnerIntegrityLevel::<Impl, IMPL_OFFSET>,
            GetOwnerElevationState::<Impl, IMPL_OFFSET>,
            SetMaximumDownloadTime::<Impl, IMPL_OFFSET>,
            GetMaximumDownloadTime::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJob4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJob5Impl: Sized + IBackgroundCopyJob4Impl + IBackgroundCopyJob3Impl + IBackgroundCopyJob2Impl + IBackgroundCopyJobImpl {
    fn SetProperty();
    fn GetProperty();
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyJob5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJob5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyJob5Vtbl {
        unsafe extern "system" fn SetProperty<Impl: IBackgroundCopyJob5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: BITS_JOB_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: IBackgroundCopyJob5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: *mut BITS_JOB_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            AddFileSet::<Impl, IMPL_OFFSET>,
            AddFile::<Impl, IMPL_OFFSET>,
            EnumFiles::<Impl, IMPL_OFFSET>,
            Suspend::<Impl, IMPL_OFFSET>,
            Resume::<Impl, IMPL_OFFSET>,
            Cancel::<Impl, IMPL_OFFSET>,
            Complete::<Impl, IMPL_OFFSET>,
            GetId::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetProgress::<Impl, IMPL_OFFSET>,
            GetTimes::<Impl, IMPL_OFFSET>,
            GetState::<Impl, IMPL_OFFSET>,
            GetError::<Impl, IMPL_OFFSET>,
            GetOwner::<Impl, IMPL_OFFSET>,
            SetDisplayName::<Impl, IMPL_OFFSET>,
            GetDisplayName::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            GetDescription::<Impl, IMPL_OFFSET>,
            SetPriority::<Impl, IMPL_OFFSET>,
            GetPriority::<Impl, IMPL_OFFSET>,
            SetNotifyFlags::<Impl, IMPL_OFFSET>,
            GetNotifyFlags::<Impl, IMPL_OFFSET>,
            SetNotifyInterface::<Impl, IMPL_OFFSET>,
            GetNotifyInterface::<Impl, IMPL_OFFSET>,
            SetMinimumRetryDelay::<Impl, IMPL_OFFSET>,
            GetMinimumRetryDelay::<Impl, IMPL_OFFSET>,
            SetNoProgressTimeout::<Impl, IMPL_OFFSET>,
            GetNoProgressTimeout::<Impl, IMPL_OFFSET>,
            GetErrorCount::<Impl, IMPL_OFFSET>,
            SetProxySettings::<Impl, IMPL_OFFSET>,
            GetProxySettings::<Impl, IMPL_OFFSET>,
            TakeOwnership::<Impl, IMPL_OFFSET>,
            SetNotifyCmdLine::<Impl, IMPL_OFFSET>,
            GetNotifyCmdLine::<Impl, IMPL_OFFSET>,
            GetReplyProgress::<Impl, IMPL_OFFSET>,
            GetReplyData::<Impl, IMPL_OFFSET>,
            SetReplyFileName::<Impl, IMPL_OFFSET>,
            GetReplyFileName::<Impl, IMPL_OFFSET>,
            SetCredentials::<Impl, IMPL_OFFSET>,
            RemoveCredentials::<Impl, IMPL_OFFSET>,
            ReplaceRemotePrefix::<Impl, IMPL_OFFSET>,
            AddFileWithRanges::<Impl, IMPL_OFFSET>,
            SetFileACLFlags::<Impl, IMPL_OFFSET>,
            GetFileACLFlags::<Impl, IMPL_OFFSET>,
            SetPeerCachingFlags::<Impl, IMPL_OFFSET>,
            GetPeerCachingFlags::<Impl, IMPL_OFFSET>,
            GetOwnerIntegrityLevel::<Impl, IMPL_OFFSET>,
            GetOwnerElevationState::<Impl, IMPL_OFFSET>,
            SetMaximumDownloadTime::<Impl, IMPL_OFFSET>,
            GetMaximumDownloadTime::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJob5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyJobHttpOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJobHttpOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyJobHttpOptionsVtbl {
        unsafe extern "system" fn SetClientCertificateByID<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storelocation: BG_CERT_STORE_LOCATION, storename: super::super::Foundation::PWSTR, pcerthashblob: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClientCertificateByName<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storelocation: BG_CERT_STORE_LOCATION, storename: super::super::Foundation::PWSTR, subjectname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveClientCertificate<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClientCertificate<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut super::super::Foundation::PWSTR, ppcerthashblob: *mut *mut u8, psubjectname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCustomHeaders<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestheaders: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCustomHeaders<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequestheaders: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSecurityFlags<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSecurityFlags<Impl: IBackgroundCopyJobHttpOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetClientCertificateByID::<Impl, IMPL_OFFSET>,
            SetClientCertificateByName::<Impl, IMPL_OFFSET>,
            RemoveClientCertificate::<Impl, IMPL_OFFSET>,
            GetClientCertificate::<Impl, IMPL_OFFSET>,
            SetCustomHeaders::<Impl, IMPL_OFFSET>,
            GetCustomHeaders::<Impl, IMPL_OFFSET>,
            SetSecurityFlags::<Impl, IMPL_OFFSET>,
            GetSecurityFlags::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJobHttpOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJobHttpOptions2Impl: Sized + IBackgroundCopyJobHttpOptionsImpl {
    fn SetHttpMethod();
    fn GetHttpMethod();
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyJobHttpOptions2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJobHttpOptions2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyJobHttpOptions2Vtbl {
        unsafe extern "system" fn SetHttpMethod<Impl: IBackgroundCopyJobHttpOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHttpMethod<Impl: IBackgroundCopyJobHttpOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetClientCertificateByID::<Impl, IMPL_OFFSET>,
            SetClientCertificateByName::<Impl, IMPL_OFFSET>,
            RemoveClientCertificate::<Impl, IMPL_OFFSET>,
            GetClientCertificate::<Impl, IMPL_OFFSET>,
            SetCustomHeaders::<Impl, IMPL_OFFSET>,
            GetCustomHeaders::<Impl, IMPL_OFFSET>,
            SetSecurityFlags::<Impl, IMPL_OFFSET>,
            GetSecurityFlags::<Impl, IMPL_OFFSET>,
            SetHttpMethod::<Impl, IMPL_OFFSET>,
            GetHttpMethod::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJobHttpOptions2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJobHttpOptions3Impl: Sized + IBackgroundCopyJobHttpOptions2Impl + IBackgroundCopyJobHttpOptionsImpl {
    fn SetServerCertificateValidationInterface();
    fn MakeCustomHeadersWriteOnly();
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyJobHttpOptions3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyJobHttpOptions3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyJobHttpOptions3Vtbl {
        unsafe extern "system" fn SetServerCertificateValidationInterface<Impl: IBackgroundCopyJobHttpOptions3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certvalidationcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MakeCustomHeadersWriteOnly<Impl: IBackgroundCopyJobHttpOptions3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetClientCertificateByID::<Impl, IMPL_OFFSET>,
            SetClientCertificateByName::<Impl, IMPL_OFFSET>,
            RemoveClientCertificate::<Impl, IMPL_OFFSET>,
            GetClientCertificate::<Impl, IMPL_OFFSET>,
            SetCustomHeaders::<Impl, IMPL_OFFSET>,
            GetCustomHeaders::<Impl, IMPL_OFFSET>,
            SetSecurityFlags::<Impl, IMPL_OFFSET>,
            GetSecurityFlags::<Impl, IMPL_OFFSET>,
            SetHttpMethod::<Impl, IMPL_OFFSET>,
            GetHttpMethod::<Impl, IMPL_OFFSET>,
            SetServerCertificateValidationInterface::<Impl, IMPL_OFFSET>,
            MakeCustomHeadersWriteOnly::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyJobHttpOptions3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyManagerImpl: Sized {
    fn CreateJob();
    fn GetJob();
    fn EnumJobs();
    fn GetErrorDescription();
}
#[cfg(feature = "Win32_Foundation")]
impl IBackgroundCopyManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyManagerVtbl {
        unsafe extern "system" fn CreateJob<Impl: IBackgroundCopyManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: super::super::Foundation::PWSTR, r#type: BG_JOB_TYPE, pjobid: *mut ::windows::core::GUID, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetJob<Impl: IBackgroundCopyManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobid: *const ::windows::core::GUID, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumJobs<Impl: IBackgroundCopyManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetErrorDescription<Impl: IBackgroundCopyManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, languageid: u32, perrordescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateJob::<Impl, IMPL_OFFSET>, GetJob::<Impl, IMPL_OFFSET>, EnumJobs::<Impl, IMPL_OFFSET>, GetErrorDescription::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyManager as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundCopyQMgrImpl: Sized {
    fn CreateGroup();
    fn GetGroup();
    fn EnumGroups();
}
impl IBackgroundCopyQMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyQMgrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyQMgrVtbl {
        unsafe extern "system" fn CreateGroup<Impl: IBackgroundCopyQMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidgroupid: ::windows::core::GUID, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGroup<Impl: IBackgroundCopyQMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, groupid: ::windows::core::GUID, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumGroups<Impl: IBackgroundCopyQMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppenumgroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateGroup::<Impl, IMPL_OFFSET>, GetGroup::<Impl, IMPL_OFFSET>, EnumGroups::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyQMgr as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundCopyServerCertificateValidationCallbackImpl: Sized {
    fn ValidateServerCertificate();
}
impl IBackgroundCopyServerCertificateValidationCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCopyServerCertificateValidationCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCopyServerCertificateValidationCallbackVtbl {
        unsafe extern "system" fn ValidateServerCertificate<Impl: IBackgroundCopyServerCertificateValidationCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, job: ::windows::core::RawPtr, file: ::windows::core::RawPtr, certlength: u32, certdata: *const u8, certencodingtype: u32, certstorelength: u32, certstoredata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ValidateServerCertificate::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCopyServerCertificateValidationCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBitsPeerImpl: Sized {
    fn GetPeerName();
    fn IsAuthenticated();
    fn IsAvailable();
}
#[cfg(feature = "Win32_Foundation")]
impl IBitsPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitsPeerVtbl {
        unsafe extern "system" fn GetPeerName<Impl: IBitsPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsAuthenticated<Impl: IBitsPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauth: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsAvailable<Impl: IBitsPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ponline: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPeerName::<Impl, IMPL_OFFSET>, IsAuthenticated::<Impl, IMPL_OFFSET>, IsAvailable::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitsPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IBitsPeerCacheAdministrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheAdministrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitsPeerCacheAdministrationVtbl {
        unsafe extern "system" fn GetMaximumCacheSize<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbytes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaximumCacheSize<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bytes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaximumContentAge<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pseconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaximumContentAge<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConfigurationFlags<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConfigurationFlags<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumRecords<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRecord<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *const ::windows::core::GUID, pprecord: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearRecords<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteRecord<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteUrl<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, url: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumPeers<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearPeers<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DiscoverPeers<Impl: IBitsPeerCacheAdministrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetMaximumCacheSize::<Impl, IMPL_OFFSET>,
            SetMaximumCacheSize::<Impl, IMPL_OFFSET>,
            GetMaximumContentAge::<Impl, IMPL_OFFSET>,
            SetMaximumContentAge::<Impl, IMPL_OFFSET>,
            GetConfigurationFlags::<Impl, IMPL_OFFSET>,
            SetConfigurationFlags::<Impl, IMPL_OFFSET>,
            EnumRecords::<Impl, IMPL_OFFSET>,
            GetRecord::<Impl, IMPL_OFFSET>,
            ClearRecords::<Impl, IMPL_OFFSET>,
            DeleteRecord::<Impl, IMPL_OFFSET>,
            DeleteUrl::<Impl, IMPL_OFFSET>,
            EnumPeers::<Impl, IMPL_OFFSET>,
            ClearPeers::<Impl, IMPL_OFFSET>,
            DiscoverPeers::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitsPeerCacheAdministration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBitsPeerCacheRecordImpl: Sized {
    fn GetId();
    fn GetOriginUrl();
    fn GetFileSize();
    fn GetFileModificationTime();
    fn GetLastAccessTime();
    fn IsFileValidated();
    fn GetFileRanges();
}
#[cfg(feature = "Win32_Foundation")]
impl IBitsPeerCacheRecordVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitsPeerCacheRecordImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitsPeerCacheRecordVtbl {
        unsafe extern "system" fn GetId<Impl: IBitsPeerCacheRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOriginUrl<Impl: IBitsPeerCacheRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileSize<Impl: IBitsPeerCacheRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileModificationTime<Impl: IBitsPeerCacheRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLastAccessTime<Impl: IBitsPeerCacheRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsFileValidated<Impl: IBitsPeerCacheRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileRanges<Impl: IBitsPeerCacheRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangecount: *mut u32, ppranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetId::<Impl, IMPL_OFFSET>, GetOriginUrl::<Impl, IMPL_OFFSET>, GetFileSize::<Impl, IMPL_OFFSET>, GetFileModificationTime::<Impl, IMPL_OFFSET>, GetLastAccessTime::<Impl, IMPL_OFFSET>, IsFileValidated::<Impl, IMPL_OFFSET>, GetFileRanges::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitsPeerCacheRecord as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBitsTokenOptionsImpl: Sized {
    fn SetHelperTokenFlags();
    fn GetHelperTokenFlags();
    fn SetHelperToken();
    fn ClearHelperToken();
    fn GetHelperTokenSid();
}
#[cfg(feature = "Win32_Foundation")]
impl IBitsTokenOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitsTokenOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitsTokenOptionsVtbl {
        unsafe extern "system" fn SetHelperTokenFlags<Impl: IBitsTokenOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usageflags: BG_TOKEN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHelperTokenFlags<Impl: IBitsTokenOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut BG_TOKEN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHelperToken<Impl: IBitsTokenOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearHelperToken<Impl: IBitsTokenOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHelperTokenSid<Impl: IBitsTokenOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetHelperTokenFlags::<Impl, IMPL_OFFSET>, GetHelperTokenFlags::<Impl, IMPL_OFFSET>, SetHelperToken::<Impl, IMPL_OFFSET>, ClearHelperToken::<Impl, IMPL_OFFSET>, GetHelperTokenSid::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitsTokenOptions as ::windows::core::Interface>::IID
    }
}
pub trait IEnumBackgroundCopyFilesImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
impl IEnumBackgroundCopyFilesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyFilesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumBackgroundCopyFilesVtbl {
        unsafe extern "system" fn Next<Impl: IEnumBackgroundCopyFilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumBackgroundCopyFilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumBackgroundCopyFilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumBackgroundCopyFilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: IEnumBackgroundCopyFilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>, GetCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumBackgroundCopyFiles as ::windows::core::Interface>::IID
    }
}
pub trait IEnumBackgroundCopyGroupsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
impl IEnumBackgroundCopyGroupsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyGroupsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumBackgroundCopyGroupsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumBackgroundCopyGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumBackgroundCopyGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumBackgroundCopyGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumBackgroundCopyGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: IEnumBackgroundCopyGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>, GetCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumBackgroundCopyGroups as ::windows::core::Interface>::IID
    }
}
pub trait IEnumBackgroundCopyJobsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
impl IEnumBackgroundCopyJobsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyJobsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumBackgroundCopyJobsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumBackgroundCopyJobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumBackgroundCopyJobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumBackgroundCopyJobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumBackgroundCopyJobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: IEnumBackgroundCopyJobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>, GetCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumBackgroundCopyJobs as ::windows::core::Interface>::IID
    }
}
pub trait IEnumBackgroundCopyJobs1Impl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
impl IEnumBackgroundCopyJobs1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBackgroundCopyJobs1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumBackgroundCopyJobs1Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumBackgroundCopyJobs1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumBackgroundCopyJobs1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumBackgroundCopyJobs1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumBackgroundCopyJobs1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: IEnumBackgroundCopyJobs1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>, GetCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumBackgroundCopyJobs1 as ::windows::core::Interface>::IID
    }
}
pub trait IEnumBitsPeerCacheRecordsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
impl IEnumBitsPeerCacheRecordsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBitsPeerCacheRecordsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumBitsPeerCacheRecordsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumBitsPeerCacheRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumBitsPeerCacheRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumBitsPeerCacheRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumBitsPeerCacheRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: IEnumBitsPeerCacheRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>, GetCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumBitsPeerCacheRecords as ::windows::core::Interface>::IID
    }
}
pub trait IEnumBitsPeersImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
impl IEnumBitsPeersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBitsPeersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumBitsPeersVtbl {
        unsafe extern "system" fn Next<Impl: IEnumBitsPeersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumBitsPeersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumBitsPeersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumBitsPeersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: IEnumBitsPeersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>, GetCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumBitsPeers as ::windows::core::Interface>::IID
    }
}
