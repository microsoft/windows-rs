pub trait IVssAdminImpl: Sized {
    fn RegisterProvider();
    fn UnregisterProvider();
    fn QueryProviders();
    fn AbortAllSnapshotsInProgress();
}
impl IVssAdminVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssAdminImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssAdminVtbl {
        unsafe extern "system" fn RegisterProvider<Impl: IVssAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproviderid: ::windows::core::GUID, classid: ::windows::core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterProvider<Impl: IVssAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryProviders<Impl: IVssAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AbortAllSnapshotsInProgress<Impl: IVssAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, RegisterProvider::<Impl, IMPL_OFFSET>, UnregisterProvider::<Impl, IMPL_OFFSET>, QueryProviders::<Impl, IMPL_OFFSET>, AbortAllSnapshotsInProgress::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssAdmin as ::windows::core::Interface>::IID
    }
}
pub trait IVssAdminExImpl: Sized + IVssAdminImpl {
    fn GetProviderCapability();
    fn GetProviderContext();
    fn SetProviderContext();
}
impl IVssAdminExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssAdminExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssAdminExVtbl {
        unsafe extern "system" fn GetProviderCapability<Impl: IVssAdminExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproviderid: ::windows::core::GUID, plloriginalcapabilitymask: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProviderContext<Impl: IVssAdminExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, plcontext: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProviderContext<Impl: IVssAdminExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, lcontext: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, RegisterProvider::<Impl, IMPL_OFFSET>, UnregisterProvider::<Impl, IMPL_OFFSET>, QueryProviders::<Impl, IMPL_OFFSET>, AbortAllSnapshotsInProgress::<Impl, IMPL_OFFSET>, GetProviderCapability::<Impl, IMPL_OFFSET>, GetProviderContext::<Impl, IMPL_OFFSET>, SetProviderContext::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssAdminEx as ::windows::core::Interface>::IID
    }
}
pub trait IVssAsyncImpl: Sized {
    fn Cancel();
    fn Wait();
    fn QueryStatus();
}
impl IVssAsyncVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssAsyncImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssAsyncVtbl {
        unsafe extern "system" fn Cancel<Impl: IVssAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Wait<Impl: IVssAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryStatus<Impl: IVssAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT, preserved: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Cancel::<Impl, IMPL_OFFSET>, Wait::<Impl, IMPL_OFFSET>, QueryStatus::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssAsync as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssComponentImpl: Sized {
    fn GetLogicalPath();
    fn GetComponentType();
    fn GetComponentName();
    fn GetBackupSucceeded();
    fn GetAlternateLocationMappingCount();
    fn GetAlternateLocationMapping();
    fn SetBackupMetadata();
    fn GetBackupMetadata();
    fn AddPartialFile();
    fn GetPartialFileCount();
    fn GetPartialFile();
    fn IsSelectedForRestore();
    fn GetAdditionalRestores();
    fn GetNewTargetCount();
    fn GetNewTarget();
    fn AddDirectedTarget();
    fn GetDirectedTargetCount();
    fn GetDirectedTarget();
    fn SetRestoreMetadata();
    fn GetRestoreMetadata();
    fn SetRestoreTarget();
    fn GetRestoreTarget();
    fn SetPreRestoreFailureMsg();
    fn GetPreRestoreFailureMsg();
    fn SetPostRestoreFailureMsg();
    fn GetPostRestoreFailureMsg();
    fn SetBackupStamp();
    fn GetBackupStamp();
    fn GetPreviousBackupStamp();
    fn GetBackupOptions();
    fn GetRestoreOptions();
    fn GetRestoreSubcomponentCount();
    fn GetRestoreSubcomponent();
    fn GetFileRestoreStatus();
    fn AddDifferencedFilesByLastModifyTime();
    fn AddDifferencedFilesByLastModifyLSN();
    fn GetDifferencedFilesCount();
    fn GetDifferencedFile();
}
#[cfg(feature = "Win32_Foundation")]
impl IVssComponentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssComponentVtbl {
        unsafe extern "system" fn GetLogicalPath<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetComponentType<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pct: *mut VSS_COMPONENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetComponentName<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBackupSucceeded<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsucceeded: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAlternateLocationMappingCount<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcmappings: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAlternateLocationMapping<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imapping: u32, ppfiledesc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBackupMetadata<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszdata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBackupMetadata<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddPartialFile<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilename: super::super::Foundation::PWSTR, wszranges: super::super::Foundation::PWSTR, wszmetadata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPartialFileCount<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcpartialfiles: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPartialFile<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipartialfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilename: *mut super::super::Foundation::BSTR, pbstrrange: *mut super::super::Foundation::BSTR, pbstrmetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSelectedForRestore<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbselectedforrestore: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAdditionalRestores<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbadditionalrestores: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNewTargetCount<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnewtarget: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNewTarget<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inewtarget: u32, ppfiledesc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddDirectedTarget<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszsourcepath: super::super::Foundation::PWSTR, wszsourcefilename: super::super::Foundation::PWSTR, wszsourcerangelist: super::super::Foundation::PWSTR, wszdestinationpath: super::super::Foundation::PWSTR, wszdestinationfilename: super::super::Foundation::PWSTR, wszdestinationrangelist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDirectedTargetCount<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdirectedtarget: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDirectedTarget<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idirectedtarget: u32, pbstrsourcepath: *mut super::super::Foundation::BSTR, pbstrsourcefilename: *mut super::super::Foundation::BSTR, pbstrsourcerangelist: *mut super::super::Foundation::BSTR, pbstrdestinationpath: *mut super::super::Foundation::BSTR, pbstrdestinationfilename: *mut super::super::Foundation::BSTR, pbstrdestinationrangelist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRestoreMetadata<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszrestoremetadata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRestoreMetadata<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrestoremetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRestoreTarget<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: VSS_RESTORE_TARGET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRestoreTarget<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPreRestoreFailureMsg<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszprerestorefailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreRestoreFailureMsg<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprerestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPostRestoreFailureMsg<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpostrestorefailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPostRestoreFailureMsg<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpostrestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBackupStamp<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszbackupstamp: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBackupStamp<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreviousBackupStamp<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBackupOptions<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbackupoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRestoreOptions<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrestoreoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRestoreSubcomponentCount<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcrestoresubcomponent: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRestoreSubcomponent<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icomponent: u32, pbstrlogicalpath: *mut super::super::Foundation::BSTR, pbstrcomponentname: *mut super::super::Foundation::BSTR, pbrepair: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileRestoreStatus<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddDifferencedFilesByLastModifyTime<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: super::super::Foundation::BOOL, ftlastmodifytime: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddDifferencedFilesByLastModifyLSN<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: super::super::Foundation::BOOL, bstrlsnstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDifferencedFilesCount<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdifferencedfiles: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDifferencedFile<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idifferencedfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilespec: *mut super::super::Foundation::BSTR, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut super::super::Foundation::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetLogicalPath::<Impl, IMPL_OFFSET>,
            GetComponentType::<Impl, IMPL_OFFSET>,
            GetComponentName::<Impl, IMPL_OFFSET>,
            GetBackupSucceeded::<Impl, IMPL_OFFSET>,
            GetAlternateLocationMappingCount::<Impl, IMPL_OFFSET>,
            GetAlternateLocationMapping::<Impl, IMPL_OFFSET>,
            SetBackupMetadata::<Impl, IMPL_OFFSET>,
            GetBackupMetadata::<Impl, IMPL_OFFSET>,
            AddPartialFile::<Impl, IMPL_OFFSET>,
            GetPartialFileCount::<Impl, IMPL_OFFSET>,
            GetPartialFile::<Impl, IMPL_OFFSET>,
            IsSelectedForRestore::<Impl, IMPL_OFFSET>,
            GetAdditionalRestores::<Impl, IMPL_OFFSET>,
            GetNewTargetCount::<Impl, IMPL_OFFSET>,
            GetNewTarget::<Impl, IMPL_OFFSET>,
            AddDirectedTarget::<Impl, IMPL_OFFSET>,
            GetDirectedTargetCount::<Impl, IMPL_OFFSET>,
            GetDirectedTarget::<Impl, IMPL_OFFSET>,
            SetRestoreMetadata::<Impl, IMPL_OFFSET>,
            GetRestoreMetadata::<Impl, IMPL_OFFSET>,
            SetRestoreTarget::<Impl, IMPL_OFFSET>,
            GetRestoreTarget::<Impl, IMPL_OFFSET>,
            SetPreRestoreFailureMsg::<Impl, IMPL_OFFSET>,
            GetPreRestoreFailureMsg::<Impl, IMPL_OFFSET>,
            SetPostRestoreFailureMsg::<Impl, IMPL_OFFSET>,
            GetPostRestoreFailureMsg::<Impl, IMPL_OFFSET>,
            SetBackupStamp::<Impl, IMPL_OFFSET>,
            GetBackupStamp::<Impl, IMPL_OFFSET>,
            GetPreviousBackupStamp::<Impl, IMPL_OFFSET>,
            GetBackupOptions::<Impl, IMPL_OFFSET>,
            GetRestoreOptions::<Impl, IMPL_OFFSET>,
            GetRestoreSubcomponentCount::<Impl, IMPL_OFFSET>,
            GetRestoreSubcomponent::<Impl, IMPL_OFFSET>,
            GetFileRestoreStatus::<Impl, IMPL_OFFSET>,
            AddDifferencedFilesByLastModifyTime::<Impl, IMPL_OFFSET>,
            AddDifferencedFilesByLastModifyLSN::<Impl, IMPL_OFFSET>,
            GetDifferencedFilesCount::<Impl, IMPL_OFFSET>,
            GetDifferencedFile::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssComponent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssComponentExImpl: Sized + IVssComponentImpl {
    fn SetPrepareForBackupFailureMsg();
    fn SetPostSnapshotFailureMsg();
    fn GetPrepareForBackupFailureMsg();
    fn GetPostSnapshotFailureMsg();
    fn GetAuthoritativeRestore();
    fn GetRollForward();
    fn GetRestoreName();
}
#[cfg(feature = "Win32_Foundation")]
impl IVssComponentExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponentExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssComponentExVtbl {
        unsafe extern "system" fn SetPrepareForBackupFailureMsg<Impl: IVssComponentExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszfailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPostSnapshotFailureMsg<Impl: IVssComponentExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszfailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrepareForBackupFailureMsg<Impl: IVssComponentExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPostSnapshotFailureMsg<Impl: IVssComponentExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAuthoritativeRestore<Impl: IVssComponentExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbauth: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRollForward<Impl: IVssComponentExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRestoreName<Impl: IVssComponentExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetLogicalPath::<Impl, IMPL_OFFSET>,
            GetComponentType::<Impl, IMPL_OFFSET>,
            GetComponentName::<Impl, IMPL_OFFSET>,
            GetBackupSucceeded::<Impl, IMPL_OFFSET>,
            GetAlternateLocationMappingCount::<Impl, IMPL_OFFSET>,
            GetAlternateLocationMapping::<Impl, IMPL_OFFSET>,
            SetBackupMetadata::<Impl, IMPL_OFFSET>,
            GetBackupMetadata::<Impl, IMPL_OFFSET>,
            AddPartialFile::<Impl, IMPL_OFFSET>,
            GetPartialFileCount::<Impl, IMPL_OFFSET>,
            GetPartialFile::<Impl, IMPL_OFFSET>,
            IsSelectedForRestore::<Impl, IMPL_OFFSET>,
            GetAdditionalRestores::<Impl, IMPL_OFFSET>,
            GetNewTargetCount::<Impl, IMPL_OFFSET>,
            GetNewTarget::<Impl, IMPL_OFFSET>,
            AddDirectedTarget::<Impl, IMPL_OFFSET>,
            GetDirectedTargetCount::<Impl, IMPL_OFFSET>,
            GetDirectedTarget::<Impl, IMPL_OFFSET>,
            SetRestoreMetadata::<Impl, IMPL_OFFSET>,
            GetRestoreMetadata::<Impl, IMPL_OFFSET>,
            SetRestoreTarget::<Impl, IMPL_OFFSET>,
            GetRestoreTarget::<Impl, IMPL_OFFSET>,
            SetPreRestoreFailureMsg::<Impl, IMPL_OFFSET>,
            GetPreRestoreFailureMsg::<Impl, IMPL_OFFSET>,
            SetPostRestoreFailureMsg::<Impl, IMPL_OFFSET>,
            GetPostRestoreFailureMsg::<Impl, IMPL_OFFSET>,
            SetBackupStamp::<Impl, IMPL_OFFSET>,
            GetBackupStamp::<Impl, IMPL_OFFSET>,
            GetPreviousBackupStamp::<Impl, IMPL_OFFSET>,
            GetBackupOptions::<Impl, IMPL_OFFSET>,
            GetRestoreOptions::<Impl, IMPL_OFFSET>,
            GetRestoreSubcomponentCount::<Impl, IMPL_OFFSET>,
            GetRestoreSubcomponent::<Impl, IMPL_OFFSET>,
            GetFileRestoreStatus::<Impl, IMPL_OFFSET>,
            AddDifferencedFilesByLastModifyTime::<Impl, IMPL_OFFSET>,
            AddDifferencedFilesByLastModifyLSN::<Impl, IMPL_OFFSET>,
            GetDifferencedFilesCount::<Impl, IMPL_OFFSET>,
            GetDifferencedFile::<Impl, IMPL_OFFSET>,
            SetPrepareForBackupFailureMsg::<Impl, IMPL_OFFSET>,
            SetPostSnapshotFailureMsg::<Impl, IMPL_OFFSET>,
            GetPrepareForBackupFailureMsg::<Impl, IMPL_OFFSET>,
            GetPostSnapshotFailureMsg::<Impl, IMPL_OFFSET>,
            GetAuthoritativeRestore::<Impl, IMPL_OFFSET>,
            GetRollForward::<Impl, IMPL_OFFSET>,
            GetRestoreName::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssComponentEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssComponentEx2Impl: Sized + IVssComponentExImpl + IVssComponentImpl {
    fn SetFailure();
    fn GetFailure();
}
#[cfg(feature = "Win32_Foundation")]
impl IVssComponentEx2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponentEx2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssComponentEx2Vtbl {
        unsafe extern "system" fn SetFailure<Impl: IVssComponentEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, hrapplication: ::windows::core::HRESULT, wszapplicationmessage: super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFailure<Impl: IVssComponentEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phr: *mut ::windows::core::HRESULT, phrapplication: *mut ::windows::core::HRESULT, pbstrapplicationmessage: *mut super::super::Foundation::BSTR, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetLogicalPath::<Impl, IMPL_OFFSET>,
            GetComponentType::<Impl, IMPL_OFFSET>,
            GetComponentName::<Impl, IMPL_OFFSET>,
            GetBackupSucceeded::<Impl, IMPL_OFFSET>,
            GetAlternateLocationMappingCount::<Impl, IMPL_OFFSET>,
            GetAlternateLocationMapping::<Impl, IMPL_OFFSET>,
            SetBackupMetadata::<Impl, IMPL_OFFSET>,
            GetBackupMetadata::<Impl, IMPL_OFFSET>,
            AddPartialFile::<Impl, IMPL_OFFSET>,
            GetPartialFileCount::<Impl, IMPL_OFFSET>,
            GetPartialFile::<Impl, IMPL_OFFSET>,
            IsSelectedForRestore::<Impl, IMPL_OFFSET>,
            GetAdditionalRestores::<Impl, IMPL_OFFSET>,
            GetNewTargetCount::<Impl, IMPL_OFFSET>,
            GetNewTarget::<Impl, IMPL_OFFSET>,
            AddDirectedTarget::<Impl, IMPL_OFFSET>,
            GetDirectedTargetCount::<Impl, IMPL_OFFSET>,
            GetDirectedTarget::<Impl, IMPL_OFFSET>,
            SetRestoreMetadata::<Impl, IMPL_OFFSET>,
            GetRestoreMetadata::<Impl, IMPL_OFFSET>,
            SetRestoreTarget::<Impl, IMPL_OFFSET>,
            GetRestoreTarget::<Impl, IMPL_OFFSET>,
            SetPreRestoreFailureMsg::<Impl, IMPL_OFFSET>,
            GetPreRestoreFailureMsg::<Impl, IMPL_OFFSET>,
            SetPostRestoreFailureMsg::<Impl, IMPL_OFFSET>,
            GetPostRestoreFailureMsg::<Impl, IMPL_OFFSET>,
            SetBackupStamp::<Impl, IMPL_OFFSET>,
            GetBackupStamp::<Impl, IMPL_OFFSET>,
            GetPreviousBackupStamp::<Impl, IMPL_OFFSET>,
            GetBackupOptions::<Impl, IMPL_OFFSET>,
            GetRestoreOptions::<Impl, IMPL_OFFSET>,
            GetRestoreSubcomponentCount::<Impl, IMPL_OFFSET>,
            GetRestoreSubcomponent::<Impl, IMPL_OFFSET>,
            GetFileRestoreStatus::<Impl, IMPL_OFFSET>,
            AddDifferencedFilesByLastModifyTime::<Impl, IMPL_OFFSET>,
            AddDifferencedFilesByLastModifyLSN::<Impl, IMPL_OFFSET>,
            GetDifferencedFilesCount::<Impl, IMPL_OFFSET>,
            GetDifferencedFile::<Impl, IMPL_OFFSET>,
            SetPrepareForBackupFailureMsg::<Impl, IMPL_OFFSET>,
            SetPostSnapshotFailureMsg::<Impl, IMPL_OFFSET>,
            GetPrepareForBackupFailureMsg::<Impl, IMPL_OFFSET>,
            GetPostSnapshotFailureMsg::<Impl, IMPL_OFFSET>,
            GetAuthoritativeRestore::<Impl, IMPL_OFFSET>,
            GetRollForward::<Impl, IMPL_OFFSET>,
            GetRestoreName::<Impl, IMPL_OFFSET>,
            SetFailure::<Impl, IMPL_OFFSET>,
            GetFailure::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssComponentEx2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssCreateExpressWriterMetadataImpl: Sized {
    fn AddExcludeFiles();
    fn AddComponent();
    fn AddFilesToFileGroup();
    fn SetRestoreMethod();
    fn AddComponentDependency();
    fn SetBackupSchema();
    fn SaveAsXML();
}
#[cfg(feature = "Win32_Foundation")]
impl IVssCreateExpressWriterMetadataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateExpressWriterMetadataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssCreateExpressWriterMetadataVtbl {
        unsafe extern "system" fn AddExcludeFiles<Impl: IVssCreateExpressWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddComponent<Impl: IVssCreateExpressWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ct: VSS_COMPONENT_TYPE, wszlogicalpath: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcaption: super::super::Foundation::PWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddFilesToFileGroup<Impl: IVssCreateExpressWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlogicalpath: super::super::Foundation::PWSTR, wszgroupname: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8, wszalternatelocation: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRestoreMethod<Impl: IVssCreateExpressWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: VSS_RESTOREMETHOD_ENUM, wszservice: super::super::Foundation::PWSTR, wszuserprocedure: super::super::Foundation::PWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddComponentDependency<Impl: IVssCreateExpressWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszforlogicalpath: super::super::Foundation::PWSTR, wszforcomponentname: super::super::Foundation::PWSTR, onwriterid: ::windows::core::GUID, wszonlogicalpath: super::super::Foundation::PWSTR, wszoncomponentname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBackupSchema<Impl: IVssCreateExpressWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwschemamask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveAsXML<Impl: IVssCreateExpressWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddExcludeFiles::<Impl, IMPL_OFFSET>, AddComponent::<Impl, IMPL_OFFSET>, AddFilesToFileGroup::<Impl, IMPL_OFFSET>, SetRestoreMethod::<Impl, IMPL_OFFSET>, AddComponentDependency::<Impl, IMPL_OFFSET>, SetBackupSchema::<Impl, IMPL_OFFSET>, SaveAsXML::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssCreateExpressWriterMetadata as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
pub trait IVssCreateWriterMetadataImpl: Sized {
    fn AddIncludeFiles();
    fn AddExcludeFiles();
    fn AddComponent();
    fn AddDatabaseFiles();
    fn AddDatabaseLogFiles();
    fn AddFilesToFileGroup();
    fn SetRestoreMethod();
    fn AddAlternateLocationMapping();
    fn AddComponentDependency();
    fn SetBackupSchema();
    fn GetDocument();
    fn SaveAsXML();
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
impl IVssCreateWriterMetadataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateWriterMetadataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssCreateWriterMetadataVtbl {
        unsafe extern "system" fn AddIncludeFiles<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8, wszalternatelocation: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddExcludeFiles<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddComponent<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ct: VSS_COMPONENT_TYPE, wszlogicalpath: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcaption: super::super::Foundation::PWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddDatabaseFiles<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlogicalpath: super::super::Foundation::PWSTR, wszdatabasename: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddDatabaseLogFiles<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlogicalpath: super::super::Foundation::PWSTR, wszdatabasename: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddFilesToFileGroup<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlogicalpath: super::super::Foundation::PWSTR, wszgroupname: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8, wszalternatelocation: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRestoreMethod<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: VSS_RESTOREMETHOD_ENUM, wszservice: super::super::Foundation::PWSTR, wszuserprocedure: super::super::Foundation::PWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddAlternateLocationMapping<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszsourcepath: super::super::Foundation::PWSTR, wszsourcefilespec: super::super::Foundation::PWSTR, brecursive: u8, wszdestination: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddComponentDependency<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszforlogicalpath: super::super::Foundation::PWSTR, wszforcomponentname: super::super::Foundation::PWSTR, onwriterid: ::windows::core::GUID, wszonlogicalpath: super::super::Foundation::PWSTR, wszoncomponentname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBackupSchema<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwschemamask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDocument<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdoc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveAsXML<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(AddIncludeFiles::<Impl, IMPL_OFFSET>, AddExcludeFiles::<Impl, IMPL_OFFSET>, AddComponent::<Impl, IMPL_OFFSET>, AddDatabaseFiles::<Impl, IMPL_OFFSET>, AddDatabaseLogFiles::<Impl, IMPL_OFFSET>, AddFilesToFileGroup::<Impl, IMPL_OFFSET>, SetRestoreMethod::<Impl, IMPL_OFFSET>, AddAlternateLocationMapping::<Impl, IMPL_OFFSET>, AddComponentDependency::<Impl, IMPL_OFFSET>, SetBackupSchema::<Impl, IMPL_OFFSET>, GetDocument::<Impl, IMPL_OFFSET>, SaveAsXML::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssCreateWriterMetadata as ::windows::core::Interface>::IID
    }
}
pub trait IVssDifferentialSoftwareSnapshotMgmtImpl: Sized {
    fn AddDiffArea();
    fn ChangeDiffAreaMaximumSize();
    fn QueryVolumesSupportedForDiffAreas();
    fn QueryDiffAreasForVolume();
    fn QueryDiffAreasOnVolume();
    fn QueryDiffAreasForSnapshot();
}
impl IVssDifferentialSoftwareSnapshotMgmtVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmtImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssDifferentialSoftwareSnapshotMgmtVtbl {
        unsafe extern "system" fn AddDiffArea<Impl: IVssDifferentialSoftwareSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ChangeDiffAreaMaximumSize<Impl: IVssDifferentialSoftwareSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryVolumesSupportedForDiffAreas<Impl: IVssDifferentialSoftwareSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszoriginalvolumename: *const u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryDiffAreasForVolume<Impl: IVssDifferentialSoftwareSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryDiffAreasOnVolume<Impl: IVssDifferentialSoftwareSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryDiffAreasForSnapshot<Impl: IVssDifferentialSoftwareSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddDiffArea::<Impl, IMPL_OFFSET>, ChangeDiffAreaMaximumSize::<Impl, IMPL_OFFSET>, QueryVolumesSupportedForDiffAreas::<Impl, IMPL_OFFSET>, QueryDiffAreasForVolume::<Impl, IMPL_OFFSET>, QueryDiffAreasOnVolume::<Impl, IMPL_OFFSET>, QueryDiffAreasForSnapshot::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssDifferentialSoftwareSnapshotMgmt as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssDifferentialSoftwareSnapshotMgmt2Impl: Sized + IVssDifferentialSoftwareSnapshotMgmtImpl {
    fn ChangeDiffAreaMaximumSizeEx();
    fn MigrateDiffAreas();
    fn QueryMigrationStatus();
    fn SetSnapshotPriority();
}
#[cfg(feature = "Win32_Foundation")]
impl IVssDifferentialSoftwareSnapshotMgmt2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssDifferentialSoftwareSnapshotMgmt2Vtbl {
        unsafe extern "system" fn ChangeDiffAreaMaximumSizeEx<Impl: IVssDifferentialSoftwareSnapshotMgmt2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MigrateDiffAreas<Impl: IVssDifferentialSoftwareSnapshotMgmt2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryMigrationStatus<Impl: IVssDifferentialSoftwareSnapshotMgmt2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSnapshotPriority<Impl: IVssDifferentialSoftwareSnapshotMgmt2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idsnapshot: ::windows::core::GUID, priority: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            AddDiffArea::<Impl, IMPL_OFFSET>,
            ChangeDiffAreaMaximumSize::<Impl, IMPL_OFFSET>,
            QueryVolumesSupportedForDiffAreas::<Impl, IMPL_OFFSET>,
            QueryDiffAreasForVolume::<Impl, IMPL_OFFSET>,
            QueryDiffAreasOnVolume::<Impl, IMPL_OFFSET>,
            QueryDiffAreasForSnapshot::<Impl, IMPL_OFFSET>,
            ChangeDiffAreaMaximumSizeEx::<Impl, IMPL_OFFSET>,
            MigrateDiffAreas::<Impl, IMPL_OFFSET>,
            QueryMigrationStatus::<Impl, IMPL_OFFSET>,
            SetSnapshotPriority::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssDifferentialSoftwareSnapshotMgmt2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssDifferentialSoftwareSnapshotMgmt3Impl: Sized + IVssDifferentialSoftwareSnapshotMgmt2Impl + IVssDifferentialSoftwareSnapshotMgmtImpl {
    fn SetVolumeProtectLevel();
    fn GetVolumeProtectLevel();
    fn ClearVolumeProtectFault();
    fn DeleteUnusedDiffAreas();
    fn QuerySnapshotDeltaBitmap();
}
#[cfg(feature = "Win32_Foundation")]
impl IVssDifferentialSoftwareSnapshotMgmt3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssDifferentialSoftwareSnapshotMgmt3Vtbl {
        unsafe extern "system" fn SetVolumeProtectLevel<Impl: IVssDifferentialSoftwareSnapshotMgmt3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, protectionlevel: VSS_PROTECTION_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVolumeProtectLevel<Impl: IVssDifferentialSoftwareSnapshotMgmt3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, protectionlevel: *mut VSS_VOLUME_PROTECTION_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearVolumeProtectFault<Impl: IVssDifferentialSoftwareSnapshotMgmt3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteUnusedDiffAreas<Impl: IVssDifferentialSoftwareSnapshotMgmt3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdiffareavolumename: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QuerySnapshotDeltaBitmap<Impl: IVssDifferentialSoftwareSnapshotMgmt3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idsnapshotolder: ::windows::core::GUID, idsnapshotyounger: ::windows::core::GUID, pcblocksizeperbit: *mut u32, pcbitmaplength: *mut u32, ppbbitmap: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            AddDiffArea::<Impl, IMPL_OFFSET>,
            ChangeDiffAreaMaximumSize::<Impl, IMPL_OFFSET>,
            QueryVolumesSupportedForDiffAreas::<Impl, IMPL_OFFSET>,
            QueryDiffAreasForVolume::<Impl, IMPL_OFFSET>,
            QueryDiffAreasOnVolume::<Impl, IMPL_OFFSET>,
            QueryDiffAreasForSnapshot::<Impl, IMPL_OFFSET>,
            ChangeDiffAreaMaximumSizeEx::<Impl, IMPL_OFFSET>,
            MigrateDiffAreas::<Impl, IMPL_OFFSET>,
            QueryMigrationStatus::<Impl, IMPL_OFFSET>,
            SetSnapshotPriority::<Impl, IMPL_OFFSET>,
            SetVolumeProtectLevel::<Impl, IMPL_OFFSET>,
            GetVolumeProtectLevel::<Impl, IMPL_OFFSET>,
            ClearVolumeProtectFault::<Impl, IMPL_OFFSET>,
            DeleteUnusedDiffAreas::<Impl, IMPL_OFFSET>,
            QuerySnapshotDeltaBitmap::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssDifferentialSoftwareSnapshotMgmt3 as ::windows::core::Interface>::IID
    }
}
pub trait IVssEnumMgmtObjectImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IVssEnumMgmtObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssEnumMgmtObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssEnumMgmtObjectVtbl {
        unsafe extern "system" fn Next<Impl: IVssEnumMgmtObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut VSS_MGMT_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IVssEnumMgmtObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IVssEnumMgmtObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IVssEnumMgmtObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssEnumMgmtObject as ::windows::core::Interface>::IID
    }
}
pub trait IVssEnumObjectImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IVssEnumObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssEnumObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssEnumObjectVtbl {
        unsafe extern "system" fn Next<Impl: IVssEnumObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut VSS_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IVssEnumObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IVssEnumObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IVssEnumObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssEnumObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssExpressWriterImpl: Sized {
    fn CreateMetadata();
    fn LoadMetadata();
    fn Register();
    fn Unregister();
}
#[cfg(feature = "Win32_Foundation")]
impl IVssExpressWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssExpressWriterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssExpressWriterVtbl {
        unsafe extern "system" fn CreateMetadata<Impl: IVssExpressWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writerid: ::windows::core::GUID, writername: super::super::Foundation::PWSTR, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32, ppmetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadMetadata<Impl: IVssExpressWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadata: super::super::Foundation::PWSTR, reserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Register<Impl: IVssExpressWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unregister<Impl: IVssExpressWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writerid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateMetadata::<Impl, IMPL_OFFSET>, LoadMetadata::<Impl, IMPL_OFFSET>, Register::<Impl, IMPL_OFFSET>, Unregister::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssExpressWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVssFileShareSnapshotProviderImpl: Sized {
    fn SetContext();
    fn GetSnapshotProperties();
    fn Query();
    fn DeleteSnapshots();
    fn BeginPrepareSnapshot();
    fn IsPathSupported();
    fn IsPathSnapshotted();
    fn SetSnapshotProperty();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVssFileShareSnapshotProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssFileShareSnapshotProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssFileShareSnapshotProviderVtbl {
        unsafe extern "system" fn SetContext<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcontext: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSnapshotProperties<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Query<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queriedobjectid: ::windows::core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteSnapshots<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceobjectid: ::windows::core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, pwszsharepath: *const u16, lnewcontext: i32, providerid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsPathSupported<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsharepath: *const u16, pbsupportedbythisprovider: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsPathSnapshotted<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsharepath: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSnapshotProperty<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetContext::<Impl, IMPL_OFFSET>, GetSnapshotProperties::<Impl, IMPL_OFFSET>, Query::<Impl, IMPL_OFFSET>, DeleteSnapshots::<Impl, IMPL_OFFSET>, BeginPrepareSnapshot::<Impl, IMPL_OFFSET>, IsPathSupported::<Impl, IMPL_OFFSET>, IsPathSnapshotted::<Impl, IMPL_OFFSET>, SetSnapshotProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssFileShareSnapshotProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
pub trait IVssHardwareSnapshotProviderImpl: Sized {
    fn AreLunsSupported();
    fn FillInLunInfo();
    fn BeginPrepareSnapshot();
    fn GetTargetLuns();
    fn LocateLuns();
    fn OnLunEmpty();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
impl IVssHardwareSnapshotProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssHardwareSnapshotProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssHardwareSnapshotProviderVtbl {
        unsafe extern "system" fn AreLunsSupported<Impl: IVssHardwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FillInLunInfo<Impl: IVssHardwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Impl: IVssHardwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTargetLuns<Impl: IVssHardwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocateLuns<Impl: IVssHardwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lluncount: i32, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnLunEmpty<Impl: IVssHardwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AreLunsSupported::<Impl, IMPL_OFFSET>, FillInLunInfo::<Impl, IMPL_OFFSET>, BeginPrepareSnapshot::<Impl, IMPL_OFFSET>, GetTargetLuns::<Impl, IMPL_OFFSET>, LocateLuns::<Impl, IMPL_OFFSET>, OnLunEmpty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssHardwareSnapshotProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
pub trait IVssHardwareSnapshotProviderExImpl: Sized + IVssHardwareSnapshotProviderImpl {
    fn GetProviderCapabilities();
    fn OnLunStateChange();
    fn ResyncLuns();
    fn OnReuseLuns();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
impl IVssHardwareSnapshotProviderExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssHardwareSnapshotProviderExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssHardwareSnapshotProviderExVtbl {
        unsafe extern "system" fn GetProviderCapabilities<Impl: IVssHardwareSnapshotProviderExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plloriginalcapabilitymask: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnLunStateChange<Impl: IVssHardwareSnapshotProviderExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResyncLuns<Impl: IVssHardwareSnapshotProviderExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnReuseLuns<Impl: IVssHardwareSnapshotProviderExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            AreLunsSupported::<Impl, IMPL_OFFSET>,
            FillInLunInfo::<Impl, IMPL_OFFSET>,
            BeginPrepareSnapshot::<Impl, IMPL_OFFSET>,
            GetTargetLuns::<Impl, IMPL_OFFSET>,
            LocateLuns::<Impl, IMPL_OFFSET>,
            OnLunEmpty::<Impl, IMPL_OFFSET>,
            GetProviderCapabilities::<Impl, IMPL_OFFSET>,
            OnLunStateChange::<Impl, IMPL_OFFSET>,
            ResyncLuns::<Impl, IMPL_OFFSET>,
            OnReuseLuns::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssHardwareSnapshotProviderEx as ::windows::core::Interface>::IID
    }
}
pub trait IVssProviderCreateSnapshotSetImpl: Sized {
    fn EndPrepareSnapshots();
    fn PreCommitSnapshots();
    fn CommitSnapshots();
    fn PostCommitSnapshots();
    fn PreFinalCommitSnapshots();
    fn PostFinalCommitSnapshots();
    fn AbortSnapshots();
}
impl IVssProviderCreateSnapshotSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssProviderCreateSnapshotSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssProviderCreateSnapshotSetVtbl {
        unsafe extern "system" fn EndPrepareSnapshots<Impl: IVssProviderCreateSnapshotSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreCommitSnapshots<Impl: IVssProviderCreateSnapshotSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CommitSnapshots<Impl: IVssProviderCreateSnapshotSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PostCommitSnapshots<Impl: IVssProviderCreateSnapshotSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID, lsnapshotscount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreFinalCommitSnapshots<Impl: IVssProviderCreateSnapshotSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PostFinalCommitSnapshots<Impl: IVssProviderCreateSnapshotSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AbortSnapshots<Impl: IVssProviderCreateSnapshotSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, EndPrepareSnapshots::<Impl, IMPL_OFFSET>, PreCommitSnapshots::<Impl, IMPL_OFFSET>, CommitSnapshots::<Impl, IMPL_OFFSET>, PostCommitSnapshots::<Impl, IMPL_OFFSET>, PreFinalCommitSnapshots::<Impl, IMPL_OFFSET>, PostFinalCommitSnapshots::<Impl, IMPL_OFFSET>, AbortSnapshots::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssProviderCreateSnapshotSet as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssProviderNotificationsImpl: Sized {
    fn OnLoad();
    fn OnUnload();
}
#[cfg(feature = "Win32_Foundation")]
impl IVssProviderNotificationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssProviderNotificationsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssProviderNotificationsVtbl {
        unsafe extern "system" fn OnLoad<Impl: IVssProviderNotificationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnUnload<Impl: IVssProviderNotificationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bforceunload: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnLoad::<Impl, IMPL_OFFSET>, OnUnload::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssProviderNotifications as ::windows::core::Interface>::IID
    }
}
pub trait IVssSnapshotMgmtImpl: Sized {
    fn GetProviderMgmtInterface();
    fn QueryVolumesSupportedForSnapshots();
    fn QuerySnapshotsByVolume();
}
impl IVssSnapshotMgmtVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssSnapshotMgmtImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssSnapshotMgmtVtbl {
        unsafe extern "system" fn GetProviderMgmtInterface<Impl: IVssSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, interfaceid: *const ::windows::core::GUID, ppitf: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryVolumesSupportedForSnapshots<Impl: IVssSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, lcontext: i32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QuerySnapshotsByVolume<Impl: IVssSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, providerid: ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetProviderMgmtInterface::<Impl, IMPL_OFFSET>, QueryVolumesSupportedForSnapshots::<Impl, IMPL_OFFSET>, QuerySnapshotsByVolume::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssSnapshotMgmt as ::windows::core::Interface>::IID
    }
}
pub trait IVssSnapshotMgmt2Impl: Sized {
    fn GetMinDiffAreaSize();
}
impl IVssSnapshotMgmt2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssSnapshotMgmt2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssSnapshotMgmt2Vtbl {
        unsafe extern "system" fn GetMinDiffAreaSize<Impl: IVssSnapshotMgmt2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllmindiffareasize: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetMinDiffAreaSize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssSnapshotMgmt2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVssSoftwareSnapshotProviderImpl: Sized {
    fn SetContext();
    fn GetSnapshotProperties();
    fn Query();
    fn DeleteSnapshots();
    fn BeginPrepareSnapshot();
    fn IsVolumeSupported();
    fn IsVolumeSnapshotted();
    fn SetSnapshotProperty();
    fn RevertToSnapshot();
    fn QueryRevertStatus();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVssSoftwareSnapshotProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssSoftwareSnapshotProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssSoftwareSnapshotProviderVtbl {
        unsafe extern "system" fn SetContext<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcontext: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSnapshotProperties<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Query<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queriedobjectid: ::windows::core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteSnapshots<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceobjectid: ::windows::core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, pwszvolumename: *const u16, lnewcontext: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsVolumeSupported<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pbsupportedbythisprovider: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsVolumeSnapshotted<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSnapshotProperty<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RevertToSnapshot<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryRevertStatus<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolume: *const u16, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetContext::<Impl, IMPL_OFFSET>,
            GetSnapshotProperties::<Impl, IMPL_OFFSET>,
            Query::<Impl, IMPL_OFFSET>,
            DeleteSnapshots::<Impl, IMPL_OFFSET>,
            BeginPrepareSnapshot::<Impl, IMPL_OFFSET>,
            IsVolumeSupported::<Impl, IMPL_OFFSET>,
            IsVolumeSnapshotted::<Impl, IMPL_OFFSET>,
            SetSnapshotProperty::<Impl, IMPL_OFFSET>,
            RevertToSnapshot::<Impl, IMPL_OFFSET>,
            QueryRevertStatus::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssSoftwareSnapshotProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssWMDependencyImpl: Sized {
    fn GetWriterId();
    fn GetLogicalPath();
    fn GetComponentName();
}
#[cfg(feature = "Win32_Foundation")]
impl IVssWMDependencyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssWMDependencyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssWMDependencyVtbl {
        unsafe extern "system" fn GetWriterId<Impl: IVssWMDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwriterid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLogicalPath<Impl: IVssWMDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlogicalpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetComponentName<Impl: IVssWMDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcomponentname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetWriterId::<Impl, IMPL_OFFSET>, GetLogicalPath::<Impl, IMPL_OFFSET>, GetComponentName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssWMDependency as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssWMFiledescImpl: Sized {
    fn GetPath();
    fn GetFilespec();
    fn GetRecursive();
    fn GetAlternateLocation();
    fn GetBackupTypeMask();
}
#[cfg(feature = "Win32_Foundation")]
impl IVssWMFiledescVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssWMFiledescImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssWMFiledescVtbl {
        unsafe extern "system" fn GetPath<Impl: IVssWMFiledescImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilespec<Impl: IVssWMFiledescImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfilespec: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRecursive<Impl: IVssWMFiledescImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbrecursive: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAlternateLocation<Impl: IVssWMFiledescImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstralternatelocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBackupTypeMask<Impl: IVssWMFiledescImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtypemask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPath::<Impl, IMPL_OFFSET>, GetFilespec::<Impl, IMPL_OFFSET>, GetRecursive::<Impl, IMPL_OFFSET>, GetAlternateLocation::<Impl, IMPL_OFFSET>, GetBackupTypeMask::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssWMFiledesc as ::windows::core::Interface>::IID
    }
}
pub trait IVssWriterComponentsImpl: Sized {
    fn GetComponentCount();
    fn GetWriterInfo();
    fn GetComponent();
}
impl IVssWriterComponentsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterComponentsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssWriterComponentsVtbl {
        unsafe extern "system" fn GetComponentCount<Impl: IVssWriterComponentsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccomponents: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWriterInfo<Impl: IVssWriterComponentsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidinstance: *mut ::windows::core::GUID, pidwriter: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetComponent<Impl: IVssWriterComponentsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icomponent: u32, ppcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(GetComponentCount::<Impl, IMPL_OFFSET>, GetWriterInfo::<Impl, IMPL_OFFSET>, GetComponent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssWriterComponents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssWriterImplImpl: Sized {
    fn Initialize();
    fn Subscribe();
    fn Unsubscribe();
    fn Uninitialize();
    fn GetCurrentVolumeArray();
    fn GetCurrentVolumeCount();
    fn GetSnapshotDeviceName();
    fn GetCurrentSnapshotSetId();
    fn GetContext();
    fn GetCurrentLevel();
    fn IsPathAffected();
    fn IsBootableSystemStateBackedUp();
    fn AreComponentsSelected();
    fn GetBackupType();
    fn GetRestoreType();
    fn SetWriterFailure();
    fn IsPartialFileSupportEnabled();
    fn InstallAlternateWriter();
    fn GetIdentityInformation();
    fn SetWriterFailureEx();
    fn GetSessionId();
    fn IsWriterShuttingDown();
}
#[cfg(feature = "Win32_Foundation")]
impl IVssWriterImplVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImplImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssWriterImplVtbl {
        unsafe extern "system" fn Initialize<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writerid: ::windows::core::GUID, wszwritername: super::super::Foundation::PWSTR, wszwriterinstancename: super::super::Foundation::PWSTR, dwmajorversion: u32, dwminorversion: u32, ut: VSS_USAGE_TYPE, st: VSS_SOURCE_TYPE, nlevel: VSS_APPLICATION_LEVEL, dwtimeout: u32, aws: VSS_ALTERNATE_WRITER_STATE, biothrottlingonly: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Subscribe<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsubscribetimeout: u32, dweventflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unsubscribe<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Uninitialize<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentVolumeArray<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut super::super::Foundation::PWSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentVolumeCount<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSnapshotDeviceName<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszoriginalvolume: super::super::Foundation::PWSTR, ppwszsnapshotdevice: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentSnapshotSetId<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContext<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentLevel<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> VSS_APPLICATION_LEVEL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsPathAffected<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsBootableSystemStateBackedUp<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AreComponentsSelected<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBackupType<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> VSS_BACKUP_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRestoreType<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> VSS_RESTORE_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWriterFailure<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsPartialFileSupportEnabled<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstallAlternateWriter<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idwriter: ::windows::core::GUID, clsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIdentityInformation<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut IVssExamineWriterMetadata {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWriterFailureEx<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, hrapplication: ::windows::core::HRESULT, wszapplicationmessage: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSessionId<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idsession: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsWriterShuttingDown<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            Subscribe::<Impl, IMPL_OFFSET>,
            Unsubscribe::<Impl, IMPL_OFFSET>,
            Uninitialize::<Impl, IMPL_OFFSET>,
            GetCurrentVolumeArray::<Impl, IMPL_OFFSET>,
            GetCurrentVolumeCount::<Impl, IMPL_OFFSET>,
            GetSnapshotDeviceName::<Impl, IMPL_OFFSET>,
            GetCurrentSnapshotSetId::<Impl, IMPL_OFFSET>,
            GetContext::<Impl, IMPL_OFFSET>,
            GetCurrentLevel::<Impl, IMPL_OFFSET>,
            IsPathAffected::<Impl, IMPL_OFFSET>,
            IsBootableSystemStateBackedUp::<Impl, IMPL_OFFSET>,
            AreComponentsSelected::<Impl, IMPL_OFFSET>,
            GetBackupType::<Impl, IMPL_OFFSET>,
            GetRestoreType::<Impl, IMPL_OFFSET>,
            SetWriterFailure::<Impl, IMPL_OFFSET>,
            IsPartialFileSupportEnabled::<Impl, IMPL_OFFSET>,
            InstallAlternateWriter::<Impl, IMPL_OFFSET>,
            GetIdentityInformation::<Impl, IMPL_OFFSET>,
            SetWriterFailureEx::<Impl, IMPL_OFFSET>,
            GetSessionId::<Impl, IMPL_OFFSET>,
            IsWriterShuttingDown::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssWriterImpl as ::windows::core::Interface>::IID
    }
}
