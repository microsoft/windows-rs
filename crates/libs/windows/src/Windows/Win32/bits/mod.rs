windows_core::imp::define_interface!(AsyncIBackgroundCopyCallback, AsyncIBackgroundCopyCallback_Vtbl, 0xca29d251_b4bb_4679_a3d9_ae8006119d54);
windows_core::imp::interface_hierarchy!(AsyncIBackgroundCopyCallback, windows_core::IUnknown);
impl AsyncIBackgroundCopyCallback {
    pub unsafe fn Begin_JobTransferred<P0>(&self, pjob: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
    {
        unsafe { (windows_core::Interface::vtable(self).Begin_JobTransferred)(windows_core::Interface::as_raw(self), pjob.param().abi()) }
    }
    pub unsafe fn Finish_JobTransferred(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finish_JobTransferred)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Begin_JobError<P0, P1>(&self, pjob: P0, perror: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
        P1: windows_core::Param<IBackgroundCopyError>,
    {
        unsafe { (windows_core::Interface::vtable(self).Begin_JobError)(windows_core::Interface::as_raw(self), pjob.param().abi(), perror.param().abi()) }
    }
    pub unsafe fn Finish_JobError(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finish_JobError)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Begin_JobModification<P0>(&self, pjob: P0, dwreserved: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
    {
        unsafe { (windows_core::Interface::vtable(self).Begin_JobModification)(windows_core::Interface::as_raw(self), pjob.param().abi(), dwreserved) }
    }
    pub unsafe fn Finish_JobModification(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finish_JobModification)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIBackgroundCopyCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_JobTransferred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish_JobTransferred: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Begin_JobError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish_JobError: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Begin_JobModification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Finish_JobModification: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait AsyncIBackgroundCopyCallback_Impl: windows_core::IUnknownImpl {
    fn Begin_JobTransferred(&self, pjob: windows_core::Ref<IBackgroundCopyJob>) -> windows_core::Result<()>;
    fn Finish_JobTransferred(&self) -> windows_core::Result<()>;
    fn Begin_JobError(&self, pjob: windows_core::Ref<IBackgroundCopyJob>, perror: windows_core::Ref<IBackgroundCopyError>) -> windows_core::Result<()>;
    fn Finish_JobError(&self) -> windows_core::Result<()>;
    fn Begin_JobModification(&self, pjob: windows_core::Ref<IBackgroundCopyJob>, dwreserved: u32) -> windows_core::Result<()>;
    fn Finish_JobModification(&self) -> windows_core::Result<()>;
}
impl AsyncIBackgroundCopyCallback_Vtbl {
    pub const fn new<Identity: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_JobTransferred<Identity: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIBackgroundCopyCallback_Impl::Begin_JobTransferred(this, core::mem::transmute_copy(&pjob)).into()
            }
        }
        unsafe extern "system" fn Finish_JobTransferred<Identity: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIBackgroundCopyCallback_Impl::Finish_JobTransferred(this).into()
            }
        }
        unsafe extern "system" fn Begin_JobError<Identity: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, perror: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIBackgroundCopyCallback_Impl::Begin_JobError(this, core::mem::transmute_copy(&pjob), core::mem::transmute_copy(&perror)).into()
            }
        }
        unsafe extern "system" fn Finish_JobError<Identity: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIBackgroundCopyCallback_Impl::Finish_JobError(this).into()
            }
        }
        unsafe extern "system" fn Begin_JobModification<Identity: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIBackgroundCopyCallback_Impl::Begin_JobModification(this, core::mem::transmute_copy(&pjob), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn Finish_JobModification<Identity: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIBackgroundCopyCallback_Impl::Finish_JobModification(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_JobTransferred: Begin_JobTransferred::<Identity, OFFSET>,
            Finish_JobTransferred: Finish_JobTransferred::<Identity, OFFSET>,
            Begin_JobError: Begin_JobError::<Identity, OFFSET>,
            Finish_JobError: Finish_JobError::<Identity, OFFSET>,
            Begin_JobModification: Begin_JobModification::<Identity, OFFSET>,
            Finish_JobModification: Finish_JobModification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIBackgroundCopyCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for AsyncIBackgroundCopyCallback {}
pub type BG_ERROR_CONTEXT = i32;
pub const BG_ERROR_CONTEXT_GENERAL_QUEUE_MANAGER: BG_ERROR_CONTEXT = 2;
pub const BG_ERROR_CONTEXT_GENERAL_TRANSPORT: BG_ERROR_CONTEXT = 6;
pub const BG_ERROR_CONTEXT_LOCAL_FILE: BG_ERROR_CONTEXT = 4;
pub const BG_ERROR_CONTEXT_NONE: BG_ERROR_CONTEXT = 0;
pub const BG_ERROR_CONTEXT_QUEUE_MANAGER_NOTIFICATION: BG_ERROR_CONTEXT = 3;
pub const BG_ERROR_CONTEXT_REMOTE_APPLICATION: BG_ERROR_CONTEXT = 7;
pub const BG_ERROR_CONTEXT_REMOTE_FILE: BG_ERROR_CONTEXT = 5;
pub const BG_ERROR_CONTEXT_SERVER_CERTIFICATE_CALLBACK: BG_ERROR_CONTEXT = 8;
pub const BG_ERROR_CONTEXT_UNKNOWN: BG_ERROR_CONTEXT = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BG_FILE_INFO {
    pub RemoteName: windows_core::PWSTR,
    pub LocalName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BG_FILE_PROGRESS {
    pub BytesTotal: u64,
    pub BytesTransferred: u64,
    pub Completed: windows_core::BOOL,
}
pub const BG_JOB_ENUM_ALL_USERS: u32 = 1;
pub type BG_JOB_PRIORITY = i32;
pub const BG_JOB_PRIORITY_FOREGROUND: BG_JOB_PRIORITY = 0;
pub const BG_JOB_PRIORITY_HIGH: BG_JOB_PRIORITY = 1;
pub const BG_JOB_PRIORITY_LOW: BG_JOB_PRIORITY = 3;
pub const BG_JOB_PRIORITY_NORMAL: BG_JOB_PRIORITY = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BG_JOB_PROGRESS {
    pub BytesTotal: u64,
    pub BytesTransferred: u64,
    pub FilesTotal: u32,
    pub FilesTransferred: u32,
}
pub type BG_JOB_PROXY_USAGE = i32;
pub const BG_JOB_PROXY_USAGE_AUTODETECT: BG_JOB_PROXY_USAGE = 3;
pub const BG_JOB_PROXY_USAGE_NO_PROXY: BG_JOB_PROXY_USAGE = 1;
pub const BG_JOB_PROXY_USAGE_OVERRIDE: BG_JOB_PROXY_USAGE = 2;
pub const BG_JOB_PROXY_USAGE_PRECONFIG: BG_JOB_PROXY_USAGE = 0;
pub type BG_JOB_STATE = i32;
pub const BG_JOB_STATE_ACKNOWLEDGED: BG_JOB_STATE = 7;
pub const BG_JOB_STATE_CANCELLED: BG_JOB_STATE = 8;
pub const BG_JOB_STATE_CONNECTING: BG_JOB_STATE = 1;
pub const BG_JOB_STATE_ERROR: BG_JOB_STATE = 4;
pub const BG_JOB_STATE_QUEUED: BG_JOB_STATE = 0;
pub const BG_JOB_STATE_SUSPENDED: BG_JOB_STATE = 3;
pub const BG_JOB_STATE_TRANSFERRED: BG_JOB_STATE = 6;
pub const BG_JOB_STATE_TRANSFERRING: BG_JOB_STATE = 2;
pub const BG_JOB_STATE_TRANSIENT_ERROR: BG_JOB_STATE = 5;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BG_JOB_TIMES {
    pub CreationTime: super::minwindef::FILETIME,
    pub ModificationTime: super::minwindef::FILETIME,
    pub TransferCompletionTime: super::minwindef::FILETIME,
}
pub type BG_JOB_TYPE = i32;
pub const BG_JOB_TYPE_DOWNLOAD: BG_JOB_TYPE = 0;
pub const BG_JOB_TYPE_UPLOAD: BG_JOB_TYPE = 1;
pub const BG_JOB_TYPE_UPLOAD_REPLY: BG_JOB_TYPE = 2;
pub const BG_NOTIFY_DISABLE: u32 = 4;
pub const BG_NOTIFY_FILE_RANGES_TRANSFERRED: u32 = 32;
pub const BG_NOTIFY_FILE_TRANSFERRED: u32 = 16;
pub const BG_NOTIFY_JOB_ERROR: u32 = 2;
pub const BG_NOTIFY_JOB_MODIFICATION: u32 = 8;
pub const BG_NOTIFY_JOB_TRANSFERRED: u32 = 1;
pub const BG_SIZE_UNKNOWN: i32 = -1;
pub const BackgroundCopyManager: windows_core::GUID = windows_core::GUID::from_u128(0x4991d34b_80a1_4291_83b6_3328366b9097);
windows_core::imp::define_interface!(IBackgroundCopyCallback, IBackgroundCopyCallback_Vtbl, 0x97ea99c7_0186_4ad4_8df9_c5b4e0ed6b22);
windows_core::imp::interface_hierarchy!(IBackgroundCopyCallback, windows_core::IUnknown);
impl IBackgroundCopyCallback {
    pub unsafe fn JobTransferred<P0>(&self, pjob: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
    {
        unsafe { (windows_core::Interface::vtable(self).JobTransferred)(windows_core::Interface::as_raw(self), pjob.param().abi()) }
    }
    pub unsafe fn JobError<P0, P1>(&self, pjob: P0, perror: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
        P1: windows_core::Param<IBackgroundCopyError>,
    {
        unsafe { (windows_core::Interface::vtable(self).JobError)(windows_core::Interface::as_raw(self), pjob.param().abi(), perror.param().abi()) }
    }
    pub unsafe fn JobModification<P0>(&self, pjob: P0, dwreserved: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
    {
        unsafe { (windows_core::Interface::vtable(self).JobModification)(windows_core::Interface::as_raw(self), pjob.param().abi(), dwreserved) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub JobTransferred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub JobError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub JobModification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IBackgroundCopyCallback_Impl: windows_core::IUnknownImpl {
    fn JobTransferred(&self, pjob: windows_core::Ref<IBackgroundCopyJob>) -> windows_core::Result<()>;
    fn JobError(&self, pjob: windows_core::Ref<IBackgroundCopyJob>, perror: windows_core::Ref<IBackgroundCopyError>) -> windows_core::Result<()>;
    fn JobModification(&self, pjob: windows_core::Ref<IBackgroundCopyJob>, dwreserved: u32) -> windows_core::Result<()>;
}
impl IBackgroundCopyCallback_Vtbl {
    pub const fn new<Identity: IBackgroundCopyCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn JobTransferred<Identity: IBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyCallback_Impl::JobTransferred(this, core::mem::transmute_copy(&pjob)).into()
            }
        }
        unsafe extern "system" fn JobError<Identity: IBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, perror: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyCallback_Impl::JobError(this, core::mem::transmute_copy(&pjob), core::mem::transmute_copy(&perror)).into()
            }
        }
        unsafe extern "system" fn JobModification<Identity: IBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyCallback_Impl::JobModification(this, core::mem::transmute_copy(&pjob), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            JobTransferred: JobTransferred::<Identity, OFFSET>,
            JobError: JobError::<Identity, OFFSET>,
            JobModification: JobModification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBackgroundCopyCallback {}
windows_core::imp::define_interface!(IBackgroundCopyError, IBackgroundCopyError_Vtbl, 0x19c613a0_fcb8_4f28_81ae_897c3d078f81);
windows_core::imp::interface_hierarchy!(IBackgroundCopyError, windows_core::IUnknown);
impl IBackgroundCopyError {
    pub unsafe fn GetError(&self, pcontext: *mut BG_ERROR_CONTEXT, pcode: *mut windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetError)(windows_core::Interface::as_raw(self), pcontext as _, pcode as _) }
    }
    pub unsafe fn GetFile(&self) -> windows_core::Result<IBackgroundCopyFile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetErrorDescription(&self, languageid: u32) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorDescription)(windows_core::Interface::as_raw(self), languageid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetErrorContextDescription(&self, languageid: u32) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorContextDescription)(windows_core::Interface::as_raw(self), languageid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProtocol(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProtocol)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyError_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_ERROR_CONTEXT, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetErrorDescription: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetErrorContextDescription: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetProtocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IBackgroundCopyError_Impl: windows_core::IUnknownImpl {
    fn GetError(&self, pcontext: *mut BG_ERROR_CONTEXT, pcode: *mut windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetFile(&self) -> windows_core::Result<IBackgroundCopyFile>;
    fn GetErrorDescription(&self, languageid: u32) -> windows_core::Result<windows_core::PWSTR>;
    fn GetErrorContextDescription(&self, languageid: u32) -> windows_core::Result<windows_core::PWSTR>;
    fn GetProtocol(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IBackgroundCopyError_Vtbl {
    pub const fn new<Identity: IBackgroundCopyError_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetError<Identity: IBackgroundCopyError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut BG_ERROR_CONTEXT, pcode: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyError_Impl::GetError(this, core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&pcode)).into()
            }
        }
        unsafe extern "system" fn GetFile<Identity: IBackgroundCopyError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyError_Impl::GetFile(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetErrorDescription<Identity: IBackgroundCopyError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, languageid: u32, perrordescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyError_Impl::GetErrorDescription(this, core::mem::transmute_copy(&languageid)) {
                    Ok(ok__) => {
                        perrordescription.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetErrorContextDescription<Identity: IBackgroundCopyError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, languageid: u32, pcontextdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyError_Impl::GetErrorContextDescription(this, core::mem::transmute_copy(&languageid)) {
                    Ok(ok__) => {
                        pcontextdescription.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProtocol<Identity: IBackgroundCopyError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprotocol: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyError_Impl::GetProtocol(this) {
                    Ok(ok__) => {
                        pprotocol.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetError: GetError::<Identity, OFFSET>,
            GetFile: GetFile::<Identity, OFFSET>,
            GetErrorDescription: GetErrorDescription::<Identity, OFFSET>,
            GetErrorContextDescription: GetErrorContextDescription::<Identity, OFFSET>,
            GetProtocol: GetProtocol::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyError as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBackgroundCopyError {}
windows_core::imp::define_interface!(IBackgroundCopyFile, IBackgroundCopyFile_Vtbl, 0x01b7bd23_fb88_4a77_8490_5891d3e4653a);
windows_core::imp::interface_hierarchy!(IBackgroundCopyFile, windows_core::IUnknown);
impl IBackgroundCopyFile {
    pub unsafe fn GetRemoteName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRemoteName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLocalName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLocalName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProgress(&self, pval: *mut BG_FILE_PROGRESS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProgress)(windows_core::Interface::as_raw(self), pval as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRemoteName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetLocalName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_FILE_PROGRESS) -> windows_core::HRESULT,
}
pub trait IBackgroundCopyFile_Impl: windows_core::IUnknownImpl {
    fn GetRemoteName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetLocalName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetProgress(&self, pval: *mut BG_FILE_PROGRESS) -> windows_core::Result<()>;
}
impl IBackgroundCopyFile_Vtbl {
    pub const fn new<Identity: IBackgroundCopyFile_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRemoteName<Identity: IBackgroundCopyFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyFile_Impl::GetRemoteName(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLocalName<Identity: IBackgroundCopyFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyFile_Impl::GetLocalName(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProgress<Identity: IBackgroundCopyFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_FILE_PROGRESS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile_Impl::GetProgress(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRemoteName: GetRemoteName::<Identity, OFFSET>,
            GetLocalName: GetLocalName::<Identity, OFFSET>,
            GetProgress: GetProgress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyFile as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBackgroundCopyFile {}
windows_core::imp::define_interface!(IBackgroundCopyJob, IBackgroundCopyJob_Vtbl, 0x37668d37_507e_4160_9316_26306d150b12);
windows_core::imp::interface_hierarchy!(IBackgroundCopyJob, windows_core::IUnknown);
impl IBackgroundCopyJob {
    pub unsafe fn AddFileSet(&self, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddFileSet)(windows_core::Interface::as_raw(self), cfilecount, pfileset) }
    }
    pub unsafe fn AddFile<P0, P1>(&self, remoteurl: P0, localname: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddFile)(windows_core::Interface::as_raw(self), remoteurl.param().abi(), localname.param().abi()) }
    }
    pub unsafe fn EnumFiles(&self) -> windows_core::Result<IEnumBackgroundCopyFiles> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumFiles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Suspend(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Suspend)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Resume(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Complete(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Complete)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetId(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<BG_JOB_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProgress(&self, pval: *mut BG_JOB_PROGRESS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProgress)(windows_core::Interface::as_raw(self), pval as _) }
    }
    #[cfg(feature = "Win32_minwindef")]
    pub unsafe fn GetTimes(&self, pval: *mut BG_JOB_TIMES) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTimes)(windows_core::Interface::as_raw(self), pval as _) }
    }
    pub unsafe fn GetState(&self) -> windows_core::Result<BG_JOB_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetError(&self) -> windows_core::Result<IBackgroundCopyError> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetError)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetOwner(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDisplayName<P0>(&self, val: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), val.param().abi()) }
    }
    pub unsafe fn GetDisplayName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDescription<P0>(&self, val: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), val.param().abi()) }
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPriority(&self, val: BG_JOB_PRIORITY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), val) }
    }
    pub unsafe fn GetPriority(&self) -> windows_core::Result<BG_JOB_PRIORITY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNotifyFlags(&self, val: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNotifyFlags)(windows_core::Interface::as_raw(self), val) }
    }
    pub unsafe fn GetNotifyFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNotifyFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNotifyInterface<P0>(&self, val: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetNotifyInterface)(windows_core::Interface::as_raw(self), val.param().abi()) }
    }
    pub unsafe fn GetNotifyInterface(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNotifyInterface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetMinimumRetryDelay(&self, seconds: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMinimumRetryDelay)(windows_core::Interface::as_raw(self), seconds) }
    }
    pub unsafe fn GetMinimumRetryDelay(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMinimumRetryDelay)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNoProgressTimeout(&self, seconds: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNoProgressTimeout)(windows_core::Interface::as_raw(self), seconds) }
    }
    pub unsafe fn GetNoProgressTimeout(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNoProgressTimeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetErrorCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetProxySettings(&self, proxyusage: BG_JOB_PROXY_USAGE, proxylist: *const u16, proxybypasslist: *const u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProxySettings)(windows_core::Interface::as_raw(self), proxyusage, proxylist, proxybypasslist) }
    }
    pub unsafe fn GetProxySettings(&self, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut windows_core::PWSTR, pproxybypasslist: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProxySettings)(windows_core::Interface::as_raw(self), pproxyusage as _, pproxylist as _, pproxybypasslist as _) }
    }
    pub unsafe fn TakeOwnership(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TakeOwnership)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJob_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddFileSet: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const BG_FILE_INFO) -> windows_core::HRESULT,
    pub AddFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub EnumFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Suspend: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_JOB_TYPE) -> windows_core::HRESULT,
    pub GetProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_JOB_PROGRESS) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_minwindef")]
    pub GetTimes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_JOB_TIMES) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_minwindef"))]
    GetTimes: usize,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_JOB_STATE) -> windows_core::HRESULT,
    pub GetError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, BG_JOB_PRIORITY) -> windows_core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_JOB_PRIORITY) -> windows_core::HRESULT,
    pub SetNotifyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetNotifyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetNotifyInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNotifyInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMinimumRetryDelay: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMinimumRetryDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetNoProgressTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetNoProgressTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetErrorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetProxySettings: unsafe extern "system" fn(*mut core::ffi::c_void, BG_JOB_PROXY_USAGE, *const u16, *const u16) -> windows_core::HRESULT,
    pub GetProxySettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_JOB_PROXY_USAGE, *mut windows_core::PWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub TakeOwnership: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_minwindef")]
