#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IGPEInformationImpl: Sized {
    fn GetName();
    fn GetDisplayName();
    fn GetRegistryKey();
    fn GetDSPath();
    fn GetFileSysPath();
    fn GetOptions();
    fn GetType();
    fn GetHint();
    fn PolicyChanged();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl IGPEInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPEInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPEInformationVtbl {
        unsafe extern "system" fn GetName<Impl: IGPEInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayName<Impl: IGPEInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRegistryKey<Impl: IGPEInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDSPath<Impl: IGPEInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileSysPath<Impl: IGPEInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOptions<Impl: IGPEInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetType<Impl: IGPEInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHint<Impl: IGPEInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gphint: *mut GROUP_POLICY_HINT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PolicyChanged<Impl: IGPEInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows::core::GUID, pguidsnapin: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetName::<Impl, IMPL_OFFSET>, GetDisplayName::<Impl, IMPL_OFFSET>, GetRegistryKey::<Impl, IMPL_OFFSET>, GetDSPath::<Impl, IMPL_OFFSET>, GetFileSysPath::<Impl, IMPL_OFFSET>, GetOptions::<Impl, IMPL_OFFSET>, GetType::<Impl, IMPL_OFFSET>, GetHint::<Impl, IMPL_OFFSET>, PolicyChanged::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPEInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMImpl: Sized + IDispatchImpl {
    fn GetDomain();
    fn GetBackupDir();
    fn GetSitesContainer();
    fn GetRSOP();
    fn CreatePermission();
    fn CreateSearchCriteria();
    fn CreateTrustee();
    fn GetClientSideExtensions();
    fn GetConstants();
    fn GetMigrationTable();
    fn CreateMigrationTable();
    fn InitializeReporting();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMVtbl {
        unsafe extern "system" fn GetDomain<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomaincontroller: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ldcflags: i32, pigpmdomain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBackupDir<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pigpmbackupdir: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSitesContainer<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrforest: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomaincontroller: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ldcflags: i32, ppigpmsitescontainer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRSOP<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmrsopmode: GPMRSOPMode, bstrnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, ppigpmrsop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePermission<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, perm: GPMPermissionType, binheritable: i16, ppperm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSearchCriteria<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmsearchcriteria: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTrustee<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmtrustee: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClientSideExtensions<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmcsecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConstants<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmconstants: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMigrationTable<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmigrationtablepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmigrationtable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateMigrationTable<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmigrationtable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeReporting<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetDomain::<Impl, IMPL_OFFSET>,
            GetBackupDir::<Impl, IMPL_OFFSET>,
            GetSitesContainer::<Impl, IMPL_OFFSET>,
            GetRSOP::<Impl, IMPL_OFFSET>,
            CreatePermission::<Impl, IMPL_OFFSET>,
            CreateSearchCriteria::<Impl, IMPL_OFFSET>,
            CreateTrustee::<Impl, IMPL_OFFSET>,
            GetClientSideExtensions::<Impl, IMPL_OFFSET>,
            GetConstants::<Impl, IMPL_OFFSET>,
            GetMigrationTable::<Impl, IMPL_OFFSET>,
            CreateMigrationTable::<Impl, IMPL_OFFSET>,
            InitializeReporting::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPM as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPM2Impl: Sized + IGPMImpl + IDispatchImpl {
    fn GetBackupDirEx();
    fn InitializeReportingEx();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPM2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPM2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPM2Vtbl {
        unsafe extern "system" fn GetBackupDirEx<Impl: IGPM2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, backupdirtype: GPMBackupType, ppigpmbackupdirex: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeReportingEx<Impl: IGPM2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, reportingoptions: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetDomain::<Impl, IMPL_OFFSET>,
            GetBackupDir::<Impl, IMPL_OFFSET>,
            GetSitesContainer::<Impl, IMPL_OFFSET>,
            GetRSOP::<Impl, IMPL_OFFSET>,
            CreatePermission::<Impl, IMPL_OFFSET>,
            CreateSearchCriteria::<Impl, IMPL_OFFSET>,
            CreateTrustee::<Impl, IMPL_OFFSET>,
            GetClientSideExtensions::<Impl, IMPL_OFFSET>,
            GetConstants::<Impl, IMPL_OFFSET>,
            GetMigrationTable::<Impl, IMPL_OFFSET>,
            CreateMigrationTable::<Impl, IMPL_OFFSET>,
            InitializeReporting::<Impl, IMPL_OFFSET>,
            GetBackupDirEx::<Impl, IMPL_OFFSET>,
            InitializeReportingEx::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPM2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMAsyncCancelImpl: Sized + IDispatchImpl {
    fn Cancel();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMAsyncCancelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMAsyncCancelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMAsyncCancelVtbl {
        unsafe extern "system" fn Cancel<Impl: IGPMAsyncCancelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Cancel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMAsyncCancel as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMAsyncProgressImpl: Sized + IDispatchImpl {
    fn Status();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMAsyncProgressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMAsyncProgressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMAsyncProgressVtbl {
        unsafe extern "system" fn Status<Impl: IGPMAsyncProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprogressnumerator: i32, lprogressdenominator: i32, hrstatus: ::windows::core::HRESULT, presult: *const super::Com::VARIANT, ppigpmstatusmsgcollection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Status::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMAsyncProgress as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMBackupImpl: Sized + IDispatchImpl {
    fn ID();
    fn GPOID();
    fn GPODomain();
    fn GPODisplayName();
    fn Timestamp();
    fn Comment();
    fn BackupDir();
    fn Delete();
    fn GenerateReport();
    fn GenerateReportToFile();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMBackupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMBackupVtbl {
        unsafe extern "system" fn ID<Impl: IGPMBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GPOID<Impl: IGPMBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GPODomain<Impl: IGPMBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GPODisplayName<Impl: IGPMBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Timestamp<Impl: IGPMBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Comment<Impl: IGPMBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackupDir<Impl: IGPMBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IGPMBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateReport<Impl: IGPMBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateReportToFile<Impl: IGPMBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            ID::<Impl, IMPL_OFFSET>,
            GPOID::<Impl, IMPL_OFFSET>,
            GPODomain::<Impl, IMPL_OFFSET>,
            GPODisplayName::<Impl, IMPL_OFFSET>,
            Timestamp::<Impl, IMPL_OFFSET>,
            Comment::<Impl, IMPL_OFFSET>,
            BackupDir::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            GenerateReport::<Impl, IMPL_OFFSET>,
            GenerateReportToFile::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMBackup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMBackupCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMBackupCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackupCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMBackupCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IGPMBackupCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IGPMBackupCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IGPMBackupCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmbackup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMBackupCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMBackupDirImpl: Sized + IDispatchImpl {
    fn BackupDirectory();
    fn GetBackup();
    fn SearchBackups();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMBackupDirVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackupDirImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMBackupDirVtbl {
        unsafe extern "system" fn BackupDirectory<Impl: IGPMBackupDirImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBackup<Impl: IGPMBackupDirImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppbackup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchBackups<Impl: IGPMBackupDirImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmbackupcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, BackupDirectory::<Impl, IMPL_OFFSET>, GetBackup::<Impl, IMPL_OFFSET>, SearchBackups::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMBackupDir as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMBackupDirExImpl: Sized + IDispatchImpl {
    fn BackupDir();
    fn BackupType();
    fn GetBackup();
    fn SearchBackups();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMBackupDirExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackupDirExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMBackupDirExVtbl {
        unsafe extern "system" fn BackupDir<Impl: IGPMBackupDirExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbackupdir: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackupType<Impl: IGPMBackupDirExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgpmbackuptype: *mut GPMBackupType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBackup<Impl: IGPMBackupDirExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarbackup: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchBackups<Impl: IGPMBackupDirExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, pvarbackupcollection: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, BackupDir::<Impl, IMPL_OFFSET>, BackupType::<Impl, IMPL_OFFSET>, GetBackup::<Impl, IMPL_OFFSET>, SearchBackups::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMBackupDirEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMCSECollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMCSECollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMCSECollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMCSECollectionVtbl {
        unsafe extern "system" fn Count<Impl: IGPMCSECollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IGPMCSECollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IGPMCSECollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmcses: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMCSECollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMClientSideExtensionImpl: Sized + IDispatchImpl {
    fn ID();
    fn DisplayName();
    fn IsUserEnabled();
    fn IsComputerEnabled();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMClientSideExtensionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMClientSideExtensionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMClientSideExtensionVtbl {
        unsafe extern "system" fn ID<Impl: IGPMClientSideExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplayName<Impl: IGPMClientSideExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsUserEnabled<Impl: IGPMClientSideExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsComputerEnabled<Impl: IGPMClientSideExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, ID::<Impl, IMPL_OFFSET>, DisplayName::<Impl, IMPL_OFFSET>, IsUserEnabled::<Impl, IMPL_OFFSET>, IsComputerEnabled::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMClientSideExtension as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMConstantsImpl: Sized + IDispatchImpl {
    fn PermGPOApply();
    fn PermGPORead();
    fn PermGPOEdit();
    fn PermGPOEditSecurityAndDelete();
    fn PermGPOCustom();
    fn PermWMIFilterEdit();
    fn PermWMIFilterFullControl();
    fn PermWMIFilterCustom();
    fn PermSOMLink();
    fn PermSOMLogging();
    fn PermSOMPlanning();
    fn PermSOMGPOCreate();
    fn PermSOMWMICreate();
    fn PermSOMWMIFullControl();
    fn SearchPropertyGPOPermissions();
    fn SearchPropertyGPOEffectivePermissions();
    fn SearchPropertyGPODisplayName();
    fn SearchPropertyGPOWMIFilter();
    fn SearchPropertyGPOID();
    fn SearchPropertyGPOComputerExtensions();
    fn SearchPropertyGPOUserExtensions();
    fn SearchPropertySOMLinks();
    fn SearchPropertyGPODomain();
    fn SearchPropertyBackupMostRecent();
    fn SearchOpEquals();
    fn SearchOpContains();
    fn SearchOpNotContains();
    fn SearchOpNotEquals();
    fn UsePDC();
    fn UseAnyDC();
    fn DoNotUseW2KDC();
    fn SOMSite();
    fn SOMDomain();
    fn SOMOU();
    fn SecurityFlags();
    fn DoNotValidateDC();
    fn ReportHTML();
    fn ReportXML();
    fn RSOPModeUnknown();
    fn RSOPModePlanning();
    fn RSOPModeLogging();
    fn EntryTypeUser();
    fn EntryTypeComputer();
    fn EntryTypeLocalGroup();
    fn EntryTypeGlobalGroup();
    fn EntryTypeUniversalGroup();
    fn EntryTypeUNCPath();
    fn EntryTypeUnknown();
    fn DestinationOptionSameAsSource();
    fn DestinationOptionNone();
    fn DestinationOptionByRelativeName();
    fn DestinationOptionSet();
    fn MigrationTableOnly();
    fn ProcessSecurity();
    fn RsopLoggingNoComputer();
    fn RsopLoggingNoUser();
    fn RsopPlanningAssumeSlowLink();
    fn RsopPlanningLoopbackOption();
    fn RsopPlanningAssumeUserWQLFilterTrue();
    fn RsopPlanningAssumeCompWQLFilterTrue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMConstantsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstantsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMConstantsVtbl {
        unsafe extern "system" fn PermGPOApply<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PermGPORead<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PermGPOEdit<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PermGPOEditSecurityAndDelete<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PermGPOCustom<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PermWMIFilterEdit<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PermWMIFilterFullControl<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PermWMIFilterCustom<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PermSOMLink<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PermSOMLogging<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PermSOMPlanning<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PermSOMGPOCreate<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PermSOMWMICreate<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PermSOMWMIFullControl<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchPropertyGPOPermissions<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchPropertyGPOEffectivePermissions<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchPropertyGPODisplayName<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchPropertyGPOWMIFilter<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchPropertyGPOID<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchPropertyGPOComputerExtensions<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchPropertyGPOUserExtensions<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchPropertySOMLinks<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchPropertyGPODomain<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchPropertyBackupMostRecent<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchOpEquals<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchOpContains<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchOpNotContains<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchOpNotEquals<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UsePDC<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UseAnyDC<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoNotUseW2KDC<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SOMSite<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SOMDomain<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SOMOU<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SecurityFlags<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbowner: i16, vbgroup: i16, vbdacl: i16, vbsacl: i16, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoNotValidateDC<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReportHTML<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMReportType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReportXML<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMReportType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RSOPModeUnknown<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RSOPModePlanning<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RSOPModeLogging<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EntryTypeUser<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EntryTypeComputer<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EntryTypeLocalGroup<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EntryTypeGlobalGroup<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EntryTypeUniversalGroup<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EntryTypeUNCPath<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EntryTypeUnknown<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestinationOptionSameAsSource<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestinationOptionNone<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestinationOptionByRelativeName<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestinationOptionSet<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MigrationTableOnly<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessSecurity<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RsopLoggingNoComputer<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RsopLoggingNoUser<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RsopPlanningAssumeSlowLink<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RsopPlanningLoopbackOption<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbmerge: i16, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RsopPlanningAssumeUserWQLFilterTrue<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RsopPlanningAssumeCompWQLFilterTrue<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            PermGPOApply::<Impl, IMPL_OFFSET>,
            PermGPORead::<Impl, IMPL_OFFSET>,
            PermGPOEdit::<Impl, IMPL_OFFSET>,
            PermGPOEditSecurityAndDelete::<Impl, IMPL_OFFSET>,
            PermGPOCustom::<Impl, IMPL_OFFSET>,
            PermWMIFilterEdit::<Impl, IMPL_OFFSET>,
            PermWMIFilterFullControl::<Impl, IMPL_OFFSET>,
            PermWMIFilterCustom::<Impl, IMPL_OFFSET>,
            PermSOMLink::<Impl, IMPL_OFFSET>,
            PermSOMLogging::<Impl, IMPL_OFFSET>,
            PermSOMPlanning::<Impl, IMPL_OFFSET>,
            PermSOMGPOCreate::<Impl, IMPL_OFFSET>,
            PermSOMWMICreate::<Impl, IMPL_OFFSET>,
            PermSOMWMIFullControl::<Impl, IMPL_OFFSET>,
            SearchPropertyGPOPermissions::<Impl, IMPL_OFFSET>,
            SearchPropertyGPOEffectivePermissions::<Impl, IMPL_OFFSET>,
            SearchPropertyGPODisplayName::<Impl, IMPL_OFFSET>,
            SearchPropertyGPOWMIFilter::<Impl, IMPL_OFFSET>,
            SearchPropertyGPOID::<Impl, IMPL_OFFSET>,
            SearchPropertyGPOComputerExtensions::<Impl, IMPL_OFFSET>,
            SearchPropertyGPOUserExtensions::<Impl, IMPL_OFFSET>,
            SearchPropertySOMLinks::<Impl, IMPL_OFFSET>,
            SearchPropertyGPODomain::<Impl, IMPL_OFFSET>,
            SearchPropertyBackupMostRecent::<Impl, IMPL_OFFSET>,
            SearchOpEquals::<Impl, IMPL_OFFSET>,
            SearchOpContains::<Impl, IMPL_OFFSET>,
            SearchOpNotContains::<Impl, IMPL_OFFSET>,
            SearchOpNotEquals::<Impl, IMPL_OFFSET>,
            UsePDC::<Impl, IMPL_OFFSET>,
            UseAnyDC::<Impl, IMPL_OFFSET>,
            DoNotUseW2KDC::<Impl, IMPL_OFFSET>,
            SOMSite::<Impl, IMPL_OFFSET>,
            SOMDomain::<Impl, IMPL_OFFSET>,
            SOMOU::<Impl, IMPL_OFFSET>,
            SecurityFlags::<Impl, IMPL_OFFSET>,
            DoNotValidateDC::<Impl, IMPL_OFFSET>,
            ReportHTML::<Impl, IMPL_OFFSET>,
            ReportXML::<Impl, IMPL_OFFSET>,
            RSOPModeUnknown::<Impl, IMPL_OFFSET>,
            RSOPModePlanning::<Impl, IMPL_OFFSET>,
            RSOPModeLogging::<Impl, IMPL_OFFSET>,
            EntryTypeUser::<Impl, IMPL_OFFSET>,
            EntryTypeComputer::<Impl, IMPL_OFFSET>,
            EntryTypeLocalGroup::<Impl, IMPL_OFFSET>,
            EntryTypeGlobalGroup::<Impl, IMPL_OFFSET>,
            EntryTypeUniversalGroup::<Impl, IMPL_OFFSET>,
            EntryTypeUNCPath::<Impl, IMPL_OFFSET>,
            EntryTypeUnknown::<Impl, IMPL_OFFSET>,
            DestinationOptionSameAsSource::<Impl, IMPL_OFFSET>,
            DestinationOptionNone::<Impl, IMPL_OFFSET>,
            DestinationOptionByRelativeName::<Impl, IMPL_OFFSET>,
            DestinationOptionSet::<Impl, IMPL_OFFSET>,
            MigrationTableOnly::<Impl, IMPL_OFFSET>,
            ProcessSecurity::<Impl, IMPL_OFFSET>,
            RsopLoggingNoComputer::<Impl, IMPL_OFFSET>,
            RsopLoggingNoUser::<Impl, IMPL_OFFSET>,
            RsopPlanningAssumeSlowLink::<Impl, IMPL_OFFSET>,
            RsopPlanningLoopbackOption::<Impl, IMPL_OFFSET>,
            RsopPlanningAssumeUserWQLFilterTrue::<Impl, IMPL_OFFSET>,
            RsopPlanningAssumeCompWQLFilterTrue::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMConstants as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMConstants2Impl: Sized + IGPMConstantsImpl + IDispatchImpl {
    fn BackupTypeGPO();
    fn BackupTypeStarterGPO();
    fn StarterGPOTypeSystem();
    fn StarterGPOTypeCustom();
    fn SearchPropertyStarterGPOPermissions();
    fn SearchPropertyStarterGPOEffectivePermissions();
    fn SearchPropertyStarterGPODisplayName();
    fn SearchPropertyStarterGPOID();
    fn SearchPropertyStarterGPODomain();
    fn PermStarterGPORead();
    fn PermStarterGPOEdit();
    fn PermStarterGPOFullControl();
    fn PermStarterGPOCustom();
    fn ReportLegacy();
    fn ReportComments();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMConstants2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMConstants2Vtbl {
        unsafe extern "system" fn BackupTypeGPO<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMBackupType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackupTypeStarterGPO<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMBackupType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StarterGPOTypeSystem<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StarterGPOTypeCustom<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchPropertyStarterGPOPermissions<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchPropertyStarterGPOEffectivePermissions<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchPropertyStarterGPODisplayName<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchPropertyStarterGPOID<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchPropertyStarterGPODomain<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PermStarterGPORead<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PermStarterGPOEdit<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PermStarterGPOFullControl<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PermStarterGPOCustom<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReportLegacy<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMReportingOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReportComments<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMReportingOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            PermGPOApply::<Impl, IMPL_OFFSET>,
            PermGPORead::<Impl, IMPL_OFFSET>,
            PermGPOEdit::<Impl, IMPL_OFFSET>,
            PermGPOEditSecurityAndDelete::<Impl, IMPL_OFFSET>,
            PermGPOCustom::<Impl, IMPL_OFFSET>,
            PermWMIFilterEdit::<Impl, IMPL_OFFSET>,
            PermWMIFilterFullControl::<Impl, IMPL_OFFSET>,
            PermWMIFilterCustom::<Impl, IMPL_OFFSET>,
            PermSOMLink::<Impl, IMPL_OFFSET>,
            PermSOMLogging::<Impl, IMPL_OFFSET>,
            PermSOMPlanning::<Impl, IMPL_OFFSET>,
            PermSOMGPOCreate::<Impl, IMPL_OFFSET>,
            PermSOMWMICreate::<Impl, IMPL_OFFSET>,
            PermSOMWMIFullControl::<Impl, IMPL_OFFSET>,
            SearchPropertyGPOPermissions::<Impl, IMPL_OFFSET>,
            SearchPropertyGPOEffectivePermissions::<Impl, IMPL_OFFSET>,
            SearchPropertyGPODisplayName::<Impl, IMPL_OFFSET>,
            SearchPropertyGPOWMIFilter::<Impl, IMPL_OFFSET>,
            SearchPropertyGPOID::<Impl, IMPL_OFFSET>,
            SearchPropertyGPOComputerExtensions::<Impl, IMPL_OFFSET>,
            SearchPropertyGPOUserExtensions::<Impl, IMPL_OFFSET>,
            SearchPropertySOMLinks::<Impl, IMPL_OFFSET>,
            SearchPropertyGPODomain::<Impl, IMPL_OFFSET>,
            SearchPropertyBackupMostRecent::<Impl, IMPL_OFFSET>,
            SearchOpEquals::<Impl, IMPL_OFFSET>,
            SearchOpContains::<Impl, IMPL_OFFSET>,
            SearchOpNotContains::<Impl, IMPL_OFFSET>,
            SearchOpNotEquals::<Impl, IMPL_OFFSET>,
            UsePDC::<Impl, IMPL_OFFSET>,
            UseAnyDC::<Impl, IMPL_OFFSET>,
            DoNotUseW2KDC::<Impl, IMPL_OFFSET>,
            SOMSite::<Impl, IMPL_OFFSET>,
            SOMDomain::<Impl, IMPL_OFFSET>,
            SOMOU::<Impl, IMPL_OFFSET>,
            SecurityFlags::<Impl, IMPL_OFFSET>,
            DoNotValidateDC::<Impl, IMPL_OFFSET>,
            ReportHTML::<Impl, IMPL_OFFSET>,
            ReportXML::<Impl, IMPL_OFFSET>,
            RSOPModeUnknown::<Impl, IMPL_OFFSET>,
            RSOPModePlanning::<Impl, IMPL_OFFSET>,
            RSOPModeLogging::<Impl, IMPL_OFFSET>,
            EntryTypeUser::<Impl, IMPL_OFFSET>,
            EntryTypeComputer::<Impl, IMPL_OFFSET>,
            EntryTypeLocalGroup::<Impl, IMPL_OFFSET>,
            EntryTypeGlobalGroup::<Impl, IMPL_OFFSET>,
            EntryTypeUniversalGroup::<Impl, IMPL_OFFSET>,
            EntryTypeUNCPath::<Impl, IMPL_OFFSET>,
            EntryTypeUnknown::<Impl, IMPL_OFFSET>,
            DestinationOptionSameAsSource::<Impl, IMPL_OFFSET>,
            DestinationOptionNone::<Impl, IMPL_OFFSET>,
            DestinationOptionByRelativeName::<Impl, IMPL_OFFSET>,
            DestinationOptionSet::<Impl, IMPL_OFFSET>,
            MigrationTableOnly::<Impl, IMPL_OFFSET>,
            ProcessSecurity::<Impl, IMPL_OFFSET>,
            RsopLoggingNoComputer::<Impl, IMPL_OFFSET>,
            RsopLoggingNoUser::<Impl, IMPL_OFFSET>,
            RsopPlanningAssumeSlowLink::<Impl, IMPL_OFFSET>,
            RsopPlanningLoopbackOption::<Impl, IMPL_OFFSET>,
            RsopPlanningAssumeUserWQLFilterTrue::<Impl, IMPL_OFFSET>,
            RsopPlanningAssumeCompWQLFilterTrue::<Impl, IMPL_OFFSET>,
            BackupTypeGPO::<Impl, IMPL_OFFSET>,
            BackupTypeStarterGPO::<Impl, IMPL_OFFSET>,
            StarterGPOTypeSystem::<Impl, IMPL_OFFSET>,
            StarterGPOTypeCustom::<Impl, IMPL_OFFSET>,
            SearchPropertyStarterGPOPermissions::<Impl, IMPL_OFFSET>,
            SearchPropertyStarterGPOEffectivePermissions::<Impl, IMPL_OFFSET>,
            SearchPropertyStarterGPODisplayName::<Impl, IMPL_OFFSET>,
            SearchPropertyStarterGPOID::<Impl, IMPL_OFFSET>,
            SearchPropertyStarterGPODomain::<Impl, IMPL_OFFSET>,
            PermStarterGPORead::<Impl, IMPL_OFFSET>,
            PermStarterGPOEdit::<Impl, IMPL_OFFSET>,
            PermStarterGPOFullControl::<Impl, IMPL_OFFSET>,
            PermStarterGPOCustom::<Impl, IMPL_OFFSET>,
            ReportLegacy::<Impl, IMPL_OFFSET>,
            ReportComments::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMConstants2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMDomainImpl: Sized + IDispatchImpl {
    fn DomainController();
    fn Domain();
    fn CreateGPO();
    fn GetGPO();
    fn SearchGPOs();
    fn RestoreGPO();
    fn GetSOM();
    fn SearchSOMs();
    fn GetWMIFilter();
    fn SearchWMIFilters();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMDomainVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomainImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMDomainVtbl {
        unsafe extern "system" fn DomainController<Impl: IGPMDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Domain<Impl: IGPMDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateGPO<Impl: IGPMDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewgpo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGPO<Impl: IGPMDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppgpo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchGPOs<Impl: IGPMDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmgpocollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RestoreGPO<Impl: IGPMDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmbackup: ::windows::core::RawPtr, ldcflags: i32, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSOM<Impl: IGPMDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsom: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchSOMs<Impl: IGPMDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmsomcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWMIFilter<Impl: IGPMDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwmifilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchWMIFilters<Impl: IGPMDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmwmifiltercollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            DomainController::<Impl, IMPL_OFFSET>,
            Domain::<Impl, IMPL_OFFSET>,
            CreateGPO::<Impl, IMPL_OFFSET>,
            GetGPO::<Impl, IMPL_OFFSET>,
            SearchGPOs::<Impl, IMPL_OFFSET>,
            RestoreGPO::<Impl, IMPL_OFFSET>,
            GetSOM::<Impl, IMPL_OFFSET>,
            SearchSOMs::<Impl, IMPL_OFFSET>,
            GetWMIFilter::<Impl, IMPL_OFFSET>,
            SearchWMIFilters::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMDomain as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMDomain2Impl: Sized + IGPMDomainImpl + IDispatchImpl {
    fn CreateStarterGPO();
    fn CreateGPOFromStarterGPO();
    fn GetStarterGPO();
    fn SearchStarterGPOs();
    fn LoadStarterGPO();
    fn RestoreStarterGPO();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMDomain2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMDomain2Vtbl {
        unsafe extern "system" fn CreateStarterGPO<Impl: IGPMDomain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewtemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateGPOFromStarterGPO<Impl: IGPMDomain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgpotemplate: ::windows::core::RawPtr, ppnewgpo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStarterGPO<Impl: IGPMDomain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchStarterGPOs<Impl: IGPMDomain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmtemplatecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadStarterGPO<Impl: IGPMDomain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrloadfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, boverwrite: i16, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RestoreStarterGPO<Impl: IGPMDomain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmtmplbackup: ::windows::core::RawPtr, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            DomainController::<Impl, IMPL_OFFSET>,
            Domain::<Impl, IMPL_OFFSET>,
            CreateGPO::<Impl, IMPL_OFFSET>,
            GetGPO::<Impl, IMPL_OFFSET>,
            SearchGPOs::<Impl, IMPL_OFFSET>,
            RestoreGPO::<Impl, IMPL_OFFSET>,
            GetSOM::<Impl, IMPL_OFFSET>,
            SearchSOMs::<Impl, IMPL_OFFSET>,
            GetWMIFilter::<Impl, IMPL_OFFSET>,
            SearchWMIFilters::<Impl, IMPL_OFFSET>,
            CreateStarterGPO::<Impl, IMPL_OFFSET>,
            CreateGPOFromStarterGPO::<Impl, IMPL_OFFSET>,
            GetStarterGPO::<Impl, IMPL_OFFSET>,
            SearchStarterGPOs::<Impl, IMPL_OFFSET>,
            LoadStarterGPO::<Impl, IMPL_OFFSET>,
            RestoreStarterGPO::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMDomain2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMDomain3Impl: Sized + IGPMDomain2Impl + IGPMDomainImpl + IDispatchImpl {
    fn GenerateReport();
    fn InfrastructureDC();
    fn SetInfrastructureDC();
    fn SetInfrastructureFlags();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMDomain3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMDomain3Vtbl {
        unsafe extern "system" fn GenerateReport<Impl: IGPMDomain3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InfrastructureDC<Impl: IGPMDomain3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInfrastructureDC<Impl: IGPMDomain3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInfrastructureFlags<Impl: IGPMDomain3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            DomainController::<Impl, IMPL_OFFSET>,
            Domain::<Impl, IMPL_OFFSET>,
            CreateGPO::<Impl, IMPL_OFFSET>,
            GetGPO::<Impl, IMPL_OFFSET>,
            SearchGPOs::<Impl, IMPL_OFFSET>,
            RestoreGPO::<Impl, IMPL_OFFSET>,
            GetSOM::<Impl, IMPL_OFFSET>,
            SearchSOMs::<Impl, IMPL_OFFSET>,
            GetWMIFilter::<Impl, IMPL_OFFSET>,
            SearchWMIFilters::<Impl, IMPL_OFFSET>,
            CreateStarterGPO::<Impl, IMPL_OFFSET>,
            CreateGPOFromStarterGPO::<Impl, IMPL_OFFSET>,
            GetStarterGPO::<Impl, IMPL_OFFSET>,
            SearchStarterGPOs::<Impl, IMPL_OFFSET>,
            LoadStarterGPO::<Impl, IMPL_OFFSET>,
            RestoreStarterGPO::<Impl, IMPL_OFFSET>,
            GenerateReport::<Impl, IMPL_OFFSET>,
            InfrastructureDC::<Impl, IMPL_OFFSET>,
            SetInfrastructureDC::<Impl, IMPL_OFFSET>,
            SetInfrastructureFlags::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMDomain3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMGPOImpl: Sized + IDispatchImpl {
    fn DisplayName();
    fn SetDisplayName();
    fn Path();
    fn ID();
    fn DomainName();
    fn CreationTime();
    fn ModificationTime();
    fn UserDSVersionNumber();
    fn ComputerDSVersionNumber();
    fn UserSysvolVersionNumber();
    fn ComputerSysvolVersionNumber();
    fn GetWMIFilter();
    fn SetWMIFilter();
    fn SetUserEnabled();
    fn SetComputerEnabled();
    fn IsUserEnabled();
    fn IsComputerEnabled();
    fn GetSecurityInfo();
    fn SetSecurityInfo();
    fn Delete();
    fn Backup();
    fn Import();
    fn GenerateReport();
    fn GenerateReportToFile();
    fn CopyTo();
    fn SetSecurityDescriptor();
    fn GetSecurityDescriptor();
    fn IsACLConsistent();
    fn MakeACLConsistent();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPOVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMGPOVtbl {
        unsafe extern "system" fn DisplayName<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisplayName<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Path<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ID<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DomainName<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreationTime<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ModificationTime<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UserDSVersionNumber<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ComputerDSVersionNumber<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UserSysvolVersionNumber<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ComputerSysvolVersionNumber<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWMIFilter<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmwmifilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWMIFilter<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmwmifilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUserEnabled<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetComputerEnabled<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsUserEnabled<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsComputerEnabled<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSecurityInfo<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSecurityInfo<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecurityinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Backup<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrcomment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Import<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pigpmbackup: ::windows::core::RawPtr, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateReport<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateReportToFile<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyTo<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pigpmdomain: ::windows::core::RawPtr, pvarnewdisplayname: *const super::Com::VARIANT, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSecurityDescriptor<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, psd: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSecurityDescriptor<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, ppsd: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsACLConsistent<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbconsistent: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MakeACLConsistent<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName::<Impl, IMPL_OFFSET>,
            Path::<Impl, IMPL_OFFSET>,
            ID::<Impl, IMPL_OFFSET>,
            DomainName::<Impl, IMPL_OFFSET>,
            CreationTime::<Impl, IMPL_OFFSET>,
            ModificationTime::<Impl, IMPL_OFFSET>,
            UserDSVersionNumber::<Impl, IMPL_OFFSET>,
            ComputerDSVersionNumber::<Impl, IMPL_OFFSET>,
            UserSysvolVersionNumber::<Impl, IMPL_OFFSET>,
            ComputerSysvolVersionNumber::<Impl, IMPL_OFFSET>,
            GetWMIFilter::<Impl, IMPL_OFFSET>,
            SetWMIFilter::<Impl, IMPL_OFFSET>,
            SetUserEnabled::<Impl, IMPL_OFFSET>,
            SetComputerEnabled::<Impl, IMPL_OFFSET>,
            IsUserEnabled::<Impl, IMPL_OFFSET>,
            IsComputerEnabled::<Impl, IMPL_OFFSET>,
            GetSecurityInfo::<Impl, IMPL_OFFSET>,
            SetSecurityInfo::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Backup::<Impl, IMPL_OFFSET>,
            Import::<Impl, IMPL_OFFSET>,
            GenerateReport::<Impl, IMPL_OFFSET>,
            GenerateReportToFile::<Impl, IMPL_OFFSET>,
            CopyTo::<Impl, IMPL_OFFSET>,
            SetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            GetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            IsACLConsistent::<Impl, IMPL_OFFSET>,
            MakeACLConsistent::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMGPO as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMGPO2Impl: Sized + IGPMGPOImpl + IDispatchImpl {
    fn Description();
    fn SetDescription();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPO2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMGPO2Vtbl {
        unsafe extern "system" fn Description<Impl: IGPMGPO2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: IGPMGPO2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName::<Impl, IMPL_OFFSET>,
            Path::<Impl, IMPL_OFFSET>,
            ID::<Impl, IMPL_OFFSET>,
            DomainName::<Impl, IMPL_OFFSET>,
            CreationTime::<Impl, IMPL_OFFSET>,
            ModificationTime::<Impl, IMPL_OFFSET>,
            UserDSVersionNumber::<Impl, IMPL_OFFSET>,
            ComputerDSVersionNumber::<Impl, IMPL_OFFSET>,
            UserSysvolVersionNumber::<Impl, IMPL_OFFSET>,
            ComputerSysvolVersionNumber::<Impl, IMPL_OFFSET>,
            GetWMIFilter::<Impl, IMPL_OFFSET>,
            SetWMIFilter::<Impl, IMPL_OFFSET>,
            SetUserEnabled::<Impl, IMPL_OFFSET>,
            SetComputerEnabled::<Impl, IMPL_OFFSET>,
            IsUserEnabled::<Impl, IMPL_OFFSET>,
            IsComputerEnabled::<Impl, IMPL_OFFSET>,
            GetSecurityInfo::<Impl, IMPL_OFFSET>,
            SetSecurityInfo::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Backup::<Impl, IMPL_OFFSET>,
            Import::<Impl, IMPL_OFFSET>,
            GenerateReport::<Impl, IMPL_OFFSET>,
            GenerateReportToFile::<Impl, IMPL_OFFSET>,
            CopyTo::<Impl, IMPL_OFFSET>,
            SetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            GetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            IsACLConsistent::<Impl, IMPL_OFFSET>,
            MakeACLConsistent::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMGPO2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMGPO3Impl: Sized + IGPMGPO2Impl + IGPMGPOImpl + IDispatchImpl {
    fn InfrastructureDC();
    fn SetInfrastructureDC();
    fn SetInfrastructureFlags();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPO3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMGPO3Vtbl {
        unsafe extern "system" fn InfrastructureDC<Impl: IGPMGPO3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInfrastructureDC<Impl: IGPMGPO3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInfrastructureFlags<Impl: IGPMGPO3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName::<Impl, IMPL_OFFSET>,
            Path::<Impl, IMPL_OFFSET>,
            ID::<Impl, IMPL_OFFSET>,
            DomainName::<Impl, IMPL_OFFSET>,
            CreationTime::<Impl, IMPL_OFFSET>,
            ModificationTime::<Impl, IMPL_OFFSET>,
            UserDSVersionNumber::<Impl, IMPL_OFFSET>,
            ComputerDSVersionNumber::<Impl, IMPL_OFFSET>,
            UserSysvolVersionNumber::<Impl, IMPL_OFFSET>,
            ComputerSysvolVersionNumber::<Impl, IMPL_OFFSET>,
            GetWMIFilter::<Impl, IMPL_OFFSET>,
            SetWMIFilter::<Impl, IMPL_OFFSET>,
            SetUserEnabled::<Impl, IMPL_OFFSET>,
            SetComputerEnabled::<Impl, IMPL_OFFSET>,
            IsUserEnabled::<Impl, IMPL_OFFSET>,
            IsComputerEnabled::<Impl, IMPL_OFFSET>,
            GetSecurityInfo::<Impl, IMPL_OFFSET>,
            SetSecurityInfo::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Backup::<Impl, IMPL_OFFSET>,
            Import::<Impl, IMPL_OFFSET>,
            GenerateReport::<Impl, IMPL_OFFSET>,
            GenerateReportToFile::<Impl, IMPL_OFFSET>,
            CopyTo::<Impl, IMPL_OFFSET>,
            SetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            GetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            IsACLConsistent::<Impl, IMPL_OFFSET>,
            MakeACLConsistent::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            InfrastructureDC::<Impl, IMPL_OFFSET>,
            SetInfrastructureDC::<Impl, IMPL_OFFSET>,
            SetInfrastructureFlags::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMGPO3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMGPOCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPOCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMGPOCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IGPMGPOCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IGPMGPOCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IGPMGPOCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmgpos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMGPOCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMGPOLinkImpl: Sized + IDispatchImpl {
    fn GPOID();
    fn GPODomain();
    fn Enabled();
    fn SetEnabled();
    fn Enforced();
    fn SetEnforced();
    fn SOMLinkOrder();
    fn SOM();
    fn Delete();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPOLinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOLinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMGPOLinkVtbl {
        unsafe extern "system" fn GPOID<Impl: IGPMGPOLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GPODomain<Impl: IGPMGPOLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enabled<Impl: IGPMGPOLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnabled<Impl: IGPMGPOLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enforced<Impl: IGPMGPOLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnforced<Impl: IGPMGPOLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SOMLinkOrder<Impl: IGPMGPOLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SOM<Impl: IGPMGPOLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmsom: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IGPMGPOLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GPOID::<Impl, IMPL_OFFSET>,
            GPODomain::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            Enforced::<Impl, IMPL_OFFSET>,
            SetEnforced::<Impl, IMPL_OFFSET>,
            SOMLinkOrder::<Impl, IMPL_OFFSET>,
            SOM::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMGPOLink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMGPOLinksCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPOLinksCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOLinksCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMGPOLinksCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IGPMGPOLinksCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IGPMGPOLinksCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IGPMGPOLinksCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmlinks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMGPOLinksCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMMapEntryImpl: Sized + IDispatchImpl {
    fn Source();
    fn Destination();
    fn DestinationOption();
    fn EntryType();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMMapEntryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMapEntryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMMapEntryVtbl {
        unsafe extern "system" fn Source<Impl: IGPMMapEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Destination<Impl: IGPMMapEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdestination: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestinationOption<Impl: IGPMMapEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgpmdestoption: *mut GPMDestinationOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EntryType<Impl: IGPMMapEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgpmentrytype: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Source::<Impl, IMPL_OFFSET>, Destination::<Impl, IMPL_OFFSET>, DestinationOption::<Impl, IMPL_OFFSET>, EntryType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMMapEntry as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMMapEntryCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMMapEntryCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMapEntryCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMMapEntryCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IGPMMapEntryCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IGPMMapEntryCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IGPMMapEntryCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMMapEntryCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMMigrationTableImpl: Sized + IDispatchImpl {
    fn Save();
    fn Add();
    fn AddEntry();
    fn GetEntry();
    fn DeleteEntry();
    fn UpdateDestination();
    fn Validate();
    fn GetEntries();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMMigrationTableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMigrationTableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMMigrationTableVtbl {
        unsafe extern "system" fn Save<Impl: IGPMMigrationTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmigrationtablepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IGPMMigrationTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, var: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddEntry<Impl: IGPMMigrationTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, gpmentrytype: GPMEntryType, pvardestination: *const super::Com::VARIANT, ppentry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEntry<Impl: IGPMMigrationTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppentry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteEntry<Impl: IGPMMigrationTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateDestination<Impl: IGPMMigrationTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvardestination: *const super::Com::VARIANT, ppentry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Validate<Impl: IGPMMigrationTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEntries<Impl: IGPMMigrationTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppentries: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Save::<Impl, IMPL_OFFSET>,
            Add::<Impl, IMPL_OFFSET>,
            AddEntry::<Impl, IMPL_OFFSET>,
            GetEntry::<Impl, IMPL_OFFSET>,
            DeleteEntry::<Impl, IMPL_OFFSET>,
            UpdateDestination::<Impl, IMPL_OFFSET>,
            Validate::<Impl, IMPL_OFFSET>,
            GetEntries::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMMigrationTable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMPermissionImpl: Sized + IDispatchImpl {
    fn Inherited();
    fn Inheritable();
    fn Denied();
    fn Permission();
    fn Trustee();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMPermissionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMPermissionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMPermissionVtbl {
        unsafe extern "system" fn Inherited<Impl: IGPMPermissionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Inheritable<Impl: IGPMPermissionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Denied<Impl: IGPMPermissionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Permission<Impl: IGPMPermissionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Trustee<Impl: IGPMPermissionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmtrustee: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Inherited::<Impl, IMPL_OFFSET>, Inheritable::<Impl, IMPL_OFFSET>, Denied::<Impl, IMPL_OFFSET>, Permission::<Impl, IMPL_OFFSET>, Trustee::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMPermission as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMRSOPImpl: Sized + IDispatchImpl {
    fn Mode();
    fn Namespace();
    fn SetLoggingComputer();
    fn LoggingComputer();
    fn SetLoggingUser();
    fn LoggingUser();
    fn SetLoggingFlags();
    fn LoggingFlags();
    fn SetPlanningFlags();
    fn PlanningFlags();
    fn SetPlanningDomainController();
    fn PlanningDomainController();
    fn SetPlanningSiteName();
    fn PlanningSiteName();
    fn SetPlanningUser();
    fn PlanningUser();
    fn SetPlanningUserSOM();
    fn PlanningUserSOM();
    fn SetPlanningUserWMIFilters();
    fn PlanningUserWMIFilters();
    fn SetPlanningUserSecurityGroups();
    fn PlanningUserSecurityGroups();
    fn SetPlanningComputer();
    fn PlanningComputer();
    fn SetPlanningComputerSOM();
    fn PlanningComputerSOM();
    fn SetPlanningComputerWMIFilters();
    fn PlanningComputerWMIFilters();
    fn SetPlanningComputerSecurityGroups();
    fn PlanningComputerSecurityGroups();
    fn LoggingEnumerateUsers();
    fn CreateQueryResults();
    fn ReleaseQueryResults();
    fn GenerateReport();
    fn GenerateReportToFile();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMRSOPVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOPImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMRSOPVtbl {
        unsafe extern "system" fn Mode<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Namespace<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLoggingComputer<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoggingComputer<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLoggingUser<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoggingUser<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLoggingFlags<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoggingFlags<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPlanningFlags<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlanningFlags<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPlanningDomainController<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlanningDomainController<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPlanningSiteName<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlanningSiteName<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPlanningUser<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlanningUser<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPlanningUserSOM<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlanningUserSOM<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPlanningUserWMIFilters<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlanningUserWMIFilters<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPlanningUserSecurityGroups<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlanningUserSecurityGroups<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPlanningComputer<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlanningComputer<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPlanningComputerSOM<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlanningComputerSOM<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPlanningComputerWMIFilters<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlanningComputerWMIFilters<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPlanningComputerSecurityGroups<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlanningComputerSecurityGroups<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoggingEnumerateUsers<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateQueryResults<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseQueryResults<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateReport<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateReportToFile<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Mode::<Impl, IMPL_OFFSET>,
            Namespace::<Impl, IMPL_OFFSET>,
            SetLoggingComputer::<Impl, IMPL_OFFSET>,
            LoggingComputer::<Impl, IMPL_OFFSET>,
            SetLoggingUser::<Impl, IMPL_OFFSET>,
            LoggingUser::<Impl, IMPL_OFFSET>,
            SetLoggingFlags::<Impl, IMPL_OFFSET>,
            LoggingFlags::<Impl, IMPL_OFFSET>,
            SetPlanningFlags::<Impl, IMPL_OFFSET>,
            PlanningFlags::<Impl, IMPL_OFFSET>,
            SetPlanningDomainController::<Impl, IMPL_OFFSET>,
            PlanningDomainController::<Impl, IMPL_OFFSET>,
            SetPlanningSiteName::<Impl, IMPL_OFFSET>,
            PlanningSiteName::<Impl, IMPL_OFFSET>,
            SetPlanningUser::<Impl, IMPL_OFFSET>,
            PlanningUser::<Impl, IMPL_OFFSET>,
            SetPlanningUserSOM::<Impl, IMPL_OFFSET>,
            PlanningUserSOM::<Impl, IMPL_OFFSET>,
            SetPlanningUserWMIFilters::<Impl, IMPL_OFFSET>,
            PlanningUserWMIFilters::<Impl, IMPL_OFFSET>,
            SetPlanningUserSecurityGroups::<Impl, IMPL_OFFSET>,
            PlanningUserSecurityGroups::<Impl, IMPL_OFFSET>,
            SetPlanningComputer::<Impl, IMPL_OFFSET>,
            PlanningComputer::<Impl, IMPL_OFFSET>,
            SetPlanningComputerSOM::<Impl, IMPL_OFFSET>,
            PlanningComputerSOM::<Impl, IMPL_OFFSET>,
            SetPlanningComputerWMIFilters::<Impl, IMPL_OFFSET>,
            PlanningComputerWMIFilters::<Impl, IMPL_OFFSET>,
            SetPlanningComputerSecurityGroups::<Impl, IMPL_OFFSET>,
            PlanningComputerSecurityGroups::<Impl, IMPL_OFFSET>,
            LoggingEnumerateUsers::<Impl, IMPL_OFFSET>,
            CreateQueryResults::<Impl, IMPL_OFFSET>,
            ReleaseQueryResults::<Impl, IMPL_OFFSET>,
            GenerateReport::<Impl, IMPL_OFFSET>,
            GenerateReportToFile::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMRSOP as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMResultImpl: Sized + IDispatchImpl {
    fn Status();
    fn Result();
    fn OverallStatus();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMResultVtbl {
        unsafe extern "system" fn Status<Impl: IGPMResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmstatusmsgcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Result<Impl: IGPMResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarresult: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OverallStatus<Impl: IGPMResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Status::<Impl, IMPL_OFFSET>, Result::<Impl, IMPL_OFFSET>, OverallStatus::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMSOMImpl: Sized + IDispatchImpl {
    fn GPOInheritanceBlocked();
    fn SetGPOInheritanceBlocked();
    fn Name();
    fn Path();
    fn CreateGPOLink();
    fn Type();
    fn GetGPOLinks();
    fn GetInheritedGPOLinks();
    fn GetSecurityInfo();
    fn SetSecurityInfo();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMSOMVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSOMImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMSOMVtbl {
        unsafe extern "system" fn GPOInheritanceBlocked<Impl: IGPMSOMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGPOInheritanceBlocked<Impl: IGPMSOMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: IGPMSOMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Path<Impl: IGPMSOMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateGPOLink<Impl: IGPMSOMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llinkpos: i32, pgpo: ::windows::core::RawPtr, ppnewgpolink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Type<Impl: IGPMSOMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGPOLinks<Impl: IGPMSOMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgpolinks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInheritedGPOLinks<Impl: IGPMSOMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgpolinks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSecurityInfo<Impl: IGPMSOMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSecurityInfo<Impl: IGPMSOMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecurityinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GPOInheritanceBlocked::<Impl, IMPL_OFFSET>,
            SetGPOInheritanceBlocked::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Path::<Impl, IMPL_OFFSET>,
            CreateGPOLink::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            GetGPOLinks::<Impl, IMPL_OFFSET>,
            GetInheritedGPOLinks::<Impl, IMPL_OFFSET>,
            GetSecurityInfo::<Impl, IMPL_OFFSET>,
            SetSecurityInfo::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMSOM as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMSOMCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMSOMCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSOMCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMSOMCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IGPMSOMCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IGPMSOMCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IGPMSOMCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmsom: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMSOMCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMSearchCriteriaImpl: Sized + IDispatchImpl {
    fn Add();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMSearchCriteriaVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSearchCriteriaImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMSearchCriteriaVtbl {
        unsafe extern "system" fn Add<Impl: IGPMSearchCriteriaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, searchproperty: GPMSearchProperty, searchoperation: GPMSearchOperation, varvalue: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMSearchCriteria as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMSecurityInfoImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn RemoveTrustee();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMSecurityInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSecurityInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMSecurityInfoVtbl {
        unsafe extern "system" fn Count<Impl: IGPMSecurityInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IGPMSecurityInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IGPMSecurityInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IGPMSecurityInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IGPMSecurityInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveTrustee<Impl: IGPMSecurityInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>, Remove::<Impl, IMPL_OFFSET>, RemoveTrustee::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMSecurityInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMSitesContainerImpl: Sized + IDispatchImpl {
    fn DomainController();
    fn Domain();
    fn Forest();
    fn GetSite();
    fn SearchSites();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMSitesContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSitesContainerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMSitesContainerVtbl {
        unsafe extern "system" fn DomainController<Impl: IGPMSitesContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Domain<Impl: IGPMSitesContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Forest<Impl: IGPMSitesContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSite<Impl: IGPMSitesContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsitename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsom: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchSites<Impl: IGPMSitesContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmsomcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, DomainController::<Impl, IMPL_OFFSET>, Domain::<Impl, IMPL_OFFSET>, Forest::<Impl, IMPL_OFFSET>, GetSite::<Impl, IMPL_OFFSET>, SearchSites::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMSitesContainer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMStarterGPOImpl: Sized + IDispatchImpl {
    fn DisplayName();
    fn SetDisplayName();
    fn Description();
    fn SetDescription();
    fn Author();
    fn Product();
    fn CreationTime();
    fn ID();
    fn ModifiedTime();
    fn Type();
    fn ComputerVersion();
    fn UserVersion();
    fn StarterGPOVersion();
    fn Delete();
    fn Save();
    fn Backup();
    fn CopyTo();
    fn GenerateReport();
    fn GenerateReportToFile();
    fn GetSecurityInfo();
    fn SetSecurityInfo();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStarterGPOVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMStarterGPOVtbl {
        unsafe extern "system" fn DisplayName<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisplayName<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Author<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Product<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreationTime<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ID<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ModifiedTime<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Type<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ComputerVersion<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UserVersion<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StarterGPOVersion<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Save<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsavefile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, boverwrite: i16, bsaveassystem: i16, bstrlanguage: *const super::Com::VARIANT, bstrauthor: *const super::Com::VARIANT, bstrproduct: *const super::Com::VARIANT, bstruniqueid: *const super::Com::VARIANT, bstrversion: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Backup<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrcomment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyTo<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarnewdisplayname: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateReport<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateReportToFile<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSecurityInfo<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSecurityInfo<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecurityinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Author::<Impl, IMPL_OFFSET>,
            Product::<Impl, IMPL_OFFSET>,
            CreationTime::<Impl, IMPL_OFFSET>,
            ID::<Impl, IMPL_OFFSET>,
            ModifiedTime::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            ComputerVersion::<Impl, IMPL_OFFSET>,
            UserVersion::<Impl, IMPL_OFFSET>,
            StarterGPOVersion::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Save::<Impl, IMPL_OFFSET>,
            Backup::<Impl, IMPL_OFFSET>,
            CopyTo::<Impl, IMPL_OFFSET>,
            GenerateReport::<Impl, IMPL_OFFSET>,
            GenerateReportToFile::<Impl, IMPL_OFFSET>,
            GetSecurityInfo::<Impl, IMPL_OFFSET>,
            SetSecurityInfo::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMStarterGPO as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMStarterGPOBackupImpl: Sized + IDispatchImpl {
    fn BackupDir();
    fn Comment();
    fn DisplayName();
    fn Domain();
    fn StarterGPOID();
    fn ID();
    fn Timestamp();
    fn Type();
    fn Delete();
    fn GenerateReport();
    fn GenerateReportToFile();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStarterGPOBackupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOBackupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMStarterGPOBackupVtbl {
        unsafe extern "system" fn BackupDir<Impl: IGPMStarterGPOBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbackupdir: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Comment<Impl: IGPMStarterGPOBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcomment: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplayName<Impl: IGPMStarterGPOBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Domain<Impl: IGPMStarterGPOBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtemplatedomain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StarterGPOID<Impl: IGPMStarterGPOBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtemplateid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ID<Impl: IGPMStarterGPOBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Timestamp<Impl: IGPMStarterGPOBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimestamp: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Type<Impl: IGPMStarterGPOBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut GPMStarterGPOType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IGPMStarterGPOBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateReport<Impl: IGPMStarterGPOBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateReportToFile<Impl: IGPMStarterGPOBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            BackupDir::<Impl, IMPL_OFFSET>,
            Comment::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
            Domain::<Impl, IMPL_OFFSET>,
            StarterGPOID::<Impl, IMPL_OFFSET>,
            ID::<Impl, IMPL_OFFSET>,
            Timestamp::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            GenerateReport::<Impl, IMPL_OFFSET>,
            GenerateReportToFile::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMStarterGPOBackup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMStarterGPOBackupCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStarterGPOBackupCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOBackupCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMStarterGPOBackupCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IGPMStarterGPOBackupCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IGPMStarterGPOBackupCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IGPMStarterGPOBackupCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmtmplbackup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMStarterGPOBackupCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMStarterGPOCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStarterGPOCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMStarterGPOCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IGPMStarterGPOCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IGPMStarterGPOCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IGPMStarterGPOCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmtemplates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMStarterGPOCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMStatusMessageImpl: Sized + IDispatchImpl {
    fn ObjectPath();
    fn ErrorCode();
    fn ExtensionName();
    fn SettingsName();
    fn OperationCode();
    fn Message();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStatusMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStatusMessageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMStatusMessageVtbl {
        unsafe extern "system" fn ObjectPath<Impl: IGPMStatusMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ErrorCode<Impl: IGPMStatusMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExtensionName<Impl: IGPMStatusMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SettingsName<Impl: IGPMStatusMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OperationCode<Impl: IGPMStatusMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Message<Impl: IGPMStatusMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            ObjectPath::<Impl, IMPL_OFFSET>,
            ErrorCode::<Impl, IMPL_OFFSET>,
            ExtensionName::<Impl, IMPL_OFFSET>,
            SettingsName::<Impl, IMPL_OFFSET>,
            OperationCode::<Impl, IMPL_OFFSET>,
            Message::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMStatusMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMStatusMsgCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStatusMsgCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStatusMsgCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMStatusMsgCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IGPMStatusMsgCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IGPMStatusMsgCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IGPMStatusMsgCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMStatusMsgCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMTrusteeImpl: Sized + IDispatchImpl {
    fn TrusteeSid();
    fn TrusteeName();
    fn TrusteeDomain();
    fn TrusteeDSPath();
    fn TrusteeType();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMTrusteeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMTrusteeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMTrusteeVtbl {
        unsafe extern "system" fn TrusteeSid<Impl: IGPMTrusteeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TrusteeName<Impl: IGPMTrusteeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TrusteeDomain<Impl: IGPMTrusteeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TrusteeDSPath<Impl: IGPMTrusteeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TrusteeType<Impl: IGPMTrusteeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, TrusteeSid::<Impl, IMPL_OFFSET>, TrusteeName::<Impl, IMPL_OFFSET>, TrusteeDomain::<Impl, IMPL_OFFSET>, TrusteeDSPath::<Impl, IMPL_OFFSET>, TrusteeType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMTrustee as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMWMIFilterImpl: Sized + IDispatchImpl {
    fn Path();
    fn SetName();
    fn Name();
    fn SetDescription();
    fn Description();
    fn GetQueryList();
    fn GetSecurityInfo();
    fn SetSecurityInfo();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMWMIFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMWMIFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMWMIFilterVtbl {
        unsafe extern "system" fn Path<Impl: IGPMWMIFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: IGPMWMIFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: IGPMWMIFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: IGPMWMIFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: IGPMWMIFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetQueryList<Impl: IGPMWMIFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqrylist: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSecurityInfo<Impl: IGPMWMIFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSecurityInfo<Impl: IGPMWMIFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecurityinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Path::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            GetQueryList::<Impl, IMPL_OFFSET>,
            GetSecurityInfo::<Impl, IMPL_OFFSET>,
            SetSecurityInfo::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMWMIFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMWMIFilterCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMWMIFilterCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMWMIFilterCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMWMIFilterCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IGPMWMIFilterCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IGPMWMIFilterCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IGPMWMIFilterCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMWMIFilterCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_Controls"))]
pub trait IGroupPolicyObjectImpl: Sized {
    fn New();
    fn OpenDSGPO();
    fn OpenLocalMachineGPO();
    fn OpenRemoteMachineGPO();
    fn Save();
    fn Delete();
    fn GetName();
    fn GetDisplayName();
    fn SetDisplayName();
    fn GetPath();
    fn GetDSPath();
    fn GetFileSysPath();
    fn GetRegistryKey();
    fn GetOptions();
    fn SetOptions();
    fn GetType();
    fn GetMachineName();
    fn GetPropertySheetPages();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_Controls"))]
impl IGroupPolicyObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGroupPolicyObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGroupPolicyObjectVtbl {
        unsafe extern "system" fn New<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdomainname: super::super::Foundation::PWSTR, pszdisplayname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenDSGPO<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenLocalMachineGPO<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenRemoteMachineGPO<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcomputername: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Save<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows::core::GUID, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetName<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayName<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisplayName<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPath<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDSPath<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileSysPath<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRegistryKey<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOptions<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOptions<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: u32, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetType<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMachineName<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertySheetPages<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpages: *mut *mut super::super::UI::Controls::HPROPSHEETPAGE, upagecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            New::<Impl, IMPL_OFFSET>,
            OpenDSGPO::<Impl, IMPL_OFFSET>,
            OpenLocalMachineGPO::<Impl, IMPL_OFFSET>,
            OpenRemoteMachineGPO::<Impl, IMPL_OFFSET>,
            Save::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            GetName::<Impl, IMPL_OFFSET>,
            GetDisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName::<Impl, IMPL_OFFSET>,
            GetPath::<Impl, IMPL_OFFSET>,
            GetDSPath::<Impl, IMPL_OFFSET>,
            GetFileSysPath::<Impl, IMPL_OFFSET>,
            GetRegistryKey::<Impl, IMPL_OFFSET>,
            GetOptions::<Impl, IMPL_OFFSET>,
            SetOptions::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetMachineName::<Impl, IMPL_OFFSET>,
            GetPropertySheetPages::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGroupPolicyObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRSOPInformationImpl: Sized {
    fn GetNamespace();
    fn GetFlags();
    fn GetEventLogEntryText();
}
#[cfg(feature = "Win32_Foundation")]
impl IRSOPInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRSOPInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRSOPInformationVtbl {
        unsafe extern "system" fn GetNamespace<Impl: IRSOPInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFlags<Impl: IRSOPInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEventLogEntryText<Impl: IRSOPInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszeventsource: super::super::Foundation::PWSTR, pszeventlogname: super::super::Foundation::PWSTR, pszeventtime: super::super::Foundation::PWSTR, dweventid: u32, ppsztext: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetNamespace::<Impl, IMPL_OFFSET>, GetFlags::<Impl, IMPL_OFFSET>, GetEventLogEntryText::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRSOPInformation as ::windows::core::Interface>::IID
    }
}
