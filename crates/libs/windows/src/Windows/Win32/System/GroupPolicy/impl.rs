#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IGPEInformation_Impl: Sized {
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
impl IGPEInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPEInformation_Impl, const OFFSET: isize>() -> IGPEInformation_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetName(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetDisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDisplayName(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetRegistryKey<Identity: ::windows::core::IUnknownImpl, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRegistryKey(::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&hkey)).into()
        }
        unsafe extern "system" fn GetDSPath<Identity: ::windows::core::IUnknownImpl, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDSPath(::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&cchmaxpath)).into()
        }
        unsafe extern "system" fn GetFileSysPath<Identity: ::windows::core::IUnknownImpl, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFileSysPath(::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&cchmaxpath)).into()
        }
        unsafe extern "system" fn GetOptions<Identity: ::windows::core::IUnknownImpl, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOptions(::core::mem::transmute_copy(&dwoptions)).into()
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetType(::core::mem::transmute_copy(&gpotype)).into()
        }
        unsafe extern "system" fn GetHint<Identity: ::windows::core::IUnknownImpl, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gphint: *mut GROUP_POLICY_HINT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetHint(::core::mem::transmute_copy(&gphint)).into()
        }
        unsafe extern "system" fn PolicyChanged<Identity: ::windows::core::IUnknownImpl, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows::core::GUID, pguidsnapin: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PolicyChanged(::core::mem::transmute_copy(&bmachine), ::core::mem::transmute_copy(&badd), ::core::mem::transmute_copy(&pguidextension), ::core::mem::transmute_copy(&pguidsnapin)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            GetRegistryKey: GetRegistryKey::<Identity, Impl, OFFSET>,
            GetDSPath: GetDSPath::<Identity, Impl, OFFSET>,
            GetFileSysPath: GetFileSysPath::<Identity, Impl, OFFSET>,
            GetOptions: GetOptions::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetHint: GetHint::<Identity, Impl, OFFSET>,
            PolicyChanged: PolicyChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPEInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPM_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetDomain(&mut self, bstrdomain: &super::super::Foundation::BSTR, bstrdomaincontroller: &super::super::Foundation::BSTR, ldcflags: i32) -> ::windows::core::Result<IGPMDomain>;
    fn GetBackupDir(&mut self, bstrbackupdir: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMBackupDir>;
    fn GetSitesContainer(&mut self, bstrforest: &super::super::Foundation::BSTR, bstrdomain: &super::super::Foundation::BSTR, bstrdomaincontroller: &super::super::Foundation::BSTR, ldcflags: i32) -> ::windows::core::Result<IGPMSitesContainer>;
    fn GetRSOP(&mut self, gpmrsopmode: GPMRSOPMode, bstrnamespace: &super::super::Foundation::BSTR, lflags: i32) -> ::windows::core::Result<IGPMRSOP>;
    fn CreatePermission(&mut self, bstrtrustee: &super::super::Foundation::BSTR, perm: GPMPermissionType, binheritable: i16) -> ::windows::core::Result<IGPMPermission>;
    fn CreateSearchCriteria(&mut self) -> ::windows::core::Result<IGPMSearchCriteria>;
    fn CreateTrustee(&mut self, bstrtrustee: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMTrustee>;
    fn GetClientSideExtensions(&mut self) -> ::windows::core::Result<IGPMCSECollection>;
    fn GetConstants(&mut self) -> ::windows::core::Result<IGPMConstants>;
    fn GetMigrationTable(&mut self, bstrmigrationtablepath: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMMigrationTable>;
    fn CreateMigrationTable(&mut self) -> ::windows::core::Result<IGPMMigrationTable>;
    fn InitializeReporting(&mut self, bstradmpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPM_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPM_Impl, const OFFSET: isize>() -> IGPM_Vtbl {
        unsafe extern "system" fn GetDomain<Identity: ::windows::core::IUnknownImpl, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomaincontroller: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ldcflags: i32, pigpmdomain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDomain(::core::mem::transmute_copy(&bstrdomain), ::core::mem::transmute_copy(&bstrdomaincontroller), ::core::mem::transmute_copy(&ldcflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pigpmdomain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackupDir<Identity: ::windows::core::IUnknownImpl, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pigpmbackupdir: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBackupDir(::core::mem::transmute_copy(&bstrbackupdir)) {
                ::core::result::Result::Ok(ok__) => {
                    *pigpmbackupdir = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSitesContainer<Identity: ::windows::core::IUnknownImpl, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrforest: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomaincontroller: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ldcflags: i32, ppigpmsitescontainer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSitesContainer(::core::mem::transmute_copy(&bstrforest), ::core::mem::transmute_copy(&bstrdomain), ::core::mem::transmute_copy(&bstrdomaincontroller), ::core::mem::transmute_copy(&ldcflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmsitescontainer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRSOP<Identity: ::windows::core::IUnknownImpl, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmrsopmode: GPMRSOPMode, bstrnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, ppigpmrsop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRSOP(::core::mem::transmute_copy(&gpmrsopmode), ::core::mem::transmute_copy(&bstrnamespace), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmrsop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePermission<Identity: ::windows::core::IUnknownImpl, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, perm: GPMPermissionType, binheritable: i16, ppperm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreatePermission(::core::mem::transmute_copy(&bstrtrustee), ::core::mem::transmute_copy(&perm), ::core::mem::transmute_copy(&binheritable)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppperm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSearchCriteria<Identity: ::windows::core::IUnknownImpl, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmsearchcriteria: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSearchCriteria() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmsearchcriteria = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTrustee<Identity: ::windows::core::IUnknownImpl, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmtrustee: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTrustee(::core::mem::transmute_copy(&bstrtrustee)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmtrustee = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClientSideExtensions<Identity: ::windows::core::IUnknownImpl, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmcsecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetClientSideExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmcsecollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstants<Identity: ::windows::core::IUnknownImpl, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmconstants: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetConstants() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmconstants = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMigrationTable<Identity: ::windows::core::IUnknownImpl, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmigrationtablepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmigrationtable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMigrationTable(::core::mem::transmute_copy(&bstrmigrationtablepath)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmigrationtable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMigrationTable<Identity: ::windows::core::IUnknownImpl, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmigrationtable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateMigrationTable() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmigrationtable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeReporting<Identity: ::windows::core::IUnknownImpl, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeReporting(::core::mem::transmute_copy(&bstradmpath)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDomain: GetDomain::<Identity, Impl, OFFSET>,
            GetBackupDir: GetBackupDir::<Identity, Impl, OFFSET>,
            GetSitesContainer: GetSitesContainer::<Identity, Impl, OFFSET>,
            GetRSOP: GetRSOP::<Identity, Impl, OFFSET>,
            CreatePermission: CreatePermission::<Identity, Impl, OFFSET>,
            CreateSearchCriteria: CreateSearchCriteria::<Identity, Impl, OFFSET>,
            CreateTrustee: CreateTrustee::<Identity, Impl, OFFSET>,
            GetClientSideExtensions: GetClientSideExtensions::<Identity, Impl, OFFSET>,
            GetConstants: GetConstants::<Identity, Impl, OFFSET>,
            GetMigrationTable: GetMigrationTable::<Identity, Impl, OFFSET>,
            CreateMigrationTable: CreateMigrationTable::<Identity, Impl, OFFSET>,
            InitializeReporting: InitializeReporting::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPM as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPM2_Impl: Sized + super::Com::IDispatch_Impl + IGPM_Impl {
    fn GetBackupDirEx(&mut self, bstrbackupdir: &super::super::Foundation::BSTR, backupdirtype: GPMBackupType) -> ::windows::core::Result<IGPMBackupDirEx>;
    fn InitializeReportingEx(&mut self, bstradmpath: &super::super::Foundation::BSTR, reportingoptions: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPM2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPM2_Impl, const OFFSET: isize>() -> IGPM2_Vtbl {
        unsafe extern "system" fn GetBackupDirEx<Identity: ::windows::core::IUnknownImpl, Impl: IGPM2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, backupdirtype: GPMBackupType, ppigpmbackupdirex: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBackupDirEx(::core::mem::transmute_copy(&bstrbackupdir), ::core::mem::transmute_copy(&backupdirtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmbackupdirex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeReportingEx<Identity: ::windows::core::IUnknownImpl, Impl: IGPM2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, reportingoptions: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeReportingEx(::core::mem::transmute_copy(&bstradmpath), ::core::mem::transmute_copy(&reportingoptions)).into()
        }
        Self {
            base: IGPM_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetBackupDirEx: GetBackupDirEx::<Identity, Impl, OFFSET>,
            InitializeReportingEx: InitializeReportingEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPM2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IGPM as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMAsyncCancel_Impl: Sized + super::Com::IDispatch_Impl {
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMAsyncCancel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMAsyncCancel_Impl, const OFFSET: isize>() -> IGPMAsyncCancel_Vtbl {
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IGPMAsyncCancel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel().into()
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Cancel: Cancel::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMAsyncCancel as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMAsyncProgress_Impl: Sized + super::Com::IDispatch_Impl {
    fn Status(&mut self, lprogressnumerator: i32, lprogressdenominator: i32, hrstatus: ::windows::core::HRESULT, presult: *const super::Com::VARIANT, ppigpmstatusmsgcollection: &::core::option::Option<IGPMStatusMsgCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMAsyncProgress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMAsyncProgress_Impl, const OFFSET: isize>() -> IGPMAsyncProgress_Vtbl {
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl, Impl: IGPMAsyncProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprogressnumerator: i32, lprogressdenominator: i32, hrstatus: ::windows::core::HRESULT, presult: *const super::Com::VARIANT, ppigpmstatusmsgcollection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Status(::core::mem::transmute_copy(&lprogressnumerator), ::core::mem::transmute_copy(&lprogressdenominator), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&presult), ::core::mem::transmute(&ppigpmstatusmsgcollection)).into()
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Status: Status::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMAsyncProgress as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMBackup_Impl: Sized + super::Com::IDispatch_Impl {
    fn ID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GPOID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GPODomain(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GPODisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Timestamp(&mut self) -> ::windows::core::Result<f64>;
    fn Comment(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BackupDir(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn GenerateReport(&mut self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn GenerateReportToFile(&mut self, gpmreporttype: GPMReportType, bstrtargetfilepath: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMBackup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackup_Impl, const OFFSET: isize>() -> IGPMBackup_Vtbl {
        unsafe extern "system" fn ID<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ID() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GPOID<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GPOID() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GPODomain<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GPODomain() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GPODisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GPODisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comment<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Comment() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackupDir<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BackupDir() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn GenerateReport<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GenerateReport(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GenerateReportToFile(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&bstrtargetfilepath)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ID: ID::<Identity, Impl, OFFSET>,
            GPOID: GPOID::<Identity, Impl, OFFSET>,
            GPODomain: GPODomain::<Identity, Impl, OFFSET>,
            GPODisplayName: GPODisplayName::<Identity, Impl, OFFSET>,
            Timestamp: Timestamp::<Identity, Impl, OFFSET>,
            Comment: Comment::<Identity, Impl, OFFSET>,
            BackupDir: BackupDir::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GenerateReport: GenerateReport::<Identity, Impl, OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMBackup as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMBackupCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMBackupCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackupCollection_Impl, const OFFSET: isize>() -> IGPMBackupCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackupCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackupCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackupCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmbackup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmbackup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMBackupCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMBackupDir_Impl: Sized + super::Com::IDispatch_Impl {
    fn BackupDirectory(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetBackup(&mut self, bstrid: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMBackup>;
    fn SearchBackups(&mut self, pigpmsearchcriteria: &::core::option::Option<IGPMSearchCriteria>) -> ::windows::core::Result<IGPMBackupCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMBackupDir_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackupDir_Impl, const OFFSET: isize>() -> IGPMBackupDir_Vtbl {
        unsafe extern "system" fn BackupDirectory<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackupDir_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BackupDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackup<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackupDir_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppbackup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBackup(::core::mem::transmute_copy(&bstrid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbackup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchBackups<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackupDir_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmbackupcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchBackups(::core::mem::transmute(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmbackupcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            BackupDirectory: BackupDirectory::<Identity, Impl, OFFSET>,
            GetBackup: GetBackup::<Identity, Impl, OFFSET>,
            SearchBackups: SearchBackups::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMBackupDir as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMBackupDirEx_Impl: Sized + super::Com::IDispatch_Impl {
    fn BackupDir(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BackupType(&mut self) -> ::windows::core::Result<GPMBackupType>;
    fn GetBackup(&mut self, bstrid: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SearchBackups(&mut self, pigpmsearchcriteria: &::core::option::Option<IGPMSearchCriteria>) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMBackupDirEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackupDirEx_Impl, const OFFSET: isize>() -> IGPMBackupDirEx_Vtbl {
        unsafe extern "system" fn BackupDir<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackupDirEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbackupdir: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BackupDir() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrbackupdir = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackupType<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackupDirEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgpmbackuptype: *mut GPMBackupType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BackupType() {
                ::core::result::Result::Ok(ok__) => {
                    *pgpmbackuptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackup<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackupDirEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarbackup: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBackup(::core::mem::transmute_copy(&bstrid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbackup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchBackups<Identity: ::windows::core::IUnknownImpl, Impl: IGPMBackupDirEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, pvarbackupcollection: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchBackups(::core::mem::transmute(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbackupcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            BackupDir: BackupDir::<Identity, Impl, OFFSET>,
            BackupType: BackupType::<Identity, Impl, OFFSET>,
            GetBackup: GetBackup::<Identity, Impl, OFFSET>,
            SearchBackups: SearchBackups::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMBackupDirEx as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMCSECollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMCSECollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMCSECollection_Impl, const OFFSET: isize>() -> IGPMCSECollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IGPMCSECollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IGPMCSECollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IGPMCSECollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmcses: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmcses = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMCSECollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMClientSideExtension_Impl: Sized + super::Com::IDispatch_Impl {
    fn ID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsUserEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn IsComputerEnabled(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMClientSideExtension_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMClientSideExtension_Impl, const OFFSET: isize>() -> IGPMClientSideExtension_Vtbl {
        unsafe extern "system" fn ID<Identity: ::windows::core::IUnknownImpl, Impl: IGPMClientSideExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ID() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IGPMClientSideExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUserEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IGPMClientSideExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsUserEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pvbenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsComputerEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IGPMClientSideExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsComputerEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pvbenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ID: ID::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            IsUserEnabled: IsUserEnabled::<Identity, Impl, OFFSET>,
            IsComputerEnabled: IsComputerEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMClientSideExtension as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMConstants_Impl: Sized + super::Com::IDispatch_Impl {
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
impl IGPMConstants_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>() -> IGPMConstants_Vtbl {
        unsafe extern "system" fn PermGPOApply<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PermGPOApply() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermGPORead<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PermGPORead() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermGPOEdit<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PermGPOEdit() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermGPOEditSecurityAndDelete<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PermGPOEditSecurityAndDelete() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermGPOCustom<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PermGPOCustom() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermWMIFilterEdit<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PermWMIFilterEdit() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermWMIFilterFullControl<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PermWMIFilterFullControl() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermWMIFilterCustom<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PermWMIFilterCustom() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMLink<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PermSOMLink() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMLogging<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PermSOMLogging() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMPlanning<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PermSOMPlanning() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMGPOCreate<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PermSOMGPOCreate() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMWMICreate<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PermSOMWMICreate() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMWMIFullControl<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PermSOMWMIFullControl() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOPermissions<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchPropertyGPOPermissions() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOEffectivePermissions<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchPropertyGPOEffectivePermissions() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPODisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchPropertyGPODisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOWMIFilter<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchPropertyGPOWMIFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOID<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchPropertyGPOID() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOComputerExtensions<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchPropertyGPOComputerExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOUserExtensions<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchPropertyGPOUserExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertySOMLinks<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchPropertySOMLinks() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPODomain<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchPropertyGPODomain() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyBackupMostRecent<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchPropertyBackupMostRecent() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchOpEquals<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchOpEquals() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchOpContains<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchOpContains() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchOpNotContains<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchOpNotContains() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchOpNotEquals<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchOpNotEquals() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsePDC<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UsePDC() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseAnyDC<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UseAnyDC() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoNotUseW2KDC<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DoNotUseW2KDC() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SOMSite<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SOMSite() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SOMDomain<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SOMDomain() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SOMOU<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SOMOU() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecurityFlags<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbowner: i16, vbgroup: i16, vbdacl: i16, vbsacl: i16, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SecurityFlags(::core::mem::transmute_copy(&vbowner), ::core::mem::transmute_copy(&vbgroup), ::core::mem::transmute_copy(&vbdacl), ::core::mem::transmute_copy(&vbsacl)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoNotValidateDC<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DoNotValidateDC() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportHTML<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMReportType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReportHTML() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportXML<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMReportType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReportXML() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RSOPModeUnknown<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RSOPModeUnknown() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RSOPModePlanning<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RSOPModePlanning() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RSOPModeLogging<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RSOPModeLogging() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeUser<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EntryTypeUser() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeComputer<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EntryTypeComputer() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeLocalGroup<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EntryTypeLocalGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeGlobalGroup<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EntryTypeGlobalGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeUniversalGroup<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EntryTypeUniversalGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeUNCPath<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EntryTypeUNCPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeUnknown<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EntryTypeUnknown() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationOptionSameAsSource<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DestinationOptionSameAsSource() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationOptionNone<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DestinationOptionNone() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationOptionByRelativeName<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DestinationOptionByRelativeName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationOptionSet<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DestinationOptionSet() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MigrationTableOnly<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MigrationTableOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessSecurity<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProcessSecurity() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsopLoggingNoComputer<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RsopLoggingNoComputer() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsopLoggingNoUser<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RsopLoggingNoUser() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsopPlanningAssumeSlowLink<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RsopPlanningAssumeSlowLink() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsopPlanningLoopbackOption<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbmerge: i16, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RsopPlanningLoopbackOption(::core::mem::transmute_copy(&vbmerge)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsopPlanningAssumeUserWQLFilterTrue<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RsopPlanningAssumeUserWQLFilterTrue() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsopPlanningAssumeCompWQLFilterTrue<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RsopPlanningAssumeCompWQLFilterTrue() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            PermGPOApply: PermGPOApply::<Identity, Impl, OFFSET>,
            PermGPORead: PermGPORead::<Identity, Impl, OFFSET>,
            PermGPOEdit: PermGPOEdit::<Identity, Impl, OFFSET>,
            PermGPOEditSecurityAndDelete: PermGPOEditSecurityAndDelete::<Identity, Impl, OFFSET>,
            PermGPOCustom: PermGPOCustom::<Identity, Impl, OFFSET>,
            PermWMIFilterEdit: PermWMIFilterEdit::<Identity, Impl, OFFSET>,
            PermWMIFilterFullControl: PermWMIFilterFullControl::<Identity, Impl, OFFSET>,
            PermWMIFilterCustom: PermWMIFilterCustom::<Identity, Impl, OFFSET>,
            PermSOMLink: PermSOMLink::<Identity, Impl, OFFSET>,
            PermSOMLogging: PermSOMLogging::<Identity, Impl, OFFSET>,
            PermSOMPlanning: PermSOMPlanning::<Identity, Impl, OFFSET>,
            PermSOMGPOCreate: PermSOMGPOCreate::<Identity, Impl, OFFSET>,
            PermSOMWMICreate: PermSOMWMICreate::<Identity, Impl, OFFSET>,
            PermSOMWMIFullControl: PermSOMWMIFullControl::<Identity, Impl, OFFSET>,
            SearchPropertyGPOPermissions: SearchPropertyGPOPermissions::<Identity, Impl, OFFSET>,
            SearchPropertyGPOEffectivePermissions: SearchPropertyGPOEffectivePermissions::<Identity, Impl, OFFSET>,
            SearchPropertyGPODisplayName: SearchPropertyGPODisplayName::<Identity, Impl, OFFSET>,
            SearchPropertyGPOWMIFilter: SearchPropertyGPOWMIFilter::<Identity, Impl, OFFSET>,
            SearchPropertyGPOID: SearchPropertyGPOID::<Identity, Impl, OFFSET>,
            SearchPropertyGPOComputerExtensions: SearchPropertyGPOComputerExtensions::<Identity, Impl, OFFSET>,
            SearchPropertyGPOUserExtensions: SearchPropertyGPOUserExtensions::<Identity, Impl, OFFSET>,
            SearchPropertySOMLinks: SearchPropertySOMLinks::<Identity, Impl, OFFSET>,
            SearchPropertyGPODomain: SearchPropertyGPODomain::<Identity, Impl, OFFSET>,
            SearchPropertyBackupMostRecent: SearchPropertyBackupMostRecent::<Identity, Impl, OFFSET>,
            SearchOpEquals: SearchOpEquals::<Identity, Impl, OFFSET>,
            SearchOpContains: SearchOpContains::<Identity, Impl, OFFSET>,
            SearchOpNotContains: SearchOpNotContains::<Identity, Impl, OFFSET>,
            SearchOpNotEquals: SearchOpNotEquals::<Identity, Impl, OFFSET>,
            UsePDC: UsePDC::<Identity, Impl, OFFSET>,
            UseAnyDC: UseAnyDC::<Identity, Impl, OFFSET>,
            DoNotUseW2KDC: DoNotUseW2KDC::<Identity, Impl, OFFSET>,
            SOMSite: SOMSite::<Identity, Impl, OFFSET>,
            SOMDomain: SOMDomain::<Identity, Impl, OFFSET>,
            SOMOU: SOMOU::<Identity, Impl, OFFSET>,
            SecurityFlags: SecurityFlags::<Identity, Impl, OFFSET>,
            DoNotValidateDC: DoNotValidateDC::<Identity, Impl, OFFSET>,
            ReportHTML: ReportHTML::<Identity, Impl, OFFSET>,
            ReportXML: ReportXML::<Identity, Impl, OFFSET>,
            RSOPModeUnknown: RSOPModeUnknown::<Identity, Impl, OFFSET>,
            RSOPModePlanning: RSOPModePlanning::<Identity, Impl, OFFSET>,
            RSOPModeLogging: RSOPModeLogging::<Identity, Impl, OFFSET>,
            EntryTypeUser: EntryTypeUser::<Identity, Impl, OFFSET>,
            EntryTypeComputer: EntryTypeComputer::<Identity, Impl, OFFSET>,
            EntryTypeLocalGroup: EntryTypeLocalGroup::<Identity, Impl, OFFSET>,
            EntryTypeGlobalGroup: EntryTypeGlobalGroup::<Identity, Impl, OFFSET>,
            EntryTypeUniversalGroup: EntryTypeUniversalGroup::<Identity, Impl, OFFSET>,
            EntryTypeUNCPath: EntryTypeUNCPath::<Identity, Impl, OFFSET>,
            EntryTypeUnknown: EntryTypeUnknown::<Identity, Impl, OFFSET>,
            DestinationOptionSameAsSource: DestinationOptionSameAsSource::<Identity, Impl, OFFSET>,
            DestinationOptionNone: DestinationOptionNone::<Identity, Impl, OFFSET>,
            DestinationOptionByRelativeName: DestinationOptionByRelativeName::<Identity, Impl, OFFSET>,
            DestinationOptionSet: DestinationOptionSet::<Identity, Impl, OFFSET>,
            MigrationTableOnly: MigrationTableOnly::<Identity, Impl, OFFSET>,
            ProcessSecurity: ProcessSecurity::<Identity, Impl, OFFSET>,
            RsopLoggingNoComputer: RsopLoggingNoComputer::<Identity, Impl, OFFSET>,
            RsopLoggingNoUser: RsopLoggingNoUser::<Identity, Impl, OFFSET>,
            RsopPlanningAssumeSlowLink: RsopPlanningAssumeSlowLink::<Identity, Impl, OFFSET>,
            RsopPlanningLoopbackOption: RsopPlanningLoopbackOption::<Identity, Impl, OFFSET>,
            RsopPlanningAssumeUserWQLFilterTrue: RsopPlanningAssumeUserWQLFilterTrue::<Identity, Impl, OFFSET>,
            RsopPlanningAssumeCompWQLFilterTrue: RsopPlanningAssumeCompWQLFilterTrue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMConstants as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMConstants2_Impl: Sized + super::Com::IDispatch_Impl + IGPMConstants_Impl {
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
impl IGPMConstants2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants2_Impl, const OFFSET: isize>() -> IGPMConstants2_Vtbl {
        unsafe extern "system" fn BackupTypeGPO<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMBackupType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BackupTypeGPO() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackupTypeStarterGPO<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMBackupType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BackupTypeStarterGPO() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StarterGPOTypeSystem<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StarterGPOTypeSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StarterGPOTypeCustom<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StarterGPOTypeCustom() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPOPermissions<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchPropertyStarterGPOPermissions() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPOEffectivePermissions<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchPropertyStarterGPOEffectivePermissions() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPODisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchPropertyStarterGPODisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPOID<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchPropertyStarterGPOID() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPODomain<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchPropertyStarterGPODomain() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermStarterGPORead<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PermStarterGPORead() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermStarterGPOEdit<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PermStarterGPOEdit() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermStarterGPOFullControl<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PermStarterGPOFullControl() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermStarterGPOCustom<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PermStarterGPOCustom() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportLegacy<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMReportingOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReportLegacy() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportComments<Identity: ::windows::core::IUnknownImpl, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMReportingOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReportComments() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IGPMConstants_Vtbl::new::<Identity, Impl, OFFSET>(),
            BackupTypeGPO: BackupTypeGPO::<Identity, Impl, OFFSET>,
            BackupTypeStarterGPO: BackupTypeStarterGPO::<Identity, Impl, OFFSET>,
            StarterGPOTypeSystem: StarterGPOTypeSystem::<Identity, Impl, OFFSET>,
            StarterGPOTypeCustom: StarterGPOTypeCustom::<Identity, Impl, OFFSET>,
            SearchPropertyStarterGPOPermissions: SearchPropertyStarterGPOPermissions::<Identity, Impl, OFFSET>,
            SearchPropertyStarterGPOEffectivePermissions: SearchPropertyStarterGPOEffectivePermissions::<Identity, Impl, OFFSET>,
            SearchPropertyStarterGPODisplayName: SearchPropertyStarterGPODisplayName::<Identity, Impl, OFFSET>,
            SearchPropertyStarterGPOID: SearchPropertyStarterGPOID::<Identity, Impl, OFFSET>,
            SearchPropertyStarterGPODomain: SearchPropertyStarterGPODomain::<Identity, Impl, OFFSET>,
            PermStarterGPORead: PermStarterGPORead::<Identity, Impl, OFFSET>,
            PermStarterGPOEdit: PermStarterGPOEdit::<Identity, Impl, OFFSET>,
            PermStarterGPOFullControl: PermStarterGPOFullControl::<Identity, Impl, OFFSET>,
            PermStarterGPOCustom: PermStarterGPOCustom::<Identity, Impl, OFFSET>,
            ReportLegacy: ReportLegacy::<Identity, Impl, OFFSET>,
            ReportComments: ReportComments::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMConstants2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IGPMConstants as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMDomain_Impl: Sized + super::Com::IDispatch_Impl {
    fn DomainController(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Domain(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreateGPO(&mut self) -> ::windows::core::Result<IGPMGPO>;
    fn GetGPO(&mut self, bstrguid: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMGPO>;
    fn SearchGPOs(&mut self, pigpmsearchcriteria: &::core::option::Option<IGPMSearchCriteria>) -> ::windows::core::Result<IGPMGPOCollection>;
    fn RestoreGPO(&mut self, pigpmbackup: &::core::option::Option<IGPMBackup>, ldcflags: i32, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn GetSOM(&mut self, bstrpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMSOM>;
    fn SearchSOMs(&mut self, pigpmsearchcriteria: &::core::option::Option<IGPMSearchCriteria>) -> ::windows::core::Result<IGPMSOMCollection>;
    fn GetWMIFilter(&mut self, bstrpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMWMIFilter>;
    fn SearchWMIFilters(&mut self, pigpmsearchcriteria: &::core::option::Option<IGPMSearchCriteria>) -> ::windows::core::Result<IGPMWMIFilterCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMDomain_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain_Impl, const OFFSET: isize>() -> IGPMDomain_Vtbl {
        unsafe extern "system" fn DomainController<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DomainController() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Domain<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Domain() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGPO<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewgpo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateGPO() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnewgpo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGPO<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppgpo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGPO(::core::mem::transmute_copy(&bstrguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgpo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchGPOs<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmgpocollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchGPOs(::core::mem::transmute(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmgpocollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreGPO<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmbackup: ::windows::core::RawPtr, ldcflags: i32, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RestoreGPO(::core::mem::transmute(&pigpmbackup), ::core::mem::transmute_copy(&ldcflags), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn GetSOM<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsom: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSOM(::core::mem::transmute_copy(&bstrpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsom = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchSOMs<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmsomcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchSOMs(::core::mem::transmute(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmsomcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWMIFilter<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwmifilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetWMIFilter(::core::mem::transmute_copy(&bstrpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwmifilter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchWMIFilters<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmwmifiltercollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchWMIFilters(::core::mem::transmute(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmwmifiltercollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DomainController: DomainController::<Identity, Impl, OFFSET>,
            Domain: Domain::<Identity, Impl, OFFSET>,
            CreateGPO: CreateGPO::<Identity, Impl, OFFSET>,
            GetGPO: GetGPO::<Identity, Impl, OFFSET>,
            SearchGPOs: SearchGPOs::<Identity, Impl, OFFSET>,
            RestoreGPO: RestoreGPO::<Identity, Impl, OFFSET>,
            GetSOM: GetSOM::<Identity, Impl, OFFSET>,
            SearchSOMs: SearchSOMs::<Identity, Impl, OFFSET>,
            GetWMIFilter: GetWMIFilter::<Identity, Impl, OFFSET>,
            SearchWMIFilters: SearchWMIFilters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMDomain as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMDomain2_Impl: Sized + super::Com::IDispatch_Impl + IGPMDomain_Impl {
    fn CreateStarterGPO(&mut self) -> ::windows::core::Result<IGPMStarterGPO>;
    fn CreateGPOFromStarterGPO(&mut self, pgpotemplate: &::core::option::Option<IGPMStarterGPO>) -> ::windows::core::Result<IGPMGPO>;
    fn GetStarterGPO(&mut self, bstrguid: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMStarterGPO>;
    fn SearchStarterGPOs(&mut self, pigpmsearchcriteria: &::core::option::Option<IGPMSearchCriteria>) -> ::windows::core::Result<IGPMStarterGPOCollection>;
    fn LoadStarterGPO(&mut self, bstrloadfile: &super::super::Foundation::BSTR, boverwrite: i16, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn RestoreStarterGPO(&mut self, pigpmtmplbackup: &::core::option::Option<IGPMStarterGPOBackup>, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMDomain2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain2_Impl, const OFFSET: isize>() -> IGPMDomain2_Vtbl {
        unsafe extern "system" fn CreateStarterGPO<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewtemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateStarterGPO() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnewtemplate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGPOFromStarterGPO<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgpotemplate: ::windows::core::RawPtr, ppnewgpo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateGPOFromStarterGPO(::core::mem::transmute(&pgpotemplate)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnewgpo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStarterGPO<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStarterGPO(::core::mem::transmute_copy(&bstrguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptemplate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchStarterGPOs<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmtemplatecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchStarterGPOs(::core::mem::transmute(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmtemplatecollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadStarterGPO<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrloadfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, boverwrite: i16, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadStarterGPO(::core::mem::transmute_copy(&bstrloadfile), ::core::mem::transmute_copy(&boverwrite), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn RestoreStarterGPO<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmtmplbackup: ::windows::core::RawPtr, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RestoreStarterGPO(::core::mem::transmute(&pigpmtmplbackup), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        Self {
            base: IGPMDomain_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateStarterGPO: CreateStarterGPO::<Identity, Impl, OFFSET>,
            CreateGPOFromStarterGPO: CreateGPOFromStarterGPO::<Identity, Impl, OFFSET>,
            GetStarterGPO: GetStarterGPO::<Identity, Impl, OFFSET>,
            SearchStarterGPOs: SearchStarterGPOs::<Identity, Impl, OFFSET>,
            LoadStarterGPO: LoadStarterGPO::<Identity, Impl, OFFSET>,
            RestoreStarterGPO: RestoreStarterGPO::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMDomain2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IGPMDomain as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMDomain3_Impl: Sized + super::Com::IDispatch_Impl + IGPMDomain_Impl + IGPMDomain2_Impl {
    fn GenerateReport(&mut self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn InfrastructureDC(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetInfrastructureDC(&mut self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetInfrastructureFlags(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMDomain3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain3_Impl, const OFFSET: isize>() -> IGPMDomain3_Vtbl {
        unsafe extern "system" fn GenerateReport<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GenerateReport(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn InfrastructureDC<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InfrastructureDC() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInfrastructureDC<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInfrastructureDC(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn SetInfrastructureFlags<Identity: ::windows::core::IUnknownImpl, Impl: IGPMDomain3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInfrastructureFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base: IGPMDomain2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GenerateReport: GenerateReport::<Identity, Impl, OFFSET>,
            InfrastructureDC: InfrastructureDC::<Identity, Impl, OFFSET>,
            SetInfrastructureDC: SetInfrastructureDC::<Identity, Impl, OFFSET>,
            SetInfrastructureFlags: SetInfrastructureFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMDomain3 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IGPMDomain as ::windows::core::Interface>::IID || iid == &<IGPMDomain2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMGPO_Impl: Sized + super::Com::IDispatch_Impl {
    fn DisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDisplayName(&mut self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
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
    fn SetWMIFilter(&mut self, pigpmwmifilter: &::core::option::Option<IGPMWMIFilter>) -> ::windows::core::Result<()>;
    fn SetUserEnabled(&mut self, vbenabled: i16) -> ::windows::core::Result<()>;
    fn SetComputerEnabled(&mut self, vbenabled: i16) -> ::windows::core::Result<()>;
    fn IsUserEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn IsComputerEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn GetSecurityInfo(&mut self) -> ::windows::core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(&mut self, psecurityinfo: &::core::option::Option<IGPMSecurityInfo>) -> ::windows::core::Result<()>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn Backup(&mut self, bstrbackupdir: &super::super::Foundation::BSTR, bstrcomment: &super::super::Foundation::BSTR, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn Import(&mut self, lflags: i32, pigpmbackup: &::core::option::Option<IGPMBackup>, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn GenerateReport(&mut self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn GenerateReportToFile(&mut self, gpmreporttype: GPMReportType, bstrtargetfilepath: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMResult>;
    fn CopyTo(&mut self, lflags: i32, pigpmdomain: &::core::option::Option<IGPMDomain>, pvarnewdisplayname: *const super::Com::VARIANT, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn SetSecurityDescriptor(&mut self, lflags: i32, psd: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn GetSecurityDescriptor(&mut self, lflags: i32) -> ::windows::core::Result<super::Com::IDispatch>;
    fn IsACLConsistent(&mut self) -> ::windows::core::Result<i16>;
    fn MakeACLConsistent(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPO_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>() -> IGPMGPO_Vtbl {
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ID<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ID() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainName<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DomainName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreationTime<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModificationTime<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ModificationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDSVersionNumber<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserDSVersionNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerDSVersionNumber<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ComputerDSVersionNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserSysvolVersionNumber<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserSysvolVersionNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerSysvolVersionNumber<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ComputerSysvolVersionNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWMIFilter<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmwmifilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetWMIFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmwmifilter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWMIFilter<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmwmifilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWMIFilter(::core::mem::transmute(&pigpmwmifilter)).into()
        }
        unsafe extern "system" fn SetUserEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUserEnabled(::core::mem::transmute_copy(&vbenabled)).into()
        }
        unsafe extern "system" fn SetComputerEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetComputerEnabled(::core::mem::transmute_copy(&vbenabled)).into()
        }
        unsafe extern "system" fn IsUserEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsUserEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pvbenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsComputerEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsComputerEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pvbenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityInfo<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSecurityInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsecurityinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityInfo<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecurityinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSecurityInfo(::core::mem::transmute(&psecurityinfo)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Backup<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrcomment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Backup(::core::mem::transmute_copy(&bstrbackupdir), ::core::mem::transmute_copy(&bstrcomment), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn Import<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pigpmbackup: ::windows::core::RawPtr, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Import(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pigpmbackup), ::core::mem::transmute_copy(&pvarmigrationtable), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn GenerateReport<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GenerateReport(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GenerateReportToFile(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&bstrtargetfilepath)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTo<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pigpmdomain: ::windows::core::RawPtr, pvarnewdisplayname: *const super::Com::VARIANT, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyTo(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pigpmdomain), ::core::mem::transmute_copy(&pvarnewdisplayname), ::core::mem::transmute_copy(&pvarmigrationtable), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, psd: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSecurityDescriptor(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&psd)).into()
        }
        unsafe extern "system" fn GetSecurityDescriptor<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, ppsd: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSecurityDescriptor(::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsACLConsistent<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbconsistent: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsACLConsistent() {
                ::core::result::Result::Ok(ok__) => {
                    *pvbconsistent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeACLConsistent<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MakeACLConsistent().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            ID: ID::<Identity, Impl, OFFSET>,
            DomainName: DomainName::<Identity, Impl, OFFSET>,
            CreationTime: CreationTime::<Identity, Impl, OFFSET>,
            ModificationTime: ModificationTime::<Identity, Impl, OFFSET>,
            UserDSVersionNumber: UserDSVersionNumber::<Identity, Impl, OFFSET>,
            ComputerDSVersionNumber: ComputerDSVersionNumber::<Identity, Impl, OFFSET>,
            UserSysvolVersionNumber: UserSysvolVersionNumber::<Identity, Impl, OFFSET>,
            ComputerSysvolVersionNumber: ComputerSysvolVersionNumber::<Identity, Impl, OFFSET>,
            GetWMIFilter: GetWMIFilter::<Identity, Impl, OFFSET>,
            SetWMIFilter: SetWMIFilter::<Identity, Impl, OFFSET>,
            SetUserEnabled: SetUserEnabled::<Identity, Impl, OFFSET>,
            SetComputerEnabled: SetComputerEnabled::<Identity, Impl, OFFSET>,
            IsUserEnabled: IsUserEnabled::<Identity, Impl, OFFSET>,
            IsComputerEnabled: IsComputerEnabled::<Identity, Impl, OFFSET>,
            GetSecurityInfo: GetSecurityInfo::<Identity, Impl, OFFSET>,
            SetSecurityInfo: SetSecurityInfo::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Backup: Backup::<Identity, Impl, OFFSET>,
            Import: Import::<Identity, Impl, OFFSET>,
            GenerateReport: GenerateReport::<Identity, Impl, OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Identity, Impl, OFFSET>,
            CopyTo: CopyTo::<Identity, Impl, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, Impl, OFFSET>,
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, Impl, OFFSET>,
            IsACLConsistent: IsACLConsistent::<Identity, Impl, OFFSET>,
            MakeACLConsistent: MakeACLConsistent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMGPO as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMGPO2_Impl: Sized + super::Com::IDispatch_Impl + IGPMGPO_Impl {
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPO2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO2_Impl, const OFFSET: isize>() -> IGPMGPO2_Vtbl {
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base: IGPMGPO_Vtbl::new::<Identity, Impl, OFFSET>(),
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMGPO2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IGPMGPO as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMGPO3_Impl: Sized + super::Com::IDispatch_Impl + IGPMGPO_Impl + IGPMGPO2_Impl {
    fn InfrastructureDC(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetInfrastructureDC(&mut self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetInfrastructureFlags(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPO3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO3_Impl, const OFFSET: isize>() -> IGPMGPO3_Vtbl {
        unsafe extern "system" fn InfrastructureDC<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InfrastructureDC() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInfrastructureDC<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInfrastructureDC(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn SetInfrastructureFlags<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPO3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInfrastructureFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base: IGPMGPO2_Vtbl::new::<Identity, Impl, OFFSET>(),
            InfrastructureDC: InfrastructureDC::<Identity, Impl, OFFSET>,
            SetInfrastructureDC: SetInfrastructureDC::<Identity, Impl, OFFSET>,
            SetInfrastructureFlags: SetInfrastructureFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMGPO3 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IGPMGPO as ::windows::core::Interface>::IID || iid == &<IGPMGPO2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMGPOCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPOCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOCollection_Impl, const OFFSET: isize>() -> IGPMGPOCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmgpos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmgpos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMGPOCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMGPOLink_Impl: Sized + super::Com::IDispatch_Impl {
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
impl IGPMGPOLink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOLink_Impl, const OFFSET: isize>() -> IGPMGPOLink_Vtbl {
        unsafe extern "system" fn GPOID<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GPOID() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GPODomain<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GPODomain() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Enforced<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enforced() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnforced<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnforced(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn SOMLinkOrder<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SOMLinkOrder() {
                ::core::result::Result::Ok(ok__) => {
                    *lval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SOM<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmsom: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SOM() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmsom = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GPOID: GPOID::<Identity, Impl, OFFSET>,
            GPODomain: GPODomain::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            Enforced: Enforced::<Identity, Impl, OFFSET>,
            SetEnforced: SetEnforced::<Identity, Impl, OFFSET>,
            SOMLinkOrder: SOMLinkOrder::<Identity, Impl, OFFSET>,
            SOM: SOM::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMGPOLink as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMGPOLinksCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPOLinksCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOLinksCollection_Impl, const OFFSET: isize>() -> IGPMGPOLinksCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOLinksCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOLinksCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IGPMGPOLinksCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmlinks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmlinks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMGPOLinksCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMMapEntry_Impl: Sized + super::Com::IDispatch_Impl {
    fn Source(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Destination(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DestinationOption(&mut self) -> ::windows::core::Result<GPMDestinationOption>;
    fn EntryType(&mut self) -> ::windows::core::Result<GPMEntryType>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMMapEntry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMapEntry_Impl, const OFFSET: isize>() -> IGPMMapEntry_Vtbl {
        unsafe extern "system" fn Source<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMapEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destination<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMapEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdestination: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Destination() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdestination = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationOption<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMapEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgpmdestoption: *mut GPMDestinationOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DestinationOption() {
                ::core::result::Result::Ok(ok__) => {
                    *pgpmdestoption = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryType<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMapEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgpmentrytype: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EntryType() {
                ::core::result::Result::Ok(ok__) => {
                    *pgpmentrytype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Source: Source::<Identity, Impl, OFFSET>,
            Destination: Destination::<Identity, Impl, OFFSET>,
            DestinationOption: DestinationOption::<Identity, Impl, OFFSET>,
            EntryType: EntryType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMMapEntry as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMMapEntryCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMMapEntryCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMapEntryCollection_Impl, const OFFSET: isize>() -> IGPMMapEntryCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMapEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMapEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMapEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMMapEntryCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMMigrationTable_Impl: Sized + super::Com::IDispatch_Impl {
    fn Save(&mut self, bstrmigrationtablepath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Add(&mut self, lflags: i32, var: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddEntry(&mut self, bstrsource: &super::super::Foundation::BSTR, gpmentrytype: GPMEntryType, pvardestination: *const super::Com::VARIANT) -> ::windows::core::Result<IGPMMapEntry>;
    fn GetEntry(&mut self, bstrsource: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMMapEntry>;
    fn DeleteEntry(&mut self, bstrsource: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn UpdateDestination(&mut self, bstrsource: &super::super::Foundation::BSTR, pvardestination: *const super::Com::VARIANT) -> ::windows::core::Result<IGPMMapEntry>;
    fn Validate(&mut self) -> ::windows::core::Result<IGPMResult>;
    fn GetEntries(&mut self) -> ::windows::core::Result<IGPMMapEntryCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMMigrationTable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>() -> IGPMMigrationTable_Vtbl {
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmigrationtablepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Save(::core::mem::transmute_copy(&bstrmigrationtablepath)).into()
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, var: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&var)).into()
        }
        unsafe extern "system" fn AddEntry<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, gpmentrytype: GPMEntryType, pvardestination: *const super::Com::VARIANT, ppentry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddEntry(::core::mem::transmute_copy(&bstrsource), ::core::mem::transmute_copy(&gpmentrytype), ::core::mem::transmute_copy(&pvardestination)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppentry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntry<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppentry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEntry(::core::mem::transmute_copy(&bstrsource)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppentry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteEntry<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteEntry(::core::mem::transmute_copy(&bstrsource)).into()
        }
        unsafe extern "system" fn UpdateDestination<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvardestination: *const super::Com::VARIANT, ppentry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UpdateDestination(::core::mem::transmute_copy(&bstrsource), ::core::mem::transmute_copy(&pvardestination)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppentry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Validate<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Validate() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntries<Identity: ::windows::core::IUnknownImpl, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppentries: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEntries() {
                ::core::result::Result::Ok(ok__) => {
                    *ppentries = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Save: Save::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            AddEntry: AddEntry::<Identity, Impl, OFFSET>,
            GetEntry: GetEntry::<Identity, Impl, OFFSET>,
            DeleteEntry: DeleteEntry::<Identity, Impl, OFFSET>,
            UpdateDestination: UpdateDestination::<Identity, Impl, OFFSET>,
            Validate: Validate::<Identity, Impl, OFFSET>,
            GetEntries: GetEntries::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMMigrationTable as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMPermission_Impl: Sized + super::Com::IDispatch_Impl {
    fn Inherited(&mut self) -> ::windows::core::Result<i16>;
    fn Inheritable(&mut self) -> ::windows::core::Result<i16>;
    fn Denied(&mut self) -> ::windows::core::Result<i16>;
    fn Permission(&mut self) -> ::windows::core::Result<GPMPermissionType>;
    fn Trustee(&mut self) -> ::windows::core::Result<IGPMTrustee>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMPermission_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMPermission_Impl, const OFFSET: isize>() -> IGPMPermission_Vtbl {
        unsafe extern "system" fn Inherited<Identity: ::windows::core::IUnknownImpl, Impl: IGPMPermission_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Inherited() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Inheritable<Identity: ::windows::core::IUnknownImpl, Impl: IGPMPermission_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Inheritable() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Denied<Identity: ::windows::core::IUnknownImpl, Impl: IGPMPermission_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Denied() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Permission<Identity: ::windows::core::IUnknownImpl, Impl: IGPMPermission_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Permission() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Trustee<Identity: ::windows::core::IUnknownImpl, Impl: IGPMPermission_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmtrustee: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Trustee() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmtrustee = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Inherited: Inherited::<Identity, Impl, OFFSET>,
            Inheritable: Inheritable::<Identity, Impl, OFFSET>,
            Denied: Denied::<Identity, Impl, OFFSET>,
            Permission: Permission::<Identity, Impl, OFFSET>,
            Trustee: Trustee::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMPermission as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMRSOP_Impl: Sized + super::Com::IDispatch_Impl {
    fn Mode(&mut self) -> ::windows::core::Result<GPMRSOPMode>;
    fn Namespace(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLoggingComputer(&mut self, bstrval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LoggingComputer(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLoggingUser(&mut self, bstrval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LoggingUser(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLoggingFlags(&mut self, lval: i32) -> ::windows::core::Result<()>;
    fn LoggingFlags(&mut self) -> ::windows::core::Result<i32>;
    fn SetPlanningFlags(&mut self, lval: i32) -> ::windows::core::Result<()>;
    fn PlanningFlags(&mut self) -> ::windows::core::Result<i32>;
    fn SetPlanningDomainController(&mut self, bstrval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PlanningDomainController(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPlanningSiteName(&mut self, bstrval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PlanningSiteName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPlanningUser(&mut self, bstrval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PlanningUser(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPlanningUserSOM(&mut self, bstrval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PlanningUserSOM(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPlanningUserWMIFilters(&mut self, varval: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PlanningUserWMIFilters(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetPlanningUserSecurityGroups(&mut self, varval: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PlanningUserSecurityGroups(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetPlanningComputer(&mut self, bstrval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PlanningComputer(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPlanningComputerSOM(&mut self, bstrval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PlanningComputerSOM(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPlanningComputerWMIFilters(&mut self, varval: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PlanningComputerWMIFilters(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetPlanningComputerSecurityGroups(&mut self, varval: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PlanningComputerSecurityGroups(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn LoggingEnumerateUsers(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn CreateQueryResults(&mut self) -> ::windows::core::Result<()>;
    fn ReleaseQueryResults(&mut self) -> ::windows::core::Result<()>;
    fn GenerateReport(&mut self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn GenerateReportToFile(&mut self, gpmreporttype: GPMReportType, bstrtargetfilepath: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMRSOP_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>() -> IGPMRSOP_Vtbl {
        unsafe extern "system" fn Mode<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Namespace<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Namespace() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoggingComputer<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLoggingComputer(::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn LoggingComputer<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LoggingComputer() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoggingUser<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLoggingUser(::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn LoggingUser<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LoggingUser() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoggingFlags<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLoggingFlags(::core::mem::transmute_copy(&lval)).into()
        }
        unsafe extern "system" fn LoggingFlags<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LoggingFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *lval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningFlags<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPlanningFlags(::core::mem::transmute_copy(&lval)).into()
        }
        unsafe extern "system" fn PlanningFlags<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PlanningFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *lval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningDomainController<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPlanningDomainController(::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningDomainController<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PlanningDomainController() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningSiteName<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPlanningSiteName(::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningSiteName<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PlanningSiteName() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningUser<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPlanningUser(::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningUser<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PlanningUser() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningUserSOM<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPlanningUserSOM(::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningUserSOM<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PlanningUserSOM() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningUserWMIFilters<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPlanningUserWMIFilters(::core::mem::transmute_copy(&varval)).into()
        }
        unsafe extern "system" fn PlanningUserWMIFilters<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PlanningUserWMIFilters() {
                ::core::result::Result::Ok(ok__) => {
                    *varval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningUserSecurityGroups<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPlanningUserSecurityGroups(::core::mem::transmute_copy(&varval)).into()
        }
        unsafe extern "system" fn PlanningUserSecurityGroups<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PlanningUserSecurityGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *varval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningComputer<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPlanningComputer(::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningComputer<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PlanningComputer() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningComputerSOM<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPlanningComputerSOM(::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningComputerSOM<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PlanningComputerSOM() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningComputerWMIFilters<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPlanningComputerWMIFilters(::core::mem::transmute_copy(&varval)).into()
        }
        unsafe extern "system" fn PlanningComputerWMIFilters<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PlanningComputerWMIFilters() {
                ::core::result::Result::Ok(ok__) => {
                    *varval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningComputerSecurityGroups<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPlanningComputerSecurityGroups(::core::mem::transmute_copy(&varval)).into()
        }
        unsafe extern "system" fn PlanningComputerSecurityGroups<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PlanningComputerSecurityGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *varval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoggingEnumerateUsers<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LoggingEnumerateUsers() {
                ::core::result::Result::Ok(ok__) => {
                    *varval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryResults<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateQueryResults().into()
        }
        unsafe extern "system" fn ReleaseQueryResults<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseQueryResults().into()
        }
        unsafe extern "system" fn GenerateReport<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GenerateReport(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: ::windows::core::IUnknownImpl, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GenerateReportToFile(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&bstrtargetfilepath)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Mode: Mode::<Identity, Impl, OFFSET>,
            Namespace: Namespace::<Identity, Impl, OFFSET>,
            SetLoggingComputer: SetLoggingComputer::<Identity, Impl, OFFSET>,
            LoggingComputer: LoggingComputer::<Identity, Impl, OFFSET>,
            SetLoggingUser: SetLoggingUser::<Identity, Impl, OFFSET>,
            LoggingUser: LoggingUser::<Identity, Impl, OFFSET>,
            SetLoggingFlags: SetLoggingFlags::<Identity, Impl, OFFSET>,
            LoggingFlags: LoggingFlags::<Identity, Impl, OFFSET>,
            SetPlanningFlags: SetPlanningFlags::<Identity, Impl, OFFSET>,
            PlanningFlags: PlanningFlags::<Identity, Impl, OFFSET>,
            SetPlanningDomainController: SetPlanningDomainController::<Identity, Impl, OFFSET>,
            PlanningDomainController: PlanningDomainController::<Identity, Impl, OFFSET>,
            SetPlanningSiteName: SetPlanningSiteName::<Identity, Impl, OFFSET>,
            PlanningSiteName: PlanningSiteName::<Identity, Impl, OFFSET>,
            SetPlanningUser: SetPlanningUser::<Identity, Impl, OFFSET>,
            PlanningUser: PlanningUser::<Identity, Impl, OFFSET>,
            SetPlanningUserSOM: SetPlanningUserSOM::<Identity, Impl, OFFSET>,
            PlanningUserSOM: PlanningUserSOM::<Identity, Impl, OFFSET>,
            SetPlanningUserWMIFilters: SetPlanningUserWMIFilters::<Identity, Impl, OFFSET>,
            PlanningUserWMIFilters: PlanningUserWMIFilters::<Identity, Impl, OFFSET>,
            SetPlanningUserSecurityGroups: SetPlanningUserSecurityGroups::<Identity, Impl, OFFSET>,
            PlanningUserSecurityGroups: PlanningUserSecurityGroups::<Identity, Impl, OFFSET>,
            SetPlanningComputer: SetPlanningComputer::<Identity, Impl, OFFSET>,
            PlanningComputer: PlanningComputer::<Identity, Impl, OFFSET>,
            SetPlanningComputerSOM: SetPlanningComputerSOM::<Identity, Impl, OFFSET>,
            PlanningComputerSOM: PlanningComputerSOM::<Identity, Impl, OFFSET>,
            SetPlanningComputerWMIFilters: SetPlanningComputerWMIFilters::<Identity, Impl, OFFSET>,
            PlanningComputerWMIFilters: PlanningComputerWMIFilters::<Identity, Impl, OFFSET>,
            SetPlanningComputerSecurityGroups: SetPlanningComputerSecurityGroups::<Identity, Impl, OFFSET>,
            PlanningComputerSecurityGroups: PlanningComputerSecurityGroups::<Identity, Impl, OFFSET>,
            LoggingEnumerateUsers: LoggingEnumerateUsers::<Identity, Impl, OFFSET>,
            CreateQueryResults: CreateQueryResults::<Identity, Impl, OFFSET>,
            ReleaseQueryResults: ReleaseQueryResults::<Identity, Impl, OFFSET>,
            GenerateReport: GenerateReport::<Identity, Impl, OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMRSOP as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMResult_Impl: Sized + super::Com::IDispatch_Impl {
    fn Status(&mut self) -> ::windows::core::Result<IGPMStatusMsgCollection>;
    fn Result(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn OverallStatus(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMResult_Impl, const OFFSET: isize>() -> IGPMResult_Vtbl {
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl, Impl: IGPMResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmstatusmsgcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmstatusmsgcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Result<Identity: ::windows::core::IUnknownImpl, Impl: IGPMResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarresult: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Result() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverallStatus<Identity: ::windows::core::IUnknownImpl, Impl: IGPMResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OverallStatus().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Status: Status::<Identity, Impl, OFFSET>,
            Result: Result::<Identity, Impl, OFFSET>,
            OverallStatus: OverallStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMResult as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMSOM_Impl: Sized + super::Com::IDispatch_Impl {
    fn GPOInheritanceBlocked(&mut self) -> ::windows::core::Result<i16>;
    fn SetGPOInheritanceBlocked(&mut self, newval: i16) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreateGPOLink(&mut self, llinkpos: i32, pgpo: &::core::option::Option<IGPMGPO>) -> ::windows::core::Result<IGPMGPOLink>;
    fn Type(&mut self) -> ::windows::core::Result<GPMSOMType>;
    fn GetGPOLinks(&mut self) -> ::windows::core::Result<IGPMGPOLinksCollection>;
    fn GetInheritedGPOLinks(&mut self) -> ::windows::core::Result<IGPMGPOLinksCollection>;
    fn GetSecurityInfo(&mut self) -> ::windows::core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(&mut self, psecurityinfo: &::core::option::Option<IGPMSecurityInfo>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMSOM_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSOM_Impl, const OFFSET: isize>() -> IGPMSOM_Vtbl {
        unsafe extern "system" fn GPOInheritanceBlocked<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GPOInheritanceBlocked() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGPOInheritanceBlocked<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGPOInheritanceBlocked(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGPOLink<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llinkpos: i32, pgpo: ::windows::core::RawPtr, ppnewgpolink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateGPOLink(::core::mem::transmute_copy(&llinkpos), ::core::mem::transmute(&pgpo)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnewgpolink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGPOLinks<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgpolinks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGPOLinks() {
                ::core::result::Result::Ok(ok__) => {
                    *ppgpolinks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInheritedGPOLinks<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgpolinks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInheritedGPOLinks() {
                ::core::result::Result::Ok(ok__) => {
                    *ppgpolinks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityInfo<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSecurityInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsecurityinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityInfo<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecurityinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSecurityInfo(::core::mem::transmute(&psecurityinfo)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GPOInheritanceBlocked: GPOInheritanceBlocked::<Identity, Impl, OFFSET>,
            SetGPOInheritanceBlocked: SetGPOInheritanceBlocked::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            CreateGPOLink: CreateGPOLink::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            GetGPOLinks: GetGPOLinks::<Identity, Impl, OFFSET>,
            GetInheritedGPOLinks: GetInheritedGPOLinks::<Identity, Impl, OFFSET>,
            GetSecurityInfo: GetSecurityInfo::<Identity, Impl, OFFSET>,
            SetSecurityInfo: SetSecurityInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMSOM as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMSOMCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMSOMCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSOMCollection_Impl, const OFFSET: isize>() -> IGPMSOMCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSOMCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSOMCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSOMCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmsom: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmsom = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMSOMCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMSearchCriteria_Impl: Sized + super::Com::IDispatch_Impl {
    fn Add(&mut self, searchproperty: GPMSearchProperty, searchoperation: GPMSearchOperation, varvalue: &super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMSearchCriteria_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSearchCriteria_Impl, const OFFSET: isize>() -> IGPMSearchCriteria_Vtbl {
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSearchCriteria_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, searchproperty: GPMSearchProperty, searchoperation: GPMSearchOperation, varvalue: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&searchproperty), ::core::mem::transmute_copy(&searchoperation), ::core::mem::transmute_copy(&varvalue)).into()
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Add: Add::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMSearchCriteria as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMSecurityInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
    fn Add(&mut self, pperm: &::core::option::Option<IGPMPermission>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, pperm: &::core::option::Option<IGPMPermission>) -> ::windows::core::Result<()>;
    fn RemoveTrustee(&mut self, bstrtrustee: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMSecurityInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSecurityInfo_Impl, const OFFSET: isize>() -> IGPMSecurityInfo_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute(&pperm)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute(&pperm)).into()
        }
        unsafe extern "system" fn RemoveTrustee<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveTrustee(::core::mem::transmute_copy(&bstrtrustee)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveTrustee: RemoveTrustee::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMSecurityInfo as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMSitesContainer_Impl: Sized + super::Com::IDispatch_Impl {
    fn DomainController(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Domain(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Forest(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetSite(&mut self, bstrsitename: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMSOM>;
    fn SearchSites(&mut self, pigpmsearchcriteria: &::core::option::Option<IGPMSearchCriteria>) -> ::windows::core::Result<IGPMSOMCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMSitesContainer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSitesContainer_Impl, const OFFSET: isize>() -> IGPMSitesContainer_Vtbl {
        unsafe extern "system" fn DomainController<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSitesContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DomainController() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Domain<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSitesContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Domain() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Forest<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSitesContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Forest() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSite<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSitesContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsitename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsom: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSite(::core::mem::transmute_copy(&bstrsitename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsom = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchSites<Identity: ::windows::core::IUnknownImpl, Impl: IGPMSitesContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmsomcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchSites(::core::mem::transmute(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmsomcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DomainController: DomainController::<Identity, Impl, OFFSET>,
            Domain: Domain::<Identity, Impl, OFFSET>,
            Forest: Forest::<Identity, Impl, OFFSET>,
            GetSite: GetSite::<Identity, Impl, OFFSET>,
            SearchSites: SearchSites::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMSitesContainer as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMStarterGPO_Impl: Sized + super::Com::IDispatch_Impl {
    fn DisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDisplayName(&mut self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
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
    fn Save(&mut self, bstrsavefile: &super::super::Foundation::BSTR, boverwrite: i16, bsaveassystem: i16, bstrlanguage: *const super::Com::VARIANT, bstrauthor: *const super::Com::VARIANT, bstrproduct: *const super::Com::VARIANT, bstruniqueid: *const super::Com::VARIANT, bstrversion: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn Backup(&mut self, bstrbackupdir: &super::super::Foundation::BSTR, bstrcomment: &super::super::Foundation::BSTR, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn CopyTo(&mut self, pvarnewdisplayname: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT) -> ::windows::core::Result<IGPMResult>;
    fn GenerateReport(&mut self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT) -> ::windows::core::Result<IGPMResult>;
    fn GenerateReportToFile(&mut self, gpmreporttype: GPMReportType, bstrtargetfilepath: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMResult>;
    fn GetSecurityInfo(&mut self) -> ::windows::core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(&mut self, psecurityinfo: &::core::option::Option<IGPMSecurityInfo>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStarterGPO_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>() -> IGPMStarterGPO_Vtbl {
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Author<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Author() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Product<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Product() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreationTime<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ID<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ID() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifiedTime<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ModifiedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerVersion<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ComputerVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserVersion<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StarterGPOVersion<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StarterGPOVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsavefile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, boverwrite: i16, bsaveassystem: i16, bstrlanguage: *const super::Com::VARIANT, bstrauthor: *const super::Com::VARIANT, bstrproduct: *const super::Com::VARIANT, bstruniqueid: *const super::Com::VARIANT, bstrversion: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        unsafe extern "system" fn Backup<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrcomment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Backup(::core::mem::transmute_copy(&bstrbackupdir), ::core::mem::transmute_copy(&bstrcomment), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn CopyTo<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarnewdisplayname: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CopyTo(::core::mem::transmute_copy(&pvarnewdisplayname), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateReport<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GenerateReport(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GenerateReportToFile(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&bstrtargetfilepath)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityInfo<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSecurityInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsecurityinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityInfo<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecurityinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSecurityInfo(::core::mem::transmute(&psecurityinfo)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Author: Author::<Identity, Impl, OFFSET>,
            Product: Product::<Identity, Impl, OFFSET>,
            CreationTime: CreationTime::<Identity, Impl, OFFSET>,
            ID: ID::<Identity, Impl, OFFSET>,
            ModifiedTime: ModifiedTime::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            ComputerVersion: ComputerVersion::<Identity, Impl, OFFSET>,
            UserVersion: UserVersion::<Identity, Impl, OFFSET>,
            StarterGPOVersion: StarterGPOVersion::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            Backup: Backup::<Identity, Impl, OFFSET>,
            CopyTo: CopyTo::<Identity, Impl, OFFSET>,
            GenerateReport: GenerateReport::<Identity, Impl, OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Identity, Impl, OFFSET>,
            GetSecurityInfo: GetSecurityInfo::<Identity, Impl, OFFSET>,
            SetSecurityInfo: SetSecurityInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMStarterGPO as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMStarterGPOBackup_Impl: Sized + super::Com::IDispatch_Impl {
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
    fn GenerateReportToFile(&mut self, gpmreporttype: GPMReportType, bstrtargetfilepath: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStarterGPOBackup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>() -> IGPMStarterGPOBackup_Vtbl {
        unsafe extern "system" fn BackupDir<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbackupdir: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BackupDir() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrbackupdir = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comment<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcomment: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Comment() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcomment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdisplayname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Domain<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtemplatedomain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Domain() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtemplatedomain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StarterGPOID<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtemplateid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StarterGPOID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtemplateid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ID<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimestamp: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *ptimestamp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut GPMStarterGPOType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn GenerateReport<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GenerateReport(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GenerateReportToFile(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&bstrtargetfilepath)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            BackupDir: BackupDir::<Identity, Impl, OFFSET>,
            Comment: Comment::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            Domain: Domain::<Identity, Impl, OFFSET>,
            StarterGPOID: StarterGPOID::<Identity, Impl, OFFSET>,
            ID: ID::<Identity, Impl, OFFSET>,
            Timestamp: Timestamp::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GenerateReport: GenerateReport::<Identity, Impl, OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMStarterGPOBackup as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMStarterGPOBackupCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStarterGPOBackupCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOBackupCollection_Impl, const OFFSET: isize>() -> IGPMStarterGPOBackupCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOBackupCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOBackupCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOBackupCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmtmplbackup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmtmplbackup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMStarterGPOBackupCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMStarterGPOCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStarterGPOCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOCollection_Impl, const OFFSET: isize>() -> IGPMStarterGPOCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStarterGPOCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmtemplates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppigpmtemplates = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMStarterGPOCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMStatusMessage_Impl: Sized + super::Com::IDispatch_Impl {
    fn ObjectPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ErrorCode(&mut self) -> ::windows::core::Result<()>;
    fn ExtensionName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SettingsName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn OperationCode(&mut self) -> ::windows::core::Result<()>;
    fn Message(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStatusMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStatusMessage_Impl, const OFFSET: isize>() -> IGPMStatusMessage_Vtbl {
        unsafe extern "system" fn ObjectPath<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ObjectPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ErrorCode().into()
        }
        unsafe extern "system" fn ExtensionName<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExtensionName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettingsName<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SettingsName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OperationCode<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OperationCode().into()
        }
        unsafe extern "system" fn Message<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ObjectPath: ObjectPath::<Identity, Impl, OFFSET>,
            ErrorCode: ErrorCode::<Identity, Impl, OFFSET>,
            ExtensionName: ExtensionName::<Identity, Impl, OFFSET>,
            SettingsName: SettingsName::<Identity, Impl, OFFSET>,
            OperationCode: OperationCode::<Identity, Impl, OFFSET>,
            Message: Message::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMStatusMessage as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMStatusMsgCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStatusMsgCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStatusMsgCollection_Impl, const OFFSET: isize>() -> IGPMStatusMsgCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStatusMsgCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStatusMsgCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IGPMStatusMsgCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMStatusMsgCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMTrustee_Impl: Sized + super::Com::IDispatch_Impl {
    fn TrusteeSid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TrusteeName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TrusteeDomain(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TrusteeDSPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TrusteeType(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMTrustee_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMTrustee_Impl, const OFFSET: isize>() -> IGPMTrustee_Vtbl {
        unsafe extern "system" fn TrusteeSid<Identity: ::windows::core::IUnknownImpl, Impl: IGPMTrustee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TrusteeSid() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrusteeName<Identity: ::windows::core::IUnknownImpl, Impl: IGPMTrustee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TrusteeName() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrusteeDomain<Identity: ::windows::core::IUnknownImpl, Impl: IGPMTrustee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TrusteeDomain() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrusteeDSPath<Identity: ::windows::core::IUnknownImpl, Impl: IGPMTrustee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TrusteeDSPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrusteeType<Identity: ::windows::core::IUnknownImpl, Impl: IGPMTrustee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TrusteeType() {
                ::core::result::Result::Ok(ok__) => {
                    *lval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            TrusteeSid: TrusteeSid::<Identity, Impl, OFFSET>,
            TrusteeName: TrusteeName::<Identity, Impl, OFFSET>,
            TrusteeDomain: TrusteeDomain::<Identity, Impl, OFFSET>,
            TrusteeDSPath: TrusteeDSPath::<Identity, Impl, OFFSET>,
            TrusteeType: TrusteeType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMTrustee as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMWMIFilter_Impl: Sized + super::Com::IDispatch_Impl {
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetQueryList(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn GetSecurityInfo(&mut self) -> ::windows::core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(&mut self, psecurityinfo: &::core::option::Option<IGPMSecurityInfo>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMWMIFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>() -> IGPMWMIFilter_Vtbl {
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQueryList<Identity: ::windows::core::IUnknownImpl, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqrylist: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetQueryList() {
                ::core::result::Result::Ok(ok__) => {
                    *pqrylist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityInfo<Identity: ::windows::core::IUnknownImpl, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSecurityInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsecurityinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityInfo<Identity: ::windows::core::IUnknownImpl, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecurityinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSecurityInfo(::core::mem::transmute(&psecurityinfo)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Path: Path::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            GetQueryList: GetQueryList::<Identity, Impl, OFFSET>,
            GetSecurityInfo: GetSecurityInfo::<Identity, Impl, OFFSET>,
            SetSecurityInfo: SetSecurityInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMWMIFilter as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMWMIFilterCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMWMIFilterCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGPMWMIFilterCollection_Impl, const OFFSET: isize>() -> IGPMWMIFilterCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IGPMWMIFilterCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IGPMWMIFilterCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IGPMWMIFilterCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMWMIFilterCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_Controls"))]
pub trait IGroupPolicyObject_Impl: Sized {
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
impl IGroupPolicyObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>() -> IGroupPolicyObject_Vtbl {
        unsafe extern "system" fn New<Identity: ::windows::core::IUnknownImpl, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdomainname: super::super::Foundation::PWSTR, pszdisplayname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).New(::core::mem::transmute_copy(&pszdomainname), ::core::mem::transmute_copy(&pszdisplayname), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OpenDSGPO<Identity: ::windows::core::IUnknownImpl, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenDSGPO(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OpenLocalMachineGPO<Identity: ::windows::core::IUnknownImpl, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenLocalMachineGPO(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OpenRemoteMachineGPO<Identity: ::windows::core::IUnknownImpl, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcomputername: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenRemoteMachineGPO(::core::mem::transmute_copy(&pszcomputername), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows::core::GUID, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Save(::core::mem::transmute_copy(&bmachine), ::core::mem::transmute_copy(&badd), ::core::mem::transmute_copy(&pguidextension), ::core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetName(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetDisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDisplayName(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute_copy(&pszname)).into()
        }
        unsafe extern "system" fn GetPath<Identity: ::windows::core::IUnknownImpl, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPath(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetDSPath<Identity: ::windows::core::IUnknownImpl, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDSPath(::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&cchmaxpath)).into()
        }
        unsafe extern "system" fn GetFileSysPath<Identity: ::windows::core::IUnknownImpl, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFileSysPath(::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&cchmaxpath)).into()
        }
        unsafe extern "system" fn GetRegistryKey<Identity: ::windows::core::IUnknownImpl, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRegistryKey(::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&hkey)).into()
        }
        unsafe extern "system" fn GetOptions<Identity: ::windows::core::IUnknownImpl, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOptions(::core::mem::transmute_copy(&dwoptions)).into()
        }
        unsafe extern "system" fn SetOptions<Identity: ::windows::core::IUnknownImpl, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: u32, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOptions(::core::mem::transmute_copy(&dwoptions), ::core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetType(::core::mem::transmute_copy(&gpotype)).into()
        }
        unsafe extern "system" fn GetMachineName<Identity: ::windows::core::IUnknownImpl, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMachineName(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetPropertySheetPages<Identity: ::windows::core::IUnknownImpl, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpages: *mut *mut super::super::UI::Controls::HPROPSHEETPAGE, upagecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPropertySheetPages(::core::mem::transmute_copy(&hpages), ::core::mem::transmute_copy(&upagecount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            New: New::<Identity, Impl, OFFSET>,
            OpenDSGPO: OpenDSGPO::<Identity, Impl, OFFSET>,
            OpenLocalMachineGPO: OpenLocalMachineGPO::<Identity, Impl, OFFSET>,
            OpenRemoteMachineGPO: OpenRemoteMachineGPO::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            GetDSPath: GetDSPath::<Identity, Impl, OFFSET>,
            GetFileSysPath: GetFileSysPath::<Identity, Impl, OFFSET>,
            GetRegistryKey: GetRegistryKey::<Identity, Impl, OFFSET>,
            GetOptions: GetOptions::<Identity, Impl, OFFSET>,
            SetOptions: SetOptions::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetMachineName: GetMachineName::<Identity, Impl, OFFSET>,
            GetPropertySheetPages: GetPropertySheetPages::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGroupPolicyObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRSOPInformation_Impl: Sized {
    fn GetNamespace(&mut self, dwsection: u32, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::Result<()>;
    fn GetFlags(&mut self, pdwflags: *mut u32) -> ::windows::core::Result<()>;
    fn GetEventLogEntryText(&mut self, pszeventsource: super::super::Foundation::PWSTR, pszeventlogname: super::super::Foundation::PWSTR, pszeventtime: super::super::Foundation::PWSTR, dweventid: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRSOPInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRSOPInformation_Impl, const OFFSET: isize>() -> IRSOPInformation_Vtbl {
        unsafe extern "system" fn GetNamespace<Identity: ::windows::core::IUnknownImpl, Impl: IRSOPInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNamespace(::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows::core::IUnknownImpl, Impl: IRSOPInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFlags(::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn GetEventLogEntryText<Identity: ::windows::core::IUnknownImpl, Impl: IRSOPInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszeventsource: super::super::Foundation::PWSTR, pszeventlogname: super::super::Foundation::PWSTR, pszeventtime: super::super::Foundation::PWSTR, dweventid: u32, ppsztext: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEventLogEntryText(::core::mem::transmute_copy(&pszeventsource), ::core::mem::transmute_copy(&pszeventlogname), ::core::mem::transmute_copy(&pszeventtime), ::core::mem::transmute_copy(&dweventid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsztext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetNamespace: GetNamespace::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            GetEventLogEntryText: GetEventLogEntryText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRSOPInformation as ::windows::core::Interface>::IID
    }
}
