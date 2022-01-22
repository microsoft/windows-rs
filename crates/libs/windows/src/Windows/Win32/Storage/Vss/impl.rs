pub trait IVssAdmin_Impl: Sized {
    fn RegisterProvider(&mut self, pproviderid: &::windows::core::GUID, classid: &::windows::core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn UnregisterProvider(&mut self, providerid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn QueryProviders(&mut self) -> ::windows::core::Result<IVssEnumObject>;
    fn AbortAllSnapshotsInProgress(&mut self) -> ::windows::core::Result<()>;
}
impl IVssAdmin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssAdmin_Impl, const OFFSET: isize>() -> IVssAdmin_Vtbl {
        unsafe extern "system" fn RegisterProvider<Identity: ::windows::core::IUnknownImpl, Impl: IVssAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproviderid: ::windows::core::GUID, classid: ::windows::core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterProvider(::core::mem::transmute_copy(&pproviderid), ::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&pwszprovidername), ::core::mem::transmute_copy(&eprovidertype), ::core::mem::transmute_copy(&pwszproviderversion), ::core::mem::transmute_copy(&providerversionid)).into()
        }
        unsafe extern "system" fn UnregisterProvider<Identity: ::windows::core::IUnknownImpl, Impl: IVssAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterProvider(::core::mem::transmute_copy(&providerid)).into()
        }
        unsafe extern "system" fn QueryProviders<Identity: ::windows::core::IUnknownImpl, Impl: IVssAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryProviders() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbortAllSnapshotsInProgress<Identity: ::windows::core::IUnknownImpl, Impl: IVssAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AbortAllSnapshotsInProgress().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterProvider: RegisterProvider::<Identity, Impl, OFFSET>,
            UnregisterProvider: UnregisterProvider::<Identity, Impl, OFFSET>,
            QueryProviders: QueryProviders::<Identity, Impl, OFFSET>,
            AbortAllSnapshotsInProgress: AbortAllSnapshotsInProgress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssAdmin as ::windows::core::Interface>::IID
    }
}
pub trait IVssAdminEx_Impl: Sized + IVssAdmin_Impl {
    fn GetProviderCapability(&mut self, pproviderid: &::windows::core::GUID) -> ::windows::core::Result<u64>;
    fn GetProviderContext(&mut self, providerid: &::windows::core::GUID) -> ::windows::core::Result<i32>;
    fn SetProviderContext(&mut self, providerid: &::windows::core::GUID, lcontext: i32) -> ::windows::core::Result<()>;
}
impl IVssAdminEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssAdminEx_Impl, const OFFSET: isize>() -> IVssAdminEx_Vtbl {
        unsafe extern "system" fn GetProviderCapability<Identity: ::windows::core::IUnknownImpl, Impl: IVssAdminEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproviderid: ::windows::core::GUID, plloriginalcapabilitymask: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProviderCapability(::core::mem::transmute_copy(&pproviderid)) {
                ::core::result::Result::Ok(ok__) => {
                    *plloriginalcapabilitymask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderContext<Identity: ::windows::core::IUnknownImpl, Impl: IVssAdminEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, plcontext: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProviderContext(::core::mem::transmute_copy(&providerid)) {
                ::core::result::Result::Ok(ok__) => {
                    *plcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderContext<Identity: ::windows::core::IUnknownImpl, Impl: IVssAdminEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, lcontext: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProviderContext(::core::mem::transmute_copy(&providerid), ::core::mem::transmute_copy(&lcontext)).into()
        }
        Self {
            base: IVssAdmin_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetProviderCapability: GetProviderCapability::<Identity, Impl, OFFSET>,
            GetProviderContext: GetProviderContext::<Identity, Impl, OFFSET>,
            SetProviderContext: SetProviderContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssAdminEx as ::windows::core::Interface>::IID || iid == &<IVssAdmin as ::windows::core::Interface>::IID
    }
}
pub trait IVssAsync_Impl: Sized {
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
    fn Wait(&mut self, dwmilliseconds: u32) -> ::windows::core::Result<()>;
    fn QueryStatus(&mut self, phrresult: *mut ::windows::core::HRESULT, preserved: *mut i32) -> ::windows::core::Result<()>;
}
impl IVssAsync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssAsync_Impl, const OFFSET: isize>() -> IVssAsync_Vtbl {
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IVssAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn Wait<Identity: ::windows::core::IUnknownImpl, Impl: IVssAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Wait(::core::mem::transmute_copy(&dwmilliseconds)).into()
        }
        unsafe extern "system" fn QueryStatus<Identity: ::windows::core::IUnknownImpl, Impl: IVssAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT, preserved: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryStatus(::core::mem::transmute_copy(&phrresult), ::core::mem::transmute_copy(&preserved)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Wait: Wait::<Identity, Impl, OFFSET>,
            QueryStatus: QueryStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssAsync as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssComponent_Impl: Sized {
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
    fn AddDifferencedFilesByLastModifyTime(&mut self, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: super::super::Foundation::BOOL, ftlastmodifytime: &super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn AddDifferencedFilesByLastModifyLSN(&mut self, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: super::super::Foundation::BOOL, bstrlsnstring: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetDifferencedFilesCount(&mut self, pcdifferencedfiles: *mut u32) -> ::windows::core::Result<()>;
    fn GetDifferencedFile(&mut self, idifferencedfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilespec: *mut super::super::Foundation::BSTR, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut super::super::Foundation::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVssComponent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>() -> IVssComponent_Vtbl {
        unsafe extern "system" fn GetLogicalPath<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLogicalPath(::core::mem::transmute_copy(&pbstrpath)).into()
        }
        unsafe extern "system" fn GetComponentType<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pct: *mut VSS_COMPONENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetComponentType(::core::mem::transmute_copy(&pct)).into()
        }
        unsafe extern "system" fn GetComponentName<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetComponentName(::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn GetBackupSucceeded<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsucceeded: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBackupSucceeded(::core::mem::transmute_copy(&pbsucceeded)).into()
        }
        unsafe extern "system" fn GetAlternateLocationMappingCount<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcmappings: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAlternateLocationMappingCount(::core::mem::transmute_copy(&pcmappings)).into()
        }
        unsafe extern "system" fn GetAlternateLocationMapping<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imapping: u32, ppfiledesc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAlternateLocationMapping(::core::mem::transmute_copy(&imapping)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfiledesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackupMetadata<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszdata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBackupMetadata(::core::mem::transmute_copy(&wszdata)).into()
        }
        unsafe extern "system" fn GetBackupMetadata<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBackupMetadata(::core::mem::transmute_copy(&pbstrdata)).into()
        }
        unsafe extern "system" fn AddPartialFile<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilename: super::super::Foundation::PWSTR, wszranges: super::super::Foundation::PWSTR, wszmetadata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPartialFile(::core::mem::transmute_copy(&wszpath), ::core::mem::transmute_copy(&wszfilename), ::core::mem::transmute_copy(&wszranges), ::core::mem::transmute_copy(&wszmetadata)).into()
        }
        unsafe extern "system" fn GetPartialFileCount<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcpartialfiles: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPartialFileCount(::core::mem::transmute_copy(&pcpartialfiles)).into()
        }
        unsafe extern "system" fn GetPartialFile<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipartialfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilename: *mut super::super::Foundation::BSTR, pbstrrange: *mut super::super::Foundation::BSTR, pbstrmetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPartialFile(::core::mem::transmute_copy(&ipartialfile), ::core::mem::transmute_copy(&pbstrpath), ::core::mem::transmute_copy(&pbstrfilename), ::core::mem::transmute_copy(&pbstrrange), ::core::mem::transmute_copy(&pbstrmetadata)).into()
        }
        unsafe extern "system" fn IsSelectedForRestore<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbselectedforrestore: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsSelectedForRestore(::core::mem::transmute_copy(&pbselectedforrestore)).into()
        }
        unsafe extern "system" fn GetAdditionalRestores<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbadditionalrestores: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAdditionalRestores(::core::mem::transmute_copy(&pbadditionalrestores)).into()
        }
        unsafe extern "system" fn GetNewTargetCount<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnewtarget: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNewTargetCount(::core::mem::transmute_copy(&pcnewtarget)).into()
        }
        unsafe extern "system" fn GetNewTarget<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inewtarget: u32, ppfiledesc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNewTarget(::core::mem::transmute_copy(&inewtarget)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfiledesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDirectedTarget<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszsourcepath: super::super::Foundation::PWSTR, wszsourcefilename: super::super::Foundation::PWSTR, wszsourcerangelist: super::super::Foundation::PWSTR, wszdestinationpath: super::super::Foundation::PWSTR, wszdestinationfilename: super::super::Foundation::PWSTR, wszdestinationrangelist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddDirectedTarget(::core::mem::transmute_copy(&wszsourcepath), ::core::mem::transmute_copy(&wszsourcefilename), ::core::mem::transmute_copy(&wszsourcerangelist), ::core::mem::transmute_copy(&wszdestinationpath), ::core::mem::transmute_copy(&wszdestinationfilename), ::core::mem::transmute_copy(&wszdestinationrangelist)).into()
        }
        unsafe extern "system" fn GetDirectedTargetCount<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdirectedtarget: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDirectedTargetCount(::core::mem::transmute_copy(&pcdirectedtarget)).into()
        }
        unsafe extern "system" fn GetDirectedTarget<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idirectedtarget: u32, pbstrsourcepath: *mut super::super::Foundation::BSTR, pbstrsourcefilename: *mut super::super::Foundation::BSTR, pbstrsourcerangelist: *mut super::super::Foundation::BSTR, pbstrdestinationpath: *mut super::super::Foundation::BSTR, pbstrdestinationfilename: *mut super::super::Foundation::BSTR, pbstrdestinationrangelist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDirectedTarget(::core::mem::transmute_copy(&idirectedtarget), ::core::mem::transmute_copy(&pbstrsourcepath), ::core::mem::transmute_copy(&pbstrsourcefilename), ::core::mem::transmute_copy(&pbstrsourcerangelist), ::core::mem::transmute_copy(&pbstrdestinationpath), ::core::mem::transmute_copy(&pbstrdestinationfilename), ::core::mem::transmute_copy(&pbstrdestinationrangelist)).into()
        }
        unsafe extern "system" fn SetRestoreMetadata<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszrestoremetadata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRestoreMetadata(::core::mem::transmute_copy(&wszrestoremetadata)).into()
        }
        unsafe extern "system" fn GetRestoreMetadata<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrestoremetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRestoreMetadata(::core::mem::transmute_copy(&pbstrrestoremetadata)).into()
        }
        unsafe extern "system" fn SetRestoreTarget<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: VSS_RESTORE_TARGET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRestoreTarget(::core::mem::transmute_copy(&target)).into()
        }
        unsafe extern "system" fn GetRestoreTarget<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRestoreTarget(::core::mem::transmute_copy(&ptarget)).into()
        }
        unsafe extern "system" fn SetPreRestoreFailureMsg<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszprerestorefailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPreRestoreFailureMsg(::core::mem::transmute_copy(&wszprerestorefailuremsg)).into()
        }
        unsafe extern "system" fn GetPreRestoreFailureMsg<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprerestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPreRestoreFailureMsg(::core::mem::transmute_copy(&pbstrprerestorefailuremsg)).into()
        }
        unsafe extern "system" fn SetPostRestoreFailureMsg<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpostrestorefailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPostRestoreFailureMsg(::core::mem::transmute_copy(&wszpostrestorefailuremsg)).into()
        }
        unsafe extern "system" fn GetPostRestoreFailureMsg<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpostrestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPostRestoreFailureMsg(::core::mem::transmute_copy(&pbstrpostrestorefailuremsg)).into()
        }
        unsafe extern "system" fn SetBackupStamp<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszbackupstamp: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBackupStamp(::core::mem::transmute_copy(&wszbackupstamp)).into()
        }
        unsafe extern "system" fn GetBackupStamp<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBackupStamp(::core::mem::transmute_copy(&pbstrbackupstamp)).into()
        }
        unsafe extern "system" fn GetPreviousBackupStamp<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPreviousBackupStamp(::core::mem::transmute_copy(&pbstrbackupstamp)).into()
        }
        unsafe extern "system" fn GetBackupOptions<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbackupoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBackupOptions(::core::mem::transmute_copy(&pbstrbackupoptions)).into()
        }
        unsafe extern "system" fn GetRestoreOptions<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrestoreoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRestoreOptions(::core::mem::transmute_copy(&pbstrrestoreoptions)).into()
        }
        unsafe extern "system" fn GetRestoreSubcomponentCount<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcrestoresubcomponent: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRestoreSubcomponentCount(::core::mem::transmute_copy(&pcrestoresubcomponent)).into()
        }
        unsafe extern "system" fn GetRestoreSubcomponent<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icomponent: u32, pbstrlogicalpath: *mut super::super::Foundation::BSTR, pbstrcomponentname: *mut super::super::Foundation::BSTR, pbrepair: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRestoreSubcomponent(::core::mem::transmute_copy(&icomponent), ::core::mem::transmute_copy(&pbstrlogicalpath), ::core::mem::transmute_copy(&pbstrcomponentname), ::core::mem::transmute_copy(&pbrepair)).into()
        }
        unsafe extern "system" fn GetFileRestoreStatus<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFileRestoreStatus(::core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn AddDifferencedFilesByLastModifyTime<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: super::super::Foundation::BOOL, ftlastmodifytime: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddDifferencedFilesByLastModifyTime(::core::mem::transmute_copy(&wszpath), ::core::mem::transmute_copy(&wszfilespec), ::core::mem::transmute_copy(&brecursive), ::core::mem::transmute_copy(&ftlastmodifytime)).into()
        }
        unsafe extern "system" fn AddDifferencedFilesByLastModifyLSN<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: super::super::Foundation::BOOL, bstrlsnstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddDifferencedFilesByLastModifyLSN(::core::mem::transmute_copy(&wszpath), ::core::mem::transmute_copy(&wszfilespec), ::core::mem::transmute_copy(&brecursive), ::core::mem::transmute_copy(&bstrlsnstring)).into()
        }
        unsafe extern "system" fn GetDifferencedFilesCount<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdifferencedfiles: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDifferencedFilesCount(::core::mem::transmute_copy(&pcdifferencedfiles)).into()
        }
        unsafe extern "system" fn GetDifferencedFile<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idifferencedfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilespec: *mut super::super::Foundation::BSTR, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut super::super::Foundation::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDifferencedFile(::core::mem::transmute_copy(&idifferencedfile), ::core::mem::transmute_copy(&pbstrpath), ::core::mem::transmute_copy(&pbstrfilespec), ::core::mem::transmute_copy(&pbrecursive), ::core::mem::transmute_copy(&pbstrlsnstring), ::core::mem::transmute_copy(&pftlastmodifytime)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetLogicalPath: GetLogicalPath::<Identity, Impl, OFFSET>,
            GetComponentType: GetComponentType::<Identity, Impl, OFFSET>,
            GetComponentName: GetComponentName::<Identity, Impl, OFFSET>,
            GetBackupSucceeded: GetBackupSucceeded::<Identity, Impl, OFFSET>,
            GetAlternateLocationMappingCount: GetAlternateLocationMappingCount::<Identity, Impl, OFFSET>,
            GetAlternateLocationMapping: GetAlternateLocationMapping::<Identity, Impl, OFFSET>,
            SetBackupMetadata: SetBackupMetadata::<Identity, Impl, OFFSET>,
            GetBackupMetadata: GetBackupMetadata::<Identity, Impl, OFFSET>,
            AddPartialFile: AddPartialFile::<Identity, Impl, OFFSET>,
            GetPartialFileCount: GetPartialFileCount::<Identity, Impl, OFFSET>,
            GetPartialFile: GetPartialFile::<Identity, Impl, OFFSET>,
            IsSelectedForRestore: IsSelectedForRestore::<Identity, Impl, OFFSET>,
            GetAdditionalRestores: GetAdditionalRestores::<Identity, Impl, OFFSET>,
            GetNewTargetCount: GetNewTargetCount::<Identity, Impl, OFFSET>,
            GetNewTarget: GetNewTarget::<Identity, Impl, OFFSET>,
            AddDirectedTarget: AddDirectedTarget::<Identity, Impl, OFFSET>,
            GetDirectedTargetCount: GetDirectedTargetCount::<Identity, Impl, OFFSET>,
            GetDirectedTarget: GetDirectedTarget::<Identity, Impl, OFFSET>,
            SetRestoreMetadata: SetRestoreMetadata::<Identity, Impl, OFFSET>,
            GetRestoreMetadata: GetRestoreMetadata::<Identity, Impl, OFFSET>,
            SetRestoreTarget: SetRestoreTarget::<Identity, Impl, OFFSET>,
            GetRestoreTarget: GetRestoreTarget::<Identity, Impl, OFFSET>,
            SetPreRestoreFailureMsg: SetPreRestoreFailureMsg::<Identity, Impl, OFFSET>,
            GetPreRestoreFailureMsg: GetPreRestoreFailureMsg::<Identity, Impl, OFFSET>,
            SetPostRestoreFailureMsg: SetPostRestoreFailureMsg::<Identity, Impl, OFFSET>,
            GetPostRestoreFailureMsg: GetPostRestoreFailureMsg::<Identity, Impl, OFFSET>,
            SetBackupStamp: SetBackupStamp::<Identity, Impl, OFFSET>,
            GetBackupStamp: GetBackupStamp::<Identity, Impl, OFFSET>,
            GetPreviousBackupStamp: GetPreviousBackupStamp::<Identity, Impl, OFFSET>,
            GetBackupOptions: GetBackupOptions::<Identity, Impl, OFFSET>,
            GetRestoreOptions: GetRestoreOptions::<Identity, Impl, OFFSET>,
            GetRestoreSubcomponentCount: GetRestoreSubcomponentCount::<Identity, Impl, OFFSET>,
            GetRestoreSubcomponent: GetRestoreSubcomponent::<Identity, Impl, OFFSET>,
            GetFileRestoreStatus: GetFileRestoreStatus::<Identity, Impl, OFFSET>,
            AddDifferencedFilesByLastModifyTime: AddDifferencedFilesByLastModifyTime::<Identity, Impl, OFFSET>,
            AddDifferencedFilesByLastModifyLSN: AddDifferencedFilesByLastModifyLSN::<Identity, Impl, OFFSET>,
            GetDifferencedFilesCount: GetDifferencedFilesCount::<Identity, Impl, OFFSET>,
            GetDifferencedFile: GetDifferencedFile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssComponent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssComponentEx_Impl: Sized + IVssComponent_Impl {
    fn SetPrepareForBackupFailureMsg(&mut self, wszfailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetPostSnapshotFailureMsg(&mut self, wszfailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetPrepareForBackupFailureMsg(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetPostSnapshotFailureMsg(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetAuthoritativeRestore(&mut self) -> ::windows::core::Result<bool>;
    fn GetRollForward(&mut self, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetRestoreName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVssComponentEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponentEx_Impl, const OFFSET: isize>() -> IVssComponentEx_Vtbl {
        unsafe extern "system" fn SetPrepareForBackupFailureMsg<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszfailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrepareForBackupFailureMsg(::core::mem::transmute_copy(&wszfailuremsg)).into()
        }
        unsafe extern "system" fn SetPostSnapshotFailureMsg<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszfailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPostSnapshotFailureMsg(::core::mem::transmute_copy(&wszfailuremsg)).into()
        }
        unsafe extern "system" fn GetPrepareForBackupFailureMsg<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPrepareForBackupFailureMsg() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrfailuremsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPostSnapshotFailureMsg<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPostSnapshotFailureMsg() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrfailuremsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthoritativeRestore<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbauth: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAuthoritativeRestore() {
                ::core::result::Result::Ok(ok__) => {
                    *pbauth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRollForward<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRollForward(::core::mem::transmute_copy(&prolltype), ::core::mem::transmute_copy(&pbstrpoint)).into()
        }
        unsafe extern "system" fn GetRestoreName<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRestoreName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IVssComponent_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetPrepareForBackupFailureMsg: SetPrepareForBackupFailureMsg::<Identity, Impl, OFFSET>,
            SetPostSnapshotFailureMsg: SetPostSnapshotFailureMsg::<Identity, Impl, OFFSET>,
            GetPrepareForBackupFailureMsg: GetPrepareForBackupFailureMsg::<Identity, Impl, OFFSET>,
            GetPostSnapshotFailureMsg: GetPostSnapshotFailureMsg::<Identity, Impl, OFFSET>,
            GetAuthoritativeRestore: GetAuthoritativeRestore::<Identity, Impl, OFFSET>,
            GetRollForward: GetRollForward::<Identity, Impl, OFFSET>,
            GetRestoreName: GetRestoreName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssComponentEx as ::windows::core::Interface>::IID || iid == &<IVssComponent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssComponentEx2_Impl: Sized + IVssComponent_Impl + IVssComponentEx_Impl {
    fn SetFailure(&mut self, hr: ::windows::core::HRESULT, hrapplication: ::windows::core::HRESULT, wszapplicationmessage: super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::Result<()>;
    fn GetFailure(&mut self, phr: *mut ::windows::core::HRESULT, phrapplication: *mut ::windows::core::HRESULT, pbstrapplicationmessage: *mut super::super::Foundation::BSTR, pdwreserved: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVssComponentEx2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponentEx2_Impl, const OFFSET: isize>() -> IVssComponentEx2_Vtbl {
        unsafe extern "system" fn SetFailure<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponentEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, hrapplication: ::windows::core::HRESULT, wszapplicationmessage: super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFailure(::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&hrapplication), ::core::mem::transmute_copy(&wszapplicationmessage), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn GetFailure<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponentEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phr: *mut ::windows::core::HRESULT, phrapplication: *mut ::windows::core::HRESULT, pbstrapplicationmessage: *mut super::super::Foundation::BSTR, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFailure(::core::mem::transmute_copy(&phr), ::core::mem::transmute_copy(&phrapplication), ::core::mem::transmute_copy(&pbstrapplicationmessage), ::core::mem::transmute_copy(&pdwreserved)).into()
        }
        Self {
            base: IVssComponentEx_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetFailure: SetFailure::<Identity, Impl, OFFSET>,
            GetFailure: GetFailure::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssComponentEx2 as ::windows::core::Interface>::IID || iid == &<IVssComponent as ::windows::core::Interface>::IID || iid == &<IVssComponentEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssCreateExpressWriterMetadata_Impl: Sized {
    fn AddExcludeFiles(&mut self, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8) -> ::windows::core::Result<()>;
    fn AddComponent(&mut self, ct: VSS_COMPONENT_TYPE, wszlogicalpath: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcaption: super::super::Foundation::PWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows::core::Result<()>;
    fn AddFilesToFileGroup(&mut self, wszlogicalpath: super::super::Foundation::PWSTR, wszgroupname: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8, wszalternatelocation: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::Result<()>;
    fn SetRestoreMethod(&mut self, method: VSS_RESTOREMETHOD_ENUM, wszservice: super::super::Foundation::PWSTR, wszuserprocedure: super::super::Foundation::PWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows::core::Result<()>;
    fn AddComponentDependency(&mut self, wszforlogicalpath: super::super::Foundation::PWSTR, wszforcomponentname: super::super::Foundation::PWSTR, onwriterid: &::windows::core::GUID, wszonlogicalpath: super::super::Foundation::PWSTR, wszoncomponentname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetBackupSchema(&mut self, dwschemamask: u32) -> ::windows::core::Result<()>;
    fn SaveAsXML(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVssCreateExpressWriterMetadata_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>() -> IVssCreateExpressWriterMetadata_Vtbl {
        unsafe extern "system" fn AddExcludeFiles<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddExcludeFiles(::core::mem::transmute_copy(&wszpath), ::core::mem::transmute_copy(&wszfilespec), ::core::mem::transmute_copy(&brecursive)).into()
        }
        unsafe extern "system" fn AddComponent<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ct: VSS_COMPONENT_TYPE, wszlogicalpath: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcaption: super::super::Foundation::PWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        unsafe extern "system" fn AddFilesToFileGroup<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlogicalpath: super::super::Foundation::PWSTR, wszgroupname: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8, wszalternatelocation: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddFilesToFileGroup(::core::mem::transmute_copy(&wszlogicalpath), ::core::mem::transmute_copy(&wszgroupname), ::core::mem::transmute_copy(&wszpath), ::core::mem::transmute_copy(&wszfilespec), ::core::mem::transmute_copy(&brecursive), ::core::mem::transmute_copy(&wszalternatelocation), ::core::mem::transmute_copy(&dwbackuptypemask)).into()
        }
        unsafe extern "system" fn SetRestoreMethod<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: VSS_RESTOREMETHOD_ENUM, wszservice: super::super::Foundation::PWSTR, wszuserprocedure: super::super::Foundation::PWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRestoreMethod(::core::mem::transmute_copy(&method), ::core::mem::transmute_copy(&wszservice), ::core::mem::transmute_copy(&wszuserprocedure), ::core::mem::transmute_copy(&writerrestore), ::core::mem::transmute_copy(&brebootrequired)).into()
        }
        unsafe extern "system" fn AddComponentDependency<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszforlogicalpath: super::super::Foundation::PWSTR, wszforcomponentname: super::super::Foundation::PWSTR, onwriterid: ::windows::core::GUID, wszonlogicalpath: super::super::Foundation::PWSTR, wszoncomponentname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddComponentDependency(::core::mem::transmute_copy(&wszforlogicalpath), ::core::mem::transmute_copy(&wszforcomponentname), ::core::mem::transmute_copy(&onwriterid), ::core::mem::transmute_copy(&wszonlogicalpath), ::core::mem::transmute_copy(&wszoncomponentname)).into()
        }
        unsafe extern "system" fn SetBackupSchema<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwschemamask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBackupSchema(::core::mem::transmute_copy(&dwschemamask)).into()
        }
        unsafe extern "system" fn SaveAsXML<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SaveAsXML() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrxml = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddExcludeFiles: AddExcludeFiles::<Identity, Impl, OFFSET>,
            AddComponent: AddComponent::<Identity, Impl, OFFSET>,
            AddFilesToFileGroup: AddFilesToFileGroup::<Identity, Impl, OFFSET>,
            SetRestoreMethod: SetRestoreMethod::<Identity, Impl, OFFSET>,
            AddComponentDependency: AddComponentDependency::<Identity, Impl, OFFSET>,
            SetBackupSchema: SetBackupSchema::<Identity, Impl, OFFSET>,
            SaveAsXML: SaveAsXML::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssCreateExpressWriterMetadata as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IVssCreateWriterMetadata_Impl: Sized {
    fn AddIncludeFiles(&mut self, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8, wszalternatelocation: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn AddExcludeFiles(&mut self, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8) -> ::windows::core::Result<()>;
    fn AddComponent(&mut self, ct: VSS_COMPONENT_TYPE, wszlogicalpath: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcaption: super::super::Foundation::PWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows::core::Result<()>;
    fn AddDatabaseFiles(&mut self, wszlogicalpath: super::super::Foundation::PWSTR, wszdatabasename: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::Result<()>;
    fn AddDatabaseLogFiles(&mut self, wszlogicalpath: super::super::Foundation::PWSTR, wszdatabasename: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::Result<()>;
    fn AddFilesToFileGroup(&mut self, wszlogicalpath: super::super::Foundation::PWSTR, wszgroupname: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8, wszalternatelocation: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::Result<()>;
    fn SetRestoreMethod(&mut self, method: VSS_RESTOREMETHOD_ENUM, wszservice: super::super::Foundation::PWSTR, wszuserprocedure: super::super::Foundation::PWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows::core::Result<()>;
    fn AddAlternateLocationMapping(&mut self, wszsourcepath: super::super::Foundation::PWSTR, wszsourcefilespec: super::super::Foundation::PWSTR, brecursive: u8, wszdestination: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn AddComponentDependency(&mut self, wszforlogicalpath: super::super::Foundation::PWSTR, wszforcomponentname: super::super::Foundation::PWSTR, onwriterid: &::windows::core::GUID, wszonlogicalpath: super::super::Foundation::PWSTR, wszoncomponentname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetBackupSchema(&mut self, dwschemamask: u32) -> ::windows::core::Result<()>;
    fn GetDocument(&mut self) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument>;
    fn SaveAsXML(&mut self, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IVssCreateWriterMetadata_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateWriterMetadata_Impl, const OFFSET: isize>() -> IVssCreateWriterMetadata_Vtbl {
        unsafe extern "system" fn AddIncludeFiles<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateWriterMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8, wszalternatelocation: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddIncludeFiles(::core::mem::transmute_copy(&wszpath), ::core::mem::transmute_copy(&wszfilespec), ::core::mem::transmute_copy(&brecursive), ::core::mem::transmute_copy(&wszalternatelocation)).into()
        }
        unsafe extern "system" fn AddExcludeFiles<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateWriterMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddExcludeFiles(::core::mem::transmute_copy(&wszpath), ::core::mem::transmute_copy(&wszfilespec), ::core::mem::transmute_copy(&brecursive)).into()
        }
        unsafe extern "system" fn AddComponent<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateWriterMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ct: VSS_COMPONENT_TYPE, wszlogicalpath: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcaption: super::super::Foundation::PWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        unsafe extern "system" fn AddDatabaseFiles<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateWriterMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlogicalpath: super::super::Foundation::PWSTR, wszdatabasename: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddDatabaseFiles(::core::mem::transmute_copy(&wszlogicalpath), ::core::mem::transmute_copy(&wszdatabasename), ::core::mem::transmute_copy(&wszpath), ::core::mem::transmute_copy(&wszfilespec), ::core::mem::transmute_copy(&dwbackuptypemask)).into()
        }
        unsafe extern "system" fn AddDatabaseLogFiles<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateWriterMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlogicalpath: super::super::Foundation::PWSTR, wszdatabasename: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddDatabaseLogFiles(::core::mem::transmute_copy(&wszlogicalpath), ::core::mem::transmute_copy(&wszdatabasename), ::core::mem::transmute_copy(&wszpath), ::core::mem::transmute_copy(&wszfilespec), ::core::mem::transmute_copy(&dwbackuptypemask)).into()
        }
        unsafe extern "system" fn AddFilesToFileGroup<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateWriterMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlogicalpath: super::super::Foundation::PWSTR, wszgroupname: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8, wszalternatelocation: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddFilesToFileGroup(::core::mem::transmute_copy(&wszlogicalpath), ::core::mem::transmute_copy(&wszgroupname), ::core::mem::transmute_copy(&wszpath), ::core::mem::transmute_copy(&wszfilespec), ::core::mem::transmute_copy(&brecursive), ::core::mem::transmute_copy(&wszalternatelocation), ::core::mem::transmute_copy(&dwbackuptypemask)).into()
        }
        unsafe extern "system" fn SetRestoreMethod<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateWriterMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: VSS_RESTOREMETHOD_ENUM, wszservice: super::super::Foundation::PWSTR, wszuserprocedure: super::super::Foundation::PWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRestoreMethod(::core::mem::transmute_copy(&method), ::core::mem::transmute_copy(&wszservice), ::core::mem::transmute_copy(&wszuserprocedure), ::core::mem::transmute_copy(&writerrestore), ::core::mem::transmute_copy(&brebootrequired)).into()
        }
        unsafe extern "system" fn AddAlternateLocationMapping<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateWriterMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszsourcepath: super::super::Foundation::PWSTR, wszsourcefilespec: super::super::Foundation::PWSTR, brecursive: u8, wszdestination: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddAlternateLocationMapping(::core::mem::transmute_copy(&wszsourcepath), ::core::mem::transmute_copy(&wszsourcefilespec), ::core::mem::transmute_copy(&brecursive), ::core::mem::transmute_copy(&wszdestination)).into()
        }
        unsafe extern "system" fn AddComponentDependency<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateWriterMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszforlogicalpath: super::super::Foundation::PWSTR, wszforcomponentname: super::super::Foundation::PWSTR, onwriterid: ::windows::core::GUID, wszonlogicalpath: super::super::Foundation::PWSTR, wszoncomponentname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddComponentDependency(::core::mem::transmute_copy(&wszforlogicalpath), ::core::mem::transmute_copy(&wszforcomponentname), ::core::mem::transmute_copy(&onwriterid), ::core::mem::transmute_copy(&wszonlogicalpath), ::core::mem::transmute_copy(&wszoncomponentname)).into()
        }
        unsafe extern "system" fn SetBackupSchema<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateWriterMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwschemamask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBackupSchema(::core::mem::transmute_copy(&dwschemamask)).into()
        }
        unsafe extern "system" fn GetDocument<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateWriterMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdoc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDocument() {
                ::core::result::Result::Ok(ok__) => {
                    *pdoc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAsXML<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateWriterMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SaveAsXML(::core::mem::transmute_copy(&pbstrxml)).into()
        }
        Self {
            AddIncludeFiles: AddIncludeFiles::<Identity, Impl, OFFSET>,
            AddExcludeFiles: AddExcludeFiles::<Identity, Impl, OFFSET>,
            AddComponent: AddComponent::<Identity, Impl, OFFSET>,
            AddDatabaseFiles: AddDatabaseFiles::<Identity, Impl, OFFSET>,
            AddDatabaseLogFiles: AddDatabaseLogFiles::<Identity, Impl, OFFSET>,
            AddFilesToFileGroup: AddFilesToFileGroup::<Identity, Impl, OFFSET>,
            SetRestoreMethod: SetRestoreMethod::<Identity, Impl, OFFSET>,
            AddAlternateLocationMapping: AddAlternateLocationMapping::<Identity, Impl, OFFSET>,
            AddComponentDependency: AddComponentDependency::<Identity, Impl, OFFSET>,
            SetBackupSchema: SetBackupSchema::<Identity, Impl, OFFSET>,
            GetDocument: GetDocument::<Identity, Impl, OFFSET>,
            SaveAsXML: SaveAsXML::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssCreateWriterMetadata as ::windows::core::Interface>::IID
    }
}
pub trait IVssDifferentialSoftwareSnapshotMgmt_Impl: Sized {
    fn AddDiffArea(&mut self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::Result<()>;
    fn ChangeDiffAreaMaximumSize(&mut self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::Result<()>;
    fn QueryVolumesSupportedForDiffAreas(&mut self, pwszoriginalvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject>;
    fn QueryDiffAreasForVolume(&mut self, pwszvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject>;
    fn QueryDiffAreasOnVolume(&mut self, pwszvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject>;
    fn QueryDiffAreasForSnapshot(&mut self, snapshotid: &::windows::core::GUID) -> ::windows::core::Result<IVssEnumMgmtObject>;
}
impl IVssDifferentialSoftwareSnapshotMgmt_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: isize>() -> IVssDifferentialSoftwareSnapshotMgmt_Vtbl {
        unsafe extern "system" fn AddDiffArea<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddDiffArea(::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&pwszdiffareavolumename), ::core::mem::transmute_copy(&llmaximumdiffspace)).into()
        }
        unsafe extern "system" fn ChangeDiffAreaMaximumSize<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ChangeDiffAreaMaximumSize(::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&pwszdiffareavolumename), ::core::mem::transmute_copy(&llmaximumdiffspace)).into()
        }
        unsafe extern "system" fn QueryVolumesSupportedForDiffAreas<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszoriginalvolumename: *const u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryVolumesSupportedForDiffAreas(::core::mem::transmute_copy(&pwszoriginalvolumename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDiffAreasForVolume<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryDiffAreasForVolume(::core::mem::transmute_copy(&pwszvolumename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDiffAreasOnVolume<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryDiffAreasOnVolume(::core::mem::transmute_copy(&pwszvolumename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDiffAreasForSnapshot<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryDiffAreasForSnapshot(::core::mem::transmute_copy(&snapshotid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddDiffArea: AddDiffArea::<Identity, Impl, OFFSET>,
            ChangeDiffAreaMaximumSize: ChangeDiffAreaMaximumSize::<Identity, Impl, OFFSET>,
            QueryVolumesSupportedForDiffAreas: QueryVolumesSupportedForDiffAreas::<Identity, Impl, OFFSET>,
            QueryDiffAreasForVolume: QueryDiffAreasForVolume::<Identity, Impl, OFFSET>,
            QueryDiffAreasOnVolume: QueryDiffAreasOnVolume::<Identity, Impl, OFFSET>,
            QueryDiffAreasForSnapshot: QueryDiffAreasForSnapshot::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssDifferentialSoftwareSnapshotMgmt as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssDifferentialSoftwareSnapshotMgmt2_Impl: Sized + IVssDifferentialSoftwareSnapshotMgmt_Impl {
    fn ChangeDiffAreaMaximumSizeEx(&mut self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn MigrateDiffAreas(&mut self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> ::windows::core::Result<()>;
    fn QueryMigrationStatus(&mut self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16) -> ::windows::core::Result<IVssAsync>;
    fn SetSnapshotPriority(&mut self, idsnapshot: &::windows::core::GUID, priority: u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVssDifferentialSoftwareSnapshotMgmt2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt2_Impl, const OFFSET: isize>() -> IVssDifferentialSoftwareSnapshotMgmt2_Vtbl {
        unsafe extern "system" fn ChangeDiffAreaMaximumSizeEx<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ChangeDiffAreaMaximumSizeEx(::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&pwszdiffareavolumename), ::core::mem::transmute_copy(&llmaximumdiffspace), ::core::mem::transmute_copy(&bvolatile)).into()
        }
        unsafe extern "system" fn MigrateDiffAreas<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MigrateDiffAreas(::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&pwszdiffareavolumename), ::core::mem::transmute_copy(&pwsznewdiffareavolumename)).into()
        }
        unsafe extern "system" fn QueryMigrationStatus<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryMigrationStatus(::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&pwszdiffareavolumename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSnapshotPriority<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idsnapshot: ::windows::core::GUID, priority: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSnapshotPriority(::core::mem::transmute_copy(&idsnapshot), ::core::mem::transmute_copy(&priority)).into()
        }
        Self {
            base: IVssDifferentialSoftwareSnapshotMgmt_Vtbl::new::<Identity, Impl, OFFSET>(),
            ChangeDiffAreaMaximumSizeEx: ChangeDiffAreaMaximumSizeEx::<Identity, Impl, OFFSET>,
            MigrateDiffAreas: MigrateDiffAreas::<Identity, Impl, OFFSET>,
            QueryMigrationStatus: QueryMigrationStatus::<Identity, Impl, OFFSET>,
            SetSnapshotPriority: SetSnapshotPriority::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssDifferentialSoftwareSnapshotMgmt2 as ::windows::core::Interface>::IID || iid == &<IVssDifferentialSoftwareSnapshotMgmt as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssDifferentialSoftwareSnapshotMgmt3_Impl: Sized + IVssDifferentialSoftwareSnapshotMgmt_Impl + IVssDifferentialSoftwareSnapshotMgmt2_Impl {
    fn SetVolumeProtectLevel(&mut self, pwszvolumename: *const u16, protectionlevel: VSS_PROTECTION_LEVEL) -> ::windows::core::Result<()>;
    fn GetVolumeProtectLevel(&mut self, pwszvolumename: *const u16) -> ::windows::core::Result<VSS_VOLUME_PROTECTION_INFO>;
    fn ClearVolumeProtectFault(&mut self, pwszvolumename: *const u16) -> ::windows::core::Result<()>;
    fn DeleteUnusedDiffAreas(&mut self, pwszdiffareavolumename: *const u16) -> ::windows::core::Result<()>;
    fn QuerySnapshotDeltaBitmap(&mut self, idsnapshotolder: &::windows::core::GUID, idsnapshotyounger: &::windows::core::GUID, pcblocksizeperbit: *mut u32, pcbitmaplength: *mut u32, ppbbitmap: *mut *mut u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVssDifferentialSoftwareSnapshotMgmt3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: isize>() -> IVssDifferentialSoftwareSnapshotMgmt3_Vtbl {
        unsafe extern "system" fn SetVolumeProtectLevel<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, protectionlevel: VSS_PROTECTION_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVolumeProtectLevel(::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&protectionlevel)).into()
        }
        unsafe extern "system" fn GetVolumeProtectLevel<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, protectionlevel: *mut VSS_VOLUME_PROTECTION_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVolumeProtectLevel(::core::mem::transmute_copy(&pwszvolumename)) {
                ::core::result::Result::Ok(ok__) => {
                    *protectionlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearVolumeProtectFault<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearVolumeProtectFault(::core::mem::transmute_copy(&pwszvolumename)).into()
        }
        unsafe extern "system" fn DeleteUnusedDiffAreas<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdiffareavolumename: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteUnusedDiffAreas(::core::mem::transmute_copy(&pwszdiffareavolumename)).into()
        }
        unsafe extern "system" fn QuerySnapshotDeltaBitmap<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idsnapshotolder: ::windows::core::GUID, idsnapshotyounger: ::windows::core::GUID, pcblocksizeperbit: *mut u32, pcbitmaplength: *mut u32, ppbbitmap: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QuerySnapshotDeltaBitmap(::core::mem::transmute_copy(&idsnapshotolder), ::core::mem::transmute_copy(&idsnapshotyounger), ::core::mem::transmute_copy(&pcblocksizeperbit), ::core::mem::transmute_copy(&pcbitmaplength), ::core::mem::transmute_copy(&ppbbitmap)).into()
        }
        Self {
            base: IVssDifferentialSoftwareSnapshotMgmt2_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetVolumeProtectLevel: SetVolumeProtectLevel::<Identity, Impl, OFFSET>,
            GetVolumeProtectLevel: GetVolumeProtectLevel::<Identity, Impl, OFFSET>,
            ClearVolumeProtectFault: ClearVolumeProtectFault::<Identity, Impl, OFFSET>,
            DeleteUnusedDiffAreas: DeleteUnusedDiffAreas::<Identity, Impl, OFFSET>,
            QuerySnapshotDeltaBitmap: QuerySnapshotDeltaBitmap::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssDifferentialSoftwareSnapshotMgmt3 as ::windows::core::Interface>::IID || iid == &<IVssDifferentialSoftwareSnapshotMgmt as ::windows::core::Interface>::IID || iid == &<IVssDifferentialSoftwareSnapshotMgmt2 as ::windows::core::Interface>::IID
    }
}
pub trait IVssEnumMgmtObject_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut VSS_MGMT_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self, ppenum: *mut ::core::option::Option<IVssEnumMgmtObject>) -> ::windows::core::Result<()>;
}
impl IVssEnumMgmtObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssEnumMgmtObject_Impl, const OFFSET: isize>() -> IVssEnumMgmtObject_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IVssEnumMgmtObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut VSS_MGMT_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IVssEnumMgmtObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IVssEnumMgmtObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IVssEnumMgmtObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clone(::core::mem::transmute_copy(&ppenum)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssEnumMgmtObject as ::windows::core::Interface>::IID
    }
}
pub trait IVssEnumObject_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut VSS_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self, ppenum: *mut ::core::option::Option<IVssEnumObject>) -> ::windows::core::Result<()>;
}
impl IVssEnumObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssEnumObject_Impl, const OFFSET: isize>() -> IVssEnumObject_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IVssEnumObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut VSS_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IVssEnumObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IVssEnumObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IVssEnumObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clone(::core::mem::transmute_copy(&ppenum)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssEnumObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssExpressWriter_Impl: Sized {
    fn CreateMetadata(&mut self, writerid: &::windows::core::GUID, writername: super::super::Foundation::PWSTR, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32) -> ::windows::core::Result<IVssCreateExpressWriterMetadata>;
    fn LoadMetadata(&mut self, metadata: super::super::Foundation::PWSTR, reserved: u32) -> ::windows::core::Result<()>;
    fn Register(&mut self) -> ::windows::core::Result<()>;
    fn Unregister(&mut self, writerid: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVssExpressWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssExpressWriter_Impl, const OFFSET: isize>() -> IVssExpressWriter_Vtbl {
        unsafe extern "system" fn CreateMetadata<Identity: ::windows::core::IUnknownImpl, Impl: IVssExpressWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writerid: ::windows::core::GUID, writername: super::super::Foundation::PWSTR, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32, ppmetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateMetadata(::core::mem::transmute_copy(&writerid), ::core::mem::transmute_copy(&writername), ::core::mem::transmute_copy(&usagetype), ::core::mem::transmute_copy(&versionmajor), ::core::mem::transmute_copy(&versionminor), ::core::mem::transmute_copy(&reserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmetadata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadMetadata<Identity: ::windows::core::IUnknownImpl, Impl: IVssExpressWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadata: super::super::Foundation::PWSTR, reserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadMetadata(::core::mem::transmute_copy(&metadata), ::core::mem::transmute_copy(&reserved)).into()
        }
        unsafe extern "system" fn Register<Identity: ::windows::core::IUnknownImpl, Impl: IVssExpressWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Register().into()
        }
        unsafe extern "system" fn Unregister<Identity: ::windows::core::IUnknownImpl, Impl: IVssExpressWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writerid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unregister(::core::mem::transmute_copy(&writerid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateMetadata: CreateMetadata::<Identity, Impl, OFFSET>,
            LoadMetadata: LoadMetadata::<Identity, Impl, OFFSET>,
            Register: Register::<Identity, Impl, OFFSET>,
            Unregister: Unregister::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssExpressWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVssFileShareSnapshotProvider_Impl: Sized {
    fn SetContext(&mut self, lcontext: i32) -> ::windows::core::Result<()>;
    fn GetSnapshotProperties(&mut self, snapshotid: &::windows::core::GUID) -> ::windows::core::Result<VSS_SNAPSHOT_PROP>;
    fn Query(&mut self, queriedobjectid: &::windows::core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE) -> ::windows::core::Result<IVssEnumObject>;
    fn DeleteSnapshots(&mut self, sourceobjectid: &::windows::core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn BeginPrepareSnapshot(&mut self, snapshotsetid: &::windows::core::GUID, snapshotid: &::windows::core::GUID, pwszsharepath: *const u16, lnewcontext: i32, providerid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn IsPathSupported(&mut self, pwszsharepath: *const u16) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsPathSnapshotted(&mut self, pwszsharepath: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows::core::Result<()>;
    fn SetSnapshotProperty(&mut self, snapshotid: &::windows::core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVssFileShareSnapshotProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>() -> IVssFileShareSnapshotProvider_Vtbl {
        unsafe extern "system" fn SetContext<Identity: ::windows::core::IUnknownImpl, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcontext: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetContext(::core::mem::transmute_copy(&lcontext)).into()
        }
        unsafe extern "system" fn GetSnapshotProperties<Identity: ::windows::core::IUnknownImpl, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSnapshotProperties(::core::mem::transmute_copy(&snapshotid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Query<Identity: ::windows::core::IUnknownImpl, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queriedobjectid: ::windows::core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Query(::core::mem::transmute_copy(&queriedobjectid), ::core::mem::transmute_copy(&equeriedobjecttype), ::core::mem::transmute_copy(&ereturnedobjectstype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteSnapshots<Identity: ::windows::core::IUnknownImpl, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceobjectid: ::windows::core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteSnapshots(::core::mem::transmute_copy(&sourceobjectid), ::core::mem::transmute_copy(&esourceobjecttype), ::core::mem::transmute_copy(&bforcedelete), ::core::mem::transmute_copy(&pldeletedsnapshots), ::core::mem::transmute_copy(&pnondeletedsnapshotid)).into()
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Identity: ::windows::core::IUnknownImpl, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, pwszsharepath: *const u16, lnewcontext: i32, providerid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginPrepareSnapshot(::core::mem::transmute_copy(&snapshotsetid), ::core::mem::transmute_copy(&snapshotid), ::core::mem::transmute_copy(&pwszsharepath), ::core::mem::transmute_copy(&lnewcontext), ::core::mem::transmute_copy(&providerid)).into()
        }
        unsafe extern "system" fn IsPathSupported<Identity: ::windows::core::IUnknownImpl, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsharepath: *const u16, pbsupportedbythisprovider: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsPathSupported(::core::mem::transmute_copy(&pwszsharepath)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbsupportedbythisprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPathSnapshotted<Identity: ::windows::core::IUnknownImpl, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsharepath: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsPathSnapshotted(::core::mem::transmute_copy(&pwszsharepath), ::core::mem::transmute_copy(&pbsnapshotspresent), ::core::mem::transmute_copy(&plsnapshotcompatibility)).into()
        }
        unsafe extern "system" fn SetSnapshotProperty<Identity: ::windows::core::IUnknownImpl, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSnapshotProperty(::core::mem::transmute_copy(&snapshotid), ::core::mem::transmute_copy(&esnapshotpropertyid), ::core::mem::transmute_copy(&vproperty)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetContext: SetContext::<Identity, Impl, OFFSET>,
            GetSnapshotProperties: GetSnapshotProperties::<Identity, Impl, OFFSET>,
            Query: Query::<Identity, Impl, OFFSET>,
            DeleteSnapshots: DeleteSnapshots::<Identity, Impl, OFFSET>,
            BeginPrepareSnapshot: BeginPrepareSnapshot::<Identity, Impl, OFFSET>,
            IsPathSupported: IsPathSupported::<Identity, Impl, OFFSET>,
            IsPathSnapshotted: IsPathSnapshotted::<Identity, Impl, OFFSET>,
            SetSnapshotProperty: SetSnapshotProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssFileShareSnapshotProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
pub trait IVssHardwareSnapshotProvider_Impl: Sized {
    fn AreLunsSupported(&mut self, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn FillInLunInfo(&mut self, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn BeginPrepareSnapshot(&mut self, snapshotsetid: &::windows::core::GUID, snapshotid: &::windows::core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()>;
    fn GetTargetLuns(&mut self, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()>;
    fn LocateLuns(&mut self, lluncount: i32, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()>;
    fn OnLunEmpty(&mut self, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
impl IVssHardwareSnapshotProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssHardwareSnapshotProvider_Impl, const OFFSET: isize>() -> IVssHardwareSnapshotProvider_Vtbl {
        unsafe extern "system" fn AreLunsSupported<Identity: ::windows::core::IUnknownImpl, Impl: IVssHardwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AreLunsSupported(::core::mem::transmute_copy(&lluncount), ::core::mem::transmute_copy(&lcontext), ::core::mem::transmute_copy(&rgwszdevices), ::core::mem::transmute_copy(&pluninformation), ::core::mem::transmute_copy(&pbissupported)).into()
        }
        unsafe extern "system" fn FillInLunInfo<Identity: ::windows::core::IUnknownImpl, Impl: IVssHardwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FillInLunInfo(::core::mem::transmute_copy(&wszdevicename), ::core::mem::transmute_copy(&pluninfo), ::core::mem::transmute_copy(&pbissupported)).into()
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Identity: ::windows::core::IUnknownImpl, Impl: IVssHardwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginPrepareSnapshot(::core::mem::transmute_copy(&snapshotsetid), ::core::mem::transmute_copy(&snapshotid), ::core::mem::transmute_copy(&lcontext), ::core::mem::transmute_copy(&lluncount), ::core::mem::transmute_copy(&rgdevicenames), ::core::mem::transmute_copy(&rgluninformation)).into()
        }
        unsafe extern "system" fn GetTargetLuns<Identity: ::windows::core::IUnknownImpl, Impl: IVssHardwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTargetLuns(::core::mem::transmute_copy(&lluncount), ::core::mem::transmute_copy(&rgdevicenames), ::core::mem::transmute_copy(&rgsourceluns), ::core::mem::transmute_copy(&rgdestinationluns)).into()
        }
        unsafe extern "system" fn LocateLuns<Identity: ::windows::core::IUnknownImpl, Impl: IVssHardwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lluncount: i32, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LocateLuns(::core::mem::transmute_copy(&lluncount), ::core::mem::transmute_copy(&rgsourceluns)).into()
        }
        unsafe extern "system" fn OnLunEmpty<Identity: ::windows::core::IUnknownImpl, Impl: IVssHardwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnLunEmpty(::core::mem::transmute_copy(&wszdevicename), ::core::mem::transmute_copy(&pinformation)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AreLunsSupported: AreLunsSupported::<Identity, Impl, OFFSET>,
            FillInLunInfo: FillInLunInfo::<Identity, Impl, OFFSET>,
            BeginPrepareSnapshot: BeginPrepareSnapshot::<Identity, Impl, OFFSET>,
            GetTargetLuns: GetTargetLuns::<Identity, Impl, OFFSET>,
            LocateLuns: LocateLuns::<Identity, Impl, OFFSET>,
            OnLunEmpty: OnLunEmpty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssHardwareSnapshotProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
pub trait IVssHardwareSnapshotProviderEx_Impl: Sized + IVssHardwareSnapshotProvider_Impl {
    fn GetProviderCapabilities(&mut self) -> ::windows::core::Result<u64>;
    fn OnLunStateChange(&mut self, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn ResyncLuns(&mut self, psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> ::windows::core::Result<IVssAsync>;
    fn OnReuseLuns(&mut self, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
impl IVssHardwareSnapshotProviderEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssHardwareSnapshotProviderEx_Impl, const OFFSET: isize>() -> IVssHardwareSnapshotProviderEx_Vtbl {
        unsafe extern "system" fn GetProviderCapabilities<Identity: ::windows::core::IUnknownImpl, Impl: IVssHardwareSnapshotProviderEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plloriginalcapabilitymask: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProviderCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *plloriginalcapabilitymask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLunStateChange<Identity: ::windows::core::IUnknownImpl, Impl: IVssHardwareSnapshotProviderEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnLunStateChange(::core::mem::transmute_copy(&psnapshotluns), ::core::mem::transmute_copy(&poriginalluns), ::core::mem::transmute_copy(&dwcount), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn ResyncLuns<Identity: ::windows::core::IUnknownImpl, Impl: IVssHardwareSnapshotProviderEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResyncLuns(::core::mem::transmute_copy(&psourceluns), ::core::mem::transmute_copy(&ptargetluns), ::core::mem::transmute_copy(&dwcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnReuseLuns<Identity: ::windows::core::IUnknownImpl, Impl: IVssHardwareSnapshotProviderEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnReuseLuns(::core::mem::transmute_copy(&psnapshotluns), ::core::mem::transmute_copy(&poriginalluns), ::core::mem::transmute_copy(&dwcount)).into()
        }
        Self {
            base: IVssHardwareSnapshotProvider_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetProviderCapabilities: GetProviderCapabilities::<Identity, Impl, OFFSET>,
            OnLunStateChange: OnLunStateChange::<Identity, Impl, OFFSET>,
            ResyncLuns: ResyncLuns::<Identity, Impl, OFFSET>,
            OnReuseLuns: OnReuseLuns::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssHardwareSnapshotProviderEx as ::windows::core::Interface>::IID || iid == &<IVssHardwareSnapshotProvider as ::windows::core::Interface>::IID
    }
}
pub trait IVssProviderCreateSnapshotSet_Impl: Sized {
    fn EndPrepareSnapshots(&mut self, snapshotsetid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PreCommitSnapshots(&mut self, snapshotsetid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn CommitSnapshots(&mut self, snapshotsetid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PostCommitSnapshots(&mut self, snapshotsetid: &::windows::core::GUID, lsnapshotscount: i32) -> ::windows::core::Result<()>;
    fn PreFinalCommitSnapshots(&mut self, snapshotsetid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PostFinalCommitSnapshots(&mut self, snapshotsetid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn AbortSnapshots(&mut self, snapshotsetid: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl IVssProviderCreateSnapshotSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>() -> IVssProviderCreateSnapshotSet_Vtbl {
        unsafe extern "system" fn EndPrepareSnapshots<Identity: ::windows::core::IUnknownImpl, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndPrepareSnapshots(::core::mem::transmute_copy(&snapshotsetid)).into()
        }
        unsafe extern "system" fn PreCommitSnapshots<Identity: ::windows::core::IUnknownImpl, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PreCommitSnapshots(::core::mem::transmute_copy(&snapshotsetid)).into()
        }
        unsafe extern "system" fn CommitSnapshots<Identity: ::windows::core::IUnknownImpl, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CommitSnapshots(::core::mem::transmute_copy(&snapshotsetid)).into()
        }
        unsafe extern "system" fn PostCommitSnapshots<Identity: ::windows::core::IUnknownImpl, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID, lsnapshotscount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PostCommitSnapshots(::core::mem::transmute_copy(&snapshotsetid), ::core::mem::transmute_copy(&lsnapshotscount)).into()
        }
        unsafe extern "system" fn PreFinalCommitSnapshots<Identity: ::windows::core::IUnknownImpl, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PreFinalCommitSnapshots(::core::mem::transmute_copy(&snapshotsetid)).into()
        }
        unsafe extern "system" fn PostFinalCommitSnapshots<Identity: ::windows::core::IUnknownImpl, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PostFinalCommitSnapshots(::core::mem::transmute_copy(&snapshotsetid)).into()
        }
        unsafe extern "system" fn AbortSnapshots<Identity: ::windows::core::IUnknownImpl, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AbortSnapshots(::core::mem::transmute_copy(&snapshotsetid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            EndPrepareSnapshots: EndPrepareSnapshots::<Identity, Impl, OFFSET>,
            PreCommitSnapshots: PreCommitSnapshots::<Identity, Impl, OFFSET>,
            CommitSnapshots: CommitSnapshots::<Identity, Impl, OFFSET>,
            PostCommitSnapshots: PostCommitSnapshots::<Identity, Impl, OFFSET>,
            PreFinalCommitSnapshots: PreFinalCommitSnapshots::<Identity, Impl, OFFSET>,
            PostFinalCommitSnapshots: PostFinalCommitSnapshots::<Identity, Impl, OFFSET>,
            AbortSnapshots: AbortSnapshots::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssProviderCreateSnapshotSet as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssProviderNotifications_Impl: Sized {
    fn OnLoad(&mut self, pcallback: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn OnUnload(&mut self, bforceunload: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVssProviderNotifications_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssProviderNotifications_Impl, const OFFSET: isize>() -> IVssProviderNotifications_Vtbl {
        unsafe extern "system" fn OnLoad<Identity: ::windows::core::IUnknownImpl, Impl: IVssProviderNotifications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnLoad(::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn OnUnload<Identity: ::windows::core::IUnknownImpl, Impl: IVssProviderNotifications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bforceunload: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnUnload(::core::mem::transmute_copy(&bforceunload)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnLoad: OnLoad::<Identity, Impl, OFFSET>,
            OnUnload: OnUnload::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssProviderNotifications as ::windows::core::Interface>::IID
    }
}
pub trait IVssSnapshotMgmt_Impl: Sized {
    fn GetProviderMgmtInterface(&mut self, providerid: &::windows::core::GUID, interfaceid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn QueryVolumesSupportedForSnapshots(&mut self, providerid: &::windows::core::GUID, lcontext: i32) -> ::windows::core::Result<IVssEnumMgmtObject>;
    fn QuerySnapshotsByVolume(&mut self, pwszvolumename: *const u16, providerid: &::windows::core::GUID) -> ::windows::core::Result<IVssEnumObject>;
}
impl IVssSnapshotMgmt_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssSnapshotMgmt_Impl, const OFFSET: isize>() -> IVssSnapshotMgmt_Vtbl {
        unsafe extern "system" fn GetProviderMgmtInterface<Identity: ::windows::core::IUnknownImpl, Impl: IVssSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, interfaceid: *const ::windows::core::GUID, ppitf: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProviderMgmtInterface(::core::mem::transmute_copy(&providerid), ::core::mem::transmute_copy(&interfaceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitf = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryVolumesSupportedForSnapshots<Identity: ::windows::core::IUnknownImpl, Impl: IVssSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, lcontext: i32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryVolumesSupportedForSnapshots(::core::mem::transmute_copy(&providerid), ::core::mem::transmute_copy(&lcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuerySnapshotsByVolume<Identity: ::windows::core::IUnknownImpl, Impl: IVssSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, providerid: ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QuerySnapshotsByVolume(::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&providerid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProviderMgmtInterface: GetProviderMgmtInterface::<Identity, Impl, OFFSET>,
            QueryVolumesSupportedForSnapshots: QueryVolumesSupportedForSnapshots::<Identity, Impl, OFFSET>,
            QuerySnapshotsByVolume: QuerySnapshotsByVolume::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssSnapshotMgmt as ::windows::core::Interface>::IID
    }
}
pub trait IVssSnapshotMgmt2_Impl: Sized {
    fn GetMinDiffAreaSize(&mut self) -> ::windows::core::Result<i64>;
}
impl IVssSnapshotMgmt2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssSnapshotMgmt2_Impl, const OFFSET: isize>() -> IVssSnapshotMgmt2_Vtbl {
        unsafe extern "system" fn GetMinDiffAreaSize<Identity: ::windows::core::IUnknownImpl, Impl: IVssSnapshotMgmt2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllmindiffareasize: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMinDiffAreaSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pllmindiffareasize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetMinDiffAreaSize: GetMinDiffAreaSize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssSnapshotMgmt2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVssSoftwareSnapshotProvider_Impl: Sized {
    fn SetContext(&mut self, lcontext: i32) -> ::windows::core::Result<()>;
    fn GetSnapshotProperties(&mut self, snapshotid: &::windows::core::GUID) -> ::windows::core::Result<VSS_SNAPSHOT_PROP>;
    fn Query(&mut self, queriedobjectid: &::windows::core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE) -> ::windows::core::Result<IVssEnumObject>;
    fn DeleteSnapshots(&mut self, sourceobjectid: &::windows::core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn BeginPrepareSnapshot(&mut self, snapshotsetid: &::windows::core::GUID, snapshotid: &::windows::core::GUID, pwszvolumename: *const u16, lnewcontext: i32) -> ::windows::core::Result<()>;
    fn IsVolumeSupported(&mut self, pwszvolumename: *const u16) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsVolumeSnapshotted(&mut self, pwszvolumename: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows::core::Result<()>;
    fn SetSnapshotProperty(&mut self, snapshotid: &::windows::core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn RevertToSnapshot(&mut self, snapshotid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn QueryRevertStatus(&mut self, pwszvolume: *const u16) -> ::windows::core::Result<IVssAsync>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVssSoftwareSnapshotProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>() -> IVssSoftwareSnapshotProvider_Vtbl {
        unsafe extern "system" fn SetContext<Identity: ::windows::core::IUnknownImpl, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcontext: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetContext(::core::mem::transmute_copy(&lcontext)).into()
        }
        unsafe extern "system" fn GetSnapshotProperties<Identity: ::windows::core::IUnknownImpl, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSnapshotProperties(::core::mem::transmute_copy(&snapshotid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Query<Identity: ::windows::core::IUnknownImpl, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queriedobjectid: ::windows::core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Query(::core::mem::transmute_copy(&queriedobjectid), ::core::mem::transmute_copy(&equeriedobjecttype), ::core::mem::transmute_copy(&ereturnedobjectstype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteSnapshots<Identity: ::windows::core::IUnknownImpl, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceobjectid: ::windows::core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteSnapshots(::core::mem::transmute_copy(&sourceobjectid), ::core::mem::transmute_copy(&esourceobjecttype), ::core::mem::transmute_copy(&bforcedelete), ::core::mem::transmute_copy(&pldeletedsnapshots), ::core::mem::transmute_copy(&pnondeletedsnapshotid)).into()
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Identity: ::windows::core::IUnknownImpl, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, pwszvolumename: *const u16, lnewcontext: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginPrepareSnapshot(::core::mem::transmute_copy(&snapshotsetid), ::core::mem::transmute_copy(&snapshotid), ::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&lnewcontext)).into()
        }
        unsafe extern "system" fn IsVolumeSupported<Identity: ::windows::core::IUnknownImpl, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pbsupportedbythisprovider: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsVolumeSupported(::core::mem::transmute_copy(&pwszvolumename)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbsupportedbythisprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVolumeSnapshotted<Identity: ::windows::core::IUnknownImpl, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsVolumeSnapshotted(::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&pbsnapshotspresent), ::core::mem::transmute_copy(&plsnapshotcompatibility)).into()
        }
        unsafe extern "system" fn SetSnapshotProperty<Identity: ::windows::core::IUnknownImpl, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSnapshotProperty(::core::mem::transmute_copy(&snapshotid), ::core::mem::transmute_copy(&esnapshotpropertyid), ::core::mem::transmute_copy(&vproperty)).into()
        }
        unsafe extern "system" fn RevertToSnapshot<Identity: ::windows::core::IUnknownImpl, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RevertToSnapshot(::core::mem::transmute_copy(&snapshotid)).into()
        }
        unsafe extern "system" fn QueryRevertStatus<Identity: ::windows::core::IUnknownImpl, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolume: *const u16, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryRevertStatus(::core::mem::transmute_copy(&pwszvolume)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetContext: SetContext::<Identity, Impl, OFFSET>,
            GetSnapshotProperties: GetSnapshotProperties::<Identity, Impl, OFFSET>,
            Query: Query::<Identity, Impl, OFFSET>,
            DeleteSnapshots: DeleteSnapshots::<Identity, Impl, OFFSET>,
            BeginPrepareSnapshot: BeginPrepareSnapshot::<Identity, Impl, OFFSET>,
            IsVolumeSupported: IsVolumeSupported::<Identity, Impl, OFFSET>,
            IsVolumeSnapshotted: IsVolumeSnapshotted::<Identity, Impl, OFFSET>,
            SetSnapshotProperty: SetSnapshotProperty::<Identity, Impl, OFFSET>,
            RevertToSnapshot: RevertToSnapshot::<Identity, Impl, OFFSET>,
            QueryRevertStatus: QueryRevertStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssSoftwareSnapshotProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssWMDependency_Impl: Sized {
    fn GetWriterId(&mut self, pwriterid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetLogicalPath(&mut self, pbstrlogicalpath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetComponentName(&mut self, pbstrcomponentname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVssWMDependency_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssWMDependency_Impl, const OFFSET: isize>() -> IVssWMDependency_Vtbl {
        unsafe extern "system" fn GetWriterId<Identity: ::windows::core::IUnknownImpl, Impl: IVssWMDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwriterid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetWriterId(::core::mem::transmute_copy(&pwriterid)).into()
        }
        unsafe extern "system" fn GetLogicalPath<Identity: ::windows::core::IUnknownImpl, Impl: IVssWMDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlogicalpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLogicalPath(::core::mem::transmute_copy(&pbstrlogicalpath)).into()
        }
        unsafe extern "system" fn GetComponentName<Identity: ::windows::core::IUnknownImpl, Impl: IVssWMDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcomponentname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetComponentName(::core::mem::transmute_copy(&pbstrcomponentname)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetWriterId: GetWriterId::<Identity, Impl, OFFSET>,
            GetLogicalPath: GetLogicalPath::<Identity, Impl, OFFSET>,
            GetComponentName: GetComponentName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssWMDependency as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssWMFiledesc_Impl: Sized {
    fn GetPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetFilespec(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetRecursive(&mut self) -> ::windows::core::Result<bool>;
    fn GetAlternateLocation(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetBackupTypeMask(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVssWMFiledesc_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssWMFiledesc_Impl, const OFFSET: isize>() -> IVssWMFiledesc_Vtbl {
        unsafe extern "system" fn GetPath<Identity: ::windows::core::IUnknownImpl, Impl: IVssWMFiledesc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilespec<Identity: ::windows::core::IUnknownImpl, Impl: IVssWMFiledesc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfilespec: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFilespec() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrfilespec = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecursive<Identity: ::windows::core::IUnknownImpl, Impl: IVssWMFiledesc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbrecursive: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRecursive() {
                ::core::result::Result::Ok(ok__) => {
                    *pbrecursive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAlternateLocation<Identity: ::windows::core::IUnknownImpl, Impl: IVssWMFiledesc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstralternatelocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAlternateLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstralternatelocation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackupTypeMask<Identity: ::windows::core::IUnknownImpl, Impl: IVssWMFiledesc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtypemask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBackupTypeMask() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwtypemask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            GetFilespec: GetFilespec::<Identity, Impl, OFFSET>,
            GetRecursive: GetRecursive::<Identity, Impl, OFFSET>,
            GetAlternateLocation: GetAlternateLocation::<Identity, Impl, OFFSET>,
            GetBackupTypeMask: GetBackupTypeMask::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssWMFiledesc as ::windows::core::Interface>::IID
    }
}
pub trait IVssWriterComponents_Impl: Sized {
    fn GetComponentCount(&mut self, pccomponents: *mut u32) -> ::windows::core::Result<()>;
    fn GetWriterInfo(&mut self, pidinstance: *mut ::windows::core::GUID, pidwriter: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetComponent(&mut self, icomponent: u32) -> ::windows::core::Result<IVssComponent>;
}
impl IVssWriterComponents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterComponents_Impl, const OFFSET: isize>() -> IVssWriterComponents_Vtbl {
        unsafe extern "system" fn GetComponentCount<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterComponents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccomponents: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetComponentCount(::core::mem::transmute_copy(&pccomponents)).into()
        }
        unsafe extern "system" fn GetWriterInfo<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterComponents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidinstance: *mut ::windows::core::GUID, pidwriter: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetWriterInfo(::core::mem::transmute_copy(&pidinstance), ::core::mem::transmute_copy(&pidwriter)).into()
        }
        unsafe extern "system" fn GetComponent<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterComponents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icomponent: u32, ppcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetComponent(::core::mem::transmute_copy(&icomponent)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomponent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            GetComponentCount: GetComponentCount::<Identity, Impl, OFFSET>,
            GetWriterInfo: GetWriterInfo::<Identity, Impl, OFFSET>,
            GetComponent: GetComponent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssWriterComponents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVssWriterImpl_Impl: Sized {
    fn Initialize(&mut self, writerid: &::windows::core::GUID, wszwritername: super::super::Foundation::PWSTR, wszwriterinstancename: super::super::Foundation::PWSTR, dwmajorversion: u32, dwminorversion: u32, ut: VSS_USAGE_TYPE, st: VSS_SOURCE_TYPE, nlevel: VSS_APPLICATION_LEVEL, dwtimeout: u32, aws: VSS_ALTERNATE_WRITER_STATE, biothrottlingonly: u8) -> ::windows::core::Result<()>;
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
    fn InstallAlternateWriter(&mut self, idwriter: &::windows::core::GUID, clsid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetIdentityInformation(&mut self) -> *mut IVssExamineWriterMetadata;
    fn SetWriterFailureEx(&mut self, hr: ::windows::core::HRESULT, hrapplication: ::windows::core::HRESULT, wszapplicationmessage: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetSessionId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IsWriterShuttingDown(&mut self) -> bool;
}
#[cfg(feature = "Win32_Foundation")]
impl IVssWriterImpl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImpl_Impl, const OFFSET: isize>() -> IVssWriterImpl_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writerid: ::windows::core::GUID, wszwritername: super::super::Foundation::PWSTR, wszwriterinstancename: super::super::Foundation::PWSTR, dwmajorversion: u32, dwminorversion: u32, ut: VSS_USAGE_TYPE, st: VSS_SOURCE_TYPE, nlevel: VSS_APPLICATION_LEVEL, dwtimeout: u32, aws: VSS_ALTERNATE_WRITER_STATE, biothrottlingonly: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .Initialize(::core::mem::transmute_copy(&writerid), ::core::mem::transmute_copy(&wszwritername), ::core::mem::transmute_copy(&wszwriterinstancename), ::core::mem::transmute_copy(&dwmajorversion), ::core::mem::transmute_copy(&dwminorversion), ::core::mem::transmute_copy(&ut), ::core::mem::transmute_copy(&st), ::core::mem::transmute_copy(&nlevel), ::core::mem::transmute_copy(&dwtimeout), ::core::mem::transmute_copy(&aws), ::core::mem::transmute_copy(&biothrottlingonly))
                .into()
        }
        unsafe extern "system" fn Subscribe<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsubscribetimeout: u32, dweventflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Subscribe(::core::mem::transmute_copy(&dwsubscribetimeout), ::core::mem::transmute_copy(&dweventflags)).into()
        }
        unsafe extern "system" fn Unsubscribe<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unsubscribe().into()
        }
        unsafe extern "system" fn Uninitialize<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Uninitialize()
        }
        unsafe extern "system" fn GetCurrentVolumeArray<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut super::super::Foundation::PWSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCurrentVolumeArray()
        }
        unsafe extern "system" fn GetCurrentVolumeCount<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCurrentVolumeCount()
        }
        unsafe extern "system" fn GetSnapshotDeviceName<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszoriginalvolume: super::super::Foundation::PWSTR, ppwszsnapshotdevice: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSnapshotDeviceName(::core::mem::transmute_copy(&wszoriginalvolume)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszsnapshotdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentSnapshotSetId<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            *result__ = (*this).GetCurrentSnapshotSetId()
        }
        unsafe extern "system" fn GetContext<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetContext()
        }
        unsafe extern "system" fn GetCurrentLevel<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> VSS_APPLICATION_LEVEL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCurrentLevel()
        }
        unsafe extern "system" fn IsPathAffected<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsPathAffected(::core::mem::transmute_copy(&wszpath))
        }
        unsafe extern "system" fn IsBootableSystemStateBackedUp<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsBootableSystemStateBackedUp()
        }
        unsafe extern "system" fn AreComponentsSelected<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AreComponentsSelected()
        }
        unsafe extern "system" fn GetBackupType<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> VSS_BACKUP_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBackupType()
        }
        unsafe extern "system" fn GetRestoreType<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> VSS_RESTORE_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRestoreType()
        }
        unsafe extern "system" fn SetWriterFailure<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWriterFailure(::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn IsPartialFileSupportEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsPartialFileSupportEnabled()
        }
        unsafe extern "system" fn InstallAlternateWriter<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idwriter: ::windows::core::GUID, clsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InstallAlternateWriter(::core::mem::transmute_copy(&idwriter), ::core::mem::transmute_copy(&clsid)).into()
        }
        unsafe extern "system" fn GetIdentityInformation<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut IVssExamineWriterMetadata {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetIdentityInformation()
        }
        unsafe extern "system" fn SetWriterFailureEx<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, hrapplication: ::windows::core::HRESULT, wszapplicationmessage: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWriterFailureEx(::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&hrapplication), ::core::mem::transmute_copy(&wszapplicationmessage)).into()
        }
        unsafe extern "system" fn GetSessionId<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idsession: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSessionId() {
                ::core::result::Result::Ok(ok__) => {
                    *idsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWriterShuttingDown<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsWriterShuttingDown()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Subscribe: Subscribe::<Identity, Impl, OFFSET>,
            Unsubscribe: Unsubscribe::<Identity, Impl, OFFSET>,
            Uninitialize: Uninitialize::<Identity, Impl, OFFSET>,
            GetCurrentVolumeArray: GetCurrentVolumeArray::<Identity, Impl, OFFSET>,
            GetCurrentVolumeCount: GetCurrentVolumeCount::<Identity, Impl, OFFSET>,
            GetSnapshotDeviceName: GetSnapshotDeviceName::<Identity, Impl, OFFSET>,
            GetCurrentSnapshotSetId: GetCurrentSnapshotSetId::<Identity, Impl, OFFSET>,
            GetContext: GetContext::<Identity, Impl, OFFSET>,
            GetCurrentLevel: GetCurrentLevel::<Identity, Impl, OFFSET>,
            IsPathAffected: IsPathAffected::<Identity, Impl, OFFSET>,
            IsBootableSystemStateBackedUp: IsBootableSystemStateBackedUp::<Identity, Impl, OFFSET>,
            AreComponentsSelected: AreComponentsSelected::<Identity, Impl, OFFSET>,
            GetBackupType: GetBackupType::<Identity, Impl, OFFSET>,
            GetRestoreType: GetRestoreType::<Identity, Impl, OFFSET>,
            SetWriterFailure: SetWriterFailure::<Identity, Impl, OFFSET>,
            IsPartialFileSupportEnabled: IsPartialFileSupportEnabled::<Identity, Impl, OFFSET>,
            InstallAlternateWriter: InstallAlternateWriter::<Identity, Impl, OFFSET>,
            GetIdentityInformation: GetIdentityInformation::<Identity, Impl, OFFSET>,
            SetWriterFailureEx: SetWriterFailureEx::<Identity, Impl, OFFSET>,
            GetSessionId: GetSessionId::<Identity, Impl, OFFSET>,
            IsWriterShuttingDown: IsWriterShuttingDown::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVssWriterImpl as ::windows::core::Interface>::IID
    }
}