pub trait IBackgroundCopyJob_Impl: windows_core::IUnknownImpl {
    fn AddFileSet(&self, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> windows_core::Result<()>;
    fn AddFile(&self, remoteurl: &windows_core::PCWSTR, localname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnumFiles(&self) -> windows_core::Result<IEnumBackgroundCopyFiles>;
    fn Suspend(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Complete(&self) -> windows_core::Result<()>;
    fn GetId(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetType(&self) -> windows_core::Result<BG_JOB_TYPE>;
    fn GetProgress(&self, pval: *mut BG_JOB_PROGRESS) -> windows_core::Result<()>;
    fn GetTimes(&self, pval: *mut BG_JOB_TIMES) -> windows_core::Result<()>;
    fn GetState(&self) -> windows_core::Result<BG_JOB_STATE>;
    fn GetError(&self) -> windows_core::Result<IBackgroundCopyError>;
    fn GetOwner(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetDisplayName(&self, val: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDisplayName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetDescription(&self, val: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetPriority(&self, val: BG_JOB_PRIORITY) -> windows_core::Result<()>;
    fn GetPriority(&self) -> windows_core::Result<BG_JOB_PRIORITY>;
    fn SetNotifyFlags(&self, val: u32) -> windows_core::Result<()>;
    fn GetNotifyFlags(&self) -> windows_core::Result<u32>;
    fn SetNotifyInterface(&self, val: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetNotifyInterface(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn SetMinimumRetryDelay(&self, seconds: u32) -> windows_core::Result<()>;
    fn GetMinimumRetryDelay(&self) -> windows_core::Result<u32>;
    fn SetNoProgressTimeout(&self, seconds: u32) -> windows_core::Result<()>;
    fn GetNoProgressTimeout(&self) -> windows_core::Result<u32>;
    fn GetErrorCount(&self) -> windows_core::Result<u32>;
    fn SetProxySettings(&self, proxyusage: BG_JOB_PROXY_USAGE, proxylist: *const u16, proxybypasslist: *const u16) -> windows_core::Result<()>;
    fn GetProxySettings(&self, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut windows_core::PWSTR, pproxybypasslist: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn TakeOwnership(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_minwindef")]
impl IBackgroundCopyJob_Vtbl {
    pub const fn new<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddFileSet<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::AddFileSet(this, core::mem::transmute_copy(&cfilecount), core::mem::transmute_copy(&pfileset)).into()
            }
        }
        unsafe extern "system" fn AddFile<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteurl: windows_core::PCWSTR, localname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::AddFile(this, core::mem::transmute(&remoteurl), core::mem::transmute(&localname)).into()
            }
        }
        unsafe extern "system" fn EnumFiles<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJob_Impl::EnumFiles(this) {
                    Ok(ok__) => {
                        penum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Suspend<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::Suspend(this).into()
            }
        }
        unsafe extern "system" fn Resume<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::Resume(this).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn Complete<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::Complete(this).into()
            }
        }
        unsafe extern "system" fn GetId<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJob_Impl::GetId(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetType<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_JOB_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJob_Impl::GetType(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProgress<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_JOB_PROGRESS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::GetProgress(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn GetTimes<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_JOB_TIMES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::GetTimes(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn GetState<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_JOB_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJob_Impl::GetState(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetError<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperror: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJob_Impl::GetError(this) {
                    Ok(ok__) => {
                        pperror.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOwner<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJob_Impl::GetOwner(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::SetDisplayName(this, core::mem::transmute(&val)).into()
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJob_Impl::GetDisplayName(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::SetDescription(this, core::mem::transmute(&val)).into()
            }
        }
        unsafe extern "system" fn GetDescription<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJob_Impl::GetDescription(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: BG_JOB_PRIORITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::SetPriority(this, core::mem::transmute_copy(&val)).into()
            }
        }
        unsafe extern "system" fn GetPriority<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_JOB_PRIORITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJob_Impl::GetPriority(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNotifyFlags<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::SetNotifyFlags(this, core::mem::transmute_copy(&val)).into()
            }
        }
        unsafe extern "system" fn GetNotifyFlags<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJob_Impl::GetNotifyFlags(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNotifyInterface<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::SetNotifyInterface(this, core::mem::transmute_copy(&val)).into()
            }
        }
        unsafe extern "system" fn GetNotifyInterface<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJob_Impl::GetNotifyInterface(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMinimumRetryDelay<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::SetMinimumRetryDelay(this, core::mem::transmute_copy(&seconds)).into()
            }
        }
        unsafe extern "system" fn GetMinimumRetryDelay<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJob_Impl::GetMinimumRetryDelay(this) {
                    Ok(ok__) => {
                        seconds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNoProgressTimeout<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::SetNoProgressTimeout(this, core::mem::transmute_copy(&seconds)).into()
            }
        }
        unsafe extern "system" fn GetNoProgressTimeout<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJob_Impl::GetNoProgressTimeout(this) {
                    Ok(ok__) => {
                        seconds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetErrorCount<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errors: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJob_Impl::GetErrorCount(this) {
                    Ok(ok__) => {
                        errors.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProxySettings<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, proxyusage: BG_JOB_PROXY_USAGE, proxylist: *const u16, proxybypasslist: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::SetProxySettings(this, core::mem::transmute_copy(&proxyusage), core::mem::transmute_copy(&proxylist), core::mem::transmute_copy(&proxybypasslist)).into()
            }
        }
        unsafe extern "system" fn GetProxySettings<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut windows_core::PWSTR, pproxybypasslist: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::GetProxySettings(this, core::mem::transmute_copy(&pproxyusage), core::mem::transmute_copy(&pproxylist), core::mem::transmute_copy(&pproxybypasslist)).into()
            }
        }
        unsafe extern "system" fn TakeOwnership<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::TakeOwnership(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddFileSet: AddFileSet::<Identity, OFFSET>,
            AddFile: AddFile::<Identity, OFFSET>,
            EnumFiles: EnumFiles::<Identity, OFFSET>,
            Suspend: Suspend::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            Complete: Complete::<Identity, OFFSET>,
            GetId: GetId::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetProgress: GetProgress::<Identity, OFFSET>,
            GetTimes: GetTimes::<Identity, OFFSET>,
            GetState: GetState::<Identity, OFFSET>,
            GetError: GetError::<Identity, OFFSET>,
            GetOwner: GetOwner::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            SetNotifyFlags: SetNotifyFlags::<Identity, OFFSET>,
            GetNotifyFlags: GetNotifyFlags::<Identity, OFFSET>,
            SetNotifyInterface: SetNotifyInterface::<Identity, OFFSET>,
            GetNotifyInterface: GetNotifyInterface::<Identity, OFFSET>,
            SetMinimumRetryDelay: SetMinimumRetryDelay::<Identity, OFFSET>,
            GetMinimumRetryDelay: GetMinimumRetryDelay::<Identity, OFFSET>,
            SetNoProgressTimeout: SetNoProgressTimeout::<Identity, OFFSET>,
            GetNoProgressTimeout: GetNoProgressTimeout::<Identity, OFFSET>,
            GetErrorCount: GetErrorCount::<Identity, OFFSET>,
            SetProxySettings: SetProxySettings::<Identity, OFFSET>,
            GetProxySettings: GetProxySettings::<Identity, OFFSET>,
            TakeOwnership: TakeOwnership::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJob as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_minwindef")]
impl windows_core::RuntimeName for IBackgroundCopyJob {}
windows_core::imp::define_interface!(IBackgroundCopyManager, IBackgroundCopyManager_Vtbl, 0x5ce34c0d_0dc9_4c1f_897c_daa1b78cee7c);
windows_core::imp::interface_hierarchy!(IBackgroundCopyManager, windows_core::IUnknown);
impl IBackgroundCopyManager {
    pub unsafe fn CreateJob<P0>(&self, displayname: P0, r#type: BG_JOB_TYPE, pjobid: *mut windows_core::GUID, ppjob: *mut Option<IBackgroundCopyJob>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateJob)(windows_core::Interface::as_raw(self), displayname.param().abi(), r#type, pjobid as _, core::mem::transmute(ppjob)) }
    }
    pub unsafe fn GetJob(&self, jobid: *const windows_core::GUID) -> windows_core::Result<IBackgroundCopyJob> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetJob)(windows_core::Interface::as_raw(self), jobid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumJobs(&self, dwflags: u32) -> windows_core::Result<IEnumBackgroundCopyJobs> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumJobs)(windows_core::Interface::as_raw(self), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetErrorDescription(&self, hresult: windows_core::HRESULT, languageid: u32) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorDescription)(windows_core::Interface::as_raw(self), hresult, languageid, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateJob: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, BG_JOB_TYPE, *mut windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetJob: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumJobs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetErrorDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IBackgroundCopyManager_Impl: windows_core::IUnknownImpl {
    fn CreateJob(&self, displayname: &windows_core::PCWSTR, r#type: BG_JOB_TYPE, pjobid: *mut windows_core::GUID, ppjob: windows_core::OutRef<IBackgroundCopyJob>) -> windows_core::Result<()>;
    fn GetJob(&self, jobid: *const windows_core::GUID) -> windows_core::Result<IBackgroundCopyJob>;
    fn EnumJobs(&self, dwflags: u32) -> windows_core::Result<IEnumBackgroundCopyJobs>;
    fn GetErrorDescription(&self, hresult: windows_core::HRESULT, languageid: u32) -> windows_core::Result<windows_core::PWSTR>;
}
impl IBackgroundCopyManager_Vtbl {
    pub const fn new<Identity: IBackgroundCopyManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateJob<Identity: IBackgroundCopyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: windows_core::PCWSTR, r#type: BG_JOB_TYPE, pjobid: *mut windows_core::GUID, ppjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyManager_Impl::CreateJob(this, core::mem::transmute(&displayname), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pjobid), core::mem::transmute_copy(&ppjob)).into()
            }
        }
        unsafe extern "system" fn GetJob<Identity: IBackgroundCopyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, jobid: *const windows_core::GUID, ppjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyManager_Impl::GetJob(this, core::mem::transmute_copy(&jobid)) {
                    Ok(ok__) => {
                        ppjob.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumJobs<Identity: IBackgroundCopyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyManager_Impl::EnumJobs(this, core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetErrorDescription<Identity: IBackgroundCopyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT, languageid: u32, perrordescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyManager_Impl::GetErrorDescription(this, core::mem::transmute_copy(&hresult), core::mem::transmute_copy(&languageid)) {
                    Ok(ok__) => {
                        perrordescription.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateJob: CreateJob::<Identity, OFFSET>,
            GetJob: GetJob::<Identity, OFFSET>,
            EnumJobs: EnumJobs::<Identity, OFFSET>,
            GetErrorDescription: GetErrorDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBackgroundCopyManager {}
windows_core::imp::define_interface!(IEnumBackgroundCopyFiles, IEnumBackgroundCopyFiles_Vtbl, 0xca51e165_c365_424c_8d41_24aaa4ff3c40);
windows_core::imp::interface_hierarchy!(IEnumBackgroundCopyFiles, windows_core::IUnknown);
impl IEnumBackgroundCopyFiles {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut Option<IBackgroundCopyFile>, pceltfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgelt), pceltfetched as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumBackgroundCopyFiles_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IEnumBackgroundCopyFiles_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: windows_core::OutRef<IBackgroundCopyFile>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumBackgroundCopyFiles>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl IEnumBackgroundCopyFiles_Vtbl {
    pub const fn new<Identity: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBackgroundCopyFiles_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBackgroundCopyFiles_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBackgroundCopyFiles_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumBackgroundCopyFiles_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumBackgroundCopyFiles_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pucount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumBackgroundCopyFiles as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumBackgroundCopyFiles {}
windows_core::imp::define_interface!(IEnumBackgroundCopyJobs, IEnumBackgroundCopyJobs_Vtbl, 0x1af4f612_3b71_466f_8f58_7b6f73ac57ad);
windows_core::imp::interface_hierarchy!(IEnumBackgroundCopyJobs, windows_core::IUnknown);
impl IEnumBackgroundCopyJobs {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut Option<IBackgroundCopyJob>, pceltfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgelt), pceltfetched as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumBackgroundCopyJobs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IEnumBackgroundCopyJobs_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: windows_core::OutRef<IBackgroundCopyJob>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumBackgroundCopyJobs>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl IEnumBackgroundCopyJobs_Vtbl {
    pub const fn new<Identity: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBackgroundCopyJobs_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBackgroundCopyJobs_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBackgroundCopyJobs_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumBackgroundCopyJobs_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumBackgroundCopyJobs_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pucount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumBackgroundCopyJobs as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumBackgroundCopyJobs {}
