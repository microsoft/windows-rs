pub trait IVssAdminImpl: Sized {
    fn RegisterProvider(&mut self, pproviderid: ::windows::core::GUID, classid: ::windows::core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn UnregisterProvider(&mut self, providerid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn QueryProviders(&mut self) -> ::windows::core::Result<IVssEnumObject>;
    fn AbortAllSnapshotsInProgress(&mut self) -> ::windows::core::Result<()>;
}
impl IVssAdminVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssAdminImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssAdminVtbl {
        unsafe extern "system" fn RegisterProvider<Impl: IVssAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproviderid: ::windows::core::GUID, classid: ::windows::core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterProvider(::core::mem::transmute_copy(&pproviderid), ::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&pwszprovidername), ::core::mem::transmute_copy(&eprovidertype), ::core::mem::transmute_copy(&pwszproviderversion), ::core::mem::transmute_copy(&providerversionid)).into()
        }
        unsafe extern "system" fn UnregisterProvider<Impl: IVssAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterProvider(::core::mem::transmute_copy(&providerid)).into()
        }
        unsafe extern "system" fn QueryProviders<Impl: IVssAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryProviders() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbortAllSnapshotsInProgress<Impl: IVssAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AbortAllSnapshotsInProgress().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterProvider: RegisterProvider::<Impl, IMPL_OFFSET>,
            UnregisterProvider: UnregisterProvider::<Impl, IMPL_OFFSET>,
            QueryProviders: QueryProviders::<Impl, IMPL_OFFSET>,
            AbortAllSnapshotsInProgress: AbortAllSnapshotsInProgress::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssAdmin as ::windows::core::Interface>::IID
    }
}
pub trait IVssAdminExImpl: Sized + IVssAdminImpl {
    fn GetProviderCapability(&mut self, pproviderid: ::windows::core::GUID) -> ::windows::core::Result<u64>;
    fn GetProviderContext(&mut self, providerid: ::windows::core::GUID) -> ::windows::core::Result<i32>;
    fn SetProviderContext(&mut self, providerid: ::windows::core::GUID, lcontext: i32) -> ::windows::core::Result<()>;
}
impl IVssAdminExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssAdminExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssAdminExVtbl {
        unsafe extern "system" fn GetProviderCapability<Impl: IVssAdminExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproviderid: ::windows::core::GUID, plloriginalcapabilitymask: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderCapability(::core::mem::transmute_copy(&pproviderid)) {
                ::core::result::Result::Ok(ok__) => {
                    *plloriginalcapabilitymask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderContext<Impl: IVssAdminExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, plcontext: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderContext(::core::mem::transmute_copy(&providerid)) {
                ::core::result::Result::Ok(ok__) => {
                    *plcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderContext<Impl: IVssAdminExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, lcontext: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProviderContext(::core::mem::transmute_copy(&providerid), ::core::mem::transmute_copy(&lcontext)).into()
        }
        Self {
            base: IVssAdminVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetProviderCapability: GetProviderCapability::<Impl, IMPL_OFFSET>,
            GetProviderContext: GetProviderContext::<Impl, IMPL_OFFSET>,
            SetProviderContext: SetProviderContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssAdminEx as ::windows::core::Interface>::IID
    }
}
pub trait IVssAsyncImpl: Sized {
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
    fn Wait(&mut self, dwmilliseconds: u32) -> ::windows::core::Result<()>;
    fn QueryStatus(&mut self, phrresult: *mut ::windows::core::HRESULT, preserved: *mut i32) -> ::windows::core::Result<()>;
}
impl IVssAsyncVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssAsyncImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssAsyncVtbl {
        unsafe extern "system" fn Cancel<Impl: IVssAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn Wait<Impl: IVssAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Wait(::core::mem::transmute_copy(&dwmilliseconds)).into()
        }
        unsafe extern "system" fn QueryStatus<Impl: IVssAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT, preserved: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryStatus(::core::mem::transmute_copy(&phrresult), ::core::mem::transmute_copy(&preserved)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            Wait: Wait::<Impl, IMPL_OFFSET>,
            QueryStatus: QueryStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssAsync as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssComponentImpl: Sized {
    fn GetLogicalPath(&mut self, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetComponentType(&mut self, pct: *mut VSS_COMPONENT_TYPE) -> ::windows::core::Result<()>;
    fn GetComponentName(&mut self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetBackupSucceeded(&mut self, pbsucceeded: *mut bool) -> ::windows::core::Result<()>;
    fn GetAlternateLocationMappingCount(&mut self, pcmappings: *mut u32) -> ::windows::core::Result<()>;
    fn GetAlternateLocationMapping(&mut self, imapping: u32) -> ::windows::core::Result<IVssWMFiledesc>;
    fn SetBackupMetadata(&mut self, wszdata: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetBackupMetadata(&mut self, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddPartialFile(&mut self, wszpath: super::super::Foundation::PWSTR, wszfilename: super::super::Foundation::PWSTR, wszranges: super::super::Foundation::PWSTR, wszmetadata: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetPartialFileCount(&mut self, pcpartialfiles: *mut u32) -> ::windows::core::Result<()>;
    fn GetPartialFile(&mut self, ipartialfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilename: *mut super::super::Foundation::BSTR, pbstrrange: *mut super::super::Foundation::BSTR, pbstrmetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsSelectedForRestore(&mut self, pbselectedforrestore: *mut bool) -> ::windows::core::Result<()>;
    fn GetAdditionalRestores(&mut self, pbadditionalrestores: *mut bool) -> ::windows::core::Result<()>;
    fn GetNewTargetCount(&mut self, pcnewtarget: *mut u32) -> ::windows::core::Result<()>;
    fn GetNewTarget(&mut self, inewtarget: u32) -> ::windows::core::Result<IVssWMFiledesc>;
    fn AddDirectedTarget(&mut self, wszsourcepath: super::super::Foundation::PWSTR, wszsourcefilename: super::super::Foundation::PWSTR, wszsourcerangelist: super::super::Foundation::PWSTR, wszdestinationpath: super::super::Foundation::PWSTR, wszdestinationfilename: super::super::Foundation::PWSTR, wszdestinationrangelist: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetDirectedTargetCount(&mut self, pcdirectedtarget: *mut u32) -> ::windows::core::Result<()>;
    fn GetDirectedTarget(&mut self, idirectedtarget: u32, pbstrsourcepath: *mut super::super::Foundation::BSTR, pbstrsourcefilename: *mut super::super::Foundation::BSTR, pbstrsourcerangelist: *mut super::super::Foundation::BSTR, pbstrdestinationpath: *mut super::super::Foundation::BSTR, pbstrdestinationfilename: *mut super::super::Foundation::BSTR, pbstrdestinationrangelist: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetRestoreMetadata(&mut self, wszrestoremetadata: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetRestoreMetadata(&mut self, pbstrrestoremetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetRestoreTarget(&mut self, target: VSS_RESTORE_TARGET) -> ::windows::core::Result<()>;
    fn GetRestoreTarget(&mut self, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows::core::Result<()>;
    fn SetPreRestoreFailureMsg(&mut self, wszprerestorefailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetPreRestoreFailureMsg(&mut self, pbstrprerestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetPostRestoreFailureMsg(&mut self, wszpostrestorefailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetPostRestoreFailureMsg(&mut self, pbstrpostrestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetBackupStamp(&mut self, wszbackupstamp: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetBackupStamp(&mut self, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetPreviousBackupStamp(&mut self, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetBackupOptions(&mut self, pbstrbackupoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetRestoreOptions(&mut self, pbstrrestoreoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetRestoreSubcomponentCount(&mut self, pcrestoresubcomponent: *mut u32) -> ::windows::core::Result<()>;
    fn GetRestoreSubcomponent(&mut self, icomponent: u32, pbstrlogicalpath: *mut super::super::Foundation::BSTR, pbstrcomponentname: *mut super::super::Foundation::BSTR, pbrepair: *mut bool) -> ::windows::core::Result<()>;
    fn GetFileRestoreStatus(&mut self, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows::core::Result<()>;
    fn AddDifferencedFilesByLastModifyTime(&mut self, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: super::super::Foundation::BOOL, ftlastmodifytime: super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn AddDifferencedFilesByLastModifyLSN(&mut self, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: super::super::Foundation::BOOL, bstrlsnstring: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetDifferencedFilesCount(&mut self, pcdifferencedfiles: *mut u32) -> ::windows::core::Result<()>;
    fn GetDifferencedFile(&mut self, idifferencedfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilespec: *mut super::super::Foundation::BSTR, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut super::super::Foundation::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVssComponentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssComponentVtbl {
        unsafe extern "system" fn GetLogicalPath<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLogicalPath(::core::mem::transmute_copy(&pbstrpath)).into()
        }
        unsafe extern "system" fn GetComponentType<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pct: *mut VSS_COMPONENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetComponentType(::core::mem::transmute_copy(&pct)).into()
        }
        unsafe extern "system" fn GetComponentName<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetComponentName(::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn GetBackupSucceeded<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsucceeded: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBackupSucceeded(::core::mem::transmute_copy(&pbsucceeded)).into()
        }
        unsafe extern "system" fn GetAlternateLocationMappingCount<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcmappings: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAlternateLocationMappingCount(::core::mem::transmute_copy(&pcmappings)).into()
        }
        unsafe extern "system" fn GetAlternateLocationMapping<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imapping: u32, ppfiledesc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAlternateLocationMapping(::core::mem::transmute_copy(&imapping)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfiledesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackupMetadata<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszdata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackupMetadata(::core::mem::transmute_copy(&wszdata)).into()
        }
        unsafe extern "system" fn GetBackupMetadata<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBackupMetadata(::core::mem::transmute_copy(&pbstrdata)).into()
        }
        unsafe extern "system" fn AddPartialFile<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilename: super::super::Foundation::PWSTR, wszranges: super::super::Foundation::PWSTR, wszmetadata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPartialFile(::core::mem::transmute_copy(&wszpath), ::core::mem::transmute_copy(&wszfilename), ::core::mem::transmute_copy(&wszranges), ::core::mem::transmute_copy(&wszmetadata)).into()
        }
        unsafe extern "system" fn GetPartialFileCount<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcpartialfiles: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPartialFileCount(::core::mem::transmute_copy(&pcpartialfiles)).into()
        }
        unsafe extern "system" fn GetPartialFile<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipartialfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilename: *mut super::super::Foundation::BSTR, pbstrrange: *mut super::super::Foundation::BSTR, pbstrmetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPartialFile(::core::mem::transmute_copy(&ipartialfile), ::core::mem::transmute_copy(&pbstrpath), ::core::mem::transmute_copy(&pbstrfilename), ::core::mem::transmute_copy(&pbstrrange), ::core::mem::transmute_copy(&pbstrmetadata)).into()
        }
        unsafe extern "system" fn IsSelectedForRestore<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbselectedforrestore: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsSelectedForRestore(::core::mem::transmute_copy(&pbselectedforrestore)).into()
        }
        unsafe extern "system" fn GetAdditionalRestores<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbadditionalrestores: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAdditionalRestores(::core::mem::transmute_copy(&pbadditionalrestores)).into()
        }
        unsafe extern "system" fn GetNewTargetCount<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnewtarget: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNewTargetCount(::core::mem::transmute_copy(&pcnewtarget)).into()
        }
        unsafe extern "system" fn GetNewTarget<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inewtarget: u32, ppfiledesc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNewTarget(::core::mem::transmute_copy(&inewtarget)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfiledesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDirectedTarget<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszsourcepath: super::super::Foundation::PWSTR, wszsourcefilename: super::super::Foundation::PWSTR, wszsourcerangelist: super::super::Foundation::PWSTR, wszdestinationpath: super::super::Foundation::PWSTR, wszdestinationfilename: super::super::Foundation::PWSTR, wszdestinationrangelist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDirectedTarget(::core::mem::transmute_copy(&wszsourcepath), ::core::mem::transmute_copy(&wszsourcefilename), ::core::mem::transmute_copy(&wszsourcerangelist), ::core::mem::transmute_copy(&wszdestinationpath), ::core::mem::transmute_copy(&wszdestinationfilename), ::core::mem::transmute_copy(&wszdestinationrangelist)).into()
        }
        unsafe extern "system" fn GetDirectedTargetCount<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdirectedtarget: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDirectedTargetCount(::core::mem::transmute_copy(&pcdirectedtarget)).into()
        }
        unsafe extern "system" fn GetDirectedTarget<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idirectedtarget: u32, pbstrsourcepath: *mut super::super::Foundation::BSTR, pbstrsourcefilename: *mut super::super::Foundation::BSTR, pbstrsourcerangelist: *mut super::super::Foundation::BSTR, pbstrdestinationpath: *mut super::super::Foundation::BSTR, pbstrdestinationfilename: *mut super::super::Foundation::BSTR, pbstrdestinationrangelist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDirectedTarget(::core::mem::transmute_copy(&idirectedtarget), ::core::mem::transmute_copy(&pbstrsourcepath), ::core::mem::transmute_copy(&pbstrsourcefilename), ::core::mem::transmute_copy(&pbstrsourcerangelist), ::core::mem::transmute_copy(&pbstrdestinationpath), ::core::mem::transmute_copy(&pbstrdestinationfilename), ::core::mem::transmute_copy(&pbstrdestinationrangelist)).into()
        }
        unsafe extern "system" fn SetRestoreMetadata<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszrestoremetadata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRestoreMetadata(::core::mem::transmute_copy(&wszrestoremetadata)).into()
        }
        unsafe extern "system" fn GetRestoreMetadata<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrestoremetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRestoreMetadata(::core::mem::transmute_copy(&pbstrrestoremetadata)).into()
        }
        unsafe extern "system" fn SetRestoreTarget<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: VSS_RESTORE_TARGET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRestoreTarget(::core::mem::transmute_copy(&target)).into()
        }
        unsafe extern "system" fn GetRestoreTarget<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRestoreTarget(::core::mem::transmute_copy(&ptarget)).into()
        }
        unsafe extern "system" fn SetPreRestoreFailureMsg<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszprerestorefailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreRestoreFailureMsg(::core::mem::transmute_copy(&wszprerestorefailuremsg)).into()
        }
        unsafe extern "system" fn GetPreRestoreFailureMsg<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprerestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPreRestoreFailureMsg(::core::mem::transmute_copy(&pbstrprerestorefailuremsg)).into()
        }
        unsafe extern "system" fn SetPostRestoreFailureMsg<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpostrestorefailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPostRestoreFailureMsg(::core::mem::transmute_copy(&wszpostrestorefailuremsg)).into()
        }
        unsafe extern "system" fn GetPostRestoreFailureMsg<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpostrestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPostRestoreFailureMsg(::core::mem::transmute_copy(&pbstrpostrestorefailuremsg)).into()
        }
        unsafe extern "system" fn SetBackupStamp<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszbackupstamp: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackupStamp(::core::mem::transmute_copy(&wszbackupstamp)).into()
        }
        unsafe extern "system" fn GetBackupStamp<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBackupStamp(::core::mem::transmute_copy(&pbstrbackupstamp)).into()
        }
        unsafe extern "system" fn GetPreviousBackupStamp<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPreviousBackupStamp(::core::mem::transmute_copy(&pbstrbackupstamp)).into()
        }
        unsafe extern "system" fn GetBackupOptions<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbackupoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBackupOptions(::core::mem::transmute_copy(&pbstrbackupoptions)).into()
        }
        unsafe extern "system" fn GetRestoreOptions<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrestoreoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRestoreOptions(::core::mem::transmute_copy(&pbstrrestoreoptions)).into()
        }
        unsafe extern "system" fn GetRestoreSubcomponentCount<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcrestoresubcomponent: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRestoreSubcomponentCount(::core::mem::transmute_copy(&pcrestoresubcomponent)).into()
        }
        unsafe extern "system" fn GetRestoreSubcomponent<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icomponent: u32, pbstrlogicalpath: *mut super::super::Foundation::BSTR, pbstrcomponentname: *mut super::super::Foundation::BSTR, pbrepair: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRestoreSubcomponent(::core::mem::transmute_copy(&icomponent), ::core::mem::transmute_copy(&pbstrlogicalpath), ::core::mem::transmute_copy(&pbstrcomponentname), ::core::mem::transmute_copy(&pbrepair)).into()
        }
        unsafe extern "system" fn GetFileRestoreStatus<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFileRestoreStatus(::core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn AddDifferencedFilesByLastModifyTime<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: super::super::Foundation::BOOL, ftlastmodifytime: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDifferencedFilesByLastModifyTime(::core::mem::transmute_copy(&wszpath), ::core::mem::transmute_copy(&wszfilespec), ::core::mem::transmute_copy(&brecursive), ::core::mem::transmute_copy(&ftlastmodifytime)).into()
        }
        unsafe extern "system" fn AddDifferencedFilesByLastModifyLSN<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: super::super::Foundation::BOOL, bstrlsnstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDifferencedFilesByLastModifyLSN(::core::mem::transmute_copy(&wszpath), ::core::mem::transmute_copy(&wszfilespec), ::core::mem::transmute_copy(&brecursive), ::core::mem::transmute_copy(&bstrlsnstring)).into()
        }
        unsafe extern "system" fn GetDifferencedFilesCount<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdifferencedfiles: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDifferencedFilesCount(::core::mem::transmute_copy(&pcdifferencedfiles)).into()
        }
        unsafe extern "system" fn GetDifferencedFile<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idifferencedfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilespec: *mut super::super::Foundation::BSTR, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut super::super::Foundation::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDifferencedFile(::core::mem::transmute_copy(&idifferencedfile), ::core::mem::transmute_copy(&pbstrpath), ::core::mem::transmute_copy(&pbstrfilespec), ::core::mem::transmute_copy(&pbrecursive), ::core::mem::transmute_copy(&pbstrlsnstring), ::core::mem::transmute_copy(&pftlastmodifytime)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetLogicalPath: GetLogicalPath::<Impl, IMPL_OFFSET>,
            GetComponentType: GetComponentType::<Impl, IMPL_OFFSET>,
            GetComponentName: GetComponentName::<Impl, IMPL_OFFSET>,
            GetBackupSucceeded: GetBackupSucceeded::<Impl, IMPL_OFFSET>,
            GetAlternateLocationMappingCount: GetAlternateLocationMappingCount::<Impl, IMPL_OFFSET>,
            GetAlternateLocationMapping: GetAlternateLocationMapping::<Impl, IMPL_OFFSET>,
            SetBackupMetadata: SetBackupMetadata::<Impl, IMPL_OFFSET>,
            GetBackupMetadata: GetBackupMetadata::<Impl, IMPL_OFFSET>,
            AddPartialFile: AddPartialFile::<Impl, IMPL_OFFSET>,
            GetPartialFileCount: GetPartialFileCount::<Impl, IMPL_OFFSET>,
            GetPartialFile: GetPartialFile::<Impl, IMPL_OFFSET>,
            IsSelectedForRestore: IsSelectedForRestore::<Impl, IMPL_OFFSET>,
            GetAdditionalRestores: GetAdditionalRestores::<Impl, IMPL_OFFSET>,
            GetNewTargetCount: GetNewTargetCount::<Impl, IMPL_OFFSET>,
            GetNewTarget: GetNewTarget::<Impl, IMPL_OFFSET>,
            AddDirectedTarget: AddDirectedTarget::<Impl, IMPL_OFFSET>,
            GetDirectedTargetCount: GetDirectedTargetCount::<Impl, IMPL_OFFSET>,
            GetDirectedTarget: GetDirectedTarget::<Impl, IMPL_OFFSET>,
            SetRestoreMetadata: SetRestoreMetadata::<Impl, IMPL_OFFSET>,
            GetRestoreMetadata: GetRestoreMetadata::<Impl, IMPL_OFFSET>,
            SetRestoreTarget: SetRestoreTarget::<Impl, IMPL_OFFSET>,
            GetRestoreTarget: GetRestoreTarget::<Impl, IMPL_OFFSET>,
            SetPreRestoreFailureMsg: SetPreRestoreFailureMsg::<Impl, IMPL_OFFSET>,
            GetPreRestoreFailureMsg: GetPreRestoreFailureMsg::<Impl, IMPL_OFFSET>,
            SetPostRestoreFailureMsg: SetPostRestoreFailureMsg::<Impl, IMPL_OFFSET>,
            GetPostRestoreFailureMsg: GetPostRestoreFailureMsg::<Impl, IMPL_OFFSET>,
            SetBackupStamp: SetBackupStamp::<Impl, IMPL_OFFSET>,
            GetBackupStamp: GetBackupStamp::<Impl, IMPL_OFFSET>,
            GetPreviousBackupStamp: GetPreviousBackupStamp::<Impl, IMPL_OFFSET>,
            GetBackupOptions: GetBackupOptions::<Impl, IMPL_OFFSET>,
            GetRestoreOptions: GetRestoreOptions::<Impl, IMPL_OFFSET>,
            GetRestoreSubcomponentCount: GetRestoreSubcomponentCount::<Impl, IMPL_OFFSET>,
            GetRestoreSubcomponent: GetRestoreSubcomponent::<Impl, IMPL_OFFSET>,
            GetFileRestoreStatus: GetFileRestoreStatus::<Impl, IMPL_OFFSET>,
            AddDifferencedFilesByLastModifyTime: AddDifferencedFilesByLastModifyTime::<Impl, IMPL_OFFSET>,
            AddDifferencedFilesByLastModifyLSN: AddDifferencedFilesByLastModifyLSN::<Impl, IMPL_OFFSET>,
            GetDifferencedFilesCount: GetDifferencedFilesCount::<Impl, IMPL_OFFSET>,
            GetDifferencedFile: GetDifferencedFile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssComponent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssComponentExImpl: Sized + IVssComponentImpl {
    fn SetPrepareForBackupFailureMsg(&mut self, wszfailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetPostSnapshotFailureMsg(&mut self, wszfailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetPrepareForBackupFailureMsg(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetPostSnapshotFailureMsg(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetAuthoritativeRestore(&mut self) -> ::windows::core::Result<bool>;
    fn GetRollForward(&mut self, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetRestoreName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVssComponentExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponentExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssComponentExVtbl {
        unsafe extern "system" fn SetPrepareForBackupFailureMsg<Impl: IVssComponentExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszfailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrepareForBackupFailureMsg(::core::mem::transmute_copy(&wszfailuremsg)).into()
        }
        unsafe extern "system" fn SetPostSnapshotFailureMsg<Impl: IVssComponentExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszfailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPostSnapshotFailureMsg(::core::mem::transmute_copy(&wszfailuremsg)).into()
        }
        unsafe extern "system" fn GetPrepareForBackupFailureMsg<Impl: IVssComponentExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrepareForBackupFailureMsg() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrfailuremsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPostSnapshotFailureMsg<Impl: IVssComponentExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPostSnapshotFailureMsg() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrfailuremsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthoritativeRestore<Impl: IVssComponentExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbauth: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAuthoritativeRestore() {
                ::core::result::Result::Ok(ok__) => {
                    *pbauth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRollForward<Impl: IVssComponentExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRollForward(::core::mem::transmute_copy(&prolltype), ::core::mem::transmute_copy(&pbstrpoint)).into()
        }
        unsafe extern "system" fn GetRestoreName<Impl: IVssComponentExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRestoreName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IVssComponentVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetPrepareForBackupFailureMsg: SetPrepareForBackupFailureMsg::<Impl, IMPL_OFFSET>,
            SetPostSnapshotFailureMsg: SetPostSnapshotFailureMsg::<Impl, IMPL_OFFSET>,
            GetPrepareForBackupFailureMsg: GetPrepareForBackupFailureMsg::<Impl, IMPL_OFFSET>,
            GetPostSnapshotFailureMsg: GetPostSnapshotFailureMsg::<Impl, IMPL_OFFSET>,
            GetAuthoritativeRestore: GetAuthoritativeRestore::<Impl, IMPL_OFFSET>,
            GetRollForward: GetRollForward::<Impl, IMPL_OFFSET>,
            GetRestoreName: GetRestoreName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssComponentEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssComponentEx2Impl: Sized + IVssComponentImpl + IVssComponentExImpl {
    fn SetFailure(&mut self, hr: ::windows::core::HRESULT, hrapplication: ::windows::core::HRESULT, wszapplicationmessage: super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::Result<()>;
    fn GetFailure(&mut self, phr: *mut ::windows::core::HRESULT, phrapplication: *mut ::windows::core::HRESULT, pbstrapplicationmessage: *mut super::super::Foundation::BSTR, pdwreserved: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVssComponentEx2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponentEx2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssComponentEx2Vtbl {
        unsafe extern "system" fn SetFailure<Impl: IVssComponentEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, hrapplication: ::windows::core::HRESULT, wszapplicationmessage: super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFailure(::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&hrapplication), ::core::mem::transmute_copy(&wszapplicationmessage), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn GetFailure<Impl: IVssComponentEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phr: *mut ::windows::core::HRESULT, phrapplication: *mut ::windows::core::HRESULT, pbstrapplicationmessage: *mut super::super::Foundation::BSTR, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFailure(::core::mem::transmute_copy(&phr), ::core::mem::transmute_copy(&phrapplication), ::core::mem::transmute_copy(&pbstrapplicationmessage), ::core::mem::transmute_copy(&pdwreserved)).into()
        }
        Self {
            base: IVssComponentExVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetFailure: SetFailure::<Impl, IMPL_OFFSET>,
            GetFailure: GetFailure::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssComponentEx2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssCreateExpressWriterMetadataImpl: Sized {
    fn AddExcludeFiles(&mut self, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8) -> ::windows::core::Result<()>;
    fn AddComponent(&mut self, ct: VSS_COMPONENT_TYPE, wszlogicalpath: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcaption: super::super::Foundation::PWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows::core::Result<()>;
    fn AddFilesToFileGroup(&mut self, wszlogicalpath: super::super::Foundation::PWSTR, wszgroupname: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8, wszalternatelocation: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::Result<()>;
    fn SetRestoreMethod(&mut self, method: VSS_RESTOREMETHOD_ENUM, wszservice: super::super::Foundation::PWSTR, wszuserprocedure: super::super::Foundation::PWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows::core::Result<()>;
    fn AddComponentDependency(&mut self, wszforlogicalpath: super::super::Foundation::PWSTR, wszforcomponentname: super::super::Foundation::PWSTR, onwriterid: ::windows::core::GUID, wszonlogicalpath: super::super::Foundation::PWSTR, wszoncomponentname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetBackupSchema(&mut self, dwschemamask: u32) -> ::windows::core::Result<()>;
    fn SaveAsXML(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVssCreateExpressWriterMetadataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateExpressWriterMetadataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssCreateExpressWriterMetadataVtbl {
        unsafe extern "system" fn AddExcludeFiles<Impl: IVssCreateExpressWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddExcludeFiles(::core::mem::transmute_copy(&wszpath), ::core::mem::transmute_copy(&wszfilespec), ::core::mem::transmute_copy(&brecursive)).into()
        }
        unsafe extern "system" fn AddComponent<Impl: IVssCreateExpressWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ct: VSS_COMPONENT_TYPE, wszlogicalpath: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcaption: super::super::Foundation::PWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .AddComponent(
                    ::core::mem::transmute_copy(&ct),
                    ::core::mem::transmute_copy(&wszlogicalpath),
                    ::core::mem::transmute_copy(&wszcomponentname),
                    ::core::mem::transmute_copy(&wszcaption),
                    ::core::mem::transmute_copy(&pbicon),
                    ::core::mem::transmute_copy(&cbicon),
                    ::core::mem::transmute_copy(&brestoremetadata),
                    ::core::mem::transmute_copy(&bnotifyonbackupcomplete),
                    ::core::mem::transmute_copy(&bselectable),
                    ::core::mem::transmute_copy(&bselectableforrestore),
                    ::core::mem::transmute_copy(&dwcomponentflags),
                )
                .into()
        }
        unsafe extern "system" fn AddFilesToFileGroup<Impl: IVssCreateExpressWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlogicalpath: super::super::Foundation::PWSTR, wszgroupname: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8, wszalternatelocation: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddFilesToFileGroup(::core::mem::transmute_copy(&wszlogicalpath), ::core::mem::transmute_copy(&wszgroupname), ::core::mem::transmute_copy(&wszpath), ::core::mem::transmute_copy(&wszfilespec), ::core::mem::transmute_copy(&brecursive), ::core::mem::transmute_copy(&wszalternatelocation), ::core::mem::transmute_copy(&dwbackuptypemask)).into()
        }
        unsafe extern "system" fn SetRestoreMethod<Impl: IVssCreateExpressWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: VSS_RESTOREMETHOD_ENUM, wszservice: super::super::Foundation::PWSTR, wszuserprocedure: super::super::Foundation::PWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRestoreMethod(::core::mem::transmute_copy(&method), ::core::mem::transmute_copy(&wszservice), ::core::mem::transmute_copy(&wszuserprocedure), ::core::mem::transmute_copy(&writerrestore), ::core::mem::transmute_copy(&brebootrequired)).into()
        }
        unsafe extern "system" fn AddComponentDependency<Impl: IVssCreateExpressWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszforlogicalpath: super::super::Foundation::PWSTR, wszforcomponentname: super::super::Foundation::PWSTR, onwriterid: ::windows::core::GUID, wszonlogicalpath: super::super::Foundation::PWSTR, wszoncomponentname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddComponentDependency(::core::mem::transmute_copy(&wszforlogicalpath), ::core::mem::transmute_copy(&wszforcomponentname), ::core::mem::transmute_copy(&onwriterid), ::core::mem::transmute_copy(&wszonlogicalpath), ::core::mem::transmute_copy(&wszoncomponentname)).into()
        }
        unsafe extern "system" fn SetBackupSchema<Impl: IVssCreateExpressWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwschemamask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackupSchema(::core::mem::transmute_copy(&dwschemamask)).into()
        }
        unsafe extern "system" fn SaveAsXML<Impl: IVssCreateExpressWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveAsXML() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrxml = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddExcludeFiles: AddExcludeFiles::<Impl, IMPL_OFFSET>,
            AddComponent: AddComponent::<Impl, IMPL_OFFSET>,
            AddFilesToFileGroup: AddFilesToFileGroup::<Impl, IMPL_OFFSET>,
            SetRestoreMethod: SetRestoreMethod::<Impl, IMPL_OFFSET>,
            AddComponentDependency: AddComponentDependency::<Impl, IMPL_OFFSET>,
            SetBackupSchema: SetBackupSchema::<Impl, IMPL_OFFSET>,
            SaveAsXML: SaveAsXML::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssCreateExpressWriterMetadata as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IVssCreateWriterMetadataImpl: Sized {
    fn AddIncludeFiles(&mut self, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8, wszalternatelocation: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn AddExcludeFiles(&mut self, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8) -> ::windows::core::Result<()>;
    fn AddComponent(&mut self, ct: VSS_COMPONENT_TYPE, wszlogicalpath: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcaption: super::super::Foundation::PWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows::core::Result<()>;
    fn AddDatabaseFiles(&mut self, wszlogicalpath: super::super::Foundation::PWSTR, wszdatabasename: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::Result<()>;
    fn AddDatabaseLogFiles(&mut self, wszlogicalpath: super::super::Foundation::PWSTR, wszdatabasename: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::Result<()>;
    fn AddFilesToFileGroup(&mut self, wszlogicalpath: super::super::Foundation::PWSTR, wszgroupname: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8, wszalternatelocation: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::Result<()>;
    fn SetRestoreMethod(&mut self, method: VSS_RESTOREMETHOD_ENUM, wszservice: super::super::Foundation::PWSTR, wszuserprocedure: super::super::Foundation::PWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows::core::Result<()>;
    fn AddAlternateLocationMapping(&mut self, wszsourcepath: super::super::Foundation::PWSTR, wszsourcefilespec: super::super::Foundation::PWSTR, brecursive: u8, wszdestination: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn AddComponentDependency(&mut self, wszforlogicalpath: super::super::Foundation::PWSTR, wszforcomponentname: super::super::Foundation::PWSTR, onwriterid: ::windows::core::GUID, wszonlogicalpath: super::super::Foundation::PWSTR, wszoncomponentname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetBackupSchema(&mut self, dwschemamask: u32) -> ::windows::core::Result<()>;
    fn GetDocument(&mut self) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument>;
    fn SaveAsXML(&mut self, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IVssCreateWriterMetadataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateWriterMetadataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssCreateWriterMetadataVtbl {
        unsafe extern "system" fn AddIncludeFiles<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8, wszalternatelocation: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddIncludeFiles(::core::mem::transmute_copy(&wszpath), ::core::mem::transmute_copy(&wszfilespec), ::core::mem::transmute_copy(&brecursive), ::core::mem::transmute_copy(&wszalternatelocation)).into()
        }
        unsafe extern "system" fn AddExcludeFiles<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddExcludeFiles(::core::mem::transmute_copy(&wszpath), ::core::mem::transmute_copy(&wszfilespec), ::core::mem::transmute_copy(&brecursive)).into()
        }
        unsafe extern "system" fn AddComponent<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ct: VSS_COMPONENT_TYPE, wszlogicalpath: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcaption: super::super::Foundation::PWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .AddComponent(
                    ::core::mem::transmute_copy(&ct),
                    ::core::mem::transmute_copy(&wszlogicalpath),
                    ::core::mem::transmute_copy(&wszcomponentname),
                    ::core::mem::transmute_copy(&wszcaption),
                    ::core::mem::transmute_copy(&pbicon),
                    ::core::mem::transmute_copy(&cbicon),
                    ::core::mem::transmute_copy(&brestoremetadata),
                    ::core::mem::transmute_copy(&bnotifyonbackupcomplete),
                    ::core::mem::transmute_copy(&bselectable),
                    ::core::mem::transmute_copy(&bselectableforrestore),
                    ::core::mem::transmute_copy(&dwcomponentflags),
                )
                .into()
        }
        unsafe extern "system" fn AddDatabaseFiles<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlogicalpath: super::super::Foundation::PWSTR, wszdatabasename: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDatabaseFiles(::core::mem::transmute_copy(&wszlogicalpath), ::core::mem::transmute_copy(&wszdatabasename), ::core::mem::transmute_copy(&wszpath), ::core::mem::transmute_copy(&wszfilespec), ::core::mem::transmute_copy(&dwbackuptypemask)).into()
        }
        unsafe extern "system" fn AddDatabaseLogFiles<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlogicalpath: super::super::Foundation::PWSTR, wszdatabasename: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDatabaseLogFiles(::core::mem::transmute_copy(&wszlogicalpath), ::core::mem::transmute_copy(&wszdatabasename), ::core::mem::transmute_copy(&wszpath), ::core::mem::transmute_copy(&wszfilespec), ::core::mem::transmute_copy(&dwbackuptypemask)).into()
        }
        unsafe extern "system" fn AddFilesToFileGroup<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlogicalpath: super::super::Foundation::PWSTR, wszgroupname: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8, wszalternatelocation: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddFilesToFileGroup(::core::mem::transmute_copy(&wszlogicalpath), ::core::mem::transmute_copy(&wszgroupname), ::core::mem::transmute_copy(&wszpath), ::core::mem::transmute_copy(&wszfilespec), ::core::mem::transmute_copy(&brecursive), ::core::mem::transmute_copy(&wszalternatelocation), ::core::mem::transmute_copy(&dwbackuptypemask)).into()
        }
        unsafe extern "system" fn SetRestoreMethod<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: VSS_RESTOREMETHOD_ENUM, wszservice: super::super::Foundation::PWSTR, wszuserprocedure: super::super::Foundation::PWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRestoreMethod(::core::mem::transmute_copy(&method), ::core::mem::transmute_copy(&wszservice), ::core::mem::transmute_copy(&wszuserprocedure), ::core::mem::transmute_copy(&writerrestore), ::core::mem::transmute_copy(&brebootrequired)).into()
        }
        unsafe extern "system" fn AddAlternateLocationMapping<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszsourcepath: super::super::Foundation::PWSTR, wszsourcefilespec: super::super::Foundation::PWSTR, brecursive: u8, wszdestination: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAlternateLocationMapping(::core::mem::transmute_copy(&wszsourcepath), ::core::mem::transmute_copy(&wszsourcefilespec), ::core::mem::transmute_copy(&brecursive), ::core::mem::transmute_copy(&wszdestination)).into()
        }
        unsafe extern "system" fn AddComponentDependency<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszforlogicalpath: super::super::Foundation::PWSTR, wszforcomponentname: super::super::Foundation::PWSTR, onwriterid: ::windows::core::GUID, wszonlogicalpath: super::super::Foundation::PWSTR, wszoncomponentname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddComponentDependency(::core::mem::transmute_copy(&wszforlogicalpath), ::core::mem::transmute_copy(&wszforcomponentname), ::core::mem::transmute_copy(&onwriterid), ::core::mem::transmute_copy(&wszonlogicalpath), ::core::mem::transmute_copy(&wszoncomponentname)).into()
        }
        unsafe extern "system" fn SetBackupSchema<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwschemamask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackupSchema(::core::mem::transmute_copy(&dwschemamask)).into()
        }
        unsafe extern "system" fn GetDocument<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdoc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocument() {
                ::core::result::Result::Ok(ok__) => {
                    *pdoc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAsXML<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveAsXML(::core::mem::transmute_copy(&pbstrxml)).into()
        }
        Self {
            AddIncludeFiles: AddIncludeFiles::<Impl, IMPL_OFFSET>,
            AddExcludeFiles: AddExcludeFiles::<Impl, IMPL_OFFSET>,
            AddComponent: AddComponent::<Impl, IMPL_OFFSET>,
            AddDatabaseFiles: AddDatabaseFiles::<Impl, IMPL_OFFSET>,
            AddDatabaseLogFiles: AddDatabaseLogFiles::<Impl, IMPL_OFFSET>,
            AddFilesToFileGroup: AddFilesToFileGroup::<Impl, IMPL_OFFSET>,
            SetRestoreMethod: SetRestoreMethod::<Impl, IMPL_OFFSET>,
            AddAlternateLocationMapping: AddAlternateLocationMapping::<Impl, IMPL_OFFSET>,
            AddComponentDependency: AddComponentDependency::<Impl, IMPL_OFFSET>,
            SetBackupSchema: SetBackupSchema::<Impl, IMPL_OFFSET>,
            GetDocument: GetDocument::<Impl, IMPL_OFFSET>,
            SaveAsXML: SaveAsXML::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssCreateWriterMetadata as ::windows::core::Interface>::IID
    }
}
pub trait IVssDifferentialSoftwareSnapshotMgmtImpl: Sized {
    fn AddDiffArea(&mut self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::Result<()>;
    fn ChangeDiffAreaMaximumSize(&mut self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::Result<()>;
    fn QueryVolumesSupportedForDiffAreas(&mut self, pwszoriginalvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject>;
    fn QueryDiffAreasForVolume(&mut self, pwszvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject>;
    fn QueryDiffAreasOnVolume(&mut self, pwszvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject>;
    fn QueryDiffAreasForSnapshot(&mut self, snapshotid: ::windows::core::GUID) -> ::windows::core::Result<IVssEnumMgmtObject>;
}
impl IVssDifferentialSoftwareSnapshotMgmtVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmtImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssDifferentialSoftwareSnapshotMgmtVtbl {
        unsafe extern "system" fn AddDiffArea<Impl: IVssDifferentialSoftwareSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDiffArea(::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&pwszdiffareavolumename), ::core::mem::transmute_copy(&llmaximumdiffspace)).into()
        }
        unsafe extern "system" fn ChangeDiffAreaMaximumSize<Impl: IVssDifferentialSoftwareSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangeDiffAreaMaximumSize(::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&pwszdiffareavolumename), ::core::mem::transmute_copy(&llmaximumdiffspace)).into()
        }
        unsafe extern "system" fn QueryVolumesSupportedForDiffAreas<Impl: IVssDifferentialSoftwareSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszoriginalvolumename: *const u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryVolumesSupportedForDiffAreas(::core::mem::transmute_copy(&pwszoriginalvolumename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDiffAreasForVolume<Impl: IVssDifferentialSoftwareSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryDiffAreasForVolume(::core::mem::transmute_copy(&pwszvolumename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDiffAreasOnVolume<Impl: IVssDifferentialSoftwareSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryDiffAreasOnVolume(::core::mem::transmute_copy(&pwszvolumename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDiffAreasForSnapshot<Impl: IVssDifferentialSoftwareSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryDiffAreasForSnapshot(::core::mem::transmute_copy(&snapshotid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddDiffArea: AddDiffArea::<Impl, IMPL_OFFSET>,
            ChangeDiffAreaMaximumSize: ChangeDiffAreaMaximumSize::<Impl, IMPL_OFFSET>,
            QueryVolumesSupportedForDiffAreas: QueryVolumesSupportedForDiffAreas::<Impl, IMPL_OFFSET>,
            QueryDiffAreasForVolume: QueryDiffAreasForVolume::<Impl, IMPL_OFFSET>,
            QueryDiffAreasOnVolume: QueryDiffAreasOnVolume::<Impl, IMPL_OFFSET>,
            QueryDiffAreasForSnapshot: QueryDiffAreasForSnapshot::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssDifferentialSoftwareSnapshotMgmt as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssDifferentialSoftwareSnapshotMgmt2Impl: Sized + IVssDifferentialSoftwareSnapshotMgmtImpl {
    fn ChangeDiffAreaMaximumSizeEx(&mut self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn MigrateDiffAreas(&mut self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> ::windows::core::Result<()>;
    fn QueryMigrationStatus(&mut self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16) -> ::windows::core::Result<IVssAsync>;
    fn SetSnapshotPriority(&mut self, idsnapshot: ::windows::core::GUID, priority: u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVssDifferentialSoftwareSnapshotMgmt2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssDifferentialSoftwareSnapshotMgmt2Vtbl {
        unsafe extern "system" fn ChangeDiffAreaMaximumSizeEx<Impl: IVssDifferentialSoftwareSnapshotMgmt2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangeDiffAreaMaximumSizeEx(::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&pwszdiffareavolumename), ::core::mem::transmute_copy(&llmaximumdiffspace), ::core::mem::transmute_copy(&bvolatile)).into()
        }
        unsafe extern "system" fn MigrateDiffAreas<Impl: IVssDifferentialSoftwareSnapshotMgmt2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MigrateDiffAreas(::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&pwszdiffareavolumename), ::core::mem::transmute_copy(&pwsznewdiffareavolumename)).into()
        }
        unsafe extern "system" fn QueryMigrationStatus<Impl: IVssDifferentialSoftwareSnapshotMgmt2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryMigrationStatus(::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&pwszdiffareavolumename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSnapshotPriority<Impl: IVssDifferentialSoftwareSnapshotMgmt2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idsnapshot: ::windows::core::GUID, priority: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSnapshotPriority(::core::mem::transmute_copy(&idsnapshot), ::core::mem::transmute_copy(&priority)).into()
        }
        Self {
            base: IVssDifferentialSoftwareSnapshotMgmtVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ChangeDiffAreaMaximumSizeEx: ChangeDiffAreaMaximumSizeEx::<Impl, IMPL_OFFSET>,
            MigrateDiffAreas: MigrateDiffAreas::<Impl, IMPL_OFFSET>,
            QueryMigrationStatus: QueryMigrationStatus::<Impl, IMPL_OFFSET>,
            SetSnapshotPriority: SetSnapshotPriority::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssDifferentialSoftwareSnapshotMgmt2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssDifferentialSoftwareSnapshotMgmt3Impl: Sized + IVssDifferentialSoftwareSnapshotMgmtImpl + IVssDifferentialSoftwareSnapshotMgmt2Impl {
    fn SetVolumeProtectLevel(&mut self, pwszvolumename: *const u16, protectionlevel: VSS_PROTECTION_LEVEL) -> ::windows::core::Result<()>;
    fn GetVolumeProtectLevel(&mut self, pwszvolumename: *const u16) -> ::windows::core::Result<VSS_VOLUME_PROTECTION_INFO>;
    fn ClearVolumeProtectFault(&mut self, pwszvolumename: *const u16) -> ::windows::core::Result<()>;
    fn DeleteUnusedDiffAreas(&mut self, pwszdiffareavolumename: *const u16) -> ::windows::core::Result<()>;
    fn QuerySnapshotDeltaBitmap(&mut self, idsnapshotolder: ::windows::core::GUID, idsnapshotyounger: ::windows::core::GUID, pcblocksizeperbit: *mut u32, pcbitmaplength: *mut u32, ppbbitmap: *mut *mut u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVssDifferentialSoftwareSnapshotMgmt3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssDifferentialSoftwareSnapshotMgmt3Vtbl {
        unsafe extern "system" fn SetVolumeProtectLevel<Impl: IVssDifferentialSoftwareSnapshotMgmt3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, protectionlevel: VSS_PROTECTION_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolumeProtectLevel(::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&protectionlevel)).into()
        }
        unsafe extern "system" fn GetVolumeProtectLevel<Impl: IVssDifferentialSoftwareSnapshotMgmt3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, protectionlevel: *mut VSS_VOLUME_PROTECTION_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVolumeProtectLevel(::core::mem::transmute_copy(&pwszvolumename)) {
                ::core::result::Result::Ok(ok__) => {
                    *protectionlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearVolumeProtectFault<Impl: IVssDifferentialSoftwareSnapshotMgmt3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearVolumeProtectFault(::core::mem::transmute_copy(&pwszvolumename)).into()
        }
        unsafe extern "system" fn DeleteUnusedDiffAreas<Impl: IVssDifferentialSoftwareSnapshotMgmt3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdiffareavolumename: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteUnusedDiffAreas(::core::mem::transmute_copy(&pwszdiffareavolumename)).into()
        }
        unsafe extern "system" fn QuerySnapshotDeltaBitmap<Impl: IVssDifferentialSoftwareSnapshotMgmt3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idsnapshotolder: ::windows::core::GUID, idsnapshotyounger: ::windows::core::GUID, pcblocksizeperbit: *mut u32, pcbitmaplength: *mut u32, ppbbitmap: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QuerySnapshotDeltaBitmap(::core::mem::transmute_copy(&idsnapshotolder), ::core::mem::transmute_copy(&idsnapshotyounger), ::core::mem::transmute_copy(&pcblocksizeperbit), ::core::mem::transmute_copy(&pcbitmaplength), ::core::mem::transmute_copy(&ppbbitmap)).into()
        }
        Self {
            base: IVssDifferentialSoftwareSnapshotMgmt2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetVolumeProtectLevel: SetVolumeProtectLevel::<Impl, IMPL_OFFSET>,
            GetVolumeProtectLevel: GetVolumeProtectLevel::<Impl, IMPL_OFFSET>,
            ClearVolumeProtectFault: ClearVolumeProtectFault::<Impl, IMPL_OFFSET>,
            DeleteUnusedDiffAreas: DeleteUnusedDiffAreas::<Impl, IMPL_OFFSET>,
            QuerySnapshotDeltaBitmap: QuerySnapshotDeltaBitmap::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssDifferentialSoftwareSnapshotMgmt3 as ::windows::core::Interface>::IID
    }
}
pub trait IVssEnumMgmtObjectImpl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut VSS_MGMT_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self, ppenum: *mut ::core::option::Option<IVssEnumMgmtObject>) -> ::windows::core::Result<()>;
}
impl IVssEnumMgmtObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssEnumMgmtObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssEnumMgmtObjectVtbl {
        unsafe extern "system" fn Next<Impl: IVssEnumMgmtObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut VSS_MGMT_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IVssEnumMgmtObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IVssEnumMgmtObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IVssEnumMgmtObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clone(::core::mem::transmute_copy(&ppenum)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssEnumMgmtObject as ::windows::core::Interface>::IID
    }
}
pub trait IVssEnumObjectImpl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut VSS_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self, ppenum: *mut ::core::option::Option<IVssEnumObject>) -> ::windows::core::Result<()>;
}
impl IVssEnumObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssEnumObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssEnumObjectVtbl {
        unsafe extern "system" fn Next<Impl: IVssEnumObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut VSS_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IVssEnumObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IVssEnumObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IVssEnumObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clone(::core::mem::transmute_copy(&ppenum)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssEnumObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssExpressWriterImpl: Sized {
    fn CreateMetadata(&mut self, writerid: ::windows::core::GUID, writername: super::super::Foundation::PWSTR, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32) -> ::windows::core::Result<IVssCreateExpressWriterMetadata>;
    fn LoadMetadata(&mut self, metadata: super::super::Foundation::PWSTR, reserved: u32) -> ::windows::core::Result<()>;
    fn Register(&mut self) -> ::windows::core::Result<()>;
    fn Unregister(&mut self, writerid: ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVssExpressWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssExpressWriterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssExpressWriterVtbl {
        unsafe extern "system" fn CreateMetadata<Impl: IVssExpressWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writerid: ::windows::core::GUID, writername: super::super::Foundation::PWSTR, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32, ppmetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMetadata(::core::mem::transmute_copy(&writerid), ::core::mem::transmute_copy(&writername), ::core::mem::transmute_copy(&usagetype), ::core::mem::transmute_copy(&versionmajor), ::core::mem::transmute_copy(&versionminor), ::core::mem::transmute_copy(&reserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmetadata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadMetadata<Impl: IVssExpressWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadata: super::super::Foundation::PWSTR, reserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadMetadata(::core::mem::transmute_copy(&metadata), ::core::mem::transmute_copy(&reserved)).into()
        }
        unsafe extern "system" fn Register<Impl: IVssExpressWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Register().into()
        }
        unsafe extern "system" fn Unregister<Impl: IVssExpressWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writerid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unregister(::core::mem::transmute_copy(&writerid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateMetadata: CreateMetadata::<Impl, IMPL_OFFSET>,
            LoadMetadata: LoadMetadata::<Impl, IMPL_OFFSET>,
            Register: Register::<Impl, IMPL_OFFSET>,
            Unregister: Unregister::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssExpressWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVssFileShareSnapshotProviderImpl: Sized {
    fn SetContext(&mut self, lcontext: i32) -> ::windows::core::Result<()>;
    fn GetSnapshotProperties(&mut self, snapshotid: ::windows::core::GUID) -> ::windows::core::Result<VSS_SNAPSHOT_PROP>;
    fn Query(&mut self, queriedobjectid: ::windows::core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE) -> ::windows::core::Result<IVssEnumObject>;
    fn DeleteSnapshots(&mut self, sourceobjectid: ::windows::core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn BeginPrepareSnapshot(&mut self, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, pwszsharepath: *const u16, lnewcontext: i32, providerid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn IsPathSupported(&mut self, pwszsharepath: *const u16) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsPathSnapshotted(&mut self, pwszsharepath: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows::core::Result<()>;
    fn SetSnapshotProperty(&mut self, snapshotid: ::windows::core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVssFileShareSnapshotProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssFileShareSnapshotProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssFileShareSnapshotProviderVtbl {
        unsafe extern "system" fn SetContext<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcontext: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContext(::core::mem::transmute_copy(&lcontext)).into()
        }
        unsafe extern "system" fn GetSnapshotProperties<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSnapshotProperties(::core::mem::transmute_copy(&snapshotid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Query<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queriedobjectid: ::windows::core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Query(::core::mem::transmute_copy(&queriedobjectid), ::core::mem::transmute_copy(&equeriedobjecttype), ::core::mem::transmute_copy(&ereturnedobjectstype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteSnapshots<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceobjectid: ::windows::core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteSnapshots(::core::mem::transmute_copy(&sourceobjectid), ::core::mem::transmute_copy(&esourceobjecttype), ::core::mem::transmute_copy(&bforcedelete), ::core::mem::transmute_copy(&pldeletedsnapshots), ::core::mem::transmute_copy(&pnondeletedsnapshotid)).into()
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, pwszsharepath: *const u16, lnewcontext: i32, providerid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginPrepareSnapshot(::core::mem::transmute_copy(&snapshotsetid), ::core::mem::transmute_copy(&snapshotid), ::core::mem::transmute_copy(&pwszsharepath), ::core::mem::transmute_copy(&lnewcontext), ::core::mem::transmute_copy(&providerid)).into()
        }
        unsafe extern "system" fn IsPathSupported<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsharepath: *const u16, pbsupportedbythisprovider: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPathSupported(::core::mem::transmute_copy(&pwszsharepath)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbsupportedbythisprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPathSnapshotted<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsharepath: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsPathSnapshotted(::core::mem::transmute_copy(&pwszsharepath), ::core::mem::transmute_copy(&pbsnapshotspresent), ::core::mem::transmute_copy(&plsnapshotcompatibility)).into()
        }
        unsafe extern "system" fn SetSnapshotProperty<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSnapshotProperty(::core::mem::transmute_copy(&snapshotid), ::core::mem::transmute_copy(&esnapshotpropertyid), ::core::mem::transmute_copy(&vproperty)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetContext: SetContext::<Impl, IMPL_OFFSET>,
            GetSnapshotProperties: GetSnapshotProperties::<Impl, IMPL_OFFSET>,
            Query: Query::<Impl, IMPL_OFFSET>,
            DeleteSnapshots: DeleteSnapshots::<Impl, IMPL_OFFSET>,
            BeginPrepareSnapshot: BeginPrepareSnapshot::<Impl, IMPL_OFFSET>,
            IsPathSupported: IsPathSupported::<Impl, IMPL_OFFSET>,
            IsPathSnapshotted: IsPathSnapshotted::<Impl, IMPL_OFFSET>,
            SetSnapshotProperty: SetSnapshotProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssFileShareSnapshotProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
pub trait IVssHardwareSnapshotProviderImpl: Sized {
    fn AreLunsSupported(&mut self, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn FillInLunInfo(&mut self, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn BeginPrepareSnapshot(&mut self, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()>;
    fn GetTargetLuns(&mut self, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()>;
    fn LocateLuns(&mut self, lluncount: i32, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()>;
    fn OnLunEmpty(&mut self, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
impl IVssHardwareSnapshotProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssHardwareSnapshotProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssHardwareSnapshotProviderVtbl {
        unsafe extern "system" fn AreLunsSupported<Impl: IVssHardwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AreLunsSupported(::core::mem::transmute_copy(&lluncount), ::core::mem::transmute_copy(&lcontext), ::core::mem::transmute_copy(&rgwszdevices), ::core::mem::transmute_copy(&pluninformation), ::core::mem::transmute_copy(&pbissupported)).into()
        }
        unsafe extern "system" fn FillInLunInfo<Impl: IVssHardwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FillInLunInfo(::core::mem::transmute_copy(&wszdevicename), ::core::mem::transmute_copy(&pluninfo), ::core::mem::transmute_copy(&pbissupported)).into()
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Impl: IVssHardwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginPrepareSnapshot(::core::mem::transmute_copy(&snapshotsetid), ::core::mem::transmute_copy(&snapshotid), ::core::mem::transmute_copy(&lcontext), ::core::mem::transmute_copy(&lluncount), ::core::mem::transmute_copy(&rgdevicenames), ::core::mem::transmute_copy(&rgluninformation)).into()
        }
        unsafe extern "system" fn GetTargetLuns<Impl: IVssHardwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTargetLuns(::core::mem::transmute_copy(&lluncount), ::core::mem::transmute_copy(&rgdevicenames), ::core::mem::transmute_copy(&rgsourceluns), ::core::mem::transmute_copy(&rgdestinationluns)).into()
        }
        unsafe extern "system" fn LocateLuns<Impl: IVssHardwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lluncount: i32, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LocateLuns(::core::mem::transmute_copy(&lluncount), ::core::mem::transmute_copy(&rgsourceluns)).into()
        }
        unsafe extern "system" fn OnLunEmpty<Impl: IVssHardwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLunEmpty(::core::mem::transmute_copy(&wszdevicename), ::core::mem::transmute_copy(&pinformation)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AreLunsSupported: AreLunsSupported::<Impl, IMPL_OFFSET>,
            FillInLunInfo: FillInLunInfo::<Impl, IMPL_OFFSET>,
            BeginPrepareSnapshot: BeginPrepareSnapshot::<Impl, IMPL_OFFSET>,
            GetTargetLuns: GetTargetLuns::<Impl, IMPL_OFFSET>,
            LocateLuns: LocateLuns::<Impl, IMPL_OFFSET>,
            OnLunEmpty: OnLunEmpty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssHardwareSnapshotProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
pub trait IVssHardwareSnapshotProviderExImpl: Sized + IVssHardwareSnapshotProviderImpl {
    fn GetProviderCapabilities(&mut self) -> ::windows::core::Result<u64>;
    fn OnLunStateChange(&mut self, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn ResyncLuns(&mut self, psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> ::windows::core::Result<IVssAsync>;
    fn OnReuseLuns(&mut self, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
impl IVssHardwareSnapshotProviderExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssHardwareSnapshotProviderExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssHardwareSnapshotProviderExVtbl {
        unsafe extern "system" fn GetProviderCapabilities<Impl: IVssHardwareSnapshotProviderExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plloriginalcapabilitymask: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *plloriginalcapabilitymask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLunStateChange<Impl: IVssHardwareSnapshotProviderExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLunStateChange(::core::mem::transmute_copy(&psnapshotluns), ::core::mem::transmute_copy(&poriginalluns), ::core::mem::transmute_copy(&dwcount), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn ResyncLuns<Impl: IVssHardwareSnapshotProviderExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResyncLuns(::core::mem::transmute_copy(&psourceluns), ::core::mem::transmute_copy(&ptargetluns), ::core::mem::transmute_copy(&dwcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnReuseLuns<Impl: IVssHardwareSnapshotProviderExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnReuseLuns(::core::mem::transmute_copy(&psnapshotluns), ::core::mem::transmute_copy(&poriginalluns), ::core::mem::transmute_copy(&dwcount)).into()
        }
        Self {
            base: IVssHardwareSnapshotProviderVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetProviderCapabilities: GetProviderCapabilities::<Impl, IMPL_OFFSET>,
            OnLunStateChange: OnLunStateChange::<Impl, IMPL_OFFSET>,
            ResyncLuns: ResyncLuns::<Impl, IMPL_OFFSET>,
            OnReuseLuns: OnReuseLuns::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssHardwareSnapshotProviderEx as ::windows::core::Interface>::IID
    }
}
pub trait IVssProviderCreateSnapshotSetImpl: Sized {
    fn EndPrepareSnapshots(&mut self, snapshotsetid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PreCommitSnapshots(&mut self, snapshotsetid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn CommitSnapshots(&mut self, snapshotsetid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PostCommitSnapshots(&mut self, snapshotsetid: ::windows::core::GUID, lsnapshotscount: i32) -> ::windows::core::Result<()>;
    fn PreFinalCommitSnapshots(&mut self, snapshotsetid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PostFinalCommitSnapshots(&mut self, snapshotsetid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn AbortSnapshots(&mut self, snapshotsetid: ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl IVssProviderCreateSnapshotSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssProviderCreateSnapshotSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssProviderCreateSnapshotSetVtbl {
        unsafe extern "system" fn EndPrepareSnapshots<Impl: IVssProviderCreateSnapshotSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndPrepareSnapshots(::core::mem::transmute_copy(&snapshotsetid)).into()
        }
        unsafe extern "system" fn PreCommitSnapshots<Impl: IVssProviderCreateSnapshotSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PreCommitSnapshots(::core::mem::transmute_copy(&snapshotsetid)).into()
        }
        unsafe extern "system" fn CommitSnapshots<Impl: IVssProviderCreateSnapshotSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CommitSnapshots(::core::mem::transmute_copy(&snapshotsetid)).into()
        }
        unsafe extern "system" fn PostCommitSnapshots<Impl: IVssProviderCreateSnapshotSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID, lsnapshotscount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PostCommitSnapshots(::core::mem::transmute_copy(&snapshotsetid), ::core::mem::transmute_copy(&lsnapshotscount)).into()
        }
        unsafe extern "system" fn PreFinalCommitSnapshots<Impl: IVssProviderCreateSnapshotSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PreFinalCommitSnapshots(::core::mem::transmute_copy(&snapshotsetid)).into()
        }
        unsafe extern "system" fn PostFinalCommitSnapshots<Impl: IVssProviderCreateSnapshotSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PostFinalCommitSnapshots(::core::mem::transmute_copy(&snapshotsetid)).into()
        }
        unsafe extern "system" fn AbortSnapshots<Impl: IVssProviderCreateSnapshotSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AbortSnapshots(::core::mem::transmute_copy(&snapshotsetid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            EndPrepareSnapshots: EndPrepareSnapshots::<Impl, IMPL_OFFSET>,
            PreCommitSnapshots: PreCommitSnapshots::<Impl, IMPL_OFFSET>,
            CommitSnapshots: CommitSnapshots::<Impl, IMPL_OFFSET>,
            PostCommitSnapshots: PostCommitSnapshots::<Impl, IMPL_OFFSET>,
            PreFinalCommitSnapshots: PreFinalCommitSnapshots::<Impl, IMPL_OFFSET>,
            PostFinalCommitSnapshots: PostFinalCommitSnapshots::<Impl, IMPL_OFFSET>,
            AbortSnapshots: AbortSnapshots::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssProviderCreateSnapshotSet as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssProviderNotificationsImpl: Sized {
    fn OnLoad(&mut self, pcallback: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn OnUnload(&mut self, bforceunload: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVssProviderNotificationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssProviderNotificationsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssProviderNotificationsVtbl {
        unsafe extern "system" fn OnLoad<Impl: IVssProviderNotificationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLoad(::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn OnUnload<Impl: IVssProviderNotificationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bforceunload: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUnload(::core::mem::transmute_copy(&bforceunload)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnLoad: OnLoad::<Impl, IMPL_OFFSET>,
            OnUnload: OnUnload::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssProviderNotifications as ::windows::core::Interface>::IID
    }
}
pub trait IVssSnapshotMgmtImpl: Sized {
    fn GetProviderMgmtInterface(&mut self, providerid: ::windows::core::GUID, interfaceid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn QueryVolumesSupportedForSnapshots(&mut self, providerid: ::windows::core::GUID, lcontext: i32) -> ::windows::core::Result<IVssEnumMgmtObject>;
    fn QuerySnapshotsByVolume(&mut self, pwszvolumename: *const u16, providerid: ::windows::core::GUID) -> ::windows::core::Result<IVssEnumObject>;
}
impl IVssSnapshotMgmtVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssSnapshotMgmtImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssSnapshotMgmtVtbl {
        unsafe extern "system" fn GetProviderMgmtInterface<Impl: IVssSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, interfaceid: *const ::windows::core::GUID, ppitf: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderMgmtInterface(::core::mem::transmute_copy(&providerid), ::core::mem::transmute_copy(&interfaceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitf = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryVolumesSupportedForSnapshots<Impl: IVssSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, lcontext: i32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryVolumesSupportedForSnapshots(::core::mem::transmute_copy(&providerid), ::core::mem::transmute_copy(&lcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuerySnapshotsByVolume<Impl: IVssSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, providerid: ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuerySnapshotsByVolume(::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&providerid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetProviderMgmtInterface: GetProviderMgmtInterface::<Impl, IMPL_OFFSET>,
            QueryVolumesSupportedForSnapshots: QueryVolumesSupportedForSnapshots::<Impl, IMPL_OFFSET>,
            QuerySnapshotsByVolume: QuerySnapshotsByVolume::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssSnapshotMgmt as ::windows::core::Interface>::IID
    }
}
pub trait IVssSnapshotMgmt2Impl: Sized {
    fn GetMinDiffAreaSize(&mut self) -> ::windows::core::Result<i64>;
}
impl IVssSnapshotMgmt2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssSnapshotMgmt2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssSnapshotMgmt2Vtbl {
        unsafe extern "system" fn GetMinDiffAreaSize<Impl: IVssSnapshotMgmt2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllmindiffareasize: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMinDiffAreaSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pllmindiffareasize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetMinDiffAreaSize: GetMinDiffAreaSize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssSnapshotMgmt2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVssSoftwareSnapshotProviderImpl: Sized {
    fn SetContext(&mut self, lcontext: i32) -> ::windows::core::Result<()>;
    fn GetSnapshotProperties(&mut self, snapshotid: ::windows::core::GUID) -> ::windows::core::Result<VSS_SNAPSHOT_PROP>;
    fn Query(&mut self, queriedobjectid: ::windows::core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE) -> ::windows::core::Result<IVssEnumObject>;
    fn DeleteSnapshots(&mut self, sourceobjectid: ::windows::core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn BeginPrepareSnapshot(&mut self, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, pwszvolumename: *const u16, lnewcontext: i32) -> ::windows::core::Result<()>;
    fn IsVolumeSupported(&mut self, pwszvolumename: *const u16) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsVolumeSnapshotted(&mut self, pwszvolumename: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows::core::Result<()>;
    fn SetSnapshotProperty(&mut self, snapshotid: ::windows::core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn RevertToSnapshot(&mut self, snapshotid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn QueryRevertStatus(&mut self, pwszvolume: *const u16) -> ::windows::core::Result<IVssAsync>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVssSoftwareSnapshotProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssSoftwareSnapshotProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssSoftwareSnapshotProviderVtbl {
        unsafe extern "system" fn SetContext<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcontext: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContext(::core::mem::transmute_copy(&lcontext)).into()
        }
        unsafe extern "system" fn GetSnapshotProperties<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSnapshotProperties(::core::mem::transmute_copy(&snapshotid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Query<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queriedobjectid: ::windows::core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Query(::core::mem::transmute_copy(&queriedobjectid), ::core::mem::transmute_copy(&equeriedobjecttype), ::core::mem::transmute_copy(&ereturnedobjectstype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteSnapshots<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceobjectid: ::windows::core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteSnapshots(::core::mem::transmute_copy(&sourceobjectid), ::core::mem::transmute_copy(&esourceobjecttype), ::core::mem::transmute_copy(&bforcedelete), ::core::mem::transmute_copy(&pldeletedsnapshots), ::core::mem::transmute_copy(&pnondeletedsnapshotid)).into()
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, pwszvolumename: *const u16, lnewcontext: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginPrepareSnapshot(::core::mem::transmute_copy(&snapshotsetid), ::core::mem::transmute_copy(&snapshotid), ::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&lnewcontext)).into()
        }
        unsafe extern "system" fn IsVolumeSupported<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pbsupportedbythisprovider: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVolumeSupported(::core::mem::transmute_copy(&pwszvolumename)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbsupportedbythisprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVolumeSnapshotted<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsVolumeSnapshotted(::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&pbsnapshotspresent), ::core::mem::transmute_copy(&plsnapshotcompatibility)).into()
        }
        unsafe extern "system" fn SetSnapshotProperty<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSnapshotProperty(::core::mem::transmute_copy(&snapshotid), ::core::mem::transmute_copy(&esnapshotpropertyid), ::core::mem::transmute_copy(&vproperty)).into()
        }
        unsafe extern "system" fn RevertToSnapshot<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RevertToSnapshot(::core::mem::transmute_copy(&snapshotid)).into()
        }
        unsafe extern "system" fn QueryRevertStatus<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolume: *const u16, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryRevertStatus(::core::mem::transmute_copy(&pwszvolume)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetContext: SetContext::<Impl, IMPL_OFFSET>,
            GetSnapshotProperties: GetSnapshotProperties::<Impl, IMPL_OFFSET>,
            Query: Query::<Impl, IMPL_OFFSET>,
            DeleteSnapshots: DeleteSnapshots::<Impl, IMPL_OFFSET>,
            BeginPrepareSnapshot: BeginPrepareSnapshot::<Impl, IMPL_OFFSET>,
            IsVolumeSupported: IsVolumeSupported::<Impl, IMPL_OFFSET>,
            IsVolumeSnapshotted: IsVolumeSnapshotted::<Impl, IMPL_OFFSET>,
            SetSnapshotProperty: SetSnapshotProperty::<Impl, IMPL_OFFSET>,
            RevertToSnapshot: RevertToSnapshot::<Impl, IMPL_OFFSET>,
            QueryRevertStatus: QueryRevertStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssSoftwareSnapshotProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssWMDependencyImpl: Sized {
    fn GetWriterId(&mut self, pwriterid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetLogicalPath(&mut self, pbstrlogicalpath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetComponentName(&mut self, pbstrcomponentname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVssWMDependencyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssWMDependencyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssWMDependencyVtbl {
        unsafe extern "system" fn GetWriterId<Impl: IVssWMDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwriterid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWriterId(::core::mem::transmute_copy(&pwriterid)).into()
        }
        unsafe extern "system" fn GetLogicalPath<Impl: IVssWMDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlogicalpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLogicalPath(::core::mem::transmute_copy(&pbstrlogicalpath)).into()
        }
        unsafe extern "system" fn GetComponentName<Impl: IVssWMDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcomponentname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetComponentName(::core::mem::transmute_copy(&pbstrcomponentname)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetWriterId: GetWriterId::<Impl, IMPL_OFFSET>,
            GetLogicalPath: GetLogicalPath::<Impl, IMPL_OFFSET>,
            GetComponentName: GetComponentName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssWMDependency as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssWMFiledescImpl: Sized {
    fn GetPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetFilespec(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetRecursive(&mut self) -> ::windows::core::Result<bool>;
    fn GetAlternateLocation(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetBackupTypeMask(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVssWMFiledescVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssWMFiledescImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssWMFiledescVtbl {
        unsafe extern "system" fn GetPath<Impl: IVssWMFiledescImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilespec<Impl: IVssWMFiledescImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfilespec: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilespec() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrfilespec = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecursive<Impl: IVssWMFiledescImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbrecursive: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecursive() {
                ::core::result::Result::Ok(ok__) => {
                    *pbrecursive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAlternateLocation<Impl: IVssWMFiledescImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstralternatelocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAlternateLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstralternatelocation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackupTypeMask<Impl: IVssWMFiledescImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtypemask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackupTypeMask() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwtypemask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPath: GetPath::<Impl, IMPL_OFFSET>,
            GetFilespec: GetFilespec::<Impl, IMPL_OFFSET>,
            GetRecursive: GetRecursive::<Impl, IMPL_OFFSET>,
            GetAlternateLocation: GetAlternateLocation::<Impl, IMPL_OFFSET>,
            GetBackupTypeMask: GetBackupTypeMask::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssWMFiledesc as ::windows::core::Interface>::IID
    }
}
pub trait IVssWriterComponentsImpl: Sized {
    fn GetComponentCount(&mut self, pccomponents: *mut u32) -> ::windows::core::Result<()>;
    fn GetWriterInfo(&mut self, pidinstance: *mut ::windows::core::GUID, pidwriter: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetComponent(&mut self, icomponent: u32) -> ::windows::core::Result<IVssComponent>;
}
impl IVssWriterComponentsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterComponentsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssWriterComponentsVtbl {
        unsafe extern "system" fn GetComponentCount<Impl: IVssWriterComponentsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccomponents: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetComponentCount(::core::mem::transmute_copy(&pccomponents)).into()
        }
        unsafe extern "system" fn GetWriterInfo<Impl: IVssWriterComponentsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidinstance: *mut ::windows::core::GUID, pidwriter: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWriterInfo(::core::mem::transmute_copy(&pidinstance), ::core::mem::transmute_copy(&pidwriter)).into()
        }
        unsafe extern "system" fn GetComponent<Impl: IVssWriterComponentsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icomponent: u32, ppcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetComponent(::core::mem::transmute_copy(&icomponent)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomponent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            GetComponentCount: GetComponentCount::<Impl, IMPL_OFFSET>,
            GetWriterInfo: GetWriterInfo::<Impl, IMPL_OFFSET>,
            GetComponent: GetComponent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssWriterComponents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssWriterImplImpl: Sized {
    fn Initialize(&mut self, writerid: ::windows::core::GUID, wszwritername: super::super::Foundation::PWSTR, wszwriterinstancename: super::super::Foundation::PWSTR, dwmajorversion: u32, dwminorversion: u32, ut: VSS_USAGE_TYPE, st: VSS_SOURCE_TYPE, nlevel: VSS_APPLICATION_LEVEL, dwtimeout: u32, aws: VSS_ALTERNATE_WRITER_STATE, biothrottlingonly: u8) -> ::windows::core::Result<()>;
    fn Subscribe(&mut self, dwsubscribetimeout: u32, dweventflags: u32) -> ::windows::core::Result<()>;
    fn Unsubscribe(&mut self) -> ::windows::core::Result<()>;
    fn Uninitialize(&mut self);
    fn GetCurrentVolumeArray(&mut self) -> *mut super::super::Foundation::PWSTR;
    fn GetCurrentVolumeCount(&mut self) -> u32;
    fn GetSnapshotDeviceName(&mut self, wszoriginalvolume: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetCurrentSnapshotSetId(&mut self) -> ::windows::core::GUID;
    fn GetContext(&mut self) -> i32;
    fn GetCurrentLevel(&mut self) -> VSS_APPLICATION_LEVEL;
    fn IsPathAffected(&mut self, wszpath: super::super::Foundation::PWSTR) -> bool;
    fn IsBootableSystemStateBackedUp(&mut self) -> bool;
    fn AreComponentsSelected(&mut self) -> bool;
    fn GetBackupType(&mut self) -> VSS_BACKUP_TYPE;
    fn GetRestoreType(&mut self) -> VSS_RESTORE_TYPE;
    fn SetWriterFailure(&mut self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn IsPartialFileSupportEnabled(&mut self) -> bool;
    fn InstallAlternateWriter(&mut self, idwriter: ::windows::core::GUID, clsid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetIdentityInformation(&mut self) -> *mut IVssExamineWriterMetadata;
    fn SetWriterFailureEx(&mut self, hr: ::windows::core::HRESULT, hrapplication: ::windows::core::HRESULT, wszapplicationmessage: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetSessionId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IsWriterShuttingDown(&mut self) -> bool;
}
#[cfg(feature = "Win32_Foundation")]
impl IVssWriterImplVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImplImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVssWriterImplVtbl {
        unsafe extern "system" fn Initialize<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writerid: ::windows::core::GUID, wszwritername: super::super::Foundation::PWSTR, wszwriterinstancename: super::super::Foundation::PWSTR, dwmajorversion: u32, dwminorversion: u32, ut: VSS_USAGE_TYPE, st: VSS_SOURCE_TYPE, nlevel: VSS_APPLICATION_LEVEL, dwtimeout: u32, aws: VSS_ALTERNATE_WRITER_STATE, biothrottlingonly: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .Initialize(::core::mem::transmute_copy(&writerid), ::core::mem::transmute_copy(&wszwritername), ::core::mem::transmute_copy(&wszwriterinstancename), ::core::mem::transmute_copy(&dwmajorversion), ::core::mem::transmute_copy(&dwminorversion), ::core::mem::transmute_copy(&ut), ::core::mem::transmute_copy(&st), ::core::mem::transmute_copy(&nlevel), ::core::mem::transmute_copy(&dwtimeout), ::core::mem::transmute_copy(&aws), ::core::mem::transmute_copy(&biothrottlingonly))
                .into()
        }
        unsafe extern "system" fn Subscribe<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsubscribetimeout: u32, dweventflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Subscribe(::core::mem::transmute_copy(&dwsubscribetimeout), ::core::mem::transmute_copy(&dweventflags)).into()
        }
        unsafe extern "system" fn Unsubscribe<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unsubscribe().into()
        }
        unsafe extern "system" fn Uninitialize<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Uninitialize()
        }
        unsafe extern "system" fn GetCurrentVolumeArray<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut super::super::Foundation::PWSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCurrentVolumeArray()
        }
        unsafe extern "system" fn GetCurrentVolumeCount<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCurrentVolumeCount()
        }
        unsafe extern "system" fn GetSnapshotDeviceName<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszoriginalvolume: super::super::Foundation::PWSTR, ppwszsnapshotdevice: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSnapshotDeviceName(::core::mem::transmute_copy(&wszoriginalvolume)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszsnapshotdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentSnapshotSetId<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCurrentSnapshotSetId()
        }
        unsafe extern "system" fn GetContext<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetContext()
        }
        unsafe extern "system" fn GetCurrentLevel<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> VSS_APPLICATION_LEVEL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCurrentLevel()
        }
        unsafe extern "system" fn IsPathAffected<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsPathAffected(::core::mem::transmute_copy(&wszpath))
        }
        unsafe extern "system" fn IsBootableSystemStateBackedUp<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsBootableSystemStateBackedUp()
        }
        unsafe extern "system" fn AreComponentsSelected<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AreComponentsSelected()
        }
        unsafe extern "system" fn GetBackupType<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> VSS_BACKUP_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBackupType()
        }
        unsafe extern "system" fn GetRestoreType<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> VSS_RESTORE_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRestoreType()
        }
        unsafe extern "system" fn SetWriterFailure<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWriterFailure(::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn IsPartialFileSupportEnabled<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsPartialFileSupportEnabled()
        }
        unsafe extern "system" fn InstallAlternateWriter<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idwriter: ::windows::core::GUID, clsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InstallAlternateWriter(::core::mem::transmute_copy(&idwriter), ::core::mem::transmute_copy(&clsid)).into()
        }
        unsafe extern "system" fn GetIdentityInformation<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut IVssExamineWriterMetadata {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIdentityInformation()
        }
        unsafe extern "system" fn SetWriterFailureEx<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, hrapplication: ::windows::core::HRESULT, wszapplicationmessage: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWriterFailureEx(::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&hrapplication), ::core::mem::transmute_copy(&wszapplicationmessage)).into()
        }
        unsafe extern "system" fn GetSessionId<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idsession: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSessionId() {
                ::core::result::Result::Ok(ok__) => {
                    *idsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWriterShuttingDown<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsWriterShuttingDown()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Subscribe: Subscribe::<Impl, IMPL_OFFSET>,
            Unsubscribe: Unsubscribe::<Impl, IMPL_OFFSET>,
            Uninitialize: Uninitialize::<Impl, IMPL_OFFSET>,
            GetCurrentVolumeArray: GetCurrentVolumeArray::<Impl, IMPL_OFFSET>,
            GetCurrentVolumeCount: GetCurrentVolumeCount::<Impl, IMPL_OFFSET>,
            GetSnapshotDeviceName: GetSnapshotDeviceName::<Impl, IMPL_OFFSET>,
            GetCurrentSnapshotSetId: GetCurrentSnapshotSetId::<Impl, IMPL_OFFSET>,
            GetContext: GetContext::<Impl, IMPL_OFFSET>,
            GetCurrentLevel: GetCurrentLevel::<Impl, IMPL_OFFSET>,
            IsPathAffected: IsPathAffected::<Impl, IMPL_OFFSET>,
            IsBootableSystemStateBackedUp: IsBootableSystemStateBackedUp::<Impl, IMPL_OFFSET>,
            AreComponentsSelected: AreComponentsSelected::<Impl, IMPL_OFFSET>,
            GetBackupType: GetBackupType::<Impl, IMPL_OFFSET>,
            GetRestoreType: GetRestoreType::<Impl, IMPL_OFFSET>,
            SetWriterFailure: SetWriterFailure::<Impl, IMPL_OFFSET>,
            IsPartialFileSupportEnabled: IsPartialFileSupportEnabled::<Impl, IMPL_OFFSET>,
            InstallAlternateWriter: InstallAlternateWriter::<Impl, IMPL_OFFSET>,
            GetIdentityInformation: GetIdentityInformation::<Impl, IMPL_OFFSET>,
            SetWriterFailureEx: SetWriterFailureEx::<Impl, IMPL_OFFSET>,
            GetSessionId: GetSessionId::<Impl, IMPL_OFFSET>,
            IsWriterShuttingDown: IsWriterShuttingDown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssWriterImpl as ::windows::core::Interface>::IID
    }
}
