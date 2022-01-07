pub trait IVssAdminImpl: Sized {
    fn RegisterProvider();
    fn UnregisterProvider();
    fn QueryProviders();
    fn AbortAllSnapshotsInProgress();
}
impl ::windows::core::RuntimeName for IVssAdmin {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssAdmin";
}
impl IVssAdminVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssAdminImpl, const OFFSET: isize>() -> IVssAdminVtbl {
        unsafe extern "system" fn RegisterProvider<Impl: IVssAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproviderid: ::windows::core::GUID, classid: ::windows::core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterProvider(
                &*(&pproviderid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&classid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                pwszprovidername,
                eprovidertype,
                pwszproviderversion,
                &*(&providerversionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterProvider<Impl: IVssAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterProvider(&*(&providerid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryProviders<Impl: IVssAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryProviders(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbortAllSnapshotsInProgress<Impl: IVssAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AbortAllSnapshotsInProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVssAdmin>, ::windows::core::GetTrustLevel, RegisterProvider::<Impl, OFFSET>, UnregisterProvider::<Impl, OFFSET>, QueryProviders::<Impl, OFFSET>, AbortAllSnapshotsInProgress::<Impl, OFFSET>)
    }
}
pub trait IVssAdminExImpl: Sized + IVssAdminImpl {
    fn GetProviderCapability();
    fn GetProviderContext();
    fn SetProviderContext();
}
impl ::windows::core::RuntimeName for IVssAdminEx {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssAdminEx";
}
impl IVssAdminExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssAdminExImpl, const OFFSET: isize>() -> IVssAdminExVtbl {
        unsafe extern "system" fn GetProviderCapability<Impl: IVssAdminExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproviderid: ::windows::core::GUID, plloriginalcapabilitymask: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderCapability(&*(&pproviderid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&plloriginalcapabilitymask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderContext<Impl: IVssAdminExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, plcontext: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderContext(&*(&providerid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&plcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderContext<Impl: IVssAdminExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, lcontext: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProviderContext(&*(&providerid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), lcontext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVssAdminEx>, ::windows::core::GetTrustLevel, GetProviderCapability::<Impl, OFFSET>, GetProviderContext::<Impl, OFFSET>, SetProviderContext::<Impl, OFFSET>)
    }
}
pub trait IVssAsyncImpl: Sized {
    fn Cancel();
    fn Wait();
    fn QueryStatus();
}
impl ::windows::core::RuntimeName for IVssAsync {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssAsync";
}
impl IVssAsyncVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssAsyncImpl, const OFFSET: isize>() -> IVssAsyncVtbl {
        unsafe extern "system" fn Cancel<Impl: IVssAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Wait<Impl: IVssAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Wait(dwmilliseconds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryStatus<Impl: IVssAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT, preserved: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryStatus(::core::mem::transmute_copy(&phrresult), preserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVssAsync>, ::windows::core::GetTrustLevel, Cancel::<Impl, OFFSET>, Wait::<Impl, OFFSET>, QueryStatus::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IVssComponent {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssComponent";
}
impl IVssComponentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponentImpl, const OFFSET: isize>() -> IVssComponentVtbl {
        unsafe extern "system" fn GetLogicalPath<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLogicalPath(&*(&pbstrpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetComponentType<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pct: *mut VSS_COMPONENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetComponentType(pct) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetComponentName<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetComponentName(&*(&pbstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackupSucceeded<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsucceeded: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackupSucceeded(pbsucceeded) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAlternateLocationMappingCount<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcmappings: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAlternateLocationMappingCount(pcmappings) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAlternateLocationMapping<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imapping: u32, ppfiledesc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAlternateLocationMapping(imapping, ::core::mem::transmute_copy(&ppfiledesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackupMetadata<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszdata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBackupMetadata(&*(&wszdata as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackupMetadata<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackupMetadata(&*(&pbstrdata as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPartialFile<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilename: super::super::Foundation::PWSTR, wszranges: super::super::Foundation::PWSTR, wszmetadata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPartialFile(
                &*(&wszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszranges as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszmetadata as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartialFileCount<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcpartialfiles: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPartialFileCount(pcpartialfiles) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartialFile<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipartialfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilename: *mut super::super::Foundation::BSTR, pbstrrange: *mut super::super::Foundation::BSTR, pbstrmetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPartialFile(
                ipartialfile,
                &*(&pbstrpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pbstrfilename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pbstrrange as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pbstrmetadata as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSelectedForRestore<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbselectedforrestore: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSelectedForRestore(pbselectedforrestore) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdditionalRestores<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbadditionalrestores: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAdditionalRestores(pbadditionalrestores) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNewTargetCount<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnewtarget: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNewTargetCount(pcnewtarget) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNewTarget<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inewtarget: u32, ppfiledesc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNewTarget(inewtarget, ::core::mem::transmute_copy(&ppfiledesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDirectedTarget<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszsourcepath: super::super::Foundation::PWSTR, wszsourcefilename: super::super::Foundation::PWSTR, wszsourcerangelist: super::super::Foundation::PWSTR, wszdestinationpath: super::super::Foundation::PWSTR, wszdestinationfilename: super::super::Foundation::PWSTR, wszdestinationrangelist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddDirectedTarget(
                &*(&wszsourcepath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszsourcefilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszsourcerangelist as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszdestinationpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszdestinationfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszdestinationrangelist as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDirectedTargetCount<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdirectedtarget: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDirectedTargetCount(pcdirectedtarget) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDirectedTarget<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idirectedtarget: u32, pbstrsourcepath: *mut super::super::Foundation::BSTR, pbstrsourcefilename: *mut super::super::Foundation::BSTR, pbstrsourcerangelist: *mut super::super::Foundation::BSTR, pbstrdestinationpath: *mut super::super::Foundation::BSTR, pbstrdestinationfilename: *mut super::super::Foundation::BSTR, pbstrdestinationrangelist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDirectedTarget(
                idirectedtarget,
                &*(&pbstrsourcepath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pbstrsourcefilename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pbstrsourcerangelist as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pbstrdestinationpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pbstrdestinationfilename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pbstrdestinationrangelist as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRestoreMetadata<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszrestoremetadata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRestoreMetadata(&*(&wszrestoremetadata as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestoreMetadata<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrestoremetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRestoreMetadata(&*(&pbstrrestoremetadata as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRestoreTarget<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: VSS_RESTORE_TARGET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRestoreTarget(target) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestoreTarget<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRestoreTarget(ptarget) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreRestoreFailureMsg<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszprerestorefailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPreRestoreFailureMsg(&*(&wszprerestorefailuremsg as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreRestoreFailureMsg<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprerestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreRestoreFailureMsg(&*(&pbstrprerestorefailuremsg as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostRestoreFailureMsg<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpostrestorefailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPostRestoreFailureMsg(&*(&wszpostrestorefailuremsg as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPostRestoreFailureMsg<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpostrestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPostRestoreFailureMsg(&*(&pbstrpostrestorefailuremsg as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackupStamp<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszbackupstamp: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBackupStamp(&*(&wszbackupstamp as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackupStamp<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackupStamp(&*(&pbstrbackupstamp as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousBackupStamp<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreviousBackupStamp(&*(&pbstrbackupstamp as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackupOptions<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbackupoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackupOptions(&*(&pbstrbackupoptions as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestoreOptions<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrestoreoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRestoreOptions(&*(&pbstrrestoreoptions as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestoreSubcomponentCount<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcrestoresubcomponent: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRestoreSubcomponentCount(pcrestoresubcomponent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestoreSubcomponent<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icomponent: u32, pbstrlogicalpath: *mut super::super::Foundation::BSTR, pbstrcomponentname: *mut super::super::Foundation::BSTR, pbrepair: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRestoreSubcomponent(icomponent, &*(&pbstrlogicalpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pbstrcomponentname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), pbrepair) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileRestoreStatus<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileRestoreStatus(pstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDifferencedFilesByLastModifyTime<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: super::super::Foundation::BOOL, ftlastmodifytime: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddDifferencedFilesByLastModifyTime(
                &*(&wszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszfilespec as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&brecursive as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&ftlastmodifytime as *const <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDifferencedFilesByLastModifyLSN<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: super::super::Foundation::BOOL, bstrlsnstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddDifferencedFilesByLastModifyLSN(
                &*(&wszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszfilespec as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&brecursive as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrlsnstring as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDifferencedFilesCount<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdifferencedfiles: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDifferencedFilesCount(pcdifferencedfiles) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDifferencedFile<Impl: IVssComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idifferencedfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilespec: *mut super::super::Foundation::BSTR, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut super::super::Foundation::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDifferencedFile(
                idifferencedfile,
                &*(&pbstrpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pbstrfilespec as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pbrecursive as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&pbstrlsnstring as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pftlastmodifytime as *const <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IVssComponent>,
            ::windows::core::GetTrustLevel,
            GetLogicalPath::<Impl, OFFSET>,
            GetComponentType::<Impl, OFFSET>,
            GetComponentName::<Impl, OFFSET>,
            GetBackupSucceeded::<Impl, OFFSET>,
            GetAlternateLocationMappingCount::<Impl, OFFSET>,
            GetAlternateLocationMapping::<Impl, OFFSET>,
            SetBackupMetadata::<Impl, OFFSET>,
            GetBackupMetadata::<Impl, OFFSET>,
            AddPartialFile::<Impl, OFFSET>,
            GetPartialFileCount::<Impl, OFFSET>,
            GetPartialFile::<Impl, OFFSET>,
            IsSelectedForRestore::<Impl, OFFSET>,
            GetAdditionalRestores::<Impl, OFFSET>,
            GetNewTargetCount::<Impl, OFFSET>,
            GetNewTarget::<Impl, OFFSET>,
            AddDirectedTarget::<Impl, OFFSET>,
            GetDirectedTargetCount::<Impl, OFFSET>,
            GetDirectedTarget::<Impl, OFFSET>,
            SetRestoreMetadata::<Impl, OFFSET>,
            GetRestoreMetadata::<Impl, OFFSET>,
            SetRestoreTarget::<Impl, OFFSET>,
            GetRestoreTarget::<Impl, OFFSET>,
            SetPreRestoreFailureMsg::<Impl, OFFSET>,
            GetPreRestoreFailureMsg::<Impl, OFFSET>,
            SetPostRestoreFailureMsg::<Impl, OFFSET>,
            GetPostRestoreFailureMsg::<Impl, OFFSET>,
            SetBackupStamp::<Impl, OFFSET>,
            GetBackupStamp::<Impl, OFFSET>,
            GetPreviousBackupStamp::<Impl, OFFSET>,
            GetBackupOptions::<Impl, OFFSET>,
            GetRestoreOptions::<Impl, OFFSET>,
            GetRestoreSubcomponentCount::<Impl, OFFSET>,
            GetRestoreSubcomponent::<Impl, OFFSET>,
            GetFileRestoreStatus::<Impl, OFFSET>,
            AddDifferencedFilesByLastModifyTime::<Impl, OFFSET>,
            AddDifferencedFilesByLastModifyLSN::<Impl, OFFSET>,
            GetDifferencedFilesCount::<Impl, OFFSET>,
            GetDifferencedFile::<Impl, OFFSET>,
        )
    }
}
pub trait IVssComponentExImpl: Sized + IVssComponentImpl {
    fn SetPrepareForBackupFailureMsg();
    fn SetPostSnapshotFailureMsg();
    fn GetPrepareForBackupFailureMsg();
    fn GetPostSnapshotFailureMsg();
    fn GetAuthoritativeRestore();
    fn GetRollForward();
    fn GetRestoreName();
}
impl ::windows::core::RuntimeName for IVssComponentEx {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssComponentEx";
}
impl IVssComponentExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponentExImpl, const OFFSET: isize>() -> IVssComponentExVtbl {
        unsafe extern "system" fn SetPrepareForBackupFailureMsg<Impl: IVssComponentExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszfailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrepareForBackupFailureMsg(&*(&wszfailuremsg as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostSnapshotFailureMsg<Impl: IVssComponentExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszfailuremsg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPostSnapshotFailureMsg(&*(&wszfailuremsg as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrepareForBackupFailureMsg<Impl: IVssComponentExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrepareForBackupFailureMsg(::core::mem::transmute_copy(&pbstrfailuremsg)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPostSnapshotFailureMsg<Impl: IVssComponentExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPostSnapshotFailureMsg(::core::mem::transmute_copy(&pbstrfailuremsg)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthoritativeRestore<Impl: IVssComponentExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbauth: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAuthoritativeRestore(::core::mem::transmute_copy(&pbauth)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRollForward<Impl: IVssComponentExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRollForward(::core::mem::transmute_copy(&prolltype), ::core::mem::transmute_copy(&pbstrpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestoreName<Impl: IVssComponentExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRestoreName(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IVssComponentEx>,
            ::windows::core::GetTrustLevel,
            SetPrepareForBackupFailureMsg::<Impl, OFFSET>,
            SetPostSnapshotFailureMsg::<Impl, OFFSET>,
            GetPrepareForBackupFailureMsg::<Impl, OFFSET>,
            GetPostSnapshotFailureMsg::<Impl, OFFSET>,
            GetAuthoritativeRestore::<Impl, OFFSET>,
            GetRollForward::<Impl, OFFSET>,
            GetRestoreName::<Impl, OFFSET>,
        )
    }
}
pub trait IVssComponentEx2Impl: Sized + IVssComponentExImpl + IVssComponentImpl {
    fn SetFailure();
    fn GetFailure();
}
impl ::windows::core::RuntimeName for IVssComponentEx2 {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssComponentEx2";
}
impl IVssComponentEx2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssComponentEx2Impl, const OFFSET: isize>() -> IVssComponentEx2Vtbl {
        unsafe extern "system" fn SetFailure<Impl: IVssComponentEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, hrapplication: ::windows::core::HRESULT, wszapplicationmessage: super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFailure(hr, hrapplication, &*(&wszapplicationmessage as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFailure<Impl: IVssComponentEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phr: *mut ::windows::core::HRESULT, phrapplication: *mut ::windows::core::HRESULT, pbstrapplicationmessage: *mut super::super::Foundation::BSTR, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFailure(::core::mem::transmute_copy(&phr), ::core::mem::transmute_copy(&phrapplication), ::core::mem::transmute_copy(&pbstrapplicationmessage), ::core::mem::transmute_copy(&pdwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVssComponentEx2>, ::windows::core::GetTrustLevel, SetFailure::<Impl, OFFSET>, GetFailure::<Impl, OFFSET>)
    }
}
pub trait IVssCreateExpressWriterMetadataImpl: Sized {
    fn AddExcludeFiles();
    fn AddComponent();
    fn AddFilesToFileGroup();
    fn SetRestoreMethod();
    fn AddComponentDependency();
    fn SetBackupSchema();
    fn SaveAsXML();
}
impl ::windows::core::RuntimeName for IVssCreateExpressWriterMetadata {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssCreateExpressWriterMetadata";
}
impl IVssCreateExpressWriterMetadataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateExpressWriterMetadataImpl, const OFFSET: isize>() -> IVssCreateExpressWriterMetadataVtbl {
        unsafe extern "system" fn AddExcludeFiles<Impl: IVssCreateExpressWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddExcludeFiles(&*(&wszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&wszfilespec as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), brecursive) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddComponent<Impl: IVssCreateExpressWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ct: VSS_COMPONENT_TYPE, wszlogicalpath: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcaption: super::super::Foundation::PWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddComponent(
                ct,
                &*(&wszlogicalpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszcomponentname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszcaption as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                pbicon,
                cbicon,
                brestoremetadata,
                bnotifyonbackupcomplete,
                bselectable,
                bselectableforrestore,
                dwcomponentflags,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddFilesToFileGroup<Impl: IVssCreateExpressWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlogicalpath: super::super::Foundation::PWSTR, wszgroupname: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8, wszalternatelocation: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddFilesToFileGroup(
                &*(&wszlogicalpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszgroupname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszfilespec as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                brecursive,
                &*(&wszalternatelocation as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwbackuptypemask,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRestoreMethod<Impl: IVssCreateExpressWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: VSS_RESTOREMETHOD_ENUM, wszservice: super::super::Foundation::PWSTR, wszuserprocedure: super::super::Foundation::PWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRestoreMethod(method, &*(&wszservice as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&wszuserprocedure as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), writerrestore, brebootrequired) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddComponentDependency<Impl: IVssCreateExpressWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszforlogicalpath: super::super::Foundation::PWSTR, wszforcomponentname: super::super::Foundation::PWSTR, onwriterid: ::windows::core::GUID, wszonlogicalpath: super::super::Foundation::PWSTR, wszoncomponentname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddComponentDependency(
                &*(&wszforlogicalpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszforcomponentname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&onwriterid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&wszonlogicalpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszoncomponentname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackupSchema<Impl: IVssCreateExpressWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwschemamask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBackupSchema(dwschemamask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAsXML<Impl: IVssCreateExpressWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveAsXML(::core::mem::transmute_copy(&pbstrxml)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IVssCreateExpressWriterMetadata>,
            ::windows::core::GetTrustLevel,
            AddExcludeFiles::<Impl, OFFSET>,
            AddComponent::<Impl, OFFSET>,
            AddFilesToFileGroup::<Impl, OFFSET>,
            SetRestoreMethod::<Impl, OFFSET>,
            AddComponentDependency::<Impl, OFFSET>,
            SetBackupSchema::<Impl, OFFSET>,
            SaveAsXML::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IVssCreateWriterMetadata {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssCreateWriterMetadata";
}
impl IVssCreateWriterMetadataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>() -> IVssCreateWriterMetadataVtbl {
        unsafe extern "system" fn AddIncludeFiles<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8, wszalternatelocation: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddIncludeFiles(
                &*(&wszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszfilespec as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                brecursive,
                &*(&wszalternatelocation as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddExcludeFiles<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddExcludeFiles(&*(&wszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&wszfilespec as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), brecursive) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddComponent<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ct: VSS_COMPONENT_TYPE, wszlogicalpath: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcaption: super::super::Foundation::PWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddComponent(
                ct,
                &*(&wszlogicalpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszcomponentname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszcaption as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                pbicon,
                cbicon,
                brestoremetadata,
                bnotifyonbackupcomplete,
                bselectable,
                bselectableforrestore,
                dwcomponentflags,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDatabaseFiles<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlogicalpath: super::super::Foundation::PWSTR, wszdatabasename: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddDatabaseFiles(
                &*(&wszlogicalpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszdatabasename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszfilespec as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwbackuptypemask,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDatabaseLogFiles<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlogicalpath: super::super::Foundation::PWSTR, wszdatabasename: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddDatabaseLogFiles(
                &*(&wszlogicalpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszdatabasename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszfilespec as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwbackuptypemask,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddFilesToFileGroup<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlogicalpath: super::super::Foundation::PWSTR, wszgroupname: super::super::Foundation::PWSTR, wszpath: super::super::Foundation::PWSTR, wszfilespec: super::super::Foundation::PWSTR, brecursive: u8, wszalternatelocation: super::super::Foundation::PWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddFilesToFileGroup(
                &*(&wszlogicalpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszgroupname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszfilespec as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                brecursive,
                &*(&wszalternatelocation as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwbackuptypemask,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRestoreMethod<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: VSS_RESTOREMETHOD_ENUM, wszservice: super::super::Foundation::PWSTR, wszuserprocedure: super::super::Foundation::PWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRestoreMethod(method, &*(&wszservice as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&wszuserprocedure as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), writerrestore, brebootrequired) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAlternateLocationMapping<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszsourcepath: super::super::Foundation::PWSTR, wszsourcefilespec: super::super::Foundation::PWSTR, brecursive: u8, wszdestination: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddAlternateLocationMapping(
                &*(&wszsourcepath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszsourcefilespec as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                brecursive,
                &*(&wszdestination as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddComponentDependency<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszforlogicalpath: super::super::Foundation::PWSTR, wszforcomponentname: super::super::Foundation::PWSTR, onwriterid: ::windows::core::GUID, wszonlogicalpath: super::super::Foundation::PWSTR, wszoncomponentname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddComponentDependency(
                &*(&wszforlogicalpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszforcomponentname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&onwriterid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&wszonlogicalpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszoncomponentname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackupSchema<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwschemamask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBackupSchema(dwschemamask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocument<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdoc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocument(::core::mem::transmute_copy(&pdoc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAsXML<Impl: IVssCreateWriterMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveAsXML(&*(&pbstrxml as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IVssCreateWriterMetadata>,
            ::windows::core::GetTrustLevel,
            AddIncludeFiles::<Impl, OFFSET>,
            AddExcludeFiles::<Impl, OFFSET>,
            AddComponent::<Impl, OFFSET>,
            AddDatabaseFiles::<Impl, OFFSET>,
            AddDatabaseLogFiles::<Impl, OFFSET>,
            AddFilesToFileGroup::<Impl, OFFSET>,
            SetRestoreMethod::<Impl, OFFSET>,
            AddAlternateLocationMapping::<Impl, OFFSET>,
            AddComponentDependency::<Impl, OFFSET>,
            SetBackupSchema::<Impl, OFFSET>,
            GetDocument::<Impl, OFFSET>,
            SaveAsXML::<Impl, OFFSET>,
        )
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
impl ::windows::core::RuntimeName for IVssDifferentialSoftwareSnapshotMgmt {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssDifferentialSoftwareSnapshotMgmt";
}
impl IVssDifferentialSoftwareSnapshotMgmtVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmtImpl, const OFFSET: isize>() -> IVssDifferentialSoftwareSnapshotMgmtVtbl {
        unsafe extern "system" fn AddDiffArea<Impl: IVssDifferentialSoftwareSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddDiffArea(pwszvolumename, pwszdiffareavolumename, llmaximumdiffspace) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeDiffAreaMaximumSize<Impl: IVssDifferentialSoftwareSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeDiffAreaMaximumSize(pwszvolumename, pwszdiffareavolumename, llmaximumdiffspace) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryVolumesSupportedForDiffAreas<Impl: IVssDifferentialSoftwareSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszoriginalvolumename: *const u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryVolumesSupportedForDiffAreas(pwszoriginalvolumename, ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDiffAreasForVolume<Impl: IVssDifferentialSoftwareSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryDiffAreasForVolume(pwszvolumename, ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDiffAreasOnVolume<Impl: IVssDifferentialSoftwareSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryDiffAreasOnVolume(pwszvolumename, ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDiffAreasForSnapshot<Impl: IVssDifferentialSoftwareSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryDiffAreasForSnapshot(&*(&snapshotid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IVssDifferentialSoftwareSnapshotMgmt>,
            ::windows::core::GetTrustLevel,
            AddDiffArea::<Impl, OFFSET>,
            ChangeDiffAreaMaximumSize::<Impl, OFFSET>,
            QueryVolumesSupportedForDiffAreas::<Impl, OFFSET>,
            QueryDiffAreasForVolume::<Impl, OFFSET>,
            QueryDiffAreasOnVolume::<Impl, OFFSET>,
            QueryDiffAreasForSnapshot::<Impl, OFFSET>,
        )
    }
}
pub trait IVssDifferentialSoftwareSnapshotMgmt2Impl: Sized + IVssDifferentialSoftwareSnapshotMgmtImpl {
    fn ChangeDiffAreaMaximumSizeEx();
    fn MigrateDiffAreas();
    fn QueryMigrationStatus();
    fn SetSnapshotPriority();
}
impl ::windows::core::RuntimeName for IVssDifferentialSoftwareSnapshotMgmt2 {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssDifferentialSoftwareSnapshotMgmt2";
}
impl IVssDifferentialSoftwareSnapshotMgmt2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt2Impl, const OFFSET: isize>() -> IVssDifferentialSoftwareSnapshotMgmt2Vtbl {
        unsafe extern "system" fn ChangeDiffAreaMaximumSizeEx<Impl: IVssDifferentialSoftwareSnapshotMgmt2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeDiffAreaMaximumSizeEx(pwszvolumename, pwszdiffareavolumename, llmaximumdiffspace, &*(&bvolatile as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MigrateDiffAreas<Impl: IVssDifferentialSoftwareSnapshotMgmt2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MigrateDiffAreas(pwszvolumename, pwszdiffareavolumename, pwsznewdiffareavolumename) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryMigrationStatus<Impl: IVssDifferentialSoftwareSnapshotMgmt2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryMigrationStatus(pwszvolumename, pwszdiffareavolumename, ::core::mem::transmute_copy(&ppasync)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSnapshotPriority<Impl: IVssDifferentialSoftwareSnapshotMgmt2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idsnapshot: ::windows::core::GUID, priority: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSnapshotPriority(&*(&idsnapshot as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), priority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVssDifferentialSoftwareSnapshotMgmt2>, ::windows::core::GetTrustLevel, ChangeDiffAreaMaximumSizeEx::<Impl, OFFSET>, MigrateDiffAreas::<Impl, OFFSET>, QueryMigrationStatus::<Impl, OFFSET>, SetSnapshotPriority::<Impl, OFFSET>)
    }
}
pub trait IVssDifferentialSoftwareSnapshotMgmt3Impl: Sized + IVssDifferentialSoftwareSnapshotMgmt2Impl + IVssDifferentialSoftwareSnapshotMgmtImpl {
    fn SetVolumeProtectLevel();
    fn GetVolumeProtectLevel();
    fn ClearVolumeProtectFault();
    fn DeleteUnusedDiffAreas();
    fn QuerySnapshotDeltaBitmap();
}
impl ::windows::core::RuntimeName for IVssDifferentialSoftwareSnapshotMgmt3 {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssDifferentialSoftwareSnapshotMgmt3";
}
impl IVssDifferentialSoftwareSnapshotMgmt3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssDifferentialSoftwareSnapshotMgmt3Impl, const OFFSET: isize>() -> IVssDifferentialSoftwareSnapshotMgmt3Vtbl {
        unsafe extern "system" fn SetVolumeProtectLevel<Impl: IVssDifferentialSoftwareSnapshotMgmt3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, protectionlevel: VSS_PROTECTION_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVolumeProtectLevel(pwszvolumename, protectionlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVolumeProtectLevel<Impl: IVssDifferentialSoftwareSnapshotMgmt3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, protectionlevel: *mut VSS_VOLUME_PROTECTION_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVolumeProtectLevel(pwszvolumename, ::core::mem::transmute_copy(&protectionlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearVolumeProtectFault<Impl: IVssDifferentialSoftwareSnapshotMgmt3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearVolumeProtectFault(pwszvolumename) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteUnusedDiffAreas<Impl: IVssDifferentialSoftwareSnapshotMgmt3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdiffareavolumename: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteUnusedDiffAreas(pwszdiffareavolumename) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuerySnapshotDeltaBitmap<Impl: IVssDifferentialSoftwareSnapshotMgmt3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idsnapshotolder: ::windows::core::GUID, idsnapshotyounger: ::windows::core::GUID, pcblocksizeperbit: *mut u32, pcbitmaplength: *mut u32, ppbbitmap: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuerySnapshotDeltaBitmap(&*(&idsnapshotolder as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&idsnapshotyounger as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcblocksizeperbit), ::core::mem::transmute_copy(&pcbitmaplength), ::core::mem::transmute_copy(&ppbbitmap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVssDifferentialSoftwareSnapshotMgmt3>, ::windows::core::GetTrustLevel, SetVolumeProtectLevel::<Impl, OFFSET>, GetVolumeProtectLevel::<Impl, OFFSET>, ClearVolumeProtectFault::<Impl, OFFSET>, DeleteUnusedDiffAreas::<Impl, OFFSET>, QuerySnapshotDeltaBitmap::<Impl, OFFSET>)
    }
}
pub trait IVssEnumMgmtObjectImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IVssEnumMgmtObject {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssEnumMgmtObject";
}
impl IVssEnumMgmtObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssEnumMgmtObjectImpl, const OFFSET: isize>() -> IVssEnumMgmtObjectVtbl {
        unsafe extern "system" fn Next<Impl: IVssEnumMgmtObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut VSS_MGMT_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IVssEnumMgmtObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IVssEnumMgmtObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IVssEnumMgmtObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(&*(&ppenum as *const <IVssEnumMgmtObject as ::windows::core::Abi>::Abi as *const <IVssEnumMgmtObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVssEnumMgmtObject>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IVssEnumObjectImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IVssEnumObject {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssEnumObject";
}
impl IVssEnumObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssEnumObjectImpl, const OFFSET: isize>() -> IVssEnumObjectVtbl {
        unsafe extern "system" fn Next<Impl: IVssEnumObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut VSS_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IVssEnumObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IVssEnumObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IVssEnumObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(&*(&ppenum as *const <IVssEnumObject as ::windows::core::Abi>::Abi as *const <IVssEnumObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVssEnumObject>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IVssExpressWriterImpl: Sized {
    fn CreateMetadata();
    fn LoadMetadata();
    fn Register();
    fn Unregister();
}
impl ::windows::core::RuntimeName for IVssExpressWriter {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssExpressWriter";
}
impl IVssExpressWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssExpressWriterImpl, const OFFSET: isize>() -> IVssExpressWriterVtbl {
        unsafe extern "system" fn CreateMetadata<Impl: IVssExpressWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writerid: ::windows::core::GUID, writername: super::super::Foundation::PWSTR, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32, ppmetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMetadata(&*(&writerid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&writername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), usagetype, versionmajor, versionminor, reserved, ::core::mem::transmute_copy(&ppmetadata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadMetadata<Impl: IVssExpressWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadata: super::super::Foundation::PWSTR, reserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadMetadata(&*(&metadata as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), reserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Register<Impl: IVssExpressWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Register() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unregister<Impl: IVssExpressWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writerid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unregister(&*(&writerid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVssExpressWriter>, ::windows::core::GetTrustLevel, CreateMetadata::<Impl, OFFSET>, LoadMetadata::<Impl, OFFSET>, Register::<Impl, OFFSET>, Unregister::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IVssFileShareSnapshotProvider {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssFileShareSnapshotProvider";
}
impl IVssFileShareSnapshotProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>() -> IVssFileShareSnapshotProviderVtbl {
        unsafe extern "system" fn SetContext<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcontext: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetContext(lcontext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSnapshotProperties<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSnapshotProperties(&*(&snapshotid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Query<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queriedobjectid: ::windows::core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Query(&*(&queriedobjectid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), equeriedobjecttype, ereturnedobjectstype, ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteSnapshots<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceobjectid: ::windows::core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteSnapshots(&*(&sourceobjectid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), esourceobjecttype, &*(&bforcedelete as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pldeletedsnapshots), ::core::mem::transmute_copy(&pnondeletedsnapshotid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, pwszsharepath: *const u16, lnewcontext: i32, providerid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginPrepareSnapshot(
                &*(&snapshotsetid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&snapshotid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                pwszsharepath,
                lnewcontext,
                &*(&providerid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPathSupported<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsharepath: *const u16, pbsupportedbythisprovider: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPathSupported(pwszsharepath, ::core::mem::transmute_copy(&pbsupportedbythisprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPathSnapshotted<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsharepath: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPathSnapshotted(pwszsharepath, ::core::mem::transmute_copy(&pbsnapshotspresent), ::core::mem::transmute_copy(&plsnapshotcompatibility)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSnapshotProperty<Impl: IVssFileShareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSnapshotProperty(&*(&snapshotid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), esnapshotpropertyid, &*(&vproperty as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IVssFileShareSnapshotProvider>,
            ::windows::core::GetTrustLevel,
            SetContext::<Impl, OFFSET>,
            GetSnapshotProperties::<Impl, OFFSET>,
            Query::<Impl, OFFSET>,
            DeleteSnapshots::<Impl, OFFSET>,
            BeginPrepareSnapshot::<Impl, OFFSET>,
            IsPathSupported::<Impl, OFFSET>,
            IsPathSnapshotted::<Impl, OFFSET>,
            SetSnapshotProperty::<Impl, OFFSET>,
        )
    }
}
pub trait IVssHardwareSnapshotProviderImpl: Sized {
    fn AreLunsSupported();
    fn FillInLunInfo();
    fn BeginPrepareSnapshot();
    fn GetTargetLuns();
    fn LocateLuns();
    fn OnLunEmpty();
}
impl ::windows::core::RuntimeName for IVssHardwareSnapshotProvider {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssHardwareSnapshotProvider";
}
impl IVssHardwareSnapshotProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssHardwareSnapshotProviderImpl, const OFFSET: isize>() -> IVssHardwareSnapshotProviderVtbl {
        unsafe extern "system" fn AreLunsSupported<Impl: IVssHardwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreLunsSupported(lluncount, lcontext, rgwszdevices, &*(&pluninformation as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::Abi>::Abi as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbissupported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillInLunInfo<Impl: IVssHardwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillInLunInfo(wszdevicename, &*(&pluninfo as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::Abi>::Abi as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbissupported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Impl: IVssHardwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginPrepareSnapshot(
                &*(&snapshotsetid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&snapshotid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                lcontext,
                lluncount,
                rgdevicenames,
                &*(&rgluninformation as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::Abi>::Abi as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetLuns<Impl: IVssHardwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTargetLuns(lluncount, rgdevicenames, &*(&rgsourceluns as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::Abi>::Abi as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::DefaultType>::DefaultType), &*(&rgdestinationluns as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::Abi>::Abi as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocateLuns<Impl: IVssHardwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lluncount: i32, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocateLuns(lluncount, &*(&rgsourceluns as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::Abi>::Abi as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLunEmpty<Impl: IVssHardwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLunEmpty(wszdevicename, &*(&pinformation as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::Abi>::Abi as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVssHardwareSnapshotProvider>, ::windows::core::GetTrustLevel, AreLunsSupported::<Impl, OFFSET>, FillInLunInfo::<Impl, OFFSET>, BeginPrepareSnapshot::<Impl, OFFSET>, GetTargetLuns::<Impl, OFFSET>, LocateLuns::<Impl, OFFSET>, OnLunEmpty::<Impl, OFFSET>)
    }
}
pub trait IVssHardwareSnapshotProviderExImpl: Sized + IVssHardwareSnapshotProviderImpl {
    fn GetProviderCapabilities();
    fn OnLunStateChange();
    fn ResyncLuns();
    fn OnReuseLuns();
}
impl ::windows::core::RuntimeName for IVssHardwareSnapshotProviderEx {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssHardwareSnapshotProviderEx";
}
impl IVssHardwareSnapshotProviderExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssHardwareSnapshotProviderExImpl, const OFFSET: isize>() -> IVssHardwareSnapshotProviderExVtbl {
        unsafe extern "system" fn GetProviderCapabilities<Impl: IVssHardwareSnapshotProviderExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plloriginalcapabilitymask: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderCapabilities(::core::mem::transmute_copy(&plloriginalcapabilitymask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLunStateChange<Impl: IVssHardwareSnapshotProviderExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLunStateChange(&*(&psnapshotluns as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::Abi>::Abi as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::DefaultType>::DefaultType), &*(&poriginalluns as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::Abi>::Abi as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::DefaultType>::DefaultType), dwcount, dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResyncLuns<Impl: IVssHardwareSnapshotProviderExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResyncLuns(&*(&psourceluns as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::Abi>::Abi as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::DefaultType>::DefaultType), &*(&ptargetluns as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::Abi>::Abi as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::DefaultType>::DefaultType), dwcount, ::core::mem::transmute_copy(&ppasync)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnReuseLuns<Impl: IVssHardwareSnapshotProviderExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnReuseLuns(&*(&psnapshotluns as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::Abi>::Abi as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::DefaultType>::DefaultType), &*(&poriginalluns as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::Abi>::Abi as *const <super::VirtualDiskService::VDS_LUN_INFORMATION as ::windows::core::DefaultType>::DefaultType), dwcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVssHardwareSnapshotProviderEx>, ::windows::core::GetTrustLevel, GetProviderCapabilities::<Impl, OFFSET>, OnLunStateChange::<Impl, OFFSET>, ResyncLuns::<Impl, OFFSET>, OnReuseLuns::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IVssProviderCreateSnapshotSet {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssProviderCreateSnapshotSet";
}
impl IVssProviderCreateSnapshotSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssProviderCreateSnapshotSetImpl, const OFFSET: isize>() -> IVssProviderCreateSnapshotSetVtbl {
        unsafe extern "system" fn EndPrepareSnapshots<Impl: IVssProviderCreateSnapshotSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndPrepareSnapshots(&*(&snapshotsetid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreCommitSnapshots<Impl: IVssProviderCreateSnapshotSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreCommitSnapshots(&*(&snapshotsetid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitSnapshots<Impl: IVssProviderCreateSnapshotSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitSnapshots(&*(&snapshotsetid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostCommitSnapshots<Impl: IVssProviderCreateSnapshotSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID, lsnapshotscount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostCommitSnapshots(&*(&snapshotsetid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), lsnapshotscount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreFinalCommitSnapshots<Impl: IVssProviderCreateSnapshotSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreFinalCommitSnapshots(&*(&snapshotsetid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostFinalCommitSnapshots<Impl: IVssProviderCreateSnapshotSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostFinalCommitSnapshots(&*(&snapshotsetid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbortSnapshots<Impl: IVssProviderCreateSnapshotSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AbortSnapshots(&*(&snapshotsetid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IVssProviderCreateSnapshotSet>,
            ::windows::core::GetTrustLevel,
            EndPrepareSnapshots::<Impl, OFFSET>,
            PreCommitSnapshots::<Impl, OFFSET>,
            CommitSnapshots::<Impl, OFFSET>,
            PostCommitSnapshots::<Impl, OFFSET>,
            PreFinalCommitSnapshots::<Impl, OFFSET>,
            PostFinalCommitSnapshots::<Impl, OFFSET>,
            AbortSnapshots::<Impl, OFFSET>,
        )
    }
}
pub trait IVssProviderNotificationsImpl: Sized {
    fn OnLoad();
    fn OnUnload();
}
impl ::windows::core::RuntimeName for IVssProviderNotifications {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssProviderNotifications";
}
impl IVssProviderNotificationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssProviderNotificationsImpl, const OFFSET: isize>() -> IVssProviderNotificationsVtbl {
        unsafe extern "system" fn OnLoad<Impl: IVssProviderNotificationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLoad(&*(&pcallback as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnUnload<Impl: IVssProviderNotificationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bforceunload: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnUnload(&*(&bforceunload as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVssProviderNotifications>, ::windows::core::GetTrustLevel, OnLoad::<Impl, OFFSET>, OnUnload::<Impl, OFFSET>)
    }
}
pub trait IVssSnapshotMgmtImpl: Sized {
    fn GetProviderMgmtInterface();
    fn QueryVolumesSupportedForSnapshots();
    fn QuerySnapshotsByVolume();
}
impl ::windows::core::RuntimeName for IVssSnapshotMgmt {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssSnapshotMgmt";
}
impl IVssSnapshotMgmtVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssSnapshotMgmtImpl, const OFFSET: isize>() -> IVssSnapshotMgmtVtbl {
        unsafe extern "system" fn GetProviderMgmtInterface<Impl: IVssSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, interfaceid: *const ::windows::core::GUID, ppitf: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderMgmtInterface(&*(&providerid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&interfaceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppitf)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryVolumesSupportedForSnapshots<Impl: IVssSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, lcontext: i32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryVolumesSupportedForSnapshots(&*(&providerid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), lcontext, ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuerySnapshotsByVolume<Impl: IVssSnapshotMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, providerid: ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuerySnapshotsByVolume(pwszvolumename, &*(&providerid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVssSnapshotMgmt>, ::windows::core::GetTrustLevel, GetProviderMgmtInterface::<Impl, OFFSET>, QueryVolumesSupportedForSnapshots::<Impl, OFFSET>, QuerySnapshotsByVolume::<Impl, OFFSET>)
    }
}
pub trait IVssSnapshotMgmt2Impl: Sized {
    fn GetMinDiffAreaSize();
}
impl ::windows::core::RuntimeName for IVssSnapshotMgmt2 {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssSnapshotMgmt2";
}
impl IVssSnapshotMgmt2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssSnapshotMgmt2Impl, const OFFSET: isize>() -> IVssSnapshotMgmt2Vtbl {
        unsafe extern "system" fn GetMinDiffAreaSize<Impl: IVssSnapshotMgmt2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllmindiffareasize: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMinDiffAreaSize(::core::mem::transmute_copy(&pllmindiffareasize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVssSnapshotMgmt2>, ::windows::core::GetTrustLevel, GetMinDiffAreaSize::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IVssSoftwareSnapshotProvider {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssSoftwareSnapshotProvider";
}
impl IVssSoftwareSnapshotProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>() -> IVssSoftwareSnapshotProviderVtbl {
        unsafe extern "system" fn SetContext<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcontext: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetContext(lcontext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSnapshotProperties<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSnapshotProperties(&*(&snapshotid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Query<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queriedobjectid: ::windows::core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Query(&*(&queriedobjectid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), equeriedobjecttype, ereturnedobjectstype, ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteSnapshots<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceobjectid: ::windows::core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteSnapshots(&*(&sourceobjectid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), esourceobjecttype, &*(&bforcedelete as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pldeletedsnapshots), ::core::mem::transmute_copy(&pnondeletedsnapshotid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, pwszvolumename: *const u16, lnewcontext: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginPrepareSnapshot(&*(&snapshotsetid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&snapshotid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), pwszvolumename, lnewcontext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVolumeSupported<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pbsupportedbythisprovider: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVolumeSupported(pwszvolumename, ::core::mem::transmute_copy(&pbsupportedbythisprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVolumeSnapshotted<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVolumeSnapshotted(pwszvolumename, ::core::mem::transmute_copy(&pbsnapshotspresent), ::core::mem::transmute_copy(&plsnapshotcompatibility)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSnapshotProperty<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSnapshotProperty(&*(&snapshotid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), esnapshotpropertyid, &*(&vproperty as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevertToSnapshot<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevertToSnapshot(&*(&snapshotid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryRevertStatus<Impl: IVssSoftwareSnapshotProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszvolume: *const u16, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryRevertStatus(pwszvolume, ::core::mem::transmute_copy(&ppasync)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IVssSoftwareSnapshotProvider>,
            ::windows::core::GetTrustLevel,
            SetContext::<Impl, OFFSET>,
            GetSnapshotProperties::<Impl, OFFSET>,
            Query::<Impl, OFFSET>,
            DeleteSnapshots::<Impl, OFFSET>,
            BeginPrepareSnapshot::<Impl, OFFSET>,
            IsVolumeSupported::<Impl, OFFSET>,
            IsVolumeSnapshotted::<Impl, OFFSET>,
            SetSnapshotProperty::<Impl, OFFSET>,
            RevertToSnapshot::<Impl, OFFSET>,
            QueryRevertStatus::<Impl, OFFSET>,
        )
    }
}
pub trait IVssWMDependencyImpl: Sized {
    fn GetWriterId();
    fn GetLogicalPath();
    fn GetComponentName();
}
impl ::windows::core::RuntimeName for IVssWMDependency {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssWMDependency";
}
impl IVssWMDependencyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssWMDependencyImpl, const OFFSET: isize>() -> IVssWMDependencyVtbl {
        unsafe extern "system" fn GetWriterId<Impl: IVssWMDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwriterid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWriterId(&*(&pwriterid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLogicalPath<Impl: IVssWMDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlogicalpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLogicalPath(&*(&pbstrlogicalpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetComponentName<Impl: IVssWMDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcomponentname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetComponentName(&*(&pbstrcomponentname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVssWMDependency>, ::windows::core::GetTrustLevel, GetWriterId::<Impl, OFFSET>, GetLogicalPath::<Impl, OFFSET>, GetComponentName::<Impl, OFFSET>)
    }
}
pub trait IVssWMFiledescImpl: Sized {
    fn GetPath();
    fn GetFilespec();
    fn GetRecursive();
    fn GetAlternateLocation();
    fn GetBackupTypeMask();
}
impl ::windows::core::RuntimeName for IVssWMFiledesc {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssWMFiledesc";
}
impl IVssWMFiledescVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssWMFiledescImpl, const OFFSET: isize>() -> IVssWMFiledescVtbl {
        unsafe extern "system" fn GetPath<Impl: IVssWMFiledescImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPath(::core::mem::transmute_copy(&pbstrpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilespec<Impl: IVssWMFiledescImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfilespec: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilespec(::core::mem::transmute_copy(&pbstrfilespec)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecursive<Impl: IVssWMFiledescImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbrecursive: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecursive(::core::mem::transmute_copy(&pbrecursive)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAlternateLocation<Impl: IVssWMFiledescImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstralternatelocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAlternateLocation(::core::mem::transmute_copy(&pbstralternatelocation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackupTypeMask<Impl: IVssWMFiledescImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtypemask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackupTypeMask(::core::mem::transmute_copy(&pdwtypemask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVssWMFiledesc>, ::windows::core::GetTrustLevel, GetPath::<Impl, OFFSET>, GetFilespec::<Impl, OFFSET>, GetRecursive::<Impl, OFFSET>, GetAlternateLocation::<Impl, OFFSET>, GetBackupTypeMask::<Impl, OFFSET>)
    }
}
pub trait IVssWriterComponentsImpl: Sized {
    fn GetComponentCount();
    fn GetWriterInfo();
    fn GetComponent();
}
impl ::windows::core::RuntimeName for IVssWriterComponents {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssWriterComponents";
}
impl IVssWriterComponentsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterComponentsImpl, const OFFSET: isize>() -> IVssWriterComponentsVtbl {
        unsafe extern "system" fn GetComponentCount<Impl: IVssWriterComponentsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccomponents: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetComponentCount(pccomponents) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWriterInfo<Impl: IVssWriterComponentsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidinstance: *mut ::windows::core::GUID, pidwriter: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWriterInfo(&*(&pidinstance as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pidwriter as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetComponent<Impl: IVssWriterComponentsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icomponent: u32, ppcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetComponent(icomponent, ::core::mem::transmute_copy(&ppcomponent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVssWriterComponents>, ::windows::core::GetTrustLevel, GetComponentCount::<Impl, OFFSET>, GetWriterInfo::<Impl, OFFSET>, GetComponent::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IVssWriterImpl {
    const NAME: &'static str = "Windows.Win32.Storage.Vss.IVssWriterImpl";
}
impl IVssWriterImplVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVssWriterImplImpl, const OFFSET: isize>() -> IVssWriterImplVtbl {
        unsafe extern "system" fn Initialize<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writerid: ::windows::core::GUID, wszwritername: super::super::Foundation::PWSTR, wszwriterinstancename: super::super::Foundation::PWSTR, dwmajorversion: u32, dwminorversion: u32, ut: VSS_USAGE_TYPE, st: VSS_SOURCE_TYPE, nlevel: VSS_APPLICATION_LEVEL, dwtimeout: u32, aws: VSS_ALTERNATE_WRITER_STATE, biothrottlingonly: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(
                &*(&writerid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&wszwritername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszwriterinstancename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwmajorversion,
                dwminorversion,
                ut,
                st,
                nlevel,
                dwtimeout,
                aws,
                biothrottlingonly,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subscribe<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsubscribetimeout: u32, dweventflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subscribe(dwsubscribetimeout, dweventflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unsubscribe<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unsubscribe() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uninitialize<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Uninitialize().into()
        }
        unsafe extern "system" fn GetCurrentVolumeArray<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut super::super::Foundation::PWSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentVolumeArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentVolumeCount<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentVolumeCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSnapshotDeviceName<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszoriginalvolume: super::super::Foundation::PWSTR, ppwszsnapshotdevice: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSnapshotDeviceName(&*(&wszoriginalvolume as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppwszsnapshotdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentSnapshotSetId<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentSnapshotSetId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentLevel<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> VSS_APPLICATION_LEVEL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPathAffected<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpath: super::super::Foundation::PWSTR) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPathAffected(&*(&wszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBootableSystemStateBackedUp<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBootableSystemStateBackedUp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreComponentsSelected<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreComponentsSelected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackupType<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> VSS_BACKUP_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackupType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestoreType<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> VSS_RESTORE_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRestoreType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWriterFailure<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWriterFailure(hr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPartialFileSupportEnabled<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPartialFileSupportEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallAlternateWriter<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idwriter: ::windows::core::GUID, clsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallAlternateWriter(&*(&idwriter as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&clsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIdentityInformation<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut IVssExamineWriterMetadata {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIdentityInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWriterFailureEx<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, hrapplication: ::windows::core::HRESULT, wszapplicationmessage: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWriterFailureEx(hr, hrapplication, &*(&wszapplicationmessage as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSessionId<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idsession: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSessionId(::core::mem::transmute_copy(&idsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWriterShuttingDown<Impl: IVssWriterImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWriterShuttingDown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IVssWriterImpl>,
            ::windows::core::GetTrustLevel,
            Initialize::<Impl, OFFSET>,
            Subscribe::<Impl, OFFSET>,
            Unsubscribe::<Impl, OFFSET>,
            Uninitialize::<Impl, OFFSET>,
            GetCurrentVolumeArray::<Impl, OFFSET>,
            GetCurrentVolumeCount::<Impl, OFFSET>,
            GetSnapshotDeviceName::<Impl, OFFSET>,
            GetCurrentSnapshotSetId::<Impl, OFFSET>,
            GetContext::<Impl, OFFSET>,
            GetCurrentLevel::<Impl, OFFSET>,
            IsPathAffected::<Impl, OFFSET>,
            IsBootableSystemStateBackedUp::<Impl, OFFSET>,
            AreComponentsSelected::<Impl, OFFSET>,
            GetBackupType::<Impl, OFFSET>,
            GetRestoreType::<Impl, OFFSET>,
            SetWriterFailure::<Impl, OFFSET>,
            IsPartialFileSupportEnabled::<Impl, OFFSET>,
            InstallAlternateWriter::<Impl, OFFSET>,
            GetIdentityInformation::<Impl, OFFSET>,
            SetWriterFailureEx::<Impl, OFFSET>,
            GetSessionId::<Impl, OFFSET>,
            IsWriterShuttingDown::<Impl, OFFSET>,
        )
    }
}
