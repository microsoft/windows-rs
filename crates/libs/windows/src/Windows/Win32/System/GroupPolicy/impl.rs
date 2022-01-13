#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IGPEInformationImpl: Sized {
    fn GetName(&mut self, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::Result<()>;
    fn GetDisplayName(&mut self, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::Result<()>;
    fn GetRegistryKey(&mut self, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows::core::Result<()>;
    fn GetDSPath(&mut self, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::core::Result<()>;
    fn GetFileSysPath(&mut self, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::core::Result<()>;
    fn GetOptions(&mut self, dwoptions: *mut u32) -> ::windows::core::Result<()>;
    fn GetType(&mut self, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows::core::Result<()>;
    fn GetHint(&mut self, gphint: *mut GROUP_POLICY_HINT_TYPE) -> ::windows::core::Result<()>;
    fn PolicyChanged(&mut self, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows::core::GUID, pguidsnapin: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl IGPEInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPEInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPEInformationVtbl {
        unsafe extern "system" fn GetName<Impl: IGPEInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetName(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetDisplayName<Impl: IGPEInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDisplayName(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetRegistryKey<Impl: IGPEInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRegistryKey(::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&hkey)).into()
        }
        unsafe extern "system" fn GetDSPath<Impl: IGPEInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDSPath(::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&cchmaxpath)).into()
        }
        unsafe extern "system" fn GetFileSysPath<Impl: IGPEInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFileSysPath(::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&cchmaxpath)).into()
        }
        unsafe extern "system" fn GetOptions<Impl: IGPEInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOptions(::core::mem::transmute_copy(&dwoptions)).into()
        }
        unsafe extern "system" fn GetType<Impl: IGPEInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetType(::core::mem::transmute_copy(&gpotype)).into()
        }
        unsafe extern "system" fn GetHint<Impl: IGPEInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gphint: *mut GROUP_POLICY_HINT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetHint(::core::mem::transmute_copy(&gphint)).into()
        }
        unsafe extern "system" fn PolicyChanged<Impl: IGPEInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows::core::GUID, pguidsnapin: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PolicyChanged(::core::mem::transmute_copy(&bmachine), ::core::mem::transmute_copy(&badd), ::core::mem::transmute_copy(&pguidextension), ::core::mem::transmute_copy(&pguidsnapin)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetDisplayName: GetDisplayName::<Impl, IMPL_OFFSET>,
            GetRegistryKey: GetRegistryKey::<Impl, IMPL_OFFSET>,
            GetDSPath: GetDSPath::<Impl, IMPL_OFFSET>,
            GetFileSysPath: GetFileSysPath::<Impl, IMPL_OFFSET>,
            GetOptions: GetOptions::<Impl, IMPL_OFFSET>,
            GetType: GetType::<Impl, IMPL_OFFSET>,
            GetHint: GetHint::<Impl, IMPL_OFFSET>,
            PolicyChanged: PolicyChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPEInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMImpl: Sized + IDispatchImpl {
    fn GetDomain(&mut self, bstrdomain: super::super::Foundation::BSTR, bstrdomaincontroller: super::super::Foundation::BSTR, ldcflags: i32) -> ::windows::core::Result<IGPMDomain>;
    fn GetBackupDir(&mut self, bstrbackupdir: super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMBackupDir>;
    fn GetSitesContainer(&mut self, bstrforest: super::super::Foundation::BSTR, bstrdomain: super::super::Foundation::BSTR, bstrdomaincontroller: super::super::Foundation::BSTR, ldcflags: i32) -> ::windows::core::Result<IGPMSitesContainer>;
    fn GetRSOP(&mut self, gpmrsopmode: GPMRSOPMode, bstrnamespace: super::super::Foundation::BSTR, lflags: i32) -> ::windows::core::Result<IGPMRSOP>;
    fn CreatePermission(&mut self, bstrtrustee: super::super::Foundation::BSTR, perm: GPMPermissionType, binheritable: i16) -> ::windows::core::Result<IGPMPermission>;
    fn CreateSearchCriteria(&mut self) -> ::windows::core::Result<IGPMSearchCriteria>;
    fn CreateTrustee(&mut self, bstrtrustee: super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMTrustee>;
    fn GetClientSideExtensions(&mut self) -> ::windows::core::Result<IGPMCSECollection>;
    fn GetConstants(&mut self) -> ::windows::core::Result<IGPMConstants>;
    fn GetMigrationTable(&mut self, bstrmigrationtablepath: super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMMigrationTable>;
    fn CreateMigrationTable(&mut self) -> ::windows::core::Result<IGPMMigrationTable>;
    fn InitializeReporting(&mut self, bstradmpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMVtbl {
        unsafe extern "system" fn GetDomain<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomaincontroller: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ldcflags: i32, pigpmdomain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDomain(::core::mem::transmute_copy(&bstrdomain), ::core::mem::transmute_copy(&bstrdomaincontroller), ::core::mem::transmute_copy(&ldcflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pigpmdomain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackupDir<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pigpmbackupdir: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackupDir(::core::mem::transmute_copy(&bstrbackupdir)) {
                ::core::result::Result::Ok(ok__) => {
                    *pigpmbackupdir = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSitesContainer<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrforest: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomaincontroller: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ldcflags: i32, ppigpmsitescontainer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSitesContainer(::core::mem::transmute_copy(&bstrforest), ::core::mem::transmute_copy(&bstrdomain), ::core::mem::transmute_copy(&bstrdomaincontroller), ::core::mem::transmute_copy(&ldcflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmsitescontainer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRSOP<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmrsopmode: GPMRSOPMode, bstrnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, ppigpmrsop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRSOP(::core::mem::transmute_copy(&gpmrsopmode), ::core::mem::transmute_copy(&bstrnamespace), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmrsop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePermission<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, perm: GPMPermissionType, binheritable: i16, ppperm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePermission(::core::mem::transmute_copy(&bstrtrustee), ::core::mem::transmute_copy(&perm), ::core::mem::transmute_copy(&binheritable)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppperm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSearchCriteria<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmsearchcriteria: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSearchCriteria() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmsearchcriteria = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTrustee<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmtrustee: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTrustee(::core::mem::transmute_copy(&bstrtrustee)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmtrustee = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClientSideExtensions<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmcsecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClientSideExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmcsecollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstants<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmconstants: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConstants() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmconstants = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMigrationTable<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmigrationtablepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmigrationtable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMigrationTable(::core::mem::transmute_copy(&bstrmigrationtablepath)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmigrationtable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMigrationTable<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmigrationtable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMigrationTable() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmigrationtable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeReporting<Impl: IGPMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeReporting(::core::mem::transmute_copy(&bstradmpath)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDomain: GetDomain::<Impl, IMPL_OFFSET>,
            GetBackupDir: GetBackupDir::<Impl, IMPL_OFFSET>,
            GetSitesContainer: GetSitesContainer::<Impl, IMPL_OFFSET>,
            GetRSOP: GetRSOP::<Impl, IMPL_OFFSET>,
            CreatePermission: CreatePermission::<Impl, IMPL_OFFSET>,
            CreateSearchCriteria: CreateSearchCriteria::<Impl, IMPL_OFFSET>,
            CreateTrustee: CreateTrustee::<Impl, IMPL_OFFSET>,
            GetClientSideExtensions: GetClientSideExtensions::<Impl, IMPL_OFFSET>,
            GetConstants: GetConstants::<Impl, IMPL_OFFSET>,
            GetMigrationTable: GetMigrationTable::<Impl, IMPL_OFFSET>,
            CreateMigrationTable: CreateMigrationTable::<Impl, IMPL_OFFSET>,
            InitializeReporting: InitializeReporting::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPM as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPM2Impl: Sized + IDispatchImpl + IGPMImpl {
    fn GetBackupDirEx(&mut self, bstrbackupdir: super::super::Foundation::BSTR, backupdirtype: GPMBackupType) -> ::windows::core::Result<IGPMBackupDirEx>;
    fn InitializeReportingEx(&mut self, bstradmpath: super::super::Foundation::BSTR, reportingoptions: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPM2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPM2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPM2Vtbl {
        unsafe extern "system" fn GetBackupDirEx<Impl: IGPM2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, backupdirtype: GPMBackupType, ppigpmbackupdirex: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackupDirEx(::core::mem::transmute_copy(&bstrbackupdir), ::core::mem::transmute_copy(&backupdirtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmbackupdirex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeReportingEx<Impl: IGPM2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, reportingoptions: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeReportingEx(::core::mem::transmute_copy(&bstradmpath), ::core::mem::transmute_copy(&reportingoptions)).into()
        }
        Self {
            base: IGPMVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetBackupDirEx: GetBackupDirEx::<Impl, IMPL_OFFSET>,
            InitializeReportingEx: InitializeReportingEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPM2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMAsyncCancelImpl: Sized + IDispatchImpl {
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMAsyncCancelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMAsyncCancelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMAsyncCancelVtbl {
        unsafe extern "system" fn Cancel<Impl: IGPMAsyncCancelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Cancel: Cancel::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMAsyncCancel as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMAsyncProgressImpl: Sized + IDispatchImpl {
    fn Status(&mut self, lprogressnumerator: i32, lprogressdenominator: i32, hrstatus: ::windows::core::HRESULT, presult: *const super::Com::VARIANT, ppigpmstatusmsgcollection: ::core::option::Option<IGPMStatusMsgCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMAsyncProgressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMAsyncProgressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMAsyncProgressVtbl {
        unsafe extern "system" fn Status<Impl: IGPMAsyncProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprogressnumerator: i32, lprogressdenominator: i32, hrstatus: ::windows::core::HRESULT, presult: *const super::Com::VARIANT, ppigpmstatusmsgcollection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Status(::core::mem::transmute_copy(&lprogressnumerator), ::core::mem::transmute_copy(&lprogressdenominator), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&presult), ::core::mem::transmute(&ppigpmstatusmsgcollection)).into()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Status: Status::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMAsyncProgress as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMBackupImpl: Sized + IDispatchImpl {
    fn ID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GPOID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GPODomain(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GPODisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Timestamp(&mut self) -> ::windows::core::Result<f64>;
    fn Comment(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BackupDir(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn GenerateReport(&mut self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn GenerateReportToFile(&mut self, gpmreporttype: GPMReportType, bstrtargetfilepath: super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMBackupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMBackupVtbl {
        unsafe extern "system" fn ID<Impl: IGPMBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ID() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GPOID<Impl: IGPMBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GPOID() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GPODomain<Impl: IGPMBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GPODomain() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GPODisplayName<Impl: IGPMBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GPODisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: IGPMBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comment<Impl: IGPMBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Comment() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackupDir<Impl: IGPMBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackupDir() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IGPMBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn GenerateReport<Impl: IGPMBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateReport(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn GenerateReportToFile<Impl: IGPMBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateReportToFile(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&bstrtargetfilepath)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ID: ID::<Impl, IMPL_OFFSET>,
            GPOID: GPOID::<Impl, IMPL_OFFSET>,
            GPODomain: GPODomain::<Impl, IMPL_OFFSET>,
            GPODisplayName: GPODisplayName::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            Comment: Comment::<Impl, IMPL_OFFSET>,
            BackupDir: BackupDir::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            GenerateReport: GenerateReport::<Impl, IMPL_OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMBackup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMBackupCollectionImpl: Sized + IDispatchImpl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMBackupCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackupCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMBackupCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IGPMBackupCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IGPMBackupCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IGPMBackupCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmbackup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmbackup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMBackupCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMBackupDirImpl: Sized + IDispatchImpl {
    fn BackupDirectory(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetBackup(&mut self, bstrid: super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMBackup>;
    fn SearchBackups(&mut self, pigpmsearchcriteria: ::core::option::Option<IGPMSearchCriteria>) -> ::windows::core::Result<IGPMBackupCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMBackupDirVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackupDirImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMBackupDirVtbl {
        unsafe extern "system" fn BackupDirectory<Impl: IGPMBackupDirImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackupDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackup<Impl: IGPMBackupDirImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppbackup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackup(::core::mem::transmute_copy(&bstrid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbackup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchBackups<Impl: IGPMBackupDirImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmbackupcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchBackups(::core::mem::transmute(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmbackupcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BackupDirectory: BackupDirectory::<Impl, IMPL_OFFSET>,
            GetBackup: GetBackup::<Impl, IMPL_OFFSET>,
            SearchBackups: SearchBackups::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMBackupDir as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMBackupDirExImpl: Sized + IDispatchImpl {
    fn BackupDir(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BackupType(&mut self) -> ::windows::core::Result<GPMBackupType>;
    fn GetBackup(&mut self, bstrid: super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SearchBackups(&mut self, pigpmsearchcriteria: ::core::option::Option<IGPMSearchCriteria>) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMBackupDirExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackupDirExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMBackupDirExVtbl {
        unsafe extern "system" fn BackupDir<Impl: IGPMBackupDirExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbackupdir: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackupDir() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrbackupdir = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackupType<Impl: IGPMBackupDirExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgpmbackuptype: *mut GPMBackupType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackupType() {
                ::core::result::Result::Ok(ok__) => {
                    *pgpmbackuptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackup<Impl: IGPMBackupDirExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarbackup: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackup(::core::mem::transmute_copy(&bstrid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbackup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchBackups<Impl: IGPMBackupDirExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, pvarbackupcollection: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchBackups(::core::mem::transmute(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbackupcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BackupDir: BackupDir::<Impl, IMPL_OFFSET>,
            BackupType: BackupType::<Impl, IMPL_OFFSET>,
            GetBackup: GetBackup::<Impl, IMPL_OFFSET>,
            SearchBackups: SearchBackups::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMBackupDirEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMCSECollectionImpl: Sized + IDispatchImpl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMCSECollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMCSECollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMCSECollectionVtbl {
        unsafe extern "system" fn Count<Impl: IGPMCSECollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IGPMCSECollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IGPMCSECollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmcses: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmcses = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMCSECollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMClientSideExtensionImpl: Sized + IDispatchImpl {
    fn ID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsUserEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn IsComputerEnabled(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMClientSideExtensionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMClientSideExtensionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMClientSideExtensionVtbl {
        unsafe extern "system" fn ID<Impl: IGPMClientSideExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ID() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IGPMClientSideExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUserEnabled<Impl: IGPMClientSideExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUserEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pvbenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsComputerEnabled<Impl: IGPMClientSideExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsComputerEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pvbenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ID: ID::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            IsUserEnabled: IsUserEnabled::<Impl, IMPL_OFFSET>,
            IsComputerEnabled: IsComputerEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMClientSideExtension as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMConstantsImpl: Sized + IDispatchImpl {
    fn PermGPOApply(&mut self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermGPORead(&mut self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermGPOEdit(&mut self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermGPOEditSecurityAndDelete(&mut self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermGPOCustom(&mut self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermWMIFilterEdit(&mut self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermWMIFilterFullControl(&mut self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermWMIFilterCustom(&mut self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermSOMLink(&mut self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermSOMLogging(&mut self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermSOMPlanning(&mut self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermSOMGPOCreate(&mut self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermSOMWMICreate(&mut self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermSOMWMIFullControl(&mut self) -> ::windows::core::Result<GPMPermissionType>;
    fn SearchPropertyGPOPermissions(&mut self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOEffectivePermissions(&mut self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyGPODisplayName(&mut self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOWMIFilter(&mut self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOID(&mut self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOComputerExtensions(&mut self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOUserExtensions(&mut self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertySOMLinks(&mut self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyGPODomain(&mut self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyBackupMostRecent(&mut self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchOpEquals(&mut self) -> ::windows::core::Result<GPMSearchOperation>;
    fn SearchOpContains(&mut self) -> ::windows::core::Result<GPMSearchOperation>;
    fn SearchOpNotContains(&mut self) -> ::windows::core::Result<GPMSearchOperation>;
    fn SearchOpNotEquals(&mut self) -> ::windows::core::Result<GPMSearchOperation>;
    fn UsePDC(&mut self) -> ::windows::core::Result<i32>;
    fn UseAnyDC(&mut self) -> ::windows::core::Result<i32>;
    fn DoNotUseW2KDC(&mut self) -> ::windows::core::Result<i32>;
    fn SOMSite(&mut self) -> ::windows::core::Result<GPMSOMType>;
    fn SOMDomain(&mut self) -> ::windows::core::Result<GPMSOMType>;
    fn SOMOU(&mut self) -> ::windows::core::Result<GPMSOMType>;
    fn SecurityFlags(&mut self, vbowner: i16, vbgroup: i16, vbdacl: i16, vbsacl: i16) -> ::windows::core::Result<i32>;
    fn DoNotValidateDC(&mut self) -> ::windows::core::Result<i32>;
    fn ReportHTML(&mut self) -> ::windows::core::Result<GPMReportType>;
    fn ReportXML(&mut self) -> ::windows::core::Result<GPMReportType>;
    fn RSOPModeUnknown(&mut self) -> ::windows::core::Result<GPMRSOPMode>;
    fn RSOPModePlanning(&mut self) -> ::windows::core::Result<GPMRSOPMode>;
    fn RSOPModeLogging(&mut self) -> ::windows::core::Result<GPMRSOPMode>;
    fn EntryTypeUser(&mut self) -> ::windows::core::Result<GPMEntryType>;
    fn EntryTypeComputer(&mut self) -> ::windows::core::Result<GPMEntryType>;
    fn EntryTypeLocalGroup(&mut self) -> ::windows::core::Result<GPMEntryType>;
    fn EntryTypeGlobalGroup(&mut self) -> ::windows::core::Result<GPMEntryType>;
    fn EntryTypeUniversalGroup(&mut self) -> ::windows::core::Result<GPMEntryType>;
    fn EntryTypeUNCPath(&mut self) -> ::windows::core::Result<GPMEntryType>;
    fn EntryTypeUnknown(&mut self) -> ::windows::core::Result<GPMEntryType>;
    fn DestinationOptionSameAsSource(&mut self) -> ::windows::core::Result<GPMDestinationOption>;
    fn DestinationOptionNone(&mut self) -> ::windows::core::Result<GPMDestinationOption>;
    fn DestinationOptionByRelativeName(&mut self) -> ::windows::core::Result<GPMDestinationOption>;
    fn DestinationOptionSet(&mut self) -> ::windows::core::Result<GPMDestinationOption>;
    fn MigrationTableOnly(&mut self) -> ::windows::core::Result<i32>;
    fn ProcessSecurity(&mut self) -> ::windows::core::Result<i32>;
    fn RsopLoggingNoComputer(&mut self) -> ::windows::core::Result<i32>;
    fn RsopLoggingNoUser(&mut self) -> ::windows::core::Result<i32>;
    fn RsopPlanningAssumeSlowLink(&mut self) -> ::windows::core::Result<i32>;
    fn RsopPlanningLoopbackOption(&mut self, vbmerge: i16) -> ::windows::core::Result<i32>;
    fn RsopPlanningAssumeUserWQLFilterTrue(&mut self) -> ::windows::core::Result<i32>;
    fn RsopPlanningAssumeCompWQLFilterTrue(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMConstantsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstantsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMConstantsVtbl {
        unsafe extern "system" fn PermGPOApply<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermGPOApply() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermGPORead<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermGPORead() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermGPOEdit<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermGPOEdit() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermGPOEditSecurityAndDelete<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermGPOEditSecurityAndDelete() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermGPOCustom<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermGPOCustom() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermWMIFilterEdit<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermWMIFilterEdit() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermWMIFilterFullControl<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermWMIFilterFullControl() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermWMIFilterCustom<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermWMIFilterCustom() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMLink<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermSOMLink() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMLogging<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermSOMLogging() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMPlanning<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermSOMPlanning() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMGPOCreate<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermSOMGPOCreate() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMWMICreate<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermSOMWMICreate() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMWMIFullControl<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermSOMWMIFullControl() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOPermissions<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchPropertyGPOPermissions() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOEffectivePermissions<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchPropertyGPOEffectivePermissions() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPODisplayName<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchPropertyGPODisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOWMIFilter<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchPropertyGPOWMIFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOID<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchPropertyGPOID() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOComputerExtensions<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchPropertyGPOComputerExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOUserExtensions<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchPropertyGPOUserExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertySOMLinks<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchPropertySOMLinks() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPODomain<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchPropertyGPODomain() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyBackupMostRecent<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchPropertyBackupMostRecent() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchOpEquals<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchOpEquals() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchOpContains<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchOpContains() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchOpNotContains<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchOpNotContains() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchOpNotEquals<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchOpNotEquals() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsePDC<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsePDC() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseAnyDC<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseAnyDC() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoNotUseW2KDC<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoNotUseW2KDC() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SOMSite<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SOMSite() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SOMDomain<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SOMDomain() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SOMOU<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SOMOU() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecurityFlags<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbowner: i16, vbgroup: i16, vbdacl: i16, vbsacl: i16, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecurityFlags(::core::mem::transmute_copy(&vbowner), ::core::mem::transmute_copy(&vbgroup), ::core::mem::transmute_copy(&vbdacl), ::core::mem::transmute_copy(&vbsacl)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoNotValidateDC<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoNotValidateDC() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportHTML<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMReportType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportHTML() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportXML<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMReportType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportXML() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RSOPModeUnknown<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RSOPModeUnknown() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RSOPModePlanning<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RSOPModePlanning() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RSOPModeLogging<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RSOPModeLogging() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeUser<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EntryTypeUser() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeComputer<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EntryTypeComputer() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeLocalGroup<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EntryTypeLocalGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeGlobalGroup<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EntryTypeGlobalGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeUniversalGroup<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EntryTypeUniversalGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeUNCPath<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EntryTypeUNCPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeUnknown<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EntryTypeUnknown() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationOptionSameAsSource<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationOptionSameAsSource() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationOptionNone<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationOptionNone() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationOptionByRelativeName<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationOptionByRelativeName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationOptionSet<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationOptionSet() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MigrationTableOnly<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MigrationTableOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessSecurity<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessSecurity() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsopLoggingNoComputer<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RsopLoggingNoComputer() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsopLoggingNoUser<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RsopLoggingNoUser() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsopPlanningAssumeSlowLink<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RsopPlanningAssumeSlowLink() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsopPlanningLoopbackOption<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbmerge: i16, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RsopPlanningLoopbackOption(::core::mem::transmute_copy(&vbmerge)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsopPlanningAssumeUserWQLFilterTrue<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RsopPlanningAssumeUserWQLFilterTrue() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsopPlanningAssumeCompWQLFilterTrue<Impl: IGPMConstantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RsopPlanningAssumeCompWQLFilterTrue() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PermGPOApply: PermGPOApply::<Impl, IMPL_OFFSET>,
            PermGPORead: PermGPORead::<Impl, IMPL_OFFSET>,
            PermGPOEdit: PermGPOEdit::<Impl, IMPL_OFFSET>,
            PermGPOEditSecurityAndDelete: PermGPOEditSecurityAndDelete::<Impl, IMPL_OFFSET>,
            PermGPOCustom: PermGPOCustom::<Impl, IMPL_OFFSET>,
            PermWMIFilterEdit: PermWMIFilterEdit::<Impl, IMPL_OFFSET>,
            PermWMIFilterFullControl: PermWMIFilterFullControl::<Impl, IMPL_OFFSET>,
            PermWMIFilterCustom: PermWMIFilterCustom::<Impl, IMPL_OFFSET>,
            PermSOMLink: PermSOMLink::<Impl, IMPL_OFFSET>,
            PermSOMLogging: PermSOMLogging::<Impl, IMPL_OFFSET>,
            PermSOMPlanning: PermSOMPlanning::<Impl, IMPL_OFFSET>,
            PermSOMGPOCreate: PermSOMGPOCreate::<Impl, IMPL_OFFSET>,
            PermSOMWMICreate: PermSOMWMICreate::<Impl, IMPL_OFFSET>,
            PermSOMWMIFullControl: PermSOMWMIFullControl::<Impl, IMPL_OFFSET>,
            SearchPropertyGPOPermissions: SearchPropertyGPOPermissions::<Impl, IMPL_OFFSET>,
            SearchPropertyGPOEffectivePermissions: SearchPropertyGPOEffectivePermissions::<Impl, IMPL_OFFSET>,
            SearchPropertyGPODisplayName: SearchPropertyGPODisplayName::<Impl, IMPL_OFFSET>,
            SearchPropertyGPOWMIFilter: SearchPropertyGPOWMIFilter::<Impl, IMPL_OFFSET>,
            SearchPropertyGPOID: SearchPropertyGPOID::<Impl, IMPL_OFFSET>,
            SearchPropertyGPOComputerExtensions: SearchPropertyGPOComputerExtensions::<Impl, IMPL_OFFSET>,
            SearchPropertyGPOUserExtensions: SearchPropertyGPOUserExtensions::<Impl, IMPL_OFFSET>,
            SearchPropertySOMLinks: SearchPropertySOMLinks::<Impl, IMPL_OFFSET>,
            SearchPropertyGPODomain: SearchPropertyGPODomain::<Impl, IMPL_OFFSET>,
            SearchPropertyBackupMostRecent: SearchPropertyBackupMostRecent::<Impl, IMPL_OFFSET>,
            SearchOpEquals: SearchOpEquals::<Impl, IMPL_OFFSET>,
            SearchOpContains: SearchOpContains::<Impl, IMPL_OFFSET>,
            SearchOpNotContains: SearchOpNotContains::<Impl, IMPL_OFFSET>,
            SearchOpNotEquals: SearchOpNotEquals::<Impl, IMPL_OFFSET>,
            UsePDC: UsePDC::<Impl, IMPL_OFFSET>,
            UseAnyDC: UseAnyDC::<Impl, IMPL_OFFSET>,
            DoNotUseW2KDC: DoNotUseW2KDC::<Impl, IMPL_OFFSET>,
            SOMSite: SOMSite::<Impl, IMPL_OFFSET>,
            SOMDomain: SOMDomain::<Impl, IMPL_OFFSET>,
            SOMOU: SOMOU::<Impl, IMPL_OFFSET>,
            SecurityFlags: SecurityFlags::<Impl, IMPL_OFFSET>,
            DoNotValidateDC: DoNotValidateDC::<Impl, IMPL_OFFSET>,
            ReportHTML: ReportHTML::<Impl, IMPL_OFFSET>,
            ReportXML: ReportXML::<Impl, IMPL_OFFSET>,
            RSOPModeUnknown: RSOPModeUnknown::<Impl, IMPL_OFFSET>,
            RSOPModePlanning: RSOPModePlanning::<Impl, IMPL_OFFSET>,
            RSOPModeLogging: RSOPModeLogging::<Impl, IMPL_OFFSET>,
            EntryTypeUser: EntryTypeUser::<Impl, IMPL_OFFSET>,
            EntryTypeComputer: EntryTypeComputer::<Impl, IMPL_OFFSET>,
            EntryTypeLocalGroup: EntryTypeLocalGroup::<Impl, IMPL_OFFSET>,
            EntryTypeGlobalGroup: EntryTypeGlobalGroup::<Impl, IMPL_OFFSET>,
            EntryTypeUniversalGroup: EntryTypeUniversalGroup::<Impl, IMPL_OFFSET>,
            EntryTypeUNCPath: EntryTypeUNCPath::<Impl, IMPL_OFFSET>,
            EntryTypeUnknown: EntryTypeUnknown::<Impl, IMPL_OFFSET>,
            DestinationOptionSameAsSource: DestinationOptionSameAsSource::<Impl, IMPL_OFFSET>,
            DestinationOptionNone: DestinationOptionNone::<Impl, IMPL_OFFSET>,
            DestinationOptionByRelativeName: DestinationOptionByRelativeName::<Impl, IMPL_OFFSET>,
            DestinationOptionSet: DestinationOptionSet::<Impl, IMPL_OFFSET>,
            MigrationTableOnly: MigrationTableOnly::<Impl, IMPL_OFFSET>,
            ProcessSecurity: ProcessSecurity::<Impl, IMPL_OFFSET>,
            RsopLoggingNoComputer: RsopLoggingNoComputer::<Impl, IMPL_OFFSET>,
            RsopLoggingNoUser: RsopLoggingNoUser::<Impl, IMPL_OFFSET>,
            RsopPlanningAssumeSlowLink: RsopPlanningAssumeSlowLink::<Impl, IMPL_OFFSET>,
            RsopPlanningLoopbackOption: RsopPlanningLoopbackOption::<Impl, IMPL_OFFSET>,
            RsopPlanningAssumeUserWQLFilterTrue: RsopPlanningAssumeUserWQLFilterTrue::<Impl, IMPL_OFFSET>,
            RsopPlanningAssumeCompWQLFilterTrue: RsopPlanningAssumeCompWQLFilterTrue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMConstants as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMConstants2Impl: Sized + IDispatchImpl + IGPMConstantsImpl {
    fn BackupTypeGPO(&mut self) -> ::windows::core::Result<GPMBackupType>;
    fn BackupTypeStarterGPO(&mut self) -> ::windows::core::Result<GPMBackupType>;
    fn StarterGPOTypeSystem(&mut self) -> ::windows::core::Result<GPMStarterGPOType>;
    fn StarterGPOTypeCustom(&mut self) -> ::windows::core::Result<GPMStarterGPOType>;
    fn SearchPropertyStarterGPOPermissions(&mut self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyStarterGPOEffectivePermissions(&mut self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyStarterGPODisplayName(&mut self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyStarterGPOID(&mut self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyStarterGPODomain(&mut self) -> ::windows::core::Result<GPMSearchProperty>;
    fn PermStarterGPORead(&mut self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermStarterGPOEdit(&mut self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermStarterGPOFullControl(&mut self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermStarterGPOCustom(&mut self) -> ::windows::core::Result<GPMPermissionType>;
    fn ReportLegacy(&mut self) -> ::windows::core::Result<GPMReportingOptions>;
    fn ReportComments(&mut self) -> ::windows::core::Result<GPMReportingOptions>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMConstants2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMConstants2Vtbl {
        unsafe extern "system" fn BackupTypeGPO<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMBackupType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackupTypeGPO() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackupTypeStarterGPO<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMBackupType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackupTypeStarterGPO() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StarterGPOTypeSystem<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StarterGPOTypeSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StarterGPOTypeCustom<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StarterGPOTypeCustom() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPOPermissions<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchPropertyStarterGPOPermissions() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPOEffectivePermissions<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchPropertyStarterGPOEffectivePermissions() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPODisplayName<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchPropertyStarterGPODisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPOID<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchPropertyStarterGPOID() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPODomain<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchPropertyStarterGPODomain() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermStarterGPORead<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermStarterGPORead() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermStarterGPOEdit<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermStarterGPOEdit() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermStarterGPOFullControl<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermStarterGPOFullControl() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermStarterGPOCustom<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermStarterGPOCustom() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportLegacy<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMReportingOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportLegacy() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportComments<Impl: IGPMConstants2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMReportingOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportComments() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IGPMConstantsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BackupTypeGPO: BackupTypeGPO::<Impl, IMPL_OFFSET>,
            BackupTypeStarterGPO: BackupTypeStarterGPO::<Impl, IMPL_OFFSET>,
            StarterGPOTypeSystem: StarterGPOTypeSystem::<Impl, IMPL_OFFSET>,
            StarterGPOTypeCustom: StarterGPOTypeCustom::<Impl, IMPL_OFFSET>,
            SearchPropertyStarterGPOPermissions: SearchPropertyStarterGPOPermissions::<Impl, IMPL_OFFSET>,
            SearchPropertyStarterGPOEffectivePermissions: SearchPropertyStarterGPOEffectivePermissions::<Impl, IMPL_OFFSET>,
            SearchPropertyStarterGPODisplayName: SearchPropertyStarterGPODisplayName::<Impl, IMPL_OFFSET>,
            SearchPropertyStarterGPOID: SearchPropertyStarterGPOID::<Impl, IMPL_OFFSET>,
            SearchPropertyStarterGPODomain: SearchPropertyStarterGPODomain::<Impl, IMPL_OFFSET>,
            PermStarterGPORead: PermStarterGPORead::<Impl, IMPL_OFFSET>,
            PermStarterGPOEdit: PermStarterGPOEdit::<Impl, IMPL_OFFSET>,
            PermStarterGPOFullControl: PermStarterGPOFullControl::<Impl, IMPL_OFFSET>,
            PermStarterGPOCustom: PermStarterGPOCustom::<Impl, IMPL_OFFSET>,
            ReportLegacy: ReportLegacy::<Impl, IMPL_OFFSET>,
            ReportComments: ReportComments::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMConstants2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMDomainImpl: Sized + IDispatchImpl {
    fn DomainController(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Domain(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreateGPO(&mut self) -> ::windows::core::Result<IGPMGPO>;
    fn GetGPO(&mut self, bstrguid: super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMGPO>;
    fn SearchGPOs(&mut self, pigpmsearchcriteria: ::core::option::Option<IGPMSearchCriteria>) -> ::windows::core::Result<IGPMGPOCollection>;
    fn RestoreGPO(&mut self, pigpmbackup: ::core::option::Option<IGPMBackup>, ldcflags: i32, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn GetSOM(&mut self, bstrpath: super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMSOM>;
    fn SearchSOMs(&mut self, pigpmsearchcriteria: ::core::option::Option<IGPMSearchCriteria>) -> ::windows::core::Result<IGPMSOMCollection>;
    fn GetWMIFilter(&mut self, bstrpath: super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMWMIFilter>;
    fn SearchWMIFilters(&mut self, pigpmsearchcriteria: ::core::option::Option<IGPMSearchCriteria>) -> ::windows::core::Result<IGPMWMIFilterCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMDomainVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomainImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMDomainVtbl {
        unsafe extern "system" fn DomainController<Impl: IGPMDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainController() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Domain<Impl: IGPMDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Domain() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGPO<Impl: IGPMDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewgpo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGPO() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnewgpo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGPO<Impl: IGPMDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppgpo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGPO(::core::mem::transmute_copy(&bstrguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgpo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchGPOs<Impl: IGPMDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmgpocollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchGPOs(::core::mem::transmute(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmgpocollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreGPO<Impl: IGPMDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmbackup: ::windows::core::RawPtr, ldcflags: i32, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestoreGPO(::core::mem::transmute(&pigpmbackup), ::core::mem::transmute_copy(&ldcflags), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn GetSOM<Impl: IGPMDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsom: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSOM(::core::mem::transmute_copy(&bstrpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsom = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchSOMs<Impl: IGPMDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmsomcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchSOMs(::core::mem::transmute(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmsomcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWMIFilter<Impl: IGPMDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwmifilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWMIFilter(::core::mem::transmute_copy(&bstrpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwmifilter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchWMIFilters<Impl: IGPMDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmwmifiltercollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchWMIFilters(::core::mem::transmute(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmwmifiltercollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DomainController: DomainController::<Impl, IMPL_OFFSET>,
            Domain: Domain::<Impl, IMPL_OFFSET>,
            CreateGPO: CreateGPO::<Impl, IMPL_OFFSET>,
            GetGPO: GetGPO::<Impl, IMPL_OFFSET>,
            SearchGPOs: SearchGPOs::<Impl, IMPL_OFFSET>,
            RestoreGPO: RestoreGPO::<Impl, IMPL_OFFSET>,
            GetSOM: GetSOM::<Impl, IMPL_OFFSET>,
            SearchSOMs: SearchSOMs::<Impl, IMPL_OFFSET>,
            GetWMIFilter: GetWMIFilter::<Impl, IMPL_OFFSET>,
            SearchWMIFilters: SearchWMIFilters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMDomain as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMDomain2Impl: Sized + IDispatchImpl + IGPMDomainImpl {
    fn CreateStarterGPO(&mut self) -> ::windows::core::Result<IGPMStarterGPO>;
    fn CreateGPOFromStarterGPO(&mut self, pgpotemplate: ::core::option::Option<IGPMStarterGPO>) -> ::windows::core::Result<IGPMGPO>;
    fn GetStarterGPO(&mut self, bstrguid: super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMStarterGPO>;
    fn SearchStarterGPOs(&mut self, pigpmsearchcriteria: ::core::option::Option<IGPMSearchCriteria>) -> ::windows::core::Result<IGPMStarterGPOCollection>;
    fn LoadStarterGPO(&mut self, bstrloadfile: super::super::Foundation::BSTR, boverwrite: i16, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn RestoreStarterGPO(&mut self, pigpmtmplbackup: ::core::option::Option<IGPMStarterGPOBackup>, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMDomain2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMDomain2Vtbl {
        unsafe extern "system" fn CreateStarterGPO<Impl: IGPMDomain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewtemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStarterGPO() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnewtemplate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGPOFromStarterGPO<Impl: IGPMDomain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgpotemplate: ::windows::core::RawPtr, ppnewgpo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGPOFromStarterGPO(::core::mem::transmute(&pgpotemplate)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnewgpo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStarterGPO<Impl: IGPMDomain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStarterGPO(::core::mem::transmute_copy(&bstrguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptemplate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchStarterGPOs<Impl: IGPMDomain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmtemplatecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchStarterGPOs(::core::mem::transmute(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmtemplatecollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadStarterGPO<Impl: IGPMDomain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrloadfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, boverwrite: i16, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadStarterGPO(::core::mem::transmute_copy(&bstrloadfile), ::core::mem::transmute_copy(&boverwrite), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn RestoreStarterGPO<Impl: IGPMDomain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmtmplbackup: ::windows::core::RawPtr, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestoreStarterGPO(::core::mem::transmute(&pigpmtmplbackup), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        Self {
            base: IGPMDomainVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateStarterGPO: CreateStarterGPO::<Impl, IMPL_OFFSET>,
            CreateGPOFromStarterGPO: CreateGPOFromStarterGPO::<Impl, IMPL_OFFSET>,
            GetStarterGPO: GetStarterGPO::<Impl, IMPL_OFFSET>,
            SearchStarterGPOs: SearchStarterGPOs::<Impl, IMPL_OFFSET>,
            LoadStarterGPO: LoadStarterGPO::<Impl, IMPL_OFFSET>,
            RestoreStarterGPO: RestoreStarterGPO::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMDomain2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMDomain3Impl: Sized + IDispatchImpl + IGPMDomainImpl + IGPMDomain2Impl {
    fn GenerateReport(&mut self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn InfrastructureDC(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetInfrastructureDC(&mut self, newval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetInfrastructureFlags(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMDomain3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMDomain3Vtbl {
        unsafe extern "system" fn GenerateReport<Impl: IGPMDomain3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateReport(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn InfrastructureDC<Impl: IGPMDomain3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InfrastructureDC() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInfrastructureDC<Impl: IGPMDomain3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInfrastructureDC(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn SetInfrastructureFlags<Impl: IGPMDomain3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInfrastructureFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base: IGPMDomain2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GenerateReport: GenerateReport::<Impl, IMPL_OFFSET>,
            InfrastructureDC: InfrastructureDC::<Impl, IMPL_OFFSET>,
            SetInfrastructureDC: SetInfrastructureDC::<Impl, IMPL_OFFSET>,
            SetInfrastructureFlags: SetInfrastructureFlags::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMDomain3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMGPOImpl: Sized + IDispatchImpl {
    fn DisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDisplayName(&mut self, newval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DomainName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreationTime(&mut self) -> ::windows::core::Result<f64>;
    fn ModificationTime(&mut self) -> ::windows::core::Result<f64>;
    fn UserDSVersionNumber(&mut self) -> ::windows::core::Result<i32>;
    fn ComputerDSVersionNumber(&mut self) -> ::windows::core::Result<i32>;
    fn UserSysvolVersionNumber(&mut self) -> ::windows::core::Result<i32>;
    fn ComputerSysvolVersionNumber(&mut self) -> ::windows::core::Result<i32>;
    fn GetWMIFilter(&mut self) -> ::windows::core::Result<IGPMWMIFilter>;
    fn SetWMIFilter(&mut self, pigpmwmifilter: ::core::option::Option<IGPMWMIFilter>) -> ::windows::core::Result<()>;
    fn SetUserEnabled(&mut self, vbenabled: i16) -> ::windows::core::Result<()>;
    fn SetComputerEnabled(&mut self, vbenabled: i16) -> ::windows::core::Result<()>;
    fn IsUserEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn IsComputerEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn GetSecurityInfo(&mut self) -> ::windows::core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(&mut self, psecurityinfo: ::core::option::Option<IGPMSecurityInfo>) -> ::windows::core::Result<()>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn Backup(&mut self, bstrbackupdir: super::super::Foundation::BSTR, bstrcomment: super::super::Foundation::BSTR, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn Import(&mut self, lflags: i32, pigpmbackup: ::core::option::Option<IGPMBackup>, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn GenerateReport(&mut self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn GenerateReportToFile(&mut self, gpmreporttype: GPMReportType, bstrtargetfilepath: super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMResult>;
    fn CopyTo(&mut self, lflags: i32, pigpmdomain: ::core::option::Option<IGPMDomain>, pvarnewdisplayname: *const super::Com::VARIANT, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn SetSecurityDescriptor(&mut self, lflags: i32, psd: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn GetSecurityDescriptor(&mut self, lflags: i32) -> ::windows::core::Result<super::Com::IDispatch>;
    fn IsACLConsistent(&mut self) -> ::windows::core::Result<i16>;
    fn MakeACLConsistent(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPOVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMGPOVtbl {
        unsafe extern "system" fn DisplayName<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Path<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ID<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ID() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainName<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreationTime<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModificationTime<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModificationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDSVersionNumber<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserDSVersionNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerDSVersionNumber<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComputerDSVersionNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserSysvolVersionNumber<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserSysvolVersionNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerSysvolVersionNumber<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComputerSysvolVersionNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWMIFilter<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmwmifilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWMIFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmwmifilter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWMIFilter<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmwmifilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWMIFilter(::core::mem::transmute(&pigpmwmifilter)).into()
        }
        unsafe extern "system" fn SetUserEnabled<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserEnabled(::core::mem::transmute_copy(&vbenabled)).into()
        }
        unsafe extern "system" fn SetComputerEnabled<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComputerEnabled(::core::mem::transmute_copy(&vbenabled)).into()
        }
        unsafe extern "system" fn IsUserEnabled<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUserEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pvbenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsComputerEnabled<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsComputerEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pvbenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityInfo<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecurityInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsecurityinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityInfo<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecurityinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurityInfo(::core::mem::transmute(&psecurityinfo)).into()
        }
        unsafe extern "system" fn Delete<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Backup<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrcomment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Backup(::core::mem::transmute_copy(&bstrbackupdir), ::core::mem::transmute_copy(&bstrcomment), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn Import<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pigpmbackup: ::windows::core::RawPtr, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Import(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pigpmbackup), ::core::mem::transmute_copy(&pvarmigrationtable), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn GenerateReport<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateReport(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn GenerateReportToFile<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateReportToFile(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&bstrtargetfilepath)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTo<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pigpmdomain: ::windows::core::RawPtr, pvarnewdisplayname: *const super::Com::VARIANT, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyTo(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pigpmdomain), ::core::mem::transmute_copy(&pvarnewdisplayname), ::core::mem::transmute_copy(&pvarmigrationtable), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn SetSecurityDescriptor<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, psd: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurityDescriptor(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&psd)).into()
        }
        unsafe extern "system" fn GetSecurityDescriptor<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, ppsd: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecurityDescriptor(::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsACLConsistent<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbconsistent: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsACLConsistent() {
                ::core::result::Result::Ok(ok__) => {
                    *pvbconsistent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeACLConsistent<Impl: IGPMGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MakeACLConsistent().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            ID: ID::<Impl, IMPL_OFFSET>,
            DomainName: DomainName::<Impl, IMPL_OFFSET>,
            CreationTime: CreationTime::<Impl, IMPL_OFFSET>,
            ModificationTime: ModificationTime::<Impl, IMPL_OFFSET>,
            UserDSVersionNumber: UserDSVersionNumber::<Impl, IMPL_OFFSET>,
            ComputerDSVersionNumber: ComputerDSVersionNumber::<Impl, IMPL_OFFSET>,
            UserSysvolVersionNumber: UserSysvolVersionNumber::<Impl, IMPL_OFFSET>,
            ComputerSysvolVersionNumber: ComputerSysvolVersionNumber::<Impl, IMPL_OFFSET>,
            GetWMIFilter: GetWMIFilter::<Impl, IMPL_OFFSET>,
            SetWMIFilter: SetWMIFilter::<Impl, IMPL_OFFSET>,
            SetUserEnabled: SetUserEnabled::<Impl, IMPL_OFFSET>,
            SetComputerEnabled: SetComputerEnabled::<Impl, IMPL_OFFSET>,
            IsUserEnabled: IsUserEnabled::<Impl, IMPL_OFFSET>,
            IsComputerEnabled: IsComputerEnabled::<Impl, IMPL_OFFSET>,
            GetSecurityInfo: GetSecurityInfo::<Impl, IMPL_OFFSET>,
            SetSecurityInfo: SetSecurityInfo::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Backup: Backup::<Impl, IMPL_OFFSET>,
            Import: Import::<Impl, IMPL_OFFSET>,
            GenerateReport: GenerateReport::<Impl, IMPL_OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Impl, IMPL_OFFSET>,
            CopyTo: CopyTo::<Impl, IMPL_OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            GetSecurityDescriptor: GetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            IsACLConsistent: IsACLConsistent::<Impl, IMPL_OFFSET>,
            MakeACLConsistent: MakeACLConsistent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMGPO as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMGPO2Impl: Sized + IDispatchImpl + IGPMGPOImpl {
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, newval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPO2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMGPO2Vtbl {
        unsafe extern "system" fn Description<Impl: IGPMGPO2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IGPMGPO2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base: IGPMGPOVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMGPO2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMGPO3Impl: Sized + IDispatchImpl + IGPMGPOImpl + IGPMGPO2Impl {
    fn InfrastructureDC(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetInfrastructureDC(&mut self, newval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetInfrastructureFlags(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPO3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMGPO3Vtbl {
        unsafe extern "system" fn InfrastructureDC<Impl: IGPMGPO3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InfrastructureDC() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInfrastructureDC<Impl: IGPMGPO3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInfrastructureDC(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn SetInfrastructureFlags<Impl: IGPMGPO3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInfrastructureFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base: IGPMGPO2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InfrastructureDC: InfrastructureDC::<Impl, IMPL_OFFSET>,
            SetInfrastructureDC: SetInfrastructureDC::<Impl, IMPL_OFFSET>,
            SetInfrastructureFlags: SetInfrastructureFlags::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMGPO3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMGPOCollectionImpl: Sized + IDispatchImpl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPOCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMGPOCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IGPMGPOCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IGPMGPOCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IGPMGPOCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmgpos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmgpos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMGPOCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMGPOLinkImpl: Sized + IDispatchImpl {
    fn GPOID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GPODomain(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Enabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&mut self, newval: i16) -> ::windows::core::Result<()>;
    fn Enforced(&mut self) -> ::windows::core::Result<i16>;
    fn SetEnforced(&mut self, newval: i16) -> ::windows::core::Result<()>;
    fn SOMLinkOrder(&mut self) -> ::windows::core::Result<i32>;
    fn SOM(&mut self) -> ::windows::core::Result<IGPMSOM>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPOLinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOLinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMGPOLinkVtbl {
        unsafe extern "system" fn GPOID<Impl: IGPMGPOLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GPOID() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GPODomain<Impl: IGPMGPOLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GPODomain() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IGPMGPOLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IGPMGPOLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Enforced<Impl: IGPMGPOLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enforced() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnforced<Impl: IGPMGPOLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnforced(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn SOMLinkOrder<Impl: IGPMGPOLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SOMLinkOrder() {
                ::core::result::Result::Ok(ok__) => {
                    *lval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SOM<Impl: IGPMGPOLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmsom: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SOM() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmsom = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IGPMGPOLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GPOID: GPOID::<Impl, IMPL_OFFSET>,
            GPODomain: GPODomain::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            Enforced: Enforced::<Impl, IMPL_OFFSET>,
            SetEnforced: SetEnforced::<Impl, IMPL_OFFSET>,
            SOMLinkOrder: SOMLinkOrder::<Impl, IMPL_OFFSET>,
            SOM: SOM::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMGPOLink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMGPOLinksCollectionImpl: Sized + IDispatchImpl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPOLinksCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOLinksCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMGPOLinksCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IGPMGPOLinksCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IGPMGPOLinksCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IGPMGPOLinksCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmlinks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmlinks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMGPOLinksCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMMapEntryImpl: Sized + IDispatchImpl {
    fn Source(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Destination(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DestinationOption(&mut self) -> ::windows::core::Result<GPMDestinationOption>;
    fn EntryType(&mut self) -> ::windows::core::Result<GPMEntryType>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMMapEntryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMapEntryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMMapEntryVtbl {
        unsafe extern "system" fn Source<Impl: IGPMMapEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destination<Impl: IGPMMapEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdestination: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Destination() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdestination = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationOption<Impl: IGPMMapEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgpmdestoption: *mut GPMDestinationOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationOption() {
                ::core::result::Result::Ok(ok__) => {
                    *pgpmdestoption = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryType<Impl: IGPMMapEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgpmentrytype: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EntryType() {
                ::core::result::Result::Ok(ok__) => {
                    *pgpmentrytype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Source: Source::<Impl, IMPL_OFFSET>,
            Destination: Destination::<Impl, IMPL_OFFSET>,
            DestinationOption: DestinationOption::<Impl, IMPL_OFFSET>,
            EntryType: EntryType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMMapEntry as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMMapEntryCollectionImpl: Sized + IDispatchImpl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMMapEntryCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMapEntryCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMMapEntryCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IGPMMapEntryCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IGPMMapEntryCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IGPMMapEntryCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMMapEntryCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMMigrationTableImpl: Sized + IDispatchImpl {
    fn Save(&mut self, bstrmigrationtablepath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Add(&mut self, lflags: i32, var: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddEntry(&mut self, bstrsource: super::super::Foundation::BSTR, gpmentrytype: GPMEntryType, pvardestination: *const super::Com::VARIANT) -> ::windows::core::Result<IGPMMapEntry>;
    fn GetEntry(&mut self, bstrsource: super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMMapEntry>;
    fn DeleteEntry(&mut self, bstrsource: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn UpdateDestination(&mut self, bstrsource: super::super::Foundation::BSTR, pvardestination: *const super::Com::VARIANT) -> ::windows::core::Result<IGPMMapEntry>;
    fn Validate(&mut self) -> ::windows::core::Result<IGPMResult>;
    fn GetEntries(&mut self) -> ::windows::core::Result<IGPMMapEntryCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMMigrationTableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMigrationTableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMMigrationTableVtbl {
        unsafe extern "system" fn Save<Impl: IGPMMigrationTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmigrationtablepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save(::core::mem::transmute_copy(&bstrmigrationtablepath)).into()
        }
        unsafe extern "system" fn Add<Impl: IGPMMigrationTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, var: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&var)).into()
        }
        unsafe extern "system" fn AddEntry<Impl: IGPMMigrationTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, gpmentrytype: GPMEntryType, pvardestination: *const super::Com::VARIANT, ppentry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddEntry(::core::mem::transmute_copy(&bstrsource), ::core::mem::transmute_copy(&gpmentrytype), ::core::mem::transmute_copy(&pvardestination)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppentry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntry<Impl: IGPMMigrationTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppentry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEntry(::core::mem::transmute_copy(&bstrsource)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppentry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteEntry<Impl: IGPMMigrationTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteEntry(::core::mem::transmute_copy(&bstrsource)).into()
        }
        unsafe extern "system" fn UpdateDestination<Impl: IGPMMigrationTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvardestination: *const super::Com::VARIANT, ppentry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateDestination(::core::mem::transmute_copy(&bstrsource), ::core::mem::transmute_copy(&pvardestination)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppentry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Validate<Impl: IGPMMigrationTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Validate() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntries<Impl: IGPMMigrationTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppentries: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEntries() {
                ::core::result::Result::Ok(ok__) => {
                    *ppentries = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Save: Save::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            AddEntry: AddEntry::<Impl, IMPL_OFFSET>,
            GetEntry: GetEntry::<Impl, IMPL_OFFSET>,
            DeleteEntry: DeleteEntry::<Impl, IMPL_OFFSET>,
            UpdateDestination: UpdateDestination::<Impl, IMPL_OFFSET>,
            Validate: Validate::<Impl, IMPL_OFFSET>,
            GetEntries: GetEntries::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMMigrationTable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMPermissionImpl: Sized + IDispatchImpl {
    fn Inherited(&mut self) -> ::windows::core::Result<i16>;
    fn Inheritable(&mut self) -> ::windows::core::Result<i16>;
    fn Denied(&mut self) -> ::windows::core::Result<i16>;
    fn Permission(&mut self) -> ::windows::core::Result<GPMPermissionType>;
    fn Trustee(&mut self) -> ::windows::core::Result<IGPMTrustee>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMPermissionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMPermissionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMPermissionVtbl {
        unsafe extern "system" fn Inherited<Impl: IGPMPermissionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Inherited() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Inheritable<Impl: IGPMPermissionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Inheritable() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Denied<Impl: IGPMPermissionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Denied() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Permission<Impl: IGPMPermissionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Permission() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Trustee<Impl: IGPMPermissionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmtrustee: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Trustee() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmtrustee = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Inherited: Inherited::<Impl, IMPL_OFFSET>,
            Inheritable: Inheritable::<Impl, IMPL_OFFSET>,
            Denied: Denied::<Impl, IMPL_OFFSET>,
            Permission: Permission::<Impl, IMPL_OFFSET>,
            Trustee: Trustee::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMPermission as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMRSOPImpl: Sized + IDispatchImpl {
    fn Mode(&mut self) -> ::windows::core::Result<GPMRSOPMode>;
    fn Namespace(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLoggingComputer(&mut self, bstrval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LoggingComputer(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLoggingUser(&mut self, bstrval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LoggingUser(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLoggingFlags(&mut self, lval: i32) -> ::windows::core::Result<()>;
    fn LoggingFlags(&mut self) -> ::windows::core::Result<i32>;
    fn SetPlanningFlags(&mut self, lval: i32) -> ::windows::core::Result<()>;
    fn PlanningFlags(&mut self) -> ::windows::core::Result<i32>;
    fn SetPlanningDomainController(&mut self, bstrval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PlanningDomainController(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPlanningSiteName(&mut self, bstrval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PlanningSiteName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPlanningUser(&mut self, bstrval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PlanningUser(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPlanningUserSOM(&mut self, bstrval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PlanningUserSOM(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPlanningUserWMIFilters(&mut self, varval: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PlanningUserWMIFilters(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetPlanningUserSecurityGroups(&mut self, varval: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PlanningUserSecurityGroups(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetPlanningComputer(&mut self, bstrval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PlanningComputer(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPlanningComputerSOM(&mut self, bstrval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PlanningComputerSOM(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPlanningComputerWMIFilters(&mut self, varval: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PlanningComputerWMIFilters(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetPlanningComputerSecurityGroups(&mut self, varval: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PlanningComputerSecurityGroups(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn LoggingEnumerateUsers(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn CreateQueryResults(&mut self) -> ::windows::core::Result<()>;
    fn ReleaseQueryResults(&mut self) -> ::windows::core::Result<()>;
    fn GenerateReport(&mut self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn GenerateReportToFile(&mut self, gpmreporttype: GPMReportType, bstrtargetfilepath: super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMRSOPVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOPImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMRSOPVtbl {
        unsafe extern "system" fn Mode<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Namespace<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Namespace() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoggingComputer<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLoggingComputer(::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn LoggingComputer<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoggingComputer() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoggingUser<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLoggingUser(::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn LoggingUser<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoggingUser() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoggingFlags<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLoggingFlags(::core::mem::transmute_copy(&lval)).into()
        }
        unsafe extern "system" fn LoggingFlags<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoggingFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *lval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningFlags<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlanningFlags(::core::mem::transmute_copy(&lval)).into()
        }
        unsafe extern "system" fn PlanningFlags<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlanningFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *lval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningDomainController<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlanningDomainController(::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningDomainController<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlanningDomainController() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningSiteName<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlanningSiteName(::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningSiteName<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlanningSiteName() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningUser<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlanningUser(::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningUser<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlanningUser() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningUserSOM<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlanningUserSOM(::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningUserSOM<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlanningUserSOM() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningUserWMIFilters<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlanningUserWMIFilters(::core::mem::transmute_copy(&varval)).into()
        }
        unsafe extern "system" fn PlanningUserWMIFilters<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlanningUserWMIFilters() {
                ::core::result::Result::Ok(ok__) => {
                    *varval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningUserSecurityGroups<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlanningUserSecurityGroups(::core::mem::transmute_copy(&varval)).into()
        }
        unsafe extern "system" fn PlanningUserSecurityGroups<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlanningUserSecurityGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *varval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningComputer<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlanningComputer(::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningComputer<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlanningComputer() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningComputerSOM<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlanningComputerSOM(::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningComputerSOM<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlanningComputerSOM() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningComputerWMIFilters<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlanningComputerWMIFilters(::core::mem::transmute_copy(&varval)).into()
        }
        unsafe extern "system" fn PlanningComputerWMIFilters<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlanningComputerWMIFilters() {
                ::core::result::Result::Ok(ok__) => {
                    *varval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningComputerSecurityGroups<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlanningComputerSecurityGroups(::core::mem::transmute_copy(&varval)).into()
        }
        unsafe extern "system" fn PlanningComputerSecurityGroups<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlanningComputerSecurityGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *varval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoggingEnumerateUsers<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoggingEnumerateUsers() {
                ::core::result::Result::Ok(ok__) => {
                    *varval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryResults<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateQueryResults().into()
        }
        unsafe extern "system" fn ReleaseQueryResults<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseQueryResults().into()
        }
        unsafe extern "system" fn GenerateReport<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateReport(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn GenerateReportToFile<Impl: IGPMRSOPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateReportToFile(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&bstrtargetfilepath)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Mode: Mode::<Impl, IMPL_OFFSET>,
            Namespace: Namespace::<Impl, IMPL_OFFSET>,
            SetLoggingComputer: SetLoggingComputer::<Impl, IMPL_OFFSET>,
            LoggingComputer: LoggingComputer::<Impl, IMPL_OFFSET>,
            SetLoggingUser: SetLoggingUser::<Impl, IMPL_OFFSET>,
            LoggingUser: LoggingUser::<Impl, IMPL_OFFSET>,
            SetLoggingFlags: SetLoggingFlags::<Impl, IMPL_OFFSET>,
            LoggingFlags: LoggingFlags::<Impl, IMPL_OFFSET>,
            SetPlanningFlags: SetPlanningFlags::<Impl, IMPL_OFFSET>,
            PlanningFlags: PlanningFlags::<Impl, IMPL_OFFSET>,
            SetPlanningDomainController: SetPlanningDomainController::<Impl, IMPL_OFFSET>,
            PlanningDomainController: PlanningDomainController::<Impl, IMPL_OFFSET>,
            SetPlanningSiteName: SetPlanningSiteName::<Impl, IMPL_OFFSET>,
            PlanningSiteName: PlanningSiteName::<Impl, IMPL_OFFSET>,
            SetPlanningUser: SetPlanningUser::<Impl, IMPL_OFFSET>,
            PlanningUser: PlanningUser::<Impl, IMPL_OFFSET>,
            SetPlanningUserSOM: SetPlanningUserSOM::<Impl, IMPL_OFFSET>,
            PlanningUserSOM: PlanningUserSOM::<Impl, IMPL_OFFSET>,
            SetPlanningUserWMIFilters: SetPlanningUserWMIFilters::<Impl, IMPL_OFFSET>,
            PlanningUserWMIFilters: PlanningUserWMIFilters::<Impl, IMPL_OFFSET>,
            SetPlanningUserSecurityGroups: SetPlanningUserSecurityGroups::<Impl, IMPL_OFFSET>,
            PlanningUserSecurityGroups: PlanningUserSecurityGroups::<Impl, IMPL_OFFSET>,
            SetPlanningComputer: SetPlanningComputer::<Impl, IMPL_OFFSET>,
            PlanningComputer: PlanningComputer::<Impl, IMPL_OFFSET>,
            SetPlanningComputerSOM: SetPlanningComputerSOM::<Impl, IMPL_OFFSET>,
            PlanningComputerSOM: PlanningComputerSOM::<Impl, IMPL_OFFSET>,
            SetPlanningComputerWMIFilters: SetPlanningComputerWMIFilters::<Impl, IMPL_OFFSET>,
            PlanningComputerWMIFilters: PlanningComputerWMIFilters::<Impl, IMPL_OFFSET>,
            SetPlanningComputerSecurityGroups: SetPlanningComputerSecurityGroups::<Impl, IMPL_OFFSET>,
            PlanningComputerSecurityGroups: PlanningComputerSecurityGroups::<Impl, IMPL_OFFSET>,
            LoggingEnumerateUsers: LoggingEnumerateUsers::<Impl, IMPL_OFFSET>,
            CreateQueryResults: CreateQueryResults::<Impl, IMPL_OFFSET>,
            ReleaseQueryResults: ReleaseQueryResults::<Impl, IMPL_OFFSET>,
            GenerateReport: GenerateReport::<Impl, IMPL_OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMRSOP as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMResultImpl: Sized + IDispatchImpl {
    fn Status(&mut self) -> ::windows::core::Result<IGPMStatusMsgCollection>;
    fn Result(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn OverallStatus(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMResultVtbl {
        unsafe extern "system" fn Status<Impl: IGPMResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmstatusmsgcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmstatusmsgcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Result<Impl: IGPMResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarresult: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Result() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverallStatus<Impl: IGPMResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OverallStatus().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Result: Result::<Impl, IMPL_OFFSET>,
            OverallStatus: OverallStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMSOMImpl: Sized + IDispatchImpl {
    fn GPOInheritanceBlocked(&mut self) -> ::windows::core::Result<i16>;
    fn SetGPOInheritanceBlocked(&mut self, newval: i16) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreateGPOLink(&mut self, llinkpos: i32, pgpo: ::core::option::Option<IGPMGPO>) -> ::windows::core::Result<IGPMGPOLink>;
    fn Type(&mut self) -> ::windows::core::Result<GPMSOMType>;
    fn GetGPOLinks(&mut self) -> ::windows::core::Result<IGPMGPOLinksCollection>;
    fn GetInheritedGPOLinks(&mut self) -> ::windows::core::Result<IGPMGPOLinksCollection>;
    fn GetSecurityInfo(&mut self) -> ::windows::core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(&mut self, psecurityinfo: ::core::option::Option<IGPMSecurityInfo>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMSOMVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSOMImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMSOMVtbl {
        unsafe extern "system" fn GPOInheritanceBlocked<Impl: IGPMSOMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GPOInheritanceBlocked() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGPOInheritanceBlocked<Impl: IGPMSOMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGPOInheritanceBlocked(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Name<Impl: IGPMSOMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Impl: IGPMSOMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGPOLink<Impl: IGPMSOMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llinkpos: i32, pgpo: ::windows::core::RawPtr, ppnewgpolink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGPOLink(::core::mem::transmute_copy(&llinkpos), ::core::mem::transmute(&pgpo)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnewgpolink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IGPMSOMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGPOLinks<Impl: IGPMSOMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgpolinks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGPOLinks() {
                ::core::result::Result::Ok(ok__) => {
                    *ppgpolinks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInheritedGPOLinks<Impl: IGPMSOMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgpolinks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInheritedGPOLinks() {
                ::core::result::Result::Ok(ok__) => {
                    *ppgpolinks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityInfo<Impl: IGPMSOMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecurityInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsecurityinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityInfo<Impl: IGPMSOMImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecurityinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurityInfo(::core::mem::transmute(&psecurityinfo)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GPOInheritanceBlocked: GPOInheritanceBlocked::<Impl, IMPL_OFFSET>,
            SetGPOInheritanceBlocked: SetGPOInheritanceBlocked::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            CreateGPOLink: CreateGPOLink::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            GetGPOLinks: GetGPOLinks::<Impl, IMPL_OFFSET>,
            GetInheritedGPOLinks: GetInheritedGPOLinks::<Impl, IMPL_OFFSET>,
            GetSecurityInfo: GetSecurityInfo::<Impl, IMPL_OFFSET>,
            SetSecurityInfo: SetSecurityInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMSOM as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMSOMCollectionImpl: Sized + IDispatchImpl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMSOMCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSOMCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMSOMCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IGPMSOMCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IGPMSOMCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IGPMSOMCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmsom: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmsom = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMSOMCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMSearchCriteriaImpl: Sized + IDispatchImpl {
    fn Add(&mut self, searchproperty: GPMSearchProperty, searchoperation: GPMSearchOperation, varvalue: super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMSearchCriteriaVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSearchCriteriaImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMSearchCriteriaVtbl {
        unsafe extern "system" fn Add<Impl: IGPMSearchCriteriaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, searchproperty: GPMSearchProperty, searchoperation: GPMSearchOperation, varvalue: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&searchproperty), ::core::mem::transmute_copy(&searchoperation), ::core::mem::transmute_copy(&varvalue)).into()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Add: Add::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMSearchCriteria as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMSecurityInfoImpl: Sized + IDispatchImpl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
    fn Add(&mut self, pperm: ::core::option::Option<IGPMPermission>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, pperm: ::core::option::Option<IGPMPermission>) -> ::windows::core::Result<()>;
    fn RemoveTrustee(&mut self, bstrtrustee: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMSecurityInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSecurityInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMSecurityInfoVtbl {
        unsafe extern "system" fn Count<Impl: IGPMSecurityInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IGPMSecurityInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IGPMSecurityInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IGPMSecurityInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pperm)).into()
        }
        unsafe extern "system" fn Remove<Impl: IGPMSecurityInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute(&pperm)).into()
        }
        unsafe extern "system" fn RemoveTrustee<Impl: IGPMSecurityInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTrustee(::core::mem::transmute_copy(&bstrtrustee)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            RemoveTrustee: RemoveTrustee::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMSecurityInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMSitesContainerImpl: Sized + IDispatchImpl {
    fn DomainController(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Domain(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Forest(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetSite(&mut self, bstrsitename: super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMSOM>;
    fn SearchSites(&mut self, pigpmsearchcriteria: ::core::option::Option<IGPMSearchCriteria>) -> ::windows::core::Result<IGPMSOMCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMSitesContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSitesContainerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMSitesContainerVtbl {
        unsafe extern "system" fn DomainController<Impl: IGPMSitesContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainController() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Domain<Impl: IGPMSitesContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Domain() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Forest<Impl: IGPMSitesContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Forest() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSite<Impl: IGPMSitesContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsitename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsom: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSite(::core::mem::transmute_copy(&bstrsitename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsom = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchSites<Impl: IGPMSitesContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmsomcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchSites(::core::mem::transmute(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmsomcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DomainController: DomainController::<Impl, IMPL_OFFSET>,
            Domain: Domain::<Impl, IMPL_OFFSET>,
            Forest: Forest::<Impl, IMPL_OFFSET>,
            GetSite: GetSite::<Impl, IMPL_OFFSET>,
            SearchSites: SearchSites::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMSitesContainer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMStarterGPOImpl: Sized + IDispatchImpl {
    fn DisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDisplayName(&mut self, newval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, newval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Author(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Product(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreationTime(&mut self) -> ::windows::core::Result<f64>;
    fn ID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ModifiedTime(&mut self) -> ::windows::core::Result<f64>;
    fn Type(&mut self) -> ::windows::core::Result<GPMStarterGPOType>;
    fn ComputerVersion(&mut self) -> ::windows::core::Result<u16>;
    fn UserVersion(&mut self) -> ::windows::core::Result<u16>;
    fn StarterGPOVersion(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn Save(&mut self, bstrsavefile: super::super::Foundation::BSTR, boverwrite: i16, bsaveassystem: i16, bstrlanguage: *const super::Com::VARIANT, bstrauthor: *const super::Com::VARIANT, bstrproduct: *const super::Com::VARIANT, bstruniqueid: *const super::Com::VARIANT, bstrversion: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn Backup(&mut self, bstrbackupdir: super::super::Foundation::BSTR, bstrcomment: super::super::Foundation::BSTR, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn CopyTo(&mut self, pvarnewdisplayname: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT) -> ::windows::core::Result<IGPMResult>;
    fn GenerateReport(&mut self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT) -> ::windows::core::Result<IGPMResult>;
    fn GenerateReportToFile(&mut self, gpmreporttype: GPMReportType, bstrtargetfilepath: super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMResult>;
    fn GetSecurityInfo(&mut self) -> ::windows::core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(&mut self, psecurityinfo: ::core::option::Option<IGPMSecurityInfo>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStarterGPOVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMStarterGPOVtbl {
        unsafe extern "system" fn DisplayName<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Description<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Author<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Author() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Product<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Product() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreationTime<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ID<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ID() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifiedTime<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifiedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerVersion<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComputerVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserVersion<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StarterGPOVersion<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StarterGPOVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Save<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsavefile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, boverwrite: i16, bsaveassystem: i16, bstrlanguage: *const super::Com::VARIANT, bstrauthor: *const super::Com::VARIANT, bstrproduct: *const super::Com::VARIANT, bstruniqueid: *const super::Com::VARIANT, bstrversion: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .Save(
                    ::core::mem::transmute_copy(&bstrsavefile),
                    ::core::mem::transmute_copy(&boverwrite),
                    ::core::mem::transmute_copy(&bsaveassystem),
                    ::core::mem::transmute_copy(&bstrlanguage),
                    ::core::mem::transmute_copy(&bstrauthor),
                    ::core::mem::transmute_copy(&bstrproduct),
                    ::core::mem::transmute_copy(&bstruniqueid),
                    ::core::mem::transmute_copy(&bstrversion),
                    ::core::mem::transmute_copy(&pvargpmprogress),
                    ::core::mem::transmute_copy(&pvargpmcancel),
                    ::core::mem::transmute_copy(&ppigpmresult),
                )
                .into()
        }
        unsafe extern "system" fn Backup<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrcomment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Backup(::core::mem::transmute_copy(&bstrbackupdir), ::core::mem::transmute_copy(&bstrcomment), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn CopyTo<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarnewdisplayname: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyTo(::core::mem::transmute_copy(&pvarnewdisplayname), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateReport<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateReport(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateReportToFile<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateReportToFile(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&bstrtargetfilepath)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityInfo<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecurityInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsecurityinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityInfo<Impl: IGPMStarterGPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecurityinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurityInfo(::core::mem::transmute(&psecurityinfo)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            Author: Author::<Impl, IMPL_OFFSET>,
            Product: Product::<Impl, IMPL_OFFSET>,
            CreationTime: CreationTime::<Impl, IMPL_OFFSET>,
            ID: ID::<Impl, IMPL_OFFSET>,
            ModifiedTime: ModifiedTime::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            ComputerVersion: ComputerVersion::<Impl, IMPL_OFFSET>,
            UserVersion: UserVersion::<Impl, IMPL_OFFSET>,
            StarterGPOVersion: StarterGPOVersion::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
            Backup: Backup::<Impl, IMPL_OFFSET>,
            CopyTo: CopyTo::<Impl, IMPL_OFFSET>,
            GenerateReport: GenerateReport::<Impl, IMPL_OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Impl, IMPL_OFFSET>,
            GetSecurityInfo: GetSecurityInfo::<Impl, IMPL_OFFSET>,
            SetSecurityInfo: SetSecurityInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMStarterGPO as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMStarterGPOBackupImpl: Sized + IDispatchImpl {
    fn BackupDir(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Comment(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Domain(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn StarterGPOID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Timestamp(&mut self) -> ::windows::core::Result<f64>;
    fn Type(&mut self) -> ::windows::core::Result<GPMStarterGPOType>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn GenerateReport(&mut self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn GenerateReportToFile(&mut self, gpmreporttype: GPMReportType, bstrtargetfilepath: super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStarterGPOBackupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOBackupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMStarterGPOBackupVtbl {
        unsafe extern "system" fn BackupDir<Impl: IGPMStarterGPOBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbackupdir: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackupDir() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrbackupdir = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comment<Impl: IGPMStarterGPOBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcomment: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Comment() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcomment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IGPMStarterGPOBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdisplayname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Domain<Impl: IGPMStarterGPOBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtemplatedomain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Domain() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtemplatedomain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StarterGPOID<Impl: IGPMStarterGPOBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtemplateid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StarterGPOID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtemplateid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ID<Impl: IGPMStarterGPOBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: IGPMStarterGPOBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimestamp: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *ptimestamp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IGPMStarterGPOBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut GPMStarterGPOType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IGPMStarterGPOBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn GenerateReport<Impl: IGPMStarterGPOBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateReport(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn GenerateReportToFile<Impl: IGPMStarterGPOBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateReportToFile(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&bstrtargetfilepath)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BackupDir: BackupDir::<Impl, IMPL_OFFSET>,
            Comment: Comment::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            Domain: Domain::<Impl, IMPL_OFFSET>,
            StarterGPOID: StarterGPOID::<Impl, IMPL_OFFSET>,
            ID: ID::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            GenerateReport: GenerateReport::<Impl, IMPL_OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMStarterGPOBackup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMStarterGPOBackupCollectionImpl: Sized + IDispatchImpl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStarterGPOBackupCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOBackupCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMStarterGPOBackupCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IGPMStarterGPOBackupCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IGPMStarterGPOBackupCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IGPMStarterGPOBackupCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmtmplbackup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmtmplbackup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMStarterGPOBackupCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMStarterGPOCollectionImpl: Sized + IDispatchImpl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStarterGPOCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMStarterGPOCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IGPMStarterGPOCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IGPMStarterGPOCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IGPMStarterGPOCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmtemplates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmtemplates = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMStarterGPOCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMStatusMessageImpl: Sized + IDispatchImpl {
    fn ObjectPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ErrorCode(&mut self) -> ::windows::core::Result<()>;
    fn ExtensionName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SettingsName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn OperationCode(&mut self) -> ::windows::core::Result<()>;
    fn Message(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStatusMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStatusMessageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMStatusMessageVtbl {
        unsafe extern "system" fn ObjectPath<Impl: IGPMStatusMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObjectPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Impl: IGPMStatusMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ErrorCode().into()
        }
        unsafe extern "system" fn ExtensionName<Impl: IGPMStatusMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtensionName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettingsName<Impl: IGPMStatusMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SettingsName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OperationCode<Impl: IGPMStatusMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OperationCode().into()
        }
        unsafe extern "system" fn Message<Impl: IGPMStatusMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ObjectPath: ObjectPath::<Impl, IMPL_OFFSET>,
            ErrorCode: ErrorCode::<Impl, IMPL_OFFSET>,
            ExtensionName: ExtensionName::<Impl, IMPL_OFFSET>,
            SettingsName: SettingsName::<Impl, IMPL_OFFSET>,
            OperationCode: OperationCode::<Impl, IMPL_OFFSET>,
            Message: Message::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMStatusMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMStatusMsgCollectionImpl: Sized + IDispatchImpl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStatusMsgCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStatusMsgCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMStatusMsgCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IGPMStatusMsgCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IGPMStatusMsgCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IGPMStatusMsgCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMStatusMsgCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMTrusteeImpl: Sized + IDispatchImpl {
    fn TrusteeSid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TrusteeName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TrusteeDomain(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TrusteeDSPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TrusteeType(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMTrusteeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMTrusteeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMTrusteeVtbl {
        unsafe extern "system" fn TrusteeSid<Impl: IGPMTrusteeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrusteeSid() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrusteeName<Impl: IGPMTrusteeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrusteeName() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrusteeDomain<Impl: IGPMTrusteeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrusteeDomain() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrusteeDSPath<Impl: IGPMTrusteeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrusteeDSPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrusteeType<Impl: IGPMTrusteeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrusteeType() {
                ::core::result::Result::Ok(ok__) => {
                    *lval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            TrusteeSid: TrusteeSid::<Impl, IMPL_OFFSET>,
            TrusteeName: TrusteeName::<Impl, IMPL_OFFSET>,
            TrusteeDomain: TrusteeDomain::<Impl, IMPL_OFFSET>,
            TrusteeDSPath: TrusteeDSPath::<Impl, IMPL_OFFSET>,
            TrusteeType: TrusteeType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMTrustee as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMWMIFilterImpl: Sized + IDispatchImpl {
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, newval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, newval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetQueryList(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn GetSecurityInfo(&mut self) -> ::windows::core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(&mut self, psecurityinfo: ::core::option::Option<IGPMSecurityInfo>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMWMIFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMWMIFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMWMIFilterVtbl {
        unsafe extern "system" fn Path<Impl: IGPMWMIFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IGPMWMIFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Name<Impl: IGPMWMIFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IGPMWMIFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Description<Impl: IGPMWMIFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQueryList<Impl: IGPMWMIFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqrylist: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetQueryList() {
                ::core::result::Result::Ok(ok__) => {
                    *pqrylist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityInfo<Impl: IGPMWMIFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecurityInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsecurityinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityInfo<Impl: IGPMWMIFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecurityinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurityInfo(::core::mem::transmute(&psecurityinfo)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Path: Path::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            GetQueryList: GetQueryList::<Impl, IMPL_OFFSET>,
            GetSecurityInfo: GetSecurityInfo::<Impl, IMPL_OFFSET>,
            SetSecurityInfo: SetSecurityInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMWMIFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMWMIFilterCollectionImpl: Sized + IDispatchImpl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMWMIFilterCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMWMIFilterCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGPMWMIFilterCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IGPMWMIFilterCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IGPMWMIFilterCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IGPMWMIFilterCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMWMIFilterCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_Controls"))]
pub trait IGroupPolicyObjectImpl: Sized {
    fn New(&mut self, pszdomainname: super::super::Foundation::PWSTR, pszdisplayname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn OpenDSGPO(&mut self, pszpath: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn OpenLocalMachineGPO(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
    fn OpenRemoteMachineGPO(&mut self, pszcomputername: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn Save(&mut self, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows::core::GUID, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn GetName(&mut self, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::Result<()>;
    fn GetDisplayName(&mut self, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::Result<()>;
    fn SetDisplayName(&mut self, pszname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetPath(&mut self, pszpath: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::Result<()>;
    fn GetDSPath(&mut self, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::core::Result<()>;
    fn GetFileSysPath(&mut self, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::core::Result<()>;
    fn GetRegistryKey(&mut self, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows::core::Result<()>;
    fn GetOptions(&mut self, dwoptions: *mut u32) -> ::windows::core::Result<()>;
    fn SetOptions(&mut self, dwoptions: u32, dwmask: u32) -> ::windows::core::Result<()>;
    fn GetType(&mut self, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows::core::Result<()>;
    fn GetMachineName(&mut self, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::Result<()>;
    fn GetPropertySheetPages(&mut self, hpages: *mut *mut super::super::UI::Controls::HPROPSHEETPAGE, upagecount: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_Controls"))]
impl IGroupPolicyObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGroupPolicyObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGroupPolicyObjectVtbl {
        unsafe extern "system" fn New<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdomainname: super::super::Foundation::PWSTR, pszdisplayname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).New(::core::mem::transmute_copy(&pszdomainname), ::core::mem::transmute_copy(&pszdisplayname), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OpenDSGPO<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenDSGPO(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OpenLocalMachineGPO<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenLocalMachineGPO(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OpenRemoteMachineGPO<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcomputername: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenRemoteMachineGPO(::core::mem::transmute_copy(&pszcomputername), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Save<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows::core::GUID, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save(::core::mem::transmute_copy(&bmachine), ::core::mem::transmute_copy(&badd), ::core::mem::transmute_copy(&pguidextension), ::core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn Delete<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn GetName<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetName(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetDisplayName<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDisplayName(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn SetDisplayName<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute_copy(&pszname)).into()
        }
        unsafe extern "system" fn GetPath<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPath(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetDSPath<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDSPath(::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&cchmaxpath)).into()
        }
        unsafe extern "system" fn GetFileSysPath<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFileSysPath(::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&cchmaxpath)).into()
        }
        unsafe extern "system" fn GetRegistryKey<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRegistryKey(::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&hkey)).into()
        }
        unsafe extern "system" fn GetOptions<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOptions(::core::mem::transmute_copy(&dwoptions)).into()
        }
        unsafe extern "system" fn SetOptions<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: u32, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOptions(::core::mem::transmute_copy(&dwoptions), ::core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn GetType<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetType(::core::mem::transmute_copy(&gpotype)).into()
        }
        unsafe extern "system" fn GetMachineName<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMachineName(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetPropertySheetPages<Impl: IGroupPolicyObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpages: *mut *mut super::super::UI::Controls::HPROPSHEETPAGE, upagecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertySheetPages(::core::mem::transmute_copy(&hpages), ::core::mem::transmute_copy(&upagecount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            New: New::<Impl, IMPL_OFFSET>,
            OpenDSGPO: OpenDSGPO::<Impl, IMPL_OFFSET>,
            OpenLocalMachineGPO: OpenLocalMachineGPO::<Impl, IMPL_OFFSET>,
            OpenRemoteMachineGPO: OpenRemoteMachineGPO::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetDisplayName: GetDisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            GetPath: GetPath::<Impl, IMPL_OFFSET>,
            GetDSPath: GetDSPath::<Impl, IMPL_OFFSET>,
            GetFileSysPath: GetFileSysPath::<Impl, IMPL_OFFSET>,
            GetRegistryKey: GetRegistryKey::<Impl, IMPL_OFFSET>,
            GetOptions: GetOptions::<Impl, IMPL_OFFSET>,
            SetOptions: SetOptions::<Impl, IMPL_OFFSET>,
            GetType: GetType::<Impl, IMPL_OFFSET>,
            GetMachineName: GetMachineName::<Impl, IMPL_OFFSET>,
            GetPropertySheetPages: GetPropertySheetPages::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGroupPolicyObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRSOPInformationImpl: Sized {
    fn GetNamespace(&mut self, dwsection: u32, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::Result<()>;
    fn GetFlags(&mut self, pdwflags: *mut u32) -> ::windows::core::Result<()>;
    fn GetEventLogEntryText(&mut self, pszeventsource: super::super::Foundation::PWSTR, pszeventlogname: super::super::Foundation::PWSTR, pszeventtime: super::super::Foundation::PWSTR, dweventid: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRSOPInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRSOPInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRSOPInformationVtbl {
        unsafe extern "system" fn GetNamespace<Impl: IRSOPInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNamespace(::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetFlags<Impl: IRSOPInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFlags(::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn GetEventLogEntryText<Impl: IRSOPInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszeventsource: super::super::Foundation::PWSTR, pszeventlogname: super::super::Foundation::PWSTR, pszeventtime: super::super::Foundation::PWSTR, dweventid: u32, ppsztext: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEventLogEntryText(::core::mem::transmute_copy(&pszeventsource), ::core::mem::transmute_copy(&pszeventlogname), ::core::mem::transmute_copy(&pszeventtime), ::core::mem::transmute_copy(&dweventid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsztext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetNamespace: GetNamespace::<Impl, IMPL_OFFSET>,
            GetFlags: GetFlags::<Impl, IMPL_OFFSET>,
            GetEventLogEntryText: GetEventLogEntryText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRSOPInformation as ::windows::core::Interface>::IID
    }
}
